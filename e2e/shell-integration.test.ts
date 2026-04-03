import { test, expect, Shell } from "@microsoft/tui-test";
import os from "node:os";
import fs from "node:fs";
import path from "node:path";
import {
  paveBin,
  isWindows,
  isMacOS,
  createFakeExecutable,
} from "./helpers.js";

test.describe("bash integration", () => {
  test.use({ shell: Shell.Bash });

  test("list with controlled PATH shows expected dirs", async ({ terminal }) => {
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), "pave-bash-list-"));
    terminal.submit(`export PATH="${dir}"`);
    terminal.submit(`"${paveBin}" list`);
    await expect(
      terminal.getByText(path.basename(dir), { strict: false })
    ).toBeVisible();
  });

  test("help shows usage", async ({ terminal }) => {
    terminal.submit(`"${paveBin}" --help`);
    await expect(
      terminal.getByText("A cross-platform CLI tool for managing the PATH")
    ).toBeVisible();
  });

  test("env bash outputs PATH value", async ({ terminal }) => {
    terminal.submit(`"${paveBin}" env bash`);
    await expect(
      terminal.getByText("/", { full: true, strict: false })
    ).toBeVisible();
  });

  test("search with controlled PATH finds executable", async ({ terminal }) => {
    const { dir } = createFakeExecutable("bashtestprog");
    terminal.submit(`export PATH="${dir}"`);
    terminal.submit(`"${paveBin}" search bashtestprog`);
    await expect(
      terminal.getByText("bashtestprog", { strict: false })
    ).toBeVisible();
  });

  test("add and remove workflow with env merge verification", async ({ terminal }) => {
    const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "pave-e2e-bash-"));

    terminal.submit(`"${paveBin}" add "${tmpDir}"`);
    await expect(terminal.getByText("Added to PATH")).toBeVisible();

    terminal.submit(`"${paveBin}" env bash`);
    await expect(
      terminal.getByText(path.basename(tmpDir), { full: true, strict: false })
    ).toBeVisible();

    terminal.submit(`"${paveBin}" remove "${tmpDir}"`);
    await expect(terminal.getByText(/Removed from PATH|Removed/g)).toBeVisible();

    fs.rmSync(tmpDir, { recursive: true, force: true });
  });
});

test.when(!isMacOS, "pwsh integration", () => {
  test.describe("pwsh integration", () => {
    test.use({ shell: Shell.Powershell });

    test("list with controlled PATH shows expected dirs", async ({ terminal }) => {
      const dir = fs.mkdtempSync(path.join(os.tmpdir(), "pave-pwsh-list-"));
      terminal.submit(`$env:PATH = "${dir}"`);
      terminal.submit(`& "${paveBin}" list`);
      await expect(
        terminal.getByText(path.basename(dir), { strict: false })
      ).toBeVisible();
    });

    test("help shows usage", async ({ terminal }) => {
      terminal.submit(`& "${paveBin}" --help`);
      await expect(
        terminal.getByText("A cross-platform CLI tool for managing the PATH")
      ).toBeVisible();
    });

    test("env pwsh outputs PATH value", async ({ terminal }) => {
      terminal.submit(`& "${paveBin}" env pwsh`);
      await expect(
        terminal.getByText("system32", { full: true, strict: false })
      ).toBeVisible();
    });

    test("search with controlled PATH finds executable", async ({ terminal }) => {
      const { dir } = createFakeExecutable("pwshtestprog");
      terminal.submit(`$env:PATH = "${dir}"`);
      terminal.submit(`& "${paveBin}" search pwshtestprog`);
      await expect(
        terminal.getByText("pwshtestprog", { strict: false })
      ).toBeVisible();
    });

    test("add and remove workflow with env merge verification", async ({ terminal }) => {
      const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "pave-e2e-pwsh-"));

      terminal.submit(`& "${paveBin}" add "${tmpDir}"`);
      await expect(terminal.getByText("Added to PATH")).toBeVisible();

      terminal.submit(`& "${paveBin}" env pwsh`);
      await expect(
        terminal.getByText(path.basename(tmpDir), { full: true, strict: false })
      ).toBeVisible();

      terminal.submit(`& "${paveBin}" remove "${tmpDir}"`);
      await expect(terminal.getByText(/Removed from PATH|Removed/g)).toBeVisible();

      fs.rmSync(tmpDir, { recursive: true, force: true });
    });
  });
});

test.when(isWindows, "cmd integration", () => {
  test.describe("cmd shell", () => {
    test.use({ shell: Shell.Cmd });

    test("list with controlled PATH shows expected dirs", async ({ terminal }) => {
      const dir = fs.mkdtempSync(path.join(os.tmpdir(), "pave-cmd-list-"));
      terminal.submit(`set "PATH=${dir}"`);
      terminal.submit(`"${paveBin}" list`);
      await expect(
        terminal.getByText(path.basename(dir), { strict: false })
      ).toBeVisible();
    });

    test("help shows usage", async ({ terminal }) => {
      terminal.submit(`"${paveBin}" --help`);
      await expect(
        terminal.getByText(
          "A cross-platform CLI tool for managing the PATH"
        )
      ).toBeVisible();
    });

    test("env pwsh outputs env:PATH statement", async ({ terminal }) => {
      terminal.submit(`"${paveBin}" env pwsh`);
      await expect(
        terminal.getByText("$env:PATH", { strict: false })
      ).toBeVisible();
    });
  });
});
