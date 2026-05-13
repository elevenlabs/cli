import { spawnSync } from "child_process";
import {
  afterEach,
  beforeEach,
  describe,
  expect,
  it,
  jest,
} from "@jest/globals";
import { createAddCommand } from "../commands/add";

jest.mock("child_process", () => ({
  spawnSync: jest.fn(),
}));

describe("components add command", () => {
  const mockedSpawnSync = spawnSync as unknown as jest.Mock;
  let logSpy: jest.SpiedFunction<typeof console.log>;

  beforeEach(() => {
    mockedSpawnSync.mockReset();
    mockedSpawnSync.mockReturnValue({ status: 0 });
    logSpy = jest.spyOn(console, "log").mockImplementation(() => {});
  });

  afterEach(() => {
    logSpy.mockRestore();
  });

  it("checks npx availability through the shell for Windows command resolution", async () => {
    const command = createAddCommand();

    await command.parseAsync(["bar-visualizer"], { from: "user" });

    expect(mockedSpawnSync).toHaveBeenNthCalledWith(1, "npx", ["--version"], {
      encoding: "utf-8",
      shell: true,
    });
    expect(mockedSpawnSync).toHaveBeenNthCalledWith(
      2,
      "npx -y shadcn@latest add https://ui.elevenlabs.io/r/bar-visualizer.json",
      {
        stdio: "inherit",
        shell: true,
      }
    );
  });
});
