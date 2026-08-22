import { describe, expect, it } from "@jest/globals";
import { formatApiKeyValidationError } from "../api-key-validation";

describe("formatApiKeyValidationError", () => {
  it("explains the required scope for a permission-restricted key", () => {
    const error = {
      statusCode: 401,
      body: {
        detail: {
          code: "unauthorized",
          status: "missing_permissions",
        },
      },
    };

    expect(formatApiKeyValidationError(error)).toBe(
      'API key is missing the required "Conversational AI: Read" permission ' +
        "(convai_read). Enable it at https://elevenlabs.io/app/settings/api-keys."
    );
  });

  it("supports missing_permissions in the current detail code field", () => {
    const error = {
      statusCode: 401,
      body: { detail: { code: "missing_permissions" } },
    };

    expect(formatApiKeyValidationError(error)).toContain("convai_read");
  });

  it("distinguishes an invalid or expired key", () => {
    expect(formatApiKeyValidationError({ statusCode: 401 })).toBe(
      "Invalid or expired API key. Create or edit a key at " +
        "https://elevenlabs.io/app/settings/api-keys."
    );
  });

  it("suggests checking both scope and IP restrictions for other access denials", () => {
    expect(formatApiKeyValidationError({ statusCode: 403 })).toBe(
      "API key access was denied. Check its Conversational AI read permission " +
        "and IP restrictions at https://elevenlabs.io/app/settings/api-keys."
    );
  });

  it("keeps network failures separate from credential failures", () => {
    expect(formatApiKeyValidationError({ code: "ENOTFOUND" })).toBe(
      "Network error: Unable to connect to ElevenLabs API"
    );
  });

  it("preserves unexpected SDK error messages", () => {
    expect(
      formatApiKeyValidationError(new Error("Unexpected response shape"))
    ).toBe("Error verifying API key: Unexpected response shape");
  });
});
