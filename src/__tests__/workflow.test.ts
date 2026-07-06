import { createAgentApi, updateAgentApi, getAgentApi } from "../shared/elevenlabs-api";
import { ElevenLabsClient } from "@elevenlabs/elevenlabs-js";

const TEST_CTX = { apiKey: "test-key", baseUrl: "https://api.test" };

const realFetch = global.fetch;
afterEach(() => {
  global.fetch = realFetch;
});

function mockFetch(response: Record<string, unknown> = { agent_id: "agent_workflow_123" }): jest.Mock {
  const fetchMock = jest.fn().mockResolvedValue({
    ok: true,
    json: async () => response,
  });
  global.fetch = fetchMock as unknown as typeof fetch;
  return fetchMock;
}

function sentBody(fetchMock: jest.Mock): Record<string, any> {
  return JSON.parse(fetchMock.mock.calls[0][1].body as string);
}

describe("Workflow support in agents", () => {
  function makeMockClient(includeWorkflow: boolean = false) {
    const mockWorkflow = includeWorkflow ? {
      nodes: {
        "start_node": {
          type: "start",
          position: { x: 0, y: 0 }
        },
        "agent_node": {
          type: "agent",
          position: { x: 100, y: 100 }
        },
        "end_node": {
          type: "end",
          position: { x: 200, y: 200 }
        }
      },
      edges: {
        "edge_1": {
          from: "start_node",
          to: "agent_node"
        },
        "edge_2": {
          from: "agent_node",
          to: "end_node"
        }
      }
    } : undefined;

    const get = jest.fn().mockResolvedValue({
      agentId: "agent_workflow_123",
      name: "Test Agent with Workflow",
      conversationConfig: {
        conversation: {
          clientEvents: ["audio"],
        },
        agent: {
          prompt: {
            prompt: "Hello",
            temperature: 0.5,
          },
        },
      },
      platformSettings: {
        widget: { textInputEnabled: true },
      },
      workflow: mockWorkflow,
      tags: ["workflow-test"],
    });

    return {
      conversationalAi: {
        agents: { get },
      },
    } as unknown as ElevenLabsClient;
  }

  describe("createAgentApi", () => {
    it("should send workflow when provided", async () => {
      const fetchMock = mockFetch();
      const conversation_config = {
        conversation: {
          client_events: ["audio"],
        },
        agent: { prompt: { prompt: "hi", temperature: 0 } },
      } as unknown as Record<string, unknown>;

      const workflow = {
        nodes: {
          "start": { type: "start", position: { x: 0, y: 0 } },
          "end": { type: "end", position: { x: 100, y: 100 } }
        },
        edges: {
          "edge_1": { from: "start", to: "end" }
        }
      };

      await createAgentApi(
        TEST_CTX,
        "Agent with Workflow",
        conversation_config,
        undefined,
        workflow,
        ["workflow"]
      );

      expect(fetchMock).toHaveBeenCalledTimes(1);
      const body = sentBody(fetchMock);

      expect(body).toEqual(
        expect.objectContaining({
          name: "Agent with Workflow",
          workflow,
          tags: ["workflow"],
        })
      );
    });

    it("should handle undefined workflow gracefully", async () => {
      const fetchMock = mockFetch();
      const conversation_config = {
        conversation: {
          client_events: ["audio"],
        },
        agent: { prompt: { prompt: "hi", temperature: 0 } },
      } as unknown as Record<string, unknown>;

      await createAgentApi(
        TEST_CTX,
        "Agent without Workflow",
        conversation_config,
        undefined,
        undefined,
        []
      );

      expect(fetchMock).toHaveBeenCalledTimes(1);
      const body = sentBody(fetchMock);

      expect(body.name).toBe("Agent without Workflow");
      expect(body).not.toHaveProperty("workflow");
    });
  });

  describe("updateAgentApi", () => {
    it("should send workflow when updating an agent", async () => {
      const fetchMock = mockFetch();
      const conversation_config = {
        conversation: {
          client_events: ["audio"],
        },
      } as unknown as Record<string, unknown>;

      const workflow = {
        nodes: {
          "updated_start": { type: "start", position: { x: 10, y: 10 } },
          "updated_end": { type: "end", position: { x: 110, y: 110 } }
        },
        edges: {
          "updated_edge": { from: "updated_start", to: "updated_end" }
        }
      };

      await updateAgentApi(
        TEST_CTX,
        "agent_workflow_123",
        "Updated Agent",
        conversation_config,
        undefined,
        workflow,
        ["updated"]
      );

      expect(fetchMock).toHaveBeenCalledTimes(1);
      const [url, init] = fetchMock.mock.calls[0];
      expect(url).toBe("https://api.test/v1/convai/agents/agent_workflow_123");
      expect(init.method).toBe("PATCH");

      expect(sentBody(fetchMock)).toEqual(
        expect.objectContaining({
          name: "Updated Agent",
          workflow,
          tags: ["updated"],
        })
      );
    });

    it("should allow clearing workflow by passing undefined", async () => {
      const fetchMock = mockFetch();
      const conversation_config = {
        conversation: {
          client_events: ["audio"],
        },
      } as unknown as Record<string, unknown>;

      await updateAgentApi(
        TEST_CTX,
        "agent_workflow_123",
        "Agent Workflow Cleared",
        conversation_config,
        undefined,
        undefined,
        []
      );

      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(sentBody(fetchMock)).not.toHaveProperty("workflow");
    });

    // Regression: workflows with expression-type edge conditions containing `llm`
    // or `null_literal` AST nodes must round-trip through push unchanged. The
    // SDK's generated serializers reject these (missing "value" key / unknown
    // union member), which is why push sends raw JSON.
    it("round-trips expression conditions with llm and null_literal nodes verbatim", async () => {
      const fetchMock = mockFetch();

      const workflow = {
        edges: {
          edge_01: {
            source: "node_a",
            target: "node_b",
            forward_condition: {
              type: "expression",
              expression: {
                type: "and_operator",
                children: [
                  {
                    type: "neq_operator",
                    left: { type: "dynamic_variable", name: "system__caller_id" },
                    right: { type: "null_literal" }
                  },
                  {
                    type: "llm",
                    value_schema: {
                      type: "boolean",
                      description: "customer expressed intention to book an appointment"
                    },
                    prompt: "customer expressed intention to book an appointment"
                  }
                ]
              }
            }
          }
        },
        nodes: {}
      };

      await updateAgentApi(
        TEST_CTX,
        "agent_workflow_123",
        "Expression Agent",
        { agent: { prompt: { prompt: "hi" } } } as unknown as Record<string, unknown>,
        undefined,
        workflow,
        []
      );

      expect(sentBody(fetchMock).workflow).toEqual(workflow);
    });
  });

  describe("getAgentApi", () => {
    it("should return workflow when present in API response", async () => {
      const client = makeMockClient(true);
      const response = await getAgentApi(client, "agent_workflow_123");

      expect(client.conversationalAi.agents.get).toHaveBeenCalledWith(
        "agent_workflow_123"
      );

      expect(response).toEqual(
        expect.objectContaining({
          agent_id: "agent_workflow_123",
          workflow: expect.objectContaining({
            nodes: expect.any(Object),
            edges: expect.any(Object),
          }),
        })
      );

      // Verify workflow structure
      const responseTyped = response as { workflow: { nodes: Record<string, unknown>; edges: Record<string, unknown> } };
      expect(responseTyped.workflow.nodes).toHaveProperty("start_node");
      expect(responseTyped.workflow.nodes).toHaveProperty("agent_node");
      expect(responseTyped.workflow.nodes).toHaveProperty("end_node");
      expect(responseTyped.workflow.edges).toHaveProperty("edge_1");
      expect(responseTyped.workflow.edges).toHaveProperty("edge_2");
    });

    it("should handle agents without workflow", async () => {
      const client = makeMockClient(false);
      const response = await getAgentApi(client, "agent_workflow_123");

      expect(client.conversationalAi.agents.get).toHaveBeenCalledWith(
        "agent_workflow_123"
      );

      expect(response).toEqual(
        expect.objectContaining({
          agent_id: "agent_workflow_123",
          workflow: undefined,
        })
      );
    });
  });

  describe("Workflow persistence in pull/push flow", () => {
    it("should preserve complex workflow structures", async () => {
      const fetchMock = mockFetch();

      // Complex workflow with multiple node types
      const complexWorkflow = {
        nodes: {
          "start_1": {
            type: "start",
            position: { x: 0, y: 0 },
            config: { initial_message: "Welcome" }
          },
          "agent_1": {
            type: "override_agent",
            position: { x: 100, y: 50 },
            agent_id: "some_agent_id"
          },
          "tool_1": {
            type: "tool",
            position: { x: 200, y: 100 },
            tool_id: "tool_123"
          },
          "end_1": {
            type: "end",
            position: { x: 300, y: 150 }
          }
        },
        edges: {
          "edge_start_to_agent": {
            from: "start_1",
            to: "agent_1",
            condition: { type: "unconditional" }
          },
          "edge_agent_to_tool": {
            from: "agent_1",
            to: "tool_1",
            condition: { type: "llm", description: "When user asks for help" }
          },
          "edge_tool_to_end": {
            from: "tool_1",
            to: "end_1",
            condition: { type: "result", expected: "success" }
          }
        }
      };

      await createAgentApi(
        TEST_CTX,
        "Complex Workflow Agent",
        { agent: { prompt: { prompt: "test", temperature: 0 } } } as unknown as Record<string, unknown>,
        undefined,
        complexWorkflow,
        ["complex"]
      );

      const body = sentBody(fetchMock);

      // The pulled snake_case workflow must reach the wire unchanged: node/edge
      // identifier keys AND schema fields (agent_id, tool_id, initial_message)
      expect(body.workflow).toEqual(complexWorkflow);
    });
  });
});
