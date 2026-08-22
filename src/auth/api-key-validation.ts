const API_KEY_SETTINGS_URL = "https://elevenlabs.io/app/settings/api-keys";

type ErrorRecord = Record<string, unknown>;

function asRecord(value: unknown): ErrorRecord | undefined {
  return typeof value === "object" && value !== null
    ? (value as ErrorRecord)
    : undefined;
}

function getDetail(error: ErrorRecord | undefined): ErrorRecord | undefined {
  const body = asRecord(error?.body);
  return asRecord(body?.detail);
}

function getString(
  record: ErrorRecord | undefined,
  key: string
): string | undefined {
  const value = record?.[key];
  return typeof value === "string" ? value : undefined;
}

export function formatApiKeyValidationError(error: unknown): string {
  const err = asRecord(error);
  const detail = getDetail(err);
  const detailCode = getString(detail, "code");
  const legacyDetailStatus = getString(detail, "status");
  const message = getString(err, "message");
  const code = getString(err, "code");
  const statusCode =
    typeof err?.statusCode === "number" ? err.statusCode : undefined;

  // ElevenLabs currently returns missing endpoint permissions as HTTP 401,
  // so this check must precede the generic invalid-key branch.
  if (
    detailCode === "missing_permissions" ||
    legacyDetailStatus === "missing_permissions"
  ) {
    return (
      'API key is missing the required "Conversational AI: Read" permission ' +
      `(convai_read). Enable it at ${API_KEY_SETTINGS_URL}.`
    );
  }

  if (statusCode === 401 || message?.includes("401")) {
    return `Invalid or expired API key. Create or edit a key at ${API_KEY_SETTINGS_URL}.`;
  }

  if (statusCode === 403 || message?.includes("403")) {
    return (
      "API key access was denied. Check its Conversational AI read permission " +
      `and IP restrictions at ${API_KEY_SETTINGS_URL}.`
    );
  }

  if (
    code === "ENOTFOUND" ||
    code === "ETIMEDOUT" ||
    message?.toLowerCase().includes("network")
  ) {
    return "Network error: Unable to connect to ElevenLabs API";
  }

  return `Error verifying API key: ${message ?? String(error)}`;
}
