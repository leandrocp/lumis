import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { on, once } from "node:events";
import { createRequire } from "node:module";
import { after, before, test } from "node:test";

const require = createRequire(import.meta.url);
const nextVersion = require("next/package.json").version;
const varyHeaderTodo = nextVersion === "16.3.8" && "https://github.com/vercel/next.js/issues/85999";
let server;
const address = { origin: undefined };

before(async () => {
  server = spawn(
    process.execPath,
    [require.resolve("next/dist/bin/next"), "start", "--hostname", "127.0.0.1", "--port", "0"],
    { cwd: new URL("../", import.meta.url), stdio: ["ignore", "pipe", "inherit"] },
  );

  let output = "";
  for await (const [chunk] of on(server.stdout, "data", { signal: AbortSignal.timeout(30_000) })) {
    output += chunk.toString();
    const match = output.match(/http:\/\/127\.0\.0\.1:\d+/u);
    if (match && output.includes("Ready")) {
      address.origin = match[0];
      break;
    }
  }
  assert.ok(address.origin, "next start must report a listening address");
});

after(async () => {
  if (server && server.exitCode === null && server.signalCode === null) {
    const exited = once(server, "exit");
    server.kill();
    await exited;
  }
});

function checkVary(response, expected) {
  const fields = (response.headers.get("vary") ?? "")
    .split(",")
    .map((field) => field.trim().toLowerCase());
  for (const field of expected) {
    assert.ok(fields.includes(field), `Vary must include ${field}: ${fields.join(", ")}`);
  }
  assert.equal(fields.length, new Set(fields).size, "Vary must not repeat fields");
}

function bodyPatternFor(method, format, status) {
  if (method === "HEAD") return /^$/u;
  if (status !== 200) return /./su;
  return format === "html" ? /<h1\b[^>]*>Installation/u : /^# Installation/mu;
}

const routerFields = [
  "rsc",
  "next-router-state-tree",
  "next-router-prefetch",
  "next-router-segment-prefetch",
];

for (const method of ["GET", "HEAD"]) {
  for (const format of ["html", "markdown"]) {
    for (const [path, status] of [
      ["/installation", 200],
      ["/agent-readability-missing-page", 404],
    ]) {
      const fields = format === "html" ? routerFields : ["accept"];
      const bodyPattern = bodyPatternFor(method, format, status);
      test(`${method} ${path} as ${format}`, async () => {
        const response = await fetch(new URL(path, address.origin), {
          method,
          headers: { Accept: `text/${format}` },
        });
        assert.equal(response.status, status);
        assert.equal(response.headers.get("content-type")?.split(";")[0], `text/${format}`);
        checkVary(response, fields);
        assert.match(await response.text(), bodyPattern);
      });
    }
  }
}

for (const [accept, format] of [
  ["text/html;q=1, text/markdown;q=0.5", "html"],
  ["text/html;q=0.5, text/markdown;q=1", "markdown"],
  ["*/*", "html"],
]) {
  const fields = format === "html" ? routerFields : ["accept"];
  test(`negotiates ${accept}`, async () => {
    const response = await fetch(new URL("/installation", address.origin), {
      headers: { Accept: accept },
    });
    assert.equal(response.status, 200);
    assert.equal(response.headers.get("content-type")?.split(";")[0], `text/${format}`);
    checkVary(response, fields);
    await response.body.cancel();
  });
}

test("RSC responses preserve the router's cache fields", async () => {
  const response = await fetch(new URL("/installation", address.origin), { headers: { RSC: "1" } });
  assert.equal(response.status, 200);
  assert.equal(response.headers.get("content-type")?.split(";")[0], "text/x-component");
  checkVary(response, routerFields);
  await response.body.cancel();
});

for (const [format, headers] of [
  ["HTML", { Accept: "text/html" }],
  ["RSC", { RSC: "1" }],
]) {
  test(`${format} responses preserve Vary: Accept`, { todo: varyHeaderTodo }, async () => {
    const response = await fetch(new URL("/installation", address.origin), { headers });
    await response.body.cancel();
    assert.equal(response.status, 200);
    checkVary(response, ["accept", ...routerFields]);
  });
}

test("explicit Markdown links remain readable", async () => {
  const response = await fetch(new URL("/installation.md", address.origin));
  assert.equal(response.status, 200);
  assert.equal(response.headers.get("content-type")?.split(";")[0], "text/markdown");
  assert.match(await response.text(), /^# Installation/mu);
});

test("Markdown routes serve a page only at its content.md path", async () => {
  const response = await fetch(new URL("/llms.mdx/installation/other.md", address.origin));
  assert.equal(response.status, 404);
  assert.equal(response.headers.get("content-type")?.split(";")[0], "text/markdown");
  assert.match(await response.text(), /^# Page not found/u);
});
