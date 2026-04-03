import { test, expect } from "@microsoft/tui-test";
import os from "node:os";
import fs from "node:fs";
import path from "node:path";
import { paveBin, isWindows, testEnv } from "./helpers.js";

const envDir1 = fs.mkdtempSync(path.join(os.tmpdir(), "pave-env-a-"));
const envDir2 = fs.mkdtempSync(path.join(os.tmpdir(), "pave-env-b-"));
const unixTestPath = `${envDir1}:${envDir2}`;

test.describe("env bash", () => {
  test.use({
    program: { file: paveBin, args: ["env", "bash"] },
    ...(!isWindows && { env: testEnv({ PATH: unixTestPath }) }),
  });

  test("outputs colon-separated PATH value", async ({ terminal }) => {
    // Should output raw PATH value (colon-separated), not a shell command
    const probe = isWindows ? "/" : path.basename(envDir1);
    await expect(
      terminal.getByText(probe, { full: true, strict: false })
    ).toBeVisible();
  });

  test.when(!isWindows, "includes explicit PATH dirs", async ({ terminal }) => {
    await expect(
      terminal.getByText(path.basename(envDir1), { full: true, strict: false })
    ).toBeVisible();
    await expect(
      terminal.getByText(path.basename(envDir2), { full: true, strict: false })
    ).toBeVisible();
  });
});

test.describe("env zsh", () => {
  test.use({
    program: { file: paveBin, args: ["env", "zsh"] },
    ...(!isWindows && { env: testEnv({ PATH: unixTestPath }) }),
  });

  test("outputs colon-separated PATH value", async ({ terminal }) => {
    const probe = isWindows ? "/" : path.basename(envDir1);
    await expect(
      terminal.getByText(probe, { full: true, strict: false })
    ).toBeVisible();
  });
});

test.describe("env fish", () => {
  test.use({
    program: { file: paveBin, args: ["env", "fish"] },
    ...(!isWindows && { env: testEnv({ PATH: unixTestPath }) }),
  });

  test("outputs one path per line", async ({ terminal }) => {
    const probe = isWindows ? "system32" : path.basename(envDir1);
    await expect(
      terminal.getByText(probe, { full: true, strict: false })
    ).toBeVisible();
  });

  test.when(!isWindows, "includes explicit PATH dirs", async ({ terminal }) => {
    await expect(
      terminal.getByText(envDir1, { full: true, strict: false })
    ).toBeVisible();
  });
});

test.describe("env pwsh", () => {
  test.use({
    program: { file: paveBin, args: ["env", "pwsh"] },
    ...(!isWindows && { env: testEnv({ PATH: unixTestPath }) }),
  });

  test("outputs separator-joined PATH value", async ({ terminal }) => {
    const probe = isWindows ? "system32" : path.basename(envDir1);
    await expect(
      terminal.getByText(probe, { full: true, strict: false })
    ).toBeVisible();
  });
});

test.describe("env xonsh", () => {
  test.use({
    program: { file: paveBin, args: ["env", "xonsh"] },
    ...(!isWindows && { env: testEnv({ PATH: unixTestPath }) }),
  });

  test("outputs one path per line", async ({ terminal }) => {
    const probe = isWindows ? "system32" : path.basename(envDir1);
    await expect(
      terminal.getByText(probe, { full: true, strict: false })
    ).toBeVisible();
  });

  test.when(!isWindows, "includes explicit PATH dirs", async ({ terminal }) => {
    await expect(
      terminal.getByText(envDir1, { full: true, strict: false })
    ).toBeVisible();
  });
});

test.describe("env nushell", () => {
  test.use({
    program: { file: paveBin, args: ["env", "nushell"] },
    ...(!isWindows && { env: testEnv({ PATH: unixTestPath }) }),
  });

  test("outputs one path per line", async ({ terminal }) => {
    const probe = isWindows ? "system32" : path.basename(envDir1);
    await expect(
      terminal.getByText(probe, { full: true, strict: false })
    ).toBeVisible();
  });

  test.when(!isWindows, "includes explicit PATH dirs", async ({ terminal }) => {
    await expect(
      terminal.getByText(envDir1, { full: true, strict: false })
    ).toBeVisible();
    await expect(
      terminal.getByText(envDir2, { full: true, strict: false })
    ).toBeVisible();
  });
});
