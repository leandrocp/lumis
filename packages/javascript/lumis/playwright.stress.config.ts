import { defineConfig } from "@playwright/test";

// The stress corpus is minutes per case and needs `stress-test/run.mjs`
// to stage a corpus first, so it runs from its own config rather than joining the
// conformance suite, which `playwright.config.ts` keeps at a 30 s timeout.
export default defineConfig({
  testDir: "./test/browser",
  testMatch: "stress.spec.ts",
  timeout: 45 * 60 * 1_000,
  expect: {
    timeout: 60_000,
  },
  reporter: process.env.CI ? "github" : "list",
  use: {
    baseURL: "http://127.0.0.1:4173",
    trace: "retain-on-failure",
  },
  webServer: {
    command: "./node_modules/.bin/vite test/browser --host 127.0.0.1 --port 4173 --strictPort",
    url: "http://127.0.0.1:4173/",
    reuseExistingServer: !process.env.CI,
  },
  projects: [{ name: "chromium", use: { browserName: "chromium" } }],
});
