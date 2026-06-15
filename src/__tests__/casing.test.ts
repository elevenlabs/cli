import {
  createAgentApi,
  updateAgentApi,
  getAgentApi,
  createToolApi,
  updateToolApi,
  getToolApi,
} from "../shared/elevenlabs-api";
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

  it("createAgentApi camelizes workflow edge conditions (forward_condition, backward_condition)", async () => {
    const client = makeMockClient();
    const conversation_config = {
      agent: { prompt: { prompt: "hi", temperature: 0 } },
    } as unknown as Record<string, unknown>;

    // This is what a workflow looks like after being pulled from the API (snake_case)
    const workflow = {
      nodes: {
        start_node: { type: "start" },
        agent_node: { type: "agent", agent_id: "abc123" }
      },
      edges: {
        edge_start_to_agent: {
          source: "start_node",
          target: "agent_node",
          forward_condition: { type: "unconditional" }
        },
        edge_agent_to_end: {
          source: "agent_node",
          target: "end_node",
          backward_condition: { type: "result", result_key: "success" }
        }
      }
    };

    await createAgentApi(
      client,
      "Workflow Agent",
      conversation_config,
      undefined,
      workflow,
      []
    );

    expect(client.conversationalAi.agents.create).toHaveBeenCalledTimes(1);
    const payload = (client.conversationalAi.agents.create as jest.Mock).mock.calls[0][0];

    // Verify workflow edge identifier keys are preserved, but schema fields within are camel-cased
    expect(payload.workflow).toBeDefined();
    expect(payload.workflow.edges.edge_start_to_agent).toEqual({
      source: "start_node",
      target: "agent_node",
      forwardCondition: { type: "unconditional" }
    });
    expect(payload.workflow.edges.edge_agent_to_end).toEqual({
      source: "agent_node",
      target: "end_node",
      backwardCondition: { type: "result", resultKey: "success" }
    });
  });

  it("updateAgentApi camelizes workflow edge conditions (forward_condition, backward_condition)", async () => {
    const client = makeMockClient();
    const conversation_config = {
      agent: { prompt: { prompt: "hi", temperature: 0 } },
    } as unknown as Record<string, unknown>;

    // This is what a workflow looks like after being pulled from the API (snake_case)
    const workflow = {
      nodes: {
        start_node: { type: "start" },
        agent_node: { type: "agent", agent_id: "abc123" }
      },
      edges: {
        edge_start_to_agent: {
          source: "start_node",
          target: "agent_node",
          forward_condition: { type: "llm", description: "When user asks for help" }
        }
      }
    };

    await updateAgentApi(
      client,
      "agent_123",
      "Workflow Agent",
      conversation_config,
      undefined,
      workflow,
      []
    );

    expect(client.conversationalAi.agents.update).toHaveBeenCalledTimes(1);
    const [, payload] = (client.conversationalAi.agents.update as jest.Mock).mock.calls[0];

    // Verify workflow edge identifier keys are preserved, but schema fields within are camel-cased
    expect(payload.workflow).toBeDefined();
    expect(payload.workflow.edges.edge_start_to_agent).toEqual({
      source: "start_node",
      target: "agent_node",
      forwardCondition: { type: "llm", description: "When user asks for help" }
    });
  });

  it("createAgentApi preserves data_collection child keys (user-defined identifiers)", async () => {
    const client = makeMockClient();
    const conversation_config = {
      agent: { prompt: { prompt: "hi", temperature: 0 } },
    } as unknown as Record<string, unknown>;
    const platform_settings = {
      data_collection: {
        need_callback: { type: "boolean", description: "Whether to call back" },
        call_end_reason: { type: "string", description: "Why the call ended" },
        human_reached: { type: "boolean", description: "Was a human reached" },
      },
    } as unknown as Record<string, unknown>;

    await createAgentApi(
      client,
      "Agent with data_collection",
      conversation_config,
      platform_settings,
      undefined,
      []
    );

    const payload = (client.conversationalAi.agents.create as jest.Mock).mock.calls[0][0];

    // data_collection top-level key is camelized to dataCollection (envelope convention)
    expect(payload.platformSettings).toHaveProperty("dataCollection");
    // Children are user-defined identifiers — must be preserved as-is (snake_case stays snake_case)
    expect(payload.platformSettings.dataCollection).toHaveProperty("need_callback");
    expect(payload.platformSettings.dataCollection).toHaveProperty("call_end_reason");
    expect(payload.platformSettings.dataCollection).toHaveProperty("human_reached");
    // Leaf values under each identifier — nested schema fields like 'type'/'description' stay as-is (no underscores to convert)
    expect(payload.platformSettings.dataCollection.need_callback).toEqual({
      type: "boolean",
      description: "Whether to call back",
    });
  });

  it("updateAgentApi preserves data_collection child keys (user-defined identifiers)", async () => {
    const client = makeMockClient();
    const conversation_config = {
      agent: { prompt: { prompt: "updated", temperature: 0 } },
    } as unknown as Record<string, unknown>;
    const platform_settings = {
      data_collection: {
        need_callback: { type: "boolean", description: "Callback requested" },
      },
    } as unknown as Record<string, unknown>;

    await updateAgentApi(
      client,
      "agent_123",
      "Updated",
      conversation_config,
      platform_settings,
      undefined,
      []
    );

    const [, payload] = (client.conversationalAi.agents.update as jest.Mock).mock.calls[0];

    expect(payload.platformSettings).toHaveProperty("dataCollection");
    expect(payload.platformSettings.dataCollection).toHaveProperty("need_callback");
  });

  it("getAgentApi preserves data_collection child keys on inbound snake_case conversion", async () => {
    const getWithDataCollection = jest.fn().mockResolvedValue({
      agentId: "agent_123",
      name: "Test",
      conversationConfig: {
        agent: { prompt: { prompt: "hi", temperature: 0 } },
      },
      platformSettings: {
        dataCollection: {
          need_callback: { type: "boolean", description: "Callback" },
          call_end_reason: { type: "string" },
        },
      },
      tags: [],
    });
    const client = {
      conversationalAi: { agents: { get: getWithDataCollection } },
    } as unknown as ElevenLabsClient;

    const response = await getAgentApi(client, "agent_123") as Record<string, any>;

    // Envelope snake_cases back for disk
    expect(response.platform_settings).toHaveProperty("data_collection");
    // User-defined identifiers preserved as-is — no round-trip corruption
    expect(response.platform_settings.data_collection).toHaveProperty("need_callback");
    expect(response.platform_settings.data_collection).toHaveProperty("call_end_reason");
    expect(response.platform_settings.data_collection.need_callback).toEqual({
      type: "boolean",
      description: "Callback",
    });
  });

  it("getAgentApi does not snake_case camelCase data_collection child keys", async () => {
    const getWithDataCollection = jest.fn().mockResolvedValue({
      agentId: "agent_123",
      name: "Test",
      conversationConfig: {
        agent: { prompt: { prompt: "hi", temperature: 0 } },
      },
      platformSettings: {
        dataCollection: {
          callEndReason: { type: "string", description: "Why the call ended" },
        },
      },
      tags: [],
    });
    const client = {
      conversationalAi: { agents: { get: getWithDataCollection } },
    } as unknown as ElevenLabsClient;

    const response = await getAgentApi(client, "agent_123") as Record<string, any>;

    expect(response.platform_settings).toHaveProperty("data_collection");
    expect(response.platform_settings.data_collection).toHaveProperty("callEndReason");
    expect(response.platform_settings.data_collection).not.toHaveProperty("call_end_reason");
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

// Regression coverage for elevenlabs/cli#90: webhook tools whose URL templates
// reference path parameters via {{snake_case}} placeholders. The keys of
// path_params_schema are user-defined identifiers that must match those
// placeholders, so they must survive the camelCase transform unchanged.
describe("Webhook tool path_params_schema key preservation (#90)", () => {
  function makeMockToolClient() {
    const create = jest.fn().mockResolvedValue({ toolId: "tool_123" });
    const update = jest.fn().mockResolvedValue({ toolId: "tool_123" });
    const get = jest.fn();
    return {
      conversationalAi: {
        tools: { create, update, get },
      },
    } as unknown as ElevenLabsClient;
  }

  it("createToolApi preserves path_params_schema child keys (URL placeholder identifiers)", async () => {
    const client = makeMockToolClient();
    const toolConfig = {
      name: "lookup-transaction",
      description: "Looks up a transaction",
      type: "webhook",
      api_schema: {
        url: "https://api.example.com/transactions/{{transaction_id}}",
        method: "GET",
        path_params_schema: {
          transaction_id: {
            type: "string",
            description: "The transaction identifier",
          },
        },
      },
    } as unknown as Record<string, unknown>;

    await createToolApi(client, toolConfig);

    expect(client.conversationalAi.tools.create).toHaveBeenCalledTimes(1);
    const payload = (client.conversationalAi.tools.create as jest.Mock).mock
      .calls[0][0].toolConfig as Record<string, any>;

    // URL string is always preserved; the placeholder must still resolve
    expect(payload.apiSchema.url).toBe(
      "https://api.example.com/transactions/{{transaction_id}}"
    );
    // path_params_schema top-level key is camelized to pathParamsSchema (envelope convention)
    expect(payload.apiSchema).toHaveProperty("pathParamsSchema");
    // User-defined identifier key must be preserved as-is, NOT camelCased to transactionId
    expect(payload.apiSchema.pathParamsSchema).toHaveProperty("transaction_id");
    expect(payload.apiSchema.pathParamsSchema).not.toHaveProperty(
      "transactionId"
    );
    // Nested schema fields stay as-is (no underscores to convert here)
    expect(payload.apiSchema.pathParamsSchema.transaction_id).toEqual({
      type: "string",
      description: "The transaction identifier",
    });
  });

  it("updateToolApi preserves path_params_schema child keys (URL placeholder identifiers)", async () => {
    const client = makeMockToolClient();
    const toolConfig = {
      name: "lookup-transaction",
      description: "Looks up a transaction",
      type: "webhook",
      api_schema: {
        url: "https://api.example.com/transactions/{{transaction_id}}",
        method: "GET",
        path_params_schema: {
          transaction_id: {
            type: "string",
            description: "The transaction identifier",
          },
        },
      },
    } as unknown as Record<string, unknown>;

    await updateToolApi(client, "tool_123", toolConfig);

    expect(client.conversationalAi.tools.update).toHaveBeenCalledTimes(1);
    const [toolId, body] = (client.conversationalAi.tools.update as jest.Mock)
      .mock.calls[0];
    expect(toolId).toBe("tool_123");
    const payload = body.toolConfig as Record<string, any>;

    expect(payload.apiSchema.pathParamsSchema).toHaveProperty("transaction_id");
    expect(payload.apiSchema.pathParamsSchema).not.toHaveProperty(
      "transactionId"
    );
  });

  it("getToolApi round-trips path_params_schema child keys without corruption", async () => {
    const get = jest.fn().mockResolvedValue({
      toolConfig: {
        name: "lookup-transaction",
        type: "webhook",
        apiSchema: {
          url: "https://api.example.com/transactions/{{transaction_id}}",
          method: "GET",
          pathParamsSchema: {
            transaction_id: {
              type: "string",
              description: "The transaction identifier",
            },
          },
        },
      },
    });
    const client = {
      conversationalAi: { tools: { get } },
    } as unknown as ElevenLabsClient;

    const response = (await getToolApi(client, "tool_123")) as Record<
      string,
      any
    >;

    // Inbound snake_case conversion for disk must preserve the identifier key
    expect(response.tool_config.api_schema.path_params_schema).toHaveProperty(
      "transaction_id"
    );
    expect(response.tool_config.api_schema.url).toBe(
      "https://api.example.com/transactions/{{transaction_id}}"
    );
  });
});
