import { findMissingPaths, verifyAgentPush, verifyToolPush } from "../shared/verify";
import { ElevenLabsClient } from "@elevenlabs/elevenlabs-js";
import { ElevenLabs } from "@elevenlabs/elevenlabs-js";

describe("findMissingPaths", () => {
  it("returns no paths when every expected key persists", () => {
    const expected = { a: 1, b: { c: "x" } };
    const actual = { a: 2, b: { c: "y" }, extra: true };

    expect(findMissingPaths(expected, actual)).toEqual([]);
  });

  it("flags a missing top-level key", () => {
    const expected = { a: 1, gone: true };
    const actual = { a: 1 };

    expect(findMissingPaths(expected, actual)).toEqual(["gone"]);
  });

  it("flags nested missing keys with dotted paths", () => {
    const expected = {
      conversation_config: {
        agent: {
          dynamic_variables: {
            dynamic_variable_placeholders: { transaction_id: "x" },
          },
        },
      },
    };
    const actual = {
      conversation_config: {
        agent: {
          dynamic_variables: {},
        },
      },
    };

    expect(findMissingPaths(expected, actual)).toEqual([
      "conversation_config.agent.dynamic_variables.dynamic_variable_placeholders",
    ]);
  });

  it("treats arrays and scalars as leaves", () => {
    const expected = { tags: ["a", "b"], turn_timeout: 7 };
    const actual = { tags: [], turn_timeout: 30 };

    // Present keys count even when values differ; values are not compared
    expect(findMissingPaths(expected, actual)).toEqual([]);
  });

  it("flags every expected child when the actual value is not an object", () => {
    const expected = { turn: { turn_timeout: 7, turn_model: "turn_v3" } };
    const actual = { turn: "invalid" };

    expect(findMissingPaths(expected, actual)).toEqual([
      "turn.turn_timeout",
      "turn.turn_model",
    ]);
  });

  it("flags an expected empty object whose key is absent", () => {
    const expected = { dynamic_variables: { dynamic_variable_placeholders: {} } };
    const actual = { dynamic_variables: {} };

    expect(findMissingPaths(expected, actual)).toEqual([
      "dynamic_variables.dynamic_variable_placeholders",
    ]);
  });

  it("returns no paths when expected is not an object", () => {
    expect(findMissingPaths("scalar", { a: 1 })).toEqual([]);
    expect(findMissingPaths(null, undefined)).toEqual([]);
  });
});

describe("verifyAgentPush", () => {
  let logSpy: jest.SpyInstance;

  beforeEach(() => {
    logSpy = jest.spyOn(console, "log").mockImplementation(() => {});
  });

  afterEach(() => {
    logSpy.mockRestore();
  });

  function loggedText(): string {
    return logSpy.mock.calls.map((call) => call.join(" ")).join("\n");
  }

  function makeClient(liveResponse: unknown) {
    const get = jest.fn().mockResolvedValue(liveResponse);
    return {
      conversationalAi: { agents: { get } },
    } as unknown as ElevenLabsClient;
  }

  it("warns with the dropped field paths when the API did not persist a field", async () => {
    // Live agent is missing the placeholders the local config carries
    const client = makeClient({
      agentId: "agent_123",
      conversationConfig: {
        agent: {
          prompt: { prompt: "hi" },
          dynamicVariables: {},
        },
      },
    });

    await verifyAgentPush(client, "My Agent", "agent_123", {
      conversation_config: {
        agent: {
          prompt: { prompt: "hi" },
          dynamic_variables: {
            dynamic_variable_placeholders: { transaction_id: "x" },
          },
        },
      },
    });

    const output = loggedText();
    expect(output).toContain("⚠ My Agent: 1 field(s) in the local config were not persisted");
    expect(output).toContain("conversation_config.agent.dynamic_variables.dynamic_variable_placeholders");
    expect(output).toContain("elevenlabs agents pull");
  });

  it("does not warn when the live config is a superset of the pushed config", async () => {
    const client = makeClient({
      agentId: "agent_123",
      conversationConfig: {
        agent: { prompt: { prompt: "hi", temperature: 0 } },
        conversation: { clientEvents: ["audio"] },
      },
      platformSettings: { widget: { textInputEnabled: true } },
      tags: ["prod"],
    });

    await verifyAgentPush(client, "My Agent", "agent_123", {
      conversation_config: {
        agent: { prompt: { prompt: "hi" } },
      },
      platform_settings: { widget: { text_input_enabled: true } },
    });

    expect(loggedText()).not.toContain("⚠");
  });

  it("does not flag the deprecated 'tools' field when tool_ids is present", async () => {
    // The push payload intentionally removes 'tools' when 'tool_ids' exists,
    // so verification must not report it as dropped
    const client = makeClient({
      agentId: "agent_123",
      conversationConfig: {
        agent: { prompt: { prompt: "hi", toolIds: ["tool_1"] } },
      },
    });

    await verifyAgentPush(client, "My Agent", "agent_123", {
      conversation_config: {
        agent: {
          prompt: {
            prompt: "hi",
            tools: [{ type: "webhook", name: "legacy" }],
            tool_ids: ["tool_1"],
          },
        },
      },
    });

    expect(loggedText()).not.toContain("⚠");
  });

  it("warns but does not throw when the verification read fails", async () => {
    const get = jest.fn().mockRejectedValue(new Error("network down"));
    const client = {
      conversationalAi: { agents: { get } },
    } as unknown as ElevenLabsClient;

    await expect(
      verifyAgentPush(client, "My Agent", "agent_123", {
        conversation_config: { agent: { prompt: { prompt: "hi" } } },
      })
    ).resolves.toBeUndefined();

    expect(loggedText()).toContain("could not verify the pushed config");
  });

  it("passes the branch id through to the read-back", async () => {
    const get = jest.fn().mockResolvedValue({ agentId: "agent_123", conversationConfig: {} });
    const client = {
      conversationalAi: { agents: { get } },
    } as unknown as ElevenLabsClient;

    await verifyAgentPush(client, "My Agent", "agent_123", {}, "agtbrch_42");

    expect(get).toHaveBeenCalledWith("agent_123", { branchId: "agtbrch_42" });
  });
});

describe("verifyToolPush", () => {
  let logSpy: jest.SpyInstance;

  beforeEach(() => {
    logSpy = jest.spyOn(console, "log").mockImplementation(() => {});
  });

  afterEach(() => {
    logSpy.mockRestore();
  });

  function loggedText(): string {
    return logSpy.mock.calls.map((call) => call.join(" ")).join("\n");
  }

  function makeToolResponse(toolConfig: unknown): ElevenLabs.ToolResponseModel {
    return { id: "tool_123", toolConfig } as unknown as ElevenLabs.ToolResponseModel;
  }

  it("warns when the persisted tool config is missing pushed fields", () => {
    const response = makeToolResponse({
      name: "lookup",
      type: "webhook",
      apiSchema: { url: "https://api.example.com/lookup" },
    });

    verifyToolPush("lookup", {
      name: "lookup",
      type: "webhook",
      api_schema: {
        url: "https://api.example.com/lookup",
        path_params_schema: { transaction_id: { type: "string" } },
      },
    }, response);

    const output = loggedText();
    expect(output).toContain("⚠ lookup:");
    expect(output).toContain("api_schema.path_params_schema");
    expect(output).toContain("elevenlabs tools pull");
  });

  it("does not warn when every pushed field persisted", () => {
    const response = makeToolResponse({
      name: "lookup",
      type: "webhook",
      apiSchema: { url: "https://api.example.com/lookup", method: "GET" },
      responseTimeoutSecs: 30,
    });

    verifyToolPush("lookup", {
      name: "lookup",
      type: "webhook",
      api_schema: { url: "https://api.example.com/lookup", method: "GET" },
      response_timeout_secs: 30,
    }, response);

    expect(loggedText()).not.toContain("⚠");
  });

  it("truncates long lists of dropped fields", () => {
    const pushed: Record<string, unknown> = { name: "lookup" };
    for (let i = 0; i < 12; i++) {
      pushed[`missing_field_${i}`] = i;
    }

    verifyToolPush("lookup", pushed, makeToolResponse({ name: "lookup" }));

    const output = loggedText();
    expect(output).toContain("12 field(s) in the local config were not persisted");
    expect(output).toContain("... and 2 more");
  });

  it("warns when the response carries no tool config", () => {
    verifyToolPush("lookup", { name: "lookup" }, undefined);

    expect(loggedText()).toContain("could not verify the pushed config");
  });
});
