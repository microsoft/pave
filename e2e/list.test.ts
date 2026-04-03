import { test, expect } from "@microsoft/tui-test";
import os from "node:os";
import fs from "node:fs";
import path from "node:path";
import { paveBin, isWindows, pathSep, testEnv } from "./helpers.js";

const dir1 = fs.mkdtempSync(path.join(os.tmpdir(), "pave-list-a-"));
const dir2 = fs.mkdtempSync(path.join(os.tmpdir(), "pave-list-b-"));

test.use({
  program: { file: paveBin, args: ["list"] },
  env: testEnv({ PATH: [dir1, dir2].join(pathSep) }),
});

test("lists the exact directories provided in PATH", async ({ terminal }) => {
  await expect(
    terminal.getByText(path.basename(dir1), { full: true, strict: false })
  ).toBeVisible();
  await expect(
    terminal.getByText(path.basename(dir2), { full: true, strict: false })
  ).toBeVisible();
});
