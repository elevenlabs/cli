/**
 * Tests for tools.json registry integrity during push and delete
 */

import * as fs from "fs-extra";
import * as path from "path";
import * as os from "os";
import {
  writeToolsConfig,
  readToolsConfig,
  writeToolConfig,
  type Tool,
} from "../../shared/tools";
import * as elevenLabsApi from "../../shared/elevenlabs-api";
import { pushTools, deleteTool } from "../commands/impl";
import { ElevenLabs, ElevenLabsClient } from "@elevenlabs/elevenlabs-js";

// Mock the entire elevenlabs-api module
jest.mock("../../shared/elevenlabs-api");
const mockedElevenLabsApi = elevenLabsApi as jest.Mocked<typeof elevenLabsApi>;

describe("tools.json registry integrity", () => {
  let tempDir: string;
  let originalCwd: string;
  let logSpy: jest.SpyInstance;

  beforeEach(async () => {
    tempDir = await fs.mkdtemp(path.join(os.tmpdir(), "tools-registry-test-"));
    originalCwd = process.cwd();
    // pushTools/deleteTool resolve tools.json against the working directory
    process.chdir(tempDir);
    logSpy = jest.spyOn(console, "log").mockImplementation(() => {});
    mockedElevenLabsApi.getElevenLabsClient.mockResolvedValue({} as ElevenLabsClient);
  });

  afterEach(async () => {
    process.chdir(originalCwd);
    logSpy.mockRestore();
    await fs.remove(tempDir);
    jest.clearAllMocks();
  });

  it("pushTools stores the tool ID from the API response when creating a tool", async () => {
    const configPath = path.join(tempDir, "tool_configs", "lookup.json");
    await writeToolConfig(configPath, { name: "lookup", type: "webhook" } as unknown as Tool);
    await writeToolsConfig(path.join(tempDir, "tools.json"), {
      tools: [{ type: "webhook", config: configPath }],
    });

    mockedElevenLabsApi.createToolApi.mockResolvedValue({
      id: "tool_real_123",
    } as ElevenLabs.ToolResponseModel);

    await pushTools();

    const saved = await readToolsConfig(path.join(tempDir, "tools.json"));
    expect(saved.tools).toHaveLength(1);
    // The real API ID must be saved, not a locally fabricated one
    expect(saved.tools[0].id).toBe("tool_real_123");
  });

  it("deleteTool removes exactly one entry from tools.json", async () => {
    const configA = path.join(tempDir, "tool_configs", "alpha.json");
    const configB = path.join(tempDir, "tool_configs", "beta.json");
    await writeToolConfig(configA, { name: "alpha", type: "webhook" } as unknown as Tool);
    await writeToolConfig(configB, { name: "beta", type: "webhook" } as unknown as Tool);
    await writeToolsConfig(path.join(tempDir, "tools.json"), {
      tools: [
        { type: "webhook", config: configA, id: "tool_a" },
        { type: "webhook", config: configB, id: "tool_b" },
      ],
    });

    mockedElevenLabsApi.deleteToolApi.mockResolvedValue(undefined);

    await deleteTool("tool_a");

    const saved = await readToolsConfig(path.join(tempDir, "tools.json"));
    // Deleting one tool must not drop other entries from the registry
    expect(saved.tools).toHaveLength(1);
    expect(saved.tools[0].id).toBe("tool_b");
  });
});
