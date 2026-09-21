import { createHash } from "node:crypto";
import { chmodSync, mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, join } from "node:path";
import { spawnSync } from "node:child_process";

const websiteDir = new URL("..", import.meta.url).pathname;
const installer = join(websiteDir, "public", "install.sh");
const tempDir = mkdtempSync(join(tmpdir(), "lumis-installer-test-"));

function run(command, args, options = {}) {
  const result = spawnSync(command, args, { encoding: "utf8", ...options });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(" ")} failed\n${result.stdout}${result.stderr}`);
  }
  return result;
}

try {
  run("sh", ["-n", installer]);

  const payloadDir = join(tempDir, "payload");
  const fixtureArchive = join(tempDir, "lumis-x86_64-unknown-linux-musl.tar.gz");
  const fixtureChecksum = join(tempDir, "lumis-x86_64-unknown-linux-musl.sha256");
  const fakeBin = join(tempDir, "bin");
  const installDir = join(tempDir, "install");
  const curlLog = join(tempDir, "curl.log");
  mkdirSync(payloadDir);
  mkdirSync(fakeBin);

  const binary = join(payloadDir, "lumis");
  writeFileSync(binary, "#!/bin/sh\nprintf 'lumis fixture\\n'\n");
  chmodSync(binary, 0o755);
  run("tar", ["-czf", fixtureArchive, "-C", payloadDir, "lumis"]);

  const digest = createHash("sha256").update(readFileSync(fixtureArchive)).digest("hex");
  writeFileSync(fixtureChecksum, `${digest}  ${basename(fixtureArchive)}\n`);

  const fakeUname = join(fakeBin, "uname");
  writeFileSync(
    fakeUname,
    '#!/bin/sh\ncase "$1" in\n  -m) echo x86_64 ;;\n  -s) echo Linux ;;\nesac\n',
  );
  chmodSync(fakeUname, 0o755);

  const fakeLdd = join(fakeBin, "ldd");
  writeFileSync(fakeLdd, "#!/bin/sh\necho 'musl libc'\n");
  chmodSync(fakeLdd, 0o755);

  const fakeCurl = join(fakeBin, "curl");
  writeFileSync(
    fakeCurl,
    `#!/bin/sh
output=
previous=
url=
for argument in "$@"; do
  if [ "$previous" = "-o" ]; then
    output="$argument"
    previous=
    continue
  fi
  case "$argument" in
    -o) previous=-o ;;
    http*) url="$argument" ;;
  esac
done
printf '%s\\n' "$url" >> "$CURL_LOG"
case "$url" in
  *.tar.gz) cp "$FIXTURE_ARCHIVE" "$output" ;;
  *.sha256) cp "$FIXTURE_CHECKSUM" "$output" ;;
  *) exit 1 ;;
esac
`,
  );
  chmodSync(fakeCurl, 0o755);

  run("sh", [installer], {
    env: {
      ...process.env,
      CURL_LOG: curlLog,
      FIXTURE_ARCHIVE: fixtureArchive,
      FIXTURE_CHECKSUM: fixtureChecksum,
      HOME: tempDir,
      LUMIS_INSTALL_DIR: installDir,
      LUMIS_NO_MODIFY_PATH: "1",
      LUMIS_VERSION: "9.9.9",
      PATH: `${fakeBin}:${process.env.PATH}`,
      SHELL: "/bin/sh",
    },
  });

  const installed = join(installDir, "lumis");
  const output = run(installed, []).stdout;
  if (output !== "lumis fixture\n") {
    throw new Error(`Installed fixture produced unexpected output: ${output}`);
  }

  const downloads = readFileSync(curlLog, "utf8");
  if (!downloads.includes("lumis-x86_64-unknown-linux-musl.tar.gz")) {
    throw new Error("The shell installer did not select the musl release asset");
  }

  const vercel = JSON.parse(readFileSync(join(websiteDir, "vercel.json"), "utf8"));
  const sources = new Set(vercel.rewrites?.map((rewrite) => rewrite.source));
  if (!sources.has("/:version/install.sh") || !sources.has("/:version/install.ps1")) {
    throw new Error("Versioned installer rewrites are missing");
  }
  const plainTextInstallers = new Set(
    vercel.headers
      ?.filter((entry) =>
        entry.headers?.some(
          (header) => header.key === "Content-Type" && header.value.startsWith("text/plain"),
        ),
      )
      .map((entry) => entry.source),
  );
  if (!plainTextInstallers.has("/install.sh") || !plainTextInstallers.has("/install.ps1")) {
    throw new Error("Static installers must be served as plain text");
  }

  const { GET } = await import("../api/versioned-installer.js");
  const originalFetch = globalThis.fetch;
  globalThis.fetch = async (url) => {
    const format = new URL(url).pathname.endsWith(".ps1") ? "ps1" : "sh";
    return new Response(readFileSync(join(websiteDir, "public", `install.${format}`), "utf8"));
  };
  try {
    const shellResponse = await GET(
      new Request("https://lumis.sh/api/versioned-installer?format=sh&version=1.2.3"),
    );
    const shell = await shellResponse.text();
    if (!shell.startsWith("#!/bin/sh\nLUMIS_VERSION='1.2.3'\nexport LUMIS_VERSION\n")) {
      throw new Error("Versioned shell installer did not preserve its shebang and pin the version");
    }

    const powershellResponse = await GET(
      new Request("https://lumis.sh/api/versioned-installer?format=ps1&version=1.2.3"),
    );
    const powershell = await powershellResponse.text();
    if (!powershell.startsWith("$env:LUMIS_VERSION = '1.2.3'\n")) {
      throw new Error("Versioned PowerShell installer did not pin the version");
    }

    const invalidResponse = await GET(
      new Request("https://lumis.sh/api/versioned-installer?format=sh&version=../main"),
    );
    if (invalidResponse.status !== 400) {
      throw new Error("Versioned installer accepted an invalid version");
    }
  } finally {
    globalThis.fetch = originalFetch;
  }
} finally {
  rmSync(tempDir, { recursive: true, force: true });
}
