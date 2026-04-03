import { defineConfig } from "@microsoft/tui-test";

export default defineConfig({
  retries: 3,
  trace: true,
  testMatch: "**/*.test.ts",
});
