import { test, expect } from "@microsoft/tui-test";
import { paveBin, testEnv, createFakeExecutable } from "./helpers.js";

const { dir: searchDir } = createFakeExecutable("myfakeprog");

test.describe("search finds executable on controlled PATH", () => {
  test.use({
    program: { file: paveBin, args: ["search", "myfakeprog"] },
    env: testEnv({ PATH: searchDir }),
  });

  test("finds the fake executable", async ({ terminal }) => {
    await expect(
      terminal.getByText("myfakeprog", { full: true, strict: false })
    ).toBeVisible();
  });
});

test.describe("search for nonexistent executable on controlled PATH", () => {
  test.use({
    program: { file: paveBin, args: ["search", "nonexistent_prog_xyz"] },
    env: testEnv({ PATH: searchDir }),
  });

  test("shows not found message", async ({ terminal }) => {
    await expect(
      terminal.getByText("No executables matching", { full: true })
    ).toBeVisible();
  });
});
