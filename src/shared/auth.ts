/**
 * Simple credential management for CLI
 * Secure file storage with proper permissions
 */

import fs from 'fs-extra';
import path from 'path';
import os from 'os';

/**
 * Get config directory path
 */
function getConfigDir(): string {
  return path.join(os.homedir(), '.elevenlabs');
}

/**
 * Get API key file path
 */
function getApiKeyFile(): string {
  return path.join(getConfigDir(), 'api_key');
}

/**
 * Ensure config directory exists with secure permissions
 */
async function ensureConfigDir(): Promise<void> {
  const configDir = getConfigDir();
  await fs.ensureDir(configDir);
  
  // Set secure permissions on Unix-like systems
  if (process.platform !== 'win32') {
    await fs.chmod(configDir, 0o700);
  }
}

/**
 * Store API key securely
 */
export async function storeApiKey(apiKey: string): Promise<void> {
  await ensureConfigDir();
  const keyFile = getApiKeyFile();

  await fs.writeFile(keyFile, apiKey.trim(), {
    mode: 0o600,
    encoding: 'utf-8'
  });
}

/**
 * Retrieve API key from secure storage
 */
export async function retrieveApiKey(): Promise<string | undefined> {
  const envKey = process.env.ELEVENLABS_API_KEY;
  if (envKey) {
    return envKey;
  }

  await ensureConfigDir();
  const keyFile = getApiKeyFile();

  try {
    if (await fs.pathExists(keyFile)) {
      const content = await fs.readFile(keyFile, 'utf-8');
      return content.trim();
    }
  } catch {
    // File not readable
  }

  return undefined;
}

/**
 * Remove API key from secure storage
 */
export async function removeApiKey(): Promise<void> {
  try {
    const keyFile = getApiKeyFile();
    if (await fs.pathExists(keyFile)) {
      await fs.remove(keyFile);
    }
  } catch {
    // Ignore errors
  }
}

/**
 * Check if user has API key stored
 */
export async function hasApiKey(): Promise<boolean> {
  const apiKey = await retrieveApiKey();
  return !!apiKey;
}