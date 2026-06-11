import { createAgentApi, updateAgentApi, getAgentApi, createToolApi, updateToolApi, getToolApi } from "../shared/elevenlabs-api";
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

  function makeToolsMockClient() {
    const create = jest.fn().mockResolvedValue({ id: "tool_123" });
    const update = jest.fn().mockResolvedValue({ id: "tool_123" });
    const get = jest.fn().mockResolvedValue({
      id: "tool_123",
      toolConfig: {
        name: "lookup",
        type: "webhook",
        apiSchema: {
          url: "https://api.example.com/lookup",
          method: "GET",
          queryParamsSchema: {
            properties: {
              user_id: { type: "string" },
              sessionToken: { type: "string" },
            },
            required: ["user_id", "sessionToken"],
          },
        },
      },
    });

    return {
      conversationalAi: {
        tools: { create, update, get },
      },
    } as unknown as ElevenLabsClient;
  }

  it("createAgentApi camelizes the placeholders wrapper but preserves placeholder names", async () => {
    const client = makeMockClient();
    const conversation_config = {
      agent: {
        prompt: { prompt: "hi", temperature: 0 },
        dynamic_variables: {
          dynamic_variable_placeholders: {
            transaction_id: "txn_default",
            verified: false,
          },
        },
      },
    } as unknown as Record<string, unknown>;

    await createAgentApi(
      client,
      "Agent with placeholders",
      conversation_config,
      undefined,
      undefined,
      []
    );

    const payload = (client.conversationalAi.agents.create as jest.Mock).mock.calls[0][0];
    const dynamicVariables = payload.conversationConfig.agent.dynamicVariables;

    // The wrapper is a schema field: it must be camelized or the SDK strips it from the request
    expect(dynamicVariables).toHaveProperty("dynamicVariablePlaceholders");
    expect(dynamicVariables).not.toHaveProperty("dynamic_variable_placeholders");
    // Placeholder names are user-defined identifiers, preserved as-is
    expect(dynamicVariables.dynamicVariablePlaceholders).toEqual({
      transaction_id: "txn_default",
      verified: false,
    });
  });

  it("updateAgentApi camelizes the placeholders wrapper but preserves placeholder names", async () => {
    const client = makeMockClient();
    const conversation_config = {
      agent: {
        prompt: { prompt: "hi", temperature: 0 },
        dynamic_variables: {
          dynamic_variable_placeholders: {
            transaction_id: "txn_default",
            user_name: "Jan",
          },
        },
      },
    } as unknown as Record<string, unknown>;

    await updateAgentApi(
      client,
      "agent_123",
      "Updated",
      conversation_config,
      undefined,
      undefined,
      []
    );

    const [, payload] = (client.conversationalAi.agents.update as jest.Mock).mock.calls[0];
    const dynamicVariables = payload.conversationConfig.agent.dynamicVariables;

    expect(dynamicVariables).toHaveProperty("dynamicVariablePlaceholders");
    expect(dynamicVariables).not.toHaveProperty("dynamic_variable_placeholders");
    expect(dynamicVariables.dynamicVariablePlaceholders).toEqual({
      transaction_id: "txn_default",
      user_name: "Jan",
    });
  });

  it("getAgentApi snake_cases the placeholders wrapper and preserves placeholder names", async () => {
    const getWithPlaceholders = jest.fn().mockResolvedValue({
      agentId: "agent_123",
      name: "Test",
      conversationConfig: {
        agent: {
          dynamicVariables: {
            dynamicVariablePlaceholders: {
              transaction_id: "txn_default",
              customerName: "anon",
            },
          },
        },
      },
      tags: [],
    });
    const client = {
      conversationalAi: { agents: { get: getWithPlaceholders } },
    } as unknown as ElevenLabsClient;

    const response = await getAgentApi(client, "agent_123") as Record<string, any>;
    const dynamicVariables = response.conversation_config.agent.dynamic_variables;

    expect(dynamicVariables).toHaveProperty("dynamic_variable_placeholders");
    // No round-trip corruption: snake stays snake, camel stays camel
    expect(dynamicVariables.dynamic_variable_placeholders).toEqual({
      transaction_id: "txn_default",
      customerName: "anon",
    });
  });

  it("createToolApi preserves query_params_schema property names and required entries", async () => {
    const client = makeToolsMockClient();
    const toolConfig = {
      name: "lookup",
      type: "webhook",
      api_schema: {
        url: "https://api.example.com/lookup",
        method: "GET",
        query_params_schema: {
          properties: {
            user_id: { type: "string", dynamic_variable: "user_id" },
          },
          required: ["user_id"],
        },
      },
    } as unknown as Record<string, unknown>;

    await createToolApi(client, toolConfig);

    const payload = (client.conversationalAi.tools.create as jest.Mock).mock.calls[0][0];
    const schema = payload.toolConfig.apiSchema.queryParamsSchema;

    // Property names are user-defined, preserved so required[] still matches
    expect(schema.properties).toHaveProperty("user_id");
    expect(schema.properties).not.toHaveProperty("userId");
    // Schema fields within each property definition are still converted
    expect(schema.properties.user_id).toEqual({ type: "string", dynamicVariable: "user_id" });
    expect(schema.required).toEqual(["user_id"]);
  });

  it("updateToolApi preserves request_body_schema property names at all nesting depths", async () => {
    const client = makeToolsMockClient();
    const toolConfig = {
      name: "submit",
      type: "webhook",
      api_schema: {
        url: "https://api.example.com/submit",
        method: "POST",
        request_body_schema: {
          type: "object",
          properties: {
            caller_id: { type: "string" },
            nested_obj: {
              type: "object",
              properties: {
                inner_key: { type: "string" },
              },
            },
          },
          required: ["caller_id"],
        },
      },
    } as unknown as Record<string, unknown>;

    await updateToolApi(client, "tool_123", toolConfig);

    const [toolId, payload] = (client.conversationalAi.tools.update as jest.Mock).mock.calls[0];
    expect(toolId).toBe("tool_123");
    const schema = payload.toolConfig.apiSchema.requestBodySchema;

    expect(schema.properties).toHaveProperty("caller_id");
    expect(schema.properties).not.toHaveProperty("callerId");
    expect(schema.properties.nested_obj.properties).toHaveProperty("inner_key");
    expect(schema.properties.nested_obj.properties).not.toHaveProperty("innerKey");
    expect(schema.required).toEqual(["caller_id"]);
  });

  it("getToolApi preserves schema property names on inbound snake_case conversion", async () => {
    const client = makeToolsMockClient();

    const response = await getToolApi(client, "tool_123") as Record<string, any>;
    const schema = response.tool_config.api_schema.query_params_schema;

    expect(schema.properties).toHaveProperty("user_id");
    // camelCase property names must not be snake_cased on pull
    expect(schema.properties).toHaveProperty("sessionToken");
    expect(schema.properties).not.toHaveProperty("session_token");
    expect(schema.required).toEqual(["user_id", "sessionToken"]);
  });
});
