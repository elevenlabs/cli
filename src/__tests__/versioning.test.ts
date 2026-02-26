import { updateAgentApi, getAgentApi } from "../shared/elevenlabs-api";
import { ElevenLabsClient } from "@elevenlabs/elevenlabs-js";

describe("Agent versioning and branch support", () => {
  function makeMockClient(opts: {
    versionId?: string;
    branchId?: string;
    mainBranchId?: string;
  } = {}) {
    const create = jest.fn().mockResolvedValue({ agentId: "agent_ver_123" });
    const update = jest.fn().mockResolvedValue({
      agentId: "agent_ver_123",
      versionId: opts.versionId ?? "ver_abc",
      branchId: opts.branchId ?? "branch_main",
    });
    const get = jest.fn().mockResolvedValue({
      agentId: "agent_ver_123",
      name: "Test Agent",
      versionId: opts.versionId ?? "ver_abc",
      branchId: opts.branchId ?? "branch_main",
      mainBranchId: opts.mainBranchId ?? "branch_main",
      conversationConfig: {
        agent: { prompt: { prompt: "Hello", temperature: 0.5 } },
      },
      platformSettings: {},
      tags: [],
    });

    return {
      conversationalAi: {
        agents: { create, update, get },
      },
    } as unknown as ElevenLabsClient;
  }

  describe("updateAgentApi", () => {
    it("should pass versionDescription to the API", async () => {
      const client = makeMockClient();
      const conversationConfig = {
        agent: { prompt: { prompt: "hi", temperature: 0 } },
      } as unknown as Record<string, unknown>;

      await updateAgentApi(
        client,
        "agent_ver_123",
        "Test Agent",
        conversationConfig,
        undefined,
        undefined,
        ["tag"],
        "release v1.0"
      );

      expect(client.conversationalAi.agents.update).toHaveBeenCalledTimes(1);
      const [agentId, payload] = (
        client.conversationalAi.agents.update as jest.Mock
      ).mock.calls[0];

      expect(agentId).toBe("agent_ver_123");
      expect(payload).toEqual(
        expect.objectContaining({
          versionDescription: "release v1.0",
        })
      );
    });

    it("should not include versionDescription when not provided", async () => {
      const client = makeMockClient();
      const conversationConfig = {
        agent: { prompt: { prompt: "hi", temperature: 0 } },
      } as unknown as Record<string, unknown>;

      await updateAgentApi(
        client,
        "agent_ver_123",
        "Test Agent",
        conversationConfig,
        undefined,
        undefined,
        []
      );

      const [, payload] = (
        client.conversationalAi.agents.update as jest.Mock
      ).mock.calls[0];

      expect(payload.versionDescription).toBeUndefined();
    });

    it("should return versionId and branchId from API response", async () => {
      const client = makeMockClient({
        versionId: "ver_xyz",
        branchId: "branch_feat",
      });
      const conversationConfig = {
        agent: { prompt: { prompt: "hi", temperature: 0 } },
      } as unknown as Record<string, unknown>;

      const result = await updateAgentApi(
        client,
        "agent_ver_123",
        "Test Agent",
        conversationConfig,
        undefined,
        undefined,
        [],
        "my version"
      );

      expect(result).toEqual({
        agentId: "agent_ver_123",
        versionId: "ver_xyz",
        branchId: "branch_feat",
      });
    });

    it("should handle missing versionId/branchId in response", async () => {
      const client = makeMockClient();
      // Override update to return response without version fields
      (client.conversationalAi.agents.update as jest.Mock).mockResolvedValue({
        agentId: "agent_ver_123",
      });

      const conversationConfig = {
        agent: { prompt: { prompt: "hi", temperature: 0 } },
      } as unknown as Record<string, unknown>;

      const result = await updateAgentApi(
        client,
        "agent_ver_123",
        "Test Agent",
        conversationConfig
      );

      expect(result).toEqual({
        agentId: "agent_ver_123",
        versionId: undefined,
        branchId: undefined,
      });
    });
  });

  describe("getAgentApi", () => {
    it("should return version_id, branch_id, main_branch_id in snake_case", async () => {
      const client = makeMockClient({
        versionId: "ver_999",
        branchId: "branch_dev",
        mainBranchId: "branch_main",
      });

      const response = await getAgentApi(client, "agent_ver_123");
      const typed = response as Record<string, unknown>;

      // getAgentApi converts to snake_case via toSnakeCaseKeys
      expect(typed.version_id).toBe("ver_999");
      expect(typed.branch_id).toBe("branch_dev");
      expect(typed.main_branch_id).toBe("branch_main");
    });

    it("should handle agent without version/branch fields", async () => {
      const client = makeMockClient();
      // Override get to return response without version fields
      (client.conversationalAi.agents.get as jest.Mock).mockResolvedValue({
        agentId: "agent_ver_123",
        name: "Test Agent",
        conversationConfig: {},
        platformSettings: {},
        tags: [],
      });

      const response = await getAgentApi(client, "agent_ver_123");
      const typed = response as Record<string, unknown>;

      expect(typed.agent_id).toBe("agent_ver_123");
      // Fields should simply be absent
      expect(typed.version_id).toBeUndefined();
      expect(typed.branch_id).toBeUndefined();
    });
  });
});

describe("Versioning in push/pull agents.json persistence", () => {
  let tempDir: string;
  let agentsConfigPath: string;

  beforeEach(async () => {
    const fs = await import("fs-extra");
    const path = await import("path");
    const { tmpdir } = await import("os");
    tempDir = await fs.mkdtemp(path.join(tmpdir(), "test-versioning-"));
    agentsConfigPath = path.join(tempDir, "agents.json");
  });

  afterEach(async () => {
    const fs = await import("fs-extra");
    await fs.remove(tempDir);
  });

  it("should store version_id and branch_id in agents.json structure", async () => {
    const { writeConfig, readConfig } = await import("../shared/utils");

    // Simulate what push-impl and pull-impl do: write agents.json with version/branch
    const agentsConfig = {
      agents: [
        {
          config: "agent_configs/My-Agent.json",
          id: "agent_123",
          version_id: "ver_abc",
          branch_id: "branch_main",
        },
        {
          config: "agent_configs/Another-Agent.json",
          id: "agent_456",
          // no version/branch - should be fine
        },
      ],
    };

    await writeConfig(agentsConfigPath, agentsConfig);

    const loaded = await readConfig(agentsConfigPath) as {
      agents: Array<{
        config: string;
        id?: string;
        version_id?: string;
        branch_id?: string;
      }>;
    };

    expect(loaded.agents[0].version_id).toBe("ver_abc");
    expect(loaded.agents[0].branch_id).toBe("branch_main");
    expect(loaded.agents[1].version_id).toBeUndefined();
    expect(loaded.agents[1].branch_id).toBeUndefined();
  });

  it("should update version_id and branch_id on subsequent pushes", async () => {
    const { writeConfig, readConfig } = await import("../shared/utils");

    // Initial state
    const agentsConfig = {
      agents: [
        {
          config: "agent_configs/My-Agent.json",
          id: "agent_123",
          version_id: "ver_1",
          branch_id: "branch_main",
        },
      ],
    };

    await writeConfig(agentsConfigPath, agentsConfig);

    // Simulate push updating version
    const loaded = await readConfig(agentsConfigPath) as {
      agents: Array<{
        config: string;
        id?: string;
        version_id?: string;
        branch_id?: string;
      }>;
    };

    loaded.agents[0].version_id = "ver_2";
    await writeConfig(agentsConfigPath, loaded);

    const final = await readConfig(agentsConfigPath) as {
      agents: Array<{
        config: string;
        id?: string;
        version_id?: string;
        branch_id?: string;
      }>;
    };

    expect(final.agents[0].version_id).toBe("ver_2");
    expect(final.agents[0].branch_id).toBe("branch_main");
  });
});
