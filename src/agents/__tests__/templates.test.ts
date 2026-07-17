import { getTemplateByName } from "../templates";

describe("agent templates", () => {
  it.each([
    "default",
    "voice-only",
    "text-only",
    "customer-service",
    "assistant"
  ])("uses Scribe Realtime for the %s template", (templateName) => {
    const template = getTemplateByName("Test Agent", templateName);

    expect(template.conversation_config.asr).toMatchObject({
      provider: "scribe_realtime"
    });
  });
});
