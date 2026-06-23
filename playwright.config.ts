import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./e2e",
  timeout: 30_000,
  retries: 1,
  use: {
    baseURL: "http://localhost:8080",
    screenshot: "only-on-failure",
  },
  projects: [
    { name: "chromium", use: { browserName: "chromium" } },
  ],
  webServer: {
    command: "../../tools/install-dioxus-cli.sh && dx serve --port 8080",
    cwd: "examples/showcase",
    url: "http://localhost:8080",
    timeout: 180_000,
    reuseExistingServer: true,
    stdout: "pipe",
  },
});
