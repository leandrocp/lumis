import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./test/browser",
  testMatch: "*.spec.ts",
  testIgnore: "stress.spec.ts",
  timeout: 30_000,
  expect: {
    timeout: 10_000,
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
  projects: [
    // Use full Chromium's headless mode. The standalone Linux headless shell
    // crashes on Wasm out-of-bounds accesses instead of reporting RuntimeError.
    { name: "chromium", use: { browserName: "chromium", channel: "chromium" } },
    { name: "firefox", use: { browserName: "firefox" } },
    { name: "webkit", use: { browserName: "webkit" } },
  ],
});
