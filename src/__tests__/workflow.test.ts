import { createAgentApi, updateAgentApi, getAgentApi } from "../shared/elevenlabs-api";
import { ElevenLabsClient } from "@elevenlabs/elevenlabs-js";

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

    const create = jest.fn().mockResolvedValue({ agentId: "agent_workflow_123" });
    const update = jest.fn().mockResolvedValue({ agentId: "agent_workflow_123" });
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
        agents: { create, update, get },
      },
    } as unknown as ElevenLabsClient;
  }

  describe("createAgentApi", () => {
    it("should send workflow when provided", async () => {
      const client = makeMockClient();
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
        client,
        "Agent with Workflow",
        conversation_config,
        undefined,
        workflow,
        ["workflow"]
      );

      expect(client.conversationalAi.agents.create).toHaveBeenCalledTimes(1);
      const payload = (client.conversationalAi.agents.create as jest.Mock).mock.calls[0][0];

      // Workflow node/edge identifier keys should be preserved (not camel-cased)
      // Only schema fields within nodes/edges should be converted
      expect(payload).toEqual(
        expect.objectContaining({
          name: "Agent with Workflow",
          workflow: expect.objectContaining({
            nodes: expect.objectContaining({
              start: expect.any(Object),   // "start" has no underscore, stays as-is
              end: expect.any(Object),     // "end" has no underscore, stays as-is
            }),
            edges: expect.objectContaining({
              edge_1: expect.any(Object),  // edge_1 preserved as identifier
            }),
          }),
          tags: ["workflow"],
        })
      );
    });

    it("should handle undefined workflow gracefully", async () => {
      const client = makeMockClient();
      const conversation_config = {
        conversation: {
          client_events: ["audio"],
        },
        agent: { prompt: { prompt: "hi", temperature: 0 } },
      } as unknown as Record<string, unknown>;

      await createAgentApi(
        client,
        "Agent without Workflow",
        conversation_config,
        undefined,
        undefined,
        []
      );

      expect(client.conversationalAi.agents.create).toHaveBeenCalledTimes(1);
      const payload = (client.conversationalAi.agents.create as jest.Mock).mock.calls[0][0];

      expect(payload).toEqual(
        expect.objectContaining({
          name: "Agent without Workflow",
          workflow: undefined,
        })
      );
    });
  });

  describe("updateAgentApi", () => {
    it("should send workflow when updating an agent", async () => {
      const client = makeMockClient();
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
        client,
        "agent_workflow_123",
        "Updated Agent",
        conversation_config,
        undefined,
        workflow,
        ["updated"]
      );

      expect(client.conversationalAi.agents.update).toHaveBeenCalledTimes(1);
      const [agentId, payload] = (
        client.conversationalAi.agents.update as jest.Mock
      ).mock.calls[0];

      expect(agentId).toBe("agent_workflow_123");
      // Workflow node/edge identifier keys should be preserved (not camel-cased)
      expect(payload).toEqual(
        expect.objectContaining({
          name: "Updated Agent",
          workflow: expect.objectContaining({
            nodes: expect.objectContaining({
              updated_start: expect.any(Object),  // preserved as identifier
              updated_end: expect.any(Object),    // preserved as identifier
            }),
          }),
          tags: ["updated"],
        })
      );
    });

    it("should allow clearing workflow by passing undefined", async () => {
      const client = makeMockClient();
      const conversation_config = {
        conversation: {
          client_events: ["audio"],
        },
      } as unknown as Record<string, unknown>;

      await updateAgentApi(
        client,
        "agent_workflow_123",
        "Agent Workflow Cleared",
        conversation_config,
        undefined,
        undefined,
        []
      );

      expect(client.conversationalAi.agents.update).toHaveBeenCalledTimes(1);
      const [, payload] = (
        client.conversationalAi.agents.update as jest.Mock
      ).mock.calls[0];

      expect(payload).toEqual(
        expect.objectContaining({
          workflow: undefined,
        })
      );
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
      const client = makeMockClient();

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
        client,
        "Complex Workflow Agent",
        { agent: { prompt: { prompt: "test", temperature: 0 } } } as unknown as Record<string, unknown>,
        undefined,
        complexWorkflow,
        ["complex"]
      );

      const payload = (client.conversationalAi.agents.create as jest.Mock).mock.calls[0][0];

      // Workflow node/edge identifier keys should be preserved (not camel-cased)
      expect(payload.workflow.nodes).toHaveProperty("start_1");     // preserved as identifier
      expect(payload.workflow.nodes).toHaveProperty("agent_1");     // preserved as identifier
      expect(payload.workflow.nodes).toHaveProperty("tool_1");      // preserved as identifier
      expect(payload.workflow.nodes).toHaveProperty("end_1");       // preserved as identifier
      expect(payload.workflow.edges).toHaveProperty("edge_start_to_agent");  // preserved as identifier
      expect(payload.workflow.edges).toHaveProperty("edge_agent_to_tool");   // preserved as identifier
      expect(payload.workflow.edges).toHaveProperty("edge_tool_to_end");     // preserved as identifier

      // Verify nested schema properties ARE still converted to camelCase
      expect(payload.workflow.nodes.start_1.config).toHaveProperty("initialMessage"); // initial_message → initialMessage
      expect(payload.workflow.nodes.agent_1).toHaveProperty("agentId");    // agent_id → agentId
      expect(payload.workflow.nodes.tool_1).toHaveProperty("toolId");      // tool_id → toolId
    });
  });
});
