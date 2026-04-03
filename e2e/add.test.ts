import { test, expect } from "@microsoft/tui-test";
import os from "node:os";
import fs from "node:fs";
import path from "node:path";
import { execSync } from "node:child_process";
import {
  paveBin,
  testEnv,
  createFakeExecutable,
} from "./helpers.js";

test.describe("add a directory path", () => {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "pave-test-add-"));

  test.use({ program: { file: paveBin, args: ["add", tmpDir] } });

  test.afterAll(() => {
    try {
      execSync(`"${paveBin}" remove "${tmpDir}"`, { stdio: "ignore" });
    } catch {}
  });

  test("adds directory to PATH", async ({ terminal }) => {
    await expect(
      terminal.getByText("Added to PATH", { full: true })
    ).toBeVisible();
  });
});

test.describe("add executable already on PATH", () => {
  const { dir } = createFakeExecutable("myfakecli");

  test.use({
    program: { file: paveBin, args: ["add", "myfakecli"] },
    env: testEnv({ PATH: dir }),
  });

  test("shows already available message", async ({ terminal }) => {
    await expect(
      terminal.getByText("already available on PATH", { full: true })
    ).toBeVisible();
  });
});
