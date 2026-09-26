import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { buildCss } from "../../packages/javascript/themes/dist/index.js";

const root = fileURLToPath(new URL("../../", import.meta.url));
const staging = join(root, "target/html-line-qa");
const output = join(root, "qa/html-lines/generated");
const require = createRequire(
  new URL("../../packages/javascript/lumis/package.json", import.meta.url),
);
const { chromium, firefox, webkit } = require("@playwright/test");
const { createServer } = await import(pathToFileURL(require.resolve("vite")).href);
const manifest = JSON.parse(await readFile(new URL("./cases.json", import.meta.url), "utf8"));
const theme = JSON.parse(await readFile(join(root, `themes/${manifest.theme}.json`), "utf8"));
const qaFont = await readFile(new URL("./fonts/RobotoMono.ttf", import.meta.url));
const expectedCount =
  manifest.cases.length * manifest.formatters.length * manifest.lineNumbers.length;
const runtimeNames = ["rust", "cli", "elixir", "node-native", "node-wasm", "browser-wasm"];
const report = {
  implementationCommit: execFileSync("git", ["rev-parse", "HEAD"], {
    cwd: root,
    encoding: "utf8",
  }).trim(),
  platform: `${process.platform}/${process.arch}`,
  node: process.version,
  playwright: require("@playwright/test/package.json").version,
  font: { family: "Roboto Mono", file: "../fonts/RobotoMono.ttf", sha256: hash(qaFont) },
  viewport: { width: 1140, height: 900 },
  deviceScaleFactor: 1,
  comparison: "Exact PNG bytes, no threshold, masks, resizing or tolerated differences",
  cases: manifest.cases.map(({ id, source }) => ({ id, sourceSha256: hash(source) })),
  variantsPerRuntime: expectedCount,
  excluded: [
    {
      runtime: "Java",
      reason:
        "lumis4j is external to this repository, pins lumis 0.9.0, and has no multi-theme formatter or line-number/highlight options. No current-PR Java binary is available.",
    },
  ],
  html: {},
  browsers: {},
};

function hash(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function key(sample, formatter, numbered) {
  return `${sample.id}/${formatter}/${numbered ? "numbered" : "plain"}`;
}

async function saveJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function renderCli() {
  const metadata = JSON.parse(
    execFileSync("cargo", ["metadata", "--no-deps", "--format-version", "1"], {
      cwd: root,
      encoding: "utf8",
    }),
  );
  const binary = join(
    metadata.target_directory,
    "debug",
    process.platform === "win32" ? "lumis.exe" : "lumis",
  );
  const html = {};
  for (const sample of manifest.cases) {
    for (const formatter of manifest.formatters) {
      for (const numbered of manifest.lineNumbers) {
        const args = [
          "highlight",
          "-l",
          manifest.language,
          "-f",
          formatter,
          "-H",
          manifest.highlightLines.join(","),
          "--highlight-lines-class",
          "l-highlighted",
        ];
        if (numbered) args.push("--line-numbers");
        if (formatter !== "html-linked") args.push("--italic");
        if (formatter === "html-inline") args.push("--theme", manifest.theme);
        if (formatter === "html-multi-themes")
          args.push("--themes", `dark:${manifest.theme}`, "--default-theme", "dark");
        html[key(sample, formatter, numbered)] = execFileSync(binary, args, {
          cwd: root,
          input: sample.source,
          encoding: "utf8",
          env: { ...process.env, LUMIS_CONFIG: join(staging, "empty.toml") },
        });
      }
    }
  }
  return html;
}

async function recordHtml(runtime, html, reference) {
  assert.equal(Object.keys(html).length, expectedCount, `${runtime} corpus size`);
  assert.deepEqual(html, reference, `${runtime} differs from Rust HTML`);
  const sorted = Object.fromEntries(Object.entries(html).sort(([a], [b]) => a.localeCompare(b)));
  const file = `html/${runtime}.json`;
  await saveJson(join(output, file), sorted);
  report.html[runtime] = {
    file,
    sha256: hash(await readFile(join(output, file))),
    variants: expectedCount,
    matchesRust: true,
  };
}

function documentHtml(html) {
  const panels = manifest.cases
    .flatMap((sample) =>
      manifest.formatters.map((formatter) => {
        const sections = manifest.lineNumbers
          .map(
            (numbered) =>
              `<section data-numbered="${numbered}"><h4>${numbered ? "Line numbers" : "No gutter"}</h4>${html[key(sample, formatter, numbered)]}</section>`,
          )
          .join("");
        return `<article class="${formatter === "html-linked" ? "linked" : ""}" data-case="${sample.id}" data-formatter="${formatter}"><h3>${sample.id} · ${formatter}</h3>${sections}</article>`;
      }),
    )
    .join("");
  return `<!doctype html><html lang="en"><head><meta charset="utf-8"><style>
    @font-face { font-family: "Lumis QA Mono"; src: url("data:font/ttf;base64,${qaFont.toString("base64")}") format("truetype"); font-weight: 100 700; font-style: normal; font-display: block; }
    * { box-sizing: border-box; }
    body { margin: 0; padding: 20px; background: #f4f5f7; color: #172030; font: 13px/1.4 system-ui; }
    h1 { font-size: 22px; margin: 0 0 4px; } p { margin: 0 0 16px; }
    main { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 12px; }
    article { min-width: 0; background: white; border: 1px solid #d5d9e0; padding: 10px; border-radius: 6px; }
    h3 { font-size: 12px; margin: 0 0 8px; } h4 { font-size: 11px; font-weight: 500; margin: 6px 0 3px; color: #526074; }
    pre { font: 14px/20px "Lumis QA Mono", monospace; margin: 0; overflow-x: auto; }
    code { font: inherit; }
    .l-line-number { padding: 0 1ch; }
    ${buildCss(theme, { scope: ".linked" })}
  </style></head><body><h1>Lumis HTML line QA</h1><p>Same source and theme · highlighted rows 1–2 · all three formatters · no runtime-specific CSS overrides</p><main>${panels}</main></body></html>`;
}

async function inspectLayout(page) {
  return page.locator("pre > code").evaluateAll((codes) =>
    codes.map((code) => {
      const panel = code.closest("article");
      const rows = [...code.querySelectorAll(":scope > .l-line")];
      return {
        id: panel.dataset.case,
        formatter: panel.dataset.formatter,
        numbered: code.closest("section").dataset.numbered === "true",
        text: code.textContent,
        height: code.getBoundingClientRect().height,
        width: code.getBoundingClientRect().width,
        font: getComputedStyle(code).font,
        scrollWidth: code.parentElement.scrollWidth,
        rows: rows.map((row) => ({
          tag: row.tagName,
          number: row.dataset.line,
          text: row.textContent,
          height: row.getBoundingClientRect().height,
          width: row.getBoundingClientRect().width,
          font: getComputedStyle(row).font,
          gutterFont: row.firstElementChild?.classList.contains("l-line-number")
            ? getComputedStyle(row.firstElementChild).font
            : null,
        })),
      };
    }),
  );
}

function validateLayout(panels) {
  assert.equal(panels.length, expectedCount);
  for (const panel of panels) {
    const sample = manifest.cases.find(({ id }) => id === panel.id);
    const lines = sample.source.replaceAll("\r\n", "\n").split("\n");
    if (sample.source.endsWith("\n")) lines.pop();
    assert.equal(panel.rows.length, lines.length, `${panel.id} row count`);
    assert.equal(panel.height, lines.length * 20, `row heights: ${JSON.stringify(panel)}`);
    const text = lines.map((line, index) => `${panel.numbered ? index + 1 : ""}${line}`);
    assert.equal(panel.text, text.join("\n"), `${panel.id} text`);
    panel.rows.forEach((row, index) => {
      assert.equal(row.tag, "SPAN");
      assert.equal(row.number, String(index + 1));
      assert.equal(row.text, text[index]);
      if (manifest.highlightLines.includes(index + 1)) {
        assert.equal(row.height, 20, `${panel.id} highlighted/empty height`);
        assert.equal(row.width, panel.width, `${panel.id} highlight width`);
        assert.ok(row.width >= panel.scrollWidth - 1, `${panel.id} highlight covers scroll width`);
      }
    });
  }
}

async function browserHtml(page) {
  const languagePackage = JSON.parse(await readFile(join(staging, "language.json"), "utf8"));
  return page.evaluate(
    async ({ repository, languagePackage: browserPackage, theme: browserTheme }) => {
      const api = await import("/src/index.browser.ts");
      const { default: rust } = await import("/langs/rust.ts");
      const { renderCases } = await import(`/@fs/${repository}/qa/html-lines/render.mjs`);
      const language = api.withWasm(
        rust,
        `/@fs/${repository}/fixtures/test-parsers/tree-sitter-rust.wasm`,
      );
      const metadata = URL.createObjectURL(
        new Blob([JSON.stringify(browserPackage)], { type: "application/json" }),
      );
      const highlighter = await api.createHighlighter({
        languages: [language],
        languagePackageResolver: () => metadata,
      });
      if (api.runtimeKind() !== "wasm") throw new Error("Browser did not use Wasm");
      return renderCases(highlighter, language, browserTheme);
    },
    { repository: root, languagePackage, theme },
  );
}

function compare(actual, reference) {
  assert.ok(actual.equals(reference), "Screenshot differs: zero-byte/zero-pixel tolerance");
}

async function screenshot(page, browser, runtime, scroll) {
  await page.locator("pre").evaluateAll((pres, end) => {
    for (const pre of pres) pre.scrollLeft = end ? pre.scrollWidth : 0;
  }, scroll === "right");
  const positions = await page
    .locator('[data-case="horizontal-scroll"] pre')
    .evaluateAll((pres) => pres.map((pre) => pre.scrollLeft));
  assert.equal(positions.length, 6);
  for (const position of positions) {
    assert.equal(position > 0, scroll === "right", "horizontal scroll was applied");
  }
  const file = `${browser}/${runtime}-${scroll}.png`;
  await mkdir(join(output, browser), { recursive: true });
  const bytes = await page.screenshot({
    path: join(output, file),
    fullPage: true,
    animations: "disabled",
    scale: "css",
  });
  return { bytes, file, sha256: hash(bytes) };
}

async function captureBrowser(name, engine, serverUrl, generated) {
  const browser = await engine.launch();
  try {
    const page = await browser.newPage({
      viewport: report.viewport,
      deviceScaleFactor: report.deviceScaleFactor,
      colorScheme: "dark",
      reducedMotion: "reduce",
    });
    await page.goto(`${serverUrl}/qa`);
    const fromBrowser = await browserHtml(page);
    await recordHtml(`browser-wasm-${name}`, fromBrowser, generated.rust);
    const datasets = { ...generated, "browser-wasm": fromBrowser };
    const result = { version: browser.version(), screenshots: [], negativeControlRejected: false };
    const references = {};
    for (const runtime of runtimeNames) {
      await page.setContent(documentHtml(datasets[runtime]));
      await page.evaluate(async () => {
        const loaded = await Promise.all([
          document.fonts.load('14px "Lumis QA Mono"'),
          document.fonts.load('700 14px "Lumis QA Mono"'),
        ]);
        if (loaded.some((faces) => faces.length !== 1)) throw new Error("QA font did not load");
        await document.fonts.ready;
        await new Promise((resolve) => {
          requestAnimationFrame(() => requestAnimationFrame(resolve));
        });
      });
      validateLayout(await inspectLayout(page));
      for (const scroll of ["left", "right"]) {
        const shot = await screenshot(page, name, runtime, scroll);
        if (runtime === "rust") references[scroll] = shot.bytes;
        compare(shot.bytes, references[scroll]);
        result.screenshots.push({
          runtime,
          scroll,
          file: shot.file,
          sha256: shot.sha256,
          differentPixels: 0,
          identicalPngBytes: true,
        });
      }
    }
    await page.locator("pre").evaluateAll((pres) => {
      for (const pre of pres) pre.scrollLeft = 0;
    });
    await page.evaluate(() => {
      const pixel = document.createElement("div");
      pixel.style.cssText =
        "position:fixed;left:0;top:0;width:1px;height:1px;background:#ff00ff;z-index:9999";
      document.body.append(pixel);
    });
    const control = await screenshot(page, name, "negative-control-1px", "left");
    assert.throws(() => compare(control.bytes, references.left));
    result.negativeControl = { file: control.file, sha256: control.sha256 };
    result.negativeControlRejected = true;
    report.browsers[name] = result;
    console.log(
      `${name}: six runtimes × two scroll positions matched exactly; 1px negative control rejected`,
    );
  } finally {
    await browser.close();
  }
}

await saveJson(join(output, "report.json"), { ...report, status: "running" });
await mkdir(staging, { recursive: true });
await writeFile(join(staging, "empty.toml"), "");
const generated = { cli: renderCli() };
await saveJson(join(staging, "cli.json"), generated.cli);
for (const runtime of ["rust", "elixir", "node-native", "node-wasm"]) {
  generated[runtime] = JSON.parse(await readFile(join(staging, `${runtime}.json`), "utf8"));
}
assert.equal(manifest.cases.length, 9);
assert.equal(expectedCount, 54);
for (const [runtime, html] of Object.entries(generated))
  await recordHtml(runtime, html, generated.rust);

const server = await createServer({
  root: join(root, "packages/javascript/lumis"),
  configFile: join(root, "packages/javascript/lumis/vite.config.mjs"),
  optimizeDeps: {
    entries: [
      join(root, "packages/javascript/lumis/src/index.browser.ts"),
      join(root, "qa/html-lines/render.mjs"),
    ],
  },
  logLevel: "warn",
  server: { host: "127.0.0.1", port: 0, fs: { allow: [root] }, watch: null },
  plugins: [
    {
      name: "line-qa",
      configureServer(vite) {
        vite.middlewares.use("/qa", (_request, response) => {
          response.setHeader("Content-Type", "text/html");
          response.end("<!doctype html><html><head></head><body></body></html>");
        });
      },
    },
  ],
});
try {
  await server.listen();
  const url = server.resolvedUrls.local[0].replace(/\/$/, "");
  for (const [name, engine] of Object.entries({ chromium, firefox, webkit })) {
    await captureBrowser(name, engine, url, generated);
  }
  report.status = "pass";
  report.actualHtmlRenders = Object.keys(report.html).length * expectedCount;
  report.screenshotComparisons = 3 * (runtimeNames.length - 1) * 2;
  await saveJson(join(output, "report.json"), report);
  console.log(
    `PASS: ${report.actualHtmlRenders} real HTML renders, 36 screenshots, ${report.screenshotComparisons} exact screenshot comparisons`,
  );
} catch (error) {
  report.status = "failed";
  report.failure = String(error);
  await saveJson(join(output, "report.json"), report);
  throw error;
} finally {
  await server.close();
}
