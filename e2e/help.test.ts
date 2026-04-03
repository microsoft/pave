import { test, expect } from "@microsoft/tui-test";
import { paveBin } from "./helpers.js";

test.use({ program: { file: paveBin, args: ["--help"] } });

test("shows help message", async ({ terminal }) => {
  await expect(
    terminal.getByText("A cross-platform CLI tool for managing the PATH")
  ).toBeVisible();
});

test("shows available subcommands", async ({ terminal }) => {
  await expect(terminal.getByText("add")).toBeVisible();
  await expect(terminal.getByText("remove")).toBeVisible();
  await expect(terminal.getByText("list")).toBeVisible();
  await expect(terminal.getByText("search")).toBeVisible();
  await expect(terminal.getByText("env")).toBeVisible();
});
