import { test, expect } from "@microsoft/tui-test";
import os from "node:os";
import fs from "node:fs";
import path from "node:path";
import { execSync } from "node:child_process";
import { paveBin, isWindows, testEnv } from "./helpers.js";

test.describe("remove a managed path", () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "pave-test-rm-"));

  test.beforeAll(() => {
    execSync(`"${paveBin}" add "${tmpDir}"`, { stdio: "ignore" });
  });

  test.use({ program: { file: paveBin, args: ["remove", tmpDir] } });

  test.afterAll(() => {
    try {
      execSync(`"${paveBin}" remove "${tmpDir}"`, { stdio: "ignore" });
    } catch {}
  });

  test("removes directory from PATH", async ({ terminal }) => {
    await expect(
      terminal.getByText(/Removed from PATH|Removed/g, { full: true })
    ).toBeVisible();
  });
});

test.describe("remove nonexistent path", () => {
  const fakePath = isWindows
    ? "C:\\nonexistent_pave_test_dir_xyz"
    : "/nonexistent_pave_test_dir_xyz";

  test.use({
    program: { file: paveBin, args: ["remove", fakePath] },
    env: testEnv({ PATH: os.tmpdir() }),
  });

  test("shows error for path not on PATH", async ({ terminal }) => {
    await expect(
      terminal.getByText("is not on PATH", { full: true })
    ).toBeVisible();
  });
});

test.describe("remove interactive mode", () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "pave-test-rmi-"));

  test.beforeAll(() => {
    execSync(`"${paveBin}" add "${tmpDir}"`, { stdio: "ignore" });
  });

  test.use({ program: { file: paveBin, args: ["remove"] } });

  test.afterAll(() => {
    try {
      execSync(`"${paveBin}" remove "${tmpDir}"`, { stdio: "ignore" });
    } catch {}
  });

  test("shows interactive picker with managed path", async ({ terminal }) => {
    await expect(
      terminal.getByText("Select an entry to remove from PATH", { full: true })
    ).toBeVisible();
    terminal.keyEscape();
  });
});
