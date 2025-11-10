import { createAgentApi, updateAgentApi, getAgentApi } from "../shared/elevenlabs-api";
import { ElevenLabsClient } from "@elevenlabs/elevenlabs-js";

describe("Key casing normalization", () => {
  function makeMockClient() {
    const create = jest.fn().mockResolvedValue({ agentId: "agent_123" });
    const update = jest.fn().mockResolvedValue({ agentId: "agent_123" });
    const get = jest.fn().mockResolvedValue({
      agentId: "agent_123",
      name: "Test Agent",
      conversationConfig: {
        conversation: {
          clientEvents: ["audio", "agent_response"],
        },
        agent: {
          prompt: {
            prompt: "Hi",
            temperature: 0,
          },
        },
      },
      platformSettings: {
        widget: { textInputEnabled: true },
      },
      tags: ["prod"],
    });

    return {
      conversationalAi: {
        agents: { create, update, get },
      },
    } as unknown as ElevenLabsClient;
  }

  it("createAgentApi camelizes outbound conversation_config and platform_settings", async () => {
    const client = makeMockClient();
    const conversation_config = {
      conversation: {
        client_events: ["audio", "interruption"],
      },
      agent: { prompt: { prompt: "hi", temperature: 0 } },
    } as unknown as Record<string, unknown>;
    const platform_settings = {
      widget: { text_input_enabled: true },
    } as unknown as Record<string, unknown>;

    await createAgentApi(
      client,
      "Name",
      conversation_config,
      platform_settings,
      undefined,
      ["prod"]
    );

    expect(client.conversationalAi.agents.create).toHaveBeenCalledTimes(1);
    const [, payload] = [
      (client.conversationalAi.agents.create as jest.Mock).mock.calls[0][0]
        .name,
      (client.conversationalAi.agents.create as jest.Mock).mock.calls[0][0],
    ];

    expect(payload).toEqual(
      expect.objectContaining({
        name: "Name",
        conversationConfig: expect.objectContaining({
          conversation: expect.objectContaining({
            clientEvents: ["audio", "interruption"],
          }),
        }),
        platformSettings: expect.objectContaining({
          widget: expect.objectContaining({ textInputEnabled: true }),
        }),
        tags: ["prod"],
      })
    );
  });

  it("updateAgentApi camelizes outbound conversation_config", async () => {
    const client = makeMockClient();
    const conversation_config = {
      conversation: {
        client_events: ["audio", "agent_response"],
      },
    } as unknown as Record<string, unknown>;

    await updateAgentApi(
      client,
      "agent_123",
      "Name",
      conversation_config,
      undefined,
      undefined,
      ["prod"]
    );

    expect(client.conversationalAi.agents.update).toHaveBeenCalledTimes(1);
    const [agentId, payload] = (
      client.conversationalAi.agents.update as jest.Mock
    ).mock.calls[0];
    expect(agentId).toBe("agent_123");
    expect(payload).toEqual(
      expect.objectContaining({
        name: "Name",
        conversationConfig: expect.objectContaining({
          conversation: expect.objectContaining({
            clientEvents: ["audio", "agent_response"],
          }),
        }),
        tags: ["prod"],
      })
    );
  });

  it("getAgentApi snake_cases inbound response for writing to disk", async () => {
    const client = makeMockClient();
    const response = await getAgentApi(client, "agent_123");

    expect(client.conversationalAi.agents.get).toHaveBeenCalledWith(
      "agent_123"
    );
    expect(response).toEqual(
      expect.objectContaining({
        agent_id: "agent_123",
        conversation_config: expect.objectContaining({
          conversation: expect.objectContaining({
            client_events: ["audio", "agent_response"],
          }),
        }),
        platform_settings: expect.objectContaining({
          widget: expect.objectContaining({ text_input_enabled: true }),
        }),
        tags: ["prod"],
      })
    );
  });

  it("createAgentApi removes deprecated 'tools' field when 'tool_ids' is present", async () => {
    const client = makeMockClient();
    const conversation_config = {
      conversation: {
        client_events: ["audio"],
      },
      agent: {
        prompt: {
          prompt: "hi",
          temperature: 0,
          tools: [
            { type: "webhook", name: "test_tool", config: {} }
          ],
          tool_ids: ["tool_123", "tool_456"]
        }
      },
    } as unknown as Record<string, unknown>;

    await createAgentApi(
      client,
      "Agent with Tools",
      conversation_config,
      undefined,
      undefined,
      []
    );

    expect(client.conversationalAi.agents.create).toHaveBeenCalledTimes(1);
    const payload = (client.conversationalAi.agents.create as jest.Mock).mock.calls[0][0];

    // Verify that 'tools' field is removed but 'toolIds' is present
    expect(payload.conversationConfig.agent.prompt).not.toHaveProperty("tools");
    expect(payload.conversationConfig.agent.prompt).toHaveProperty("toolIds");
    expect(payload.conversationConfig.agent.prompt.toolIds).toEqual(["tool_123", "tool_456"]);
  });

  it("updateAgentApi removes deprecated 'tools' field when 'tool_ids' is present", async () => {
    const client = makeMockClient();
    const conversation_config = {
      agent: {
        prompt: {
          prompt: "updated",
          tools: [
            { type: "system", name: "calendar" }
          ],
          tool_ids: ["tool_789"]
        }
      },
    } as unknown as Record<string, unknown>;

    await updateAgentApi(
      client,
      "agent_123",
      "Updated Agent",
      conversation_config,
      undefined,
      undefined,
      []
    );

    expect(client.conversationalAi.agents.update).toHaveBeenCalledTimes(1);
    const [, payload] = (client.conversationalAi.agents.update as jest.Mock).mock.calls[0];

    // Verify that 'tools' field is removed but 'toolIds' is present
    expect(payload.conversationConfig.agent.prompt).not.toHaveProperty("tools");
    expect(payload.conversationConfig.agent.prompt).toHaveProperty("toolIds");
    expect(payload.conversationConfig.agent.prompt.toolIds).toEqual(["tool_789"]);
  });

  it("createAgentApi preserves 'tools' field when 'tool_ids' is not present", async () => {
    const client = makeMockClient();
    const conversation_config = {
      agent: {
        prompt: {
          prompt: "hi",
          tools: [
            { type: "webhook", name: "legacy_tool" }
          ]
        }
      },
    } as unknown as Record<string, unknown>;

    await createAgentApi(
      client,
      "Agent with Legacy Tools",
      conversation_config,
      undefined,
      undefined,
      []
    );

    const payload = (client.conversationalAi.agents.create as jest.Mock).mock.calls[0][0];

    // When tool_ids is not present, tools should be preserved
    expect(payload.conversationConfig.agent.prompt).toHaveProperty("tools");
    expect(payload.conversationConfig.agent.prompt.tools).toHaveLength(1);
  });
});
