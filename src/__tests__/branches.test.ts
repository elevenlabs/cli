import {
  getAgentApi,
  updateAgentApi,
  listBranchesApi,
  resolveBranchId,
} from "../shared/elevenlabs-api";
import { ElevenLabsClient } from "@elevenlabs/elevenlabs-js";

describe("Agent branch support", () => {
  function makeMockClient(opts: {
    branches?: Array<{
      id: string;
      name: string;
      agentId: string;
      description: string;
      createdAt: number;
      lastCommittedAt: number;
      isArchived: boolean;
      currentLivePercentage?: number;
    }>;
  } = {}) {
    const update = jest.fn().mockResolvedValue({
      agentId: "agent_123",
      versionId: "ver_abc",
      branchId: "agtbrch_feat",
    });
    const get = jest.fn().mockResolvedValue({
      agentId: "agent_123",
      name: "Test Agent",
      versionId: "ver_abc",
      branchId: "agtbrch_feat",
      conversationConfig: {
        agent: { prompt: { prompt: "Hello", temperature: 0.5 } },
      },
      platformSettings: {},
      tags: [],
    });
    const branchesList = jest.fn().mockResolvedValue({
      results: opts.branches ?? [
        {
          id: "agtbrch_main123",
          name: "main",
          agentId: "agent_123",
          description: "Main branch",
          createdAt: 1700000000,
          lastCommittedAt: 1700000000,
          isArchived: false,
          currentLivePercentage: 100,
        },
        {
          id: "agtbrch_feat456",
          name: "staging",
          agentId: "agent_123",
          description: "Staging branch",
          createdAt: 1700001000,
          lastCommittedAt: 1700002000,
          isArchived: false,
          currentLivePercentage: 0,
        },
        {
          id: "agtbrch_old789",
          name: "old-experiment",
          agentId: "agent_123",
          description: "Old experiment",
          createdAt: 1699000000,
          lastCommittedAt: 1699500000,
          isArchived: true,
          currentLivePercentage: 0,
        },
      ],
    });

    return {
      conversationalAi: {
        agents: {
          update,
          get,
          branches: { list: branchesList },
        },
      },
    } as unknown as ElevenLabsClient;
  }

  describe("listBranchesApi", () => {
    it("should return branches for an agent", async () => {
      const client = makeMockClient();
      const branches = await listBranchesApi(client, "agent_123");

      expect(
        client.conversationalAi.agents.branches.list
      ).toHaveBeenCalledWith("agent_123", { includeArchived: false });
      expect(branches).toHaveLength(3);
      expect(branches[0].name).toBe("main");
      expect(branches[1].name).toBe("staging");
    });

    it("should pass includeArchived flag", async () => {
      const client = makeMockClient();
      await listBranchesApi(client, "agent_123", true);

      expect(
        client.conversationalAi.agents.branches.list
      ).toHaveBeenCalledWith("agent_123", { includeArchived: true });
    });

    it("should return empty array when no branches exist", async () => {
      const client = makeMockClient({ branches: [] });
      const branches = await listBranchesApi(client, "agent_123");

      expect(branches).toHaveLength(0);
    });
  });

  describe("resolveBranchId", () => {
    it("should return branch ID directly when input starts with agtbrch_", async () => {
      const client = makeMockClient();
      const result = await resolveBranchId(
        client,
        "agent_123",
        "agtbrch_feat456"
      );

      expect(result).toBe("agtbrch_feat456");
      // Should NOT call the API when given an ID directly
      expect(
        client.conversationalAi.agents.branches.list
      ).not.toHaveBeenCalled();
    });

    it("should resolve branch name to ID via API", async () => {
      const client = makeMockClient();
      const result = await resolveBranchId(client, "agent_123", "staging");

      expect(result).toBe("agtbrch_feat456");
      expect(
        client.conversationalAi.agents.branches.list
      ).toHaveBeenCalledWith("agent_123", { includeArchived: false });
    });

    it("should throw error when branch name is not found", async () => {
      const client = makeMockClient();

      await expect(
        resolveBranchId(client, "agent_123", "nonexistent")
      ).rejects.toThrow(
        "Branch 'nonexistent' not found for agent 'agent_123'"
      );
    });

    it("should include help message in error when branch not found", async () => {
      const client = makeMockClient();

      await expect(
        resolveBranchId(client, "agent_123", "nonexistent")
      ).rejects.toThrow(
        "elevenlabs agents branches list --agent agent_123"
      );
    });
  });

  describe("getAgentApi with branchId", () => {
    it("should call API without options when no branchId provided", async () => {
      const client = makeMockClient();
      await getAgentApi(client, "agent_123");

      expect(client.conversationalAi.agents.get).toHaveBeenCalledWith(
        "agent_123"
      );
    });

    it("should call API with branchId when provided", async () => {
      const client = makeMockClient();
      await getAgentApi(client, "agent_123", "agtbrch_feat456");

      expect(client.conversationalAi.agents.get).toHaveBeenCalledWith(
        "agent_123",
        { branchId: "agtbrch_feat456" }
      );
    });

    it("should not pass branchId options when branchId is undefined", async () => {
      const client = makeMockClient();
      await getAgentApi(client, "agent_123", undefined);

      // Should be called with just agentId, no second argument
      expect(client.conversationalAi.agents.get).toHaveBeenCalledWith(
        "agent_123"
      );
    });
  });

  describe("updateAgentApi with branchId", () => {
    it("should not include branchId in payload when not provided", async () => {
      const client = makeMockClient();
      const conversationConfig = {
        agent: { prompt: { prompt: "hi", temperature: 0 } },
      } as unknown as Record<string, unknown>;

      await updateAgentApi(
        client,
        "agent_123",
        "Test Agent",
        conversationConfig,
        undefined,
        undefined,
        ["tag"],
        "v1.0"
      );

      const [, payload] = (
        client.conversationalAi.agents.update as jest.Mock
      ).mock.calls[0];

      expect(payload.branchId).toBeUndefined();
    });

    it("should include branchId in payload when provided", async () => {
      const client = makeMockClient();
      const conversationConfig = {
        agent: { prompt: { prompt: "hi", temperature: 0 } },
      } as unknown as Record<string, unknown>;

      await updateAgentApi(
        client,
        "agent_123",
        "Test Agent",
        conversationConfig,
        undefined,
        undefined,
        ["tag"],
        "v1.0",
        "agtbrch_feat456"
      );

      const [agentId, payload] = (
        client.conversationalAi.agents.update as jest.Mock
      ).mock.calls[0];

      expect(agentId).toBe("agent_123");
      expect(payload.branchId).toBe("agtbrch_feat456");
    });

    it("should return branchId from API response", async () => {
      const client = makeMockClient();
      const conversationConfig = {
        agent: { prompt: { prompt: "hi", temperature: 0 } },
      } as unknown as Record<string, unknown>;

      const result = await updateAgentApi(
        client,
        "agent_123",
        "Test Agent",
        conversationConfig,
        undefined,
        undefined,
        [],
        undefined,
        "agtbrch_feat456"
      );

      expect(result.branchId).toBe("agtbrch_feat");
    });
  });
});
