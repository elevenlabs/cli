// @ts-nocheck
import React, { useEffect } from "react";
import { Box, Text, useApp } from "ink";
import App from "../App.js";
import theme from "../themes/elevenlabs.js";
import { readFileSync } from "fs";
import { fileURLToPath } from "url";
import { dirname, join } from "path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const packageJson = JSON.parse(
  readFileSync(join(__dirname, "../../../package.json"), "utf-8")
);
const { version } = packageJson;

interface Module {
  name: string;
  description: string;
}

const modules: Module[] = [
  {
    name: "auth",
    description: "Authentication commands",
  },
  {
    name: "agents",
    description: "Agent management commands",
  },
  {
    name: "tools",
    description: "Tool management commands",
  },
  {
    name: "tests",
    description: "Test management commands",
  },
  {
    name: "components",
    description: "UI component management",
  },
];

export const HelpView: React.FC = () => {
  const { exit } = useApp();

  useEffect(() => {
    // Auto-exit after a short delay to allow the UI to render
    const timer = setTimeout(() => {
      exit();
    }, 100);

    return () => clearTimeout(timer);
  }, [exit]);

  return (
    <App>
      <Box flexDirection="column" marginBottom={1}>
        <Box marginBottom={1}>
          <Text color={theme.colors.text.secondary}>
            ElevenLabs CLI v{version}
          </Text>
        </Box>
      </Box>

      <Box flexDirection="column" marginBottom={1}>
        <Box marginBottom={1}>
          <Text color={theme.colors.accent.primary} bold>
            Usage:
          </Text>
        </Box>
        <Box marginLeft={2}>
          <Text color={theme.colors.text.primary}>
            elevenlabs [command] [options]
          </Text>
        </Box>
      </Box>

      <Box flexDirection="column" marginBottom={1}>
        <Box marginBottom={1}>
          <Text color={theme.colors.accent.primary} bold>
            Modules:
          </Text>
        </Box>

        <Box flexDirection="column">
          {modules.map((module, index) => (
            <Box key={index} marginLeft={2}>
              <Box width={16}>
                <Text color={theme.colors.text.primary}>{module.name}</Text>
              </Box>
              <Text color={theme.colors.text.secondary}>{module.description}</Text>
            </Box>
          ))}
        </Box>
      </Box>

      <Box flexDirection="column" marginTop={1}>
        <Box marginBottom={1}>
          <Text color={theme.colors.accent.primary} bold>
            Quick Start:
          </Text>
        </Box>

        <Box flexDirection="column" marginLeft={2}>
          <Text color={theme.colors.text.secondary}>
            1. Initialize a project:{" "}
            <Text color={theme.colors.success}>elevenlabs agents init</Text>
          </Text>
          <Text color={theme.colors.text.secondary}>
            2. Login with API key:{" "}
            <Text color={theme.colors.success}>elevenlabs auth login</Text>
          </Text>
          <Text color={theme.colors.text.secondary}>
            3. Create an agent:{" "}
            <Text color={theme.colors.success}>
              elevenlabs agents add "My Agent"
            </Text>
          </Text>
          <Text color={theme.colors.text.secondary}>
            4. Push to ElevenLabs:{" "}
            <Text color={theme.colors.success}>elevenlabs agents push</Text>
          </Text>
        </Box>
      </Box>

      <Box marginTop={1}>
        <Text color={theme.colors.text.muted}>
          For more information on a module, use: elevenlabs &lt;module&gt; --help
        </Text>
      </Box>

      <Box marginTop={1}>
        <Text color={theme.colors.text.muted}>
          Disable UI mode with --no-ui flag for any command
        </Text>
      </Box>
    </App>
  );
};

export default HelpView;
