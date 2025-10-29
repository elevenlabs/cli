// @ts-nocheck
import React, { useEffect } from "react";
import { Box, Text, useApp } from "ink";
import App from "../../ui/App.js";
import theme from "../../ui/themes/elevenlabs.js";

interface Command {
  name: string;
  description: string;
  options?: Array<{ flag: string; description: string }>;
}

const commands: Command[] = [
  {
    name: "add <name>",
    description: "Add a new tool",
    options: [
      { flag: "--type <type>", description: "Tool type: webhook or client (default: 'webhook')" },
      { flag: "--config-path <path>", description: "Custom config path" },
    ],
  },
  {
    name: "delete [tool_id]",
    description: "Delete a tool locally and from ElevenLabs",
    options: [
      { flag: "--all", description: "Delete all tools" },
    ],
  },
  {
    name: "push",
    description: "Push tools to ElevenLabs API",
    options: [
      { flag: "--dry-run", description: "Show what would be done without making changes" },
    ],
  },
  {
    name: "pull",
    description: "Pull tools from ElevenLabs",
    options: [
      { flag: "--tool <tool_id>", description: "Specific tool ID to pull" },
      { flag: "--output-dir <directory>", description: "Output directory for configs (default: 'tool_configs')" },
      { flag: "--dry-run", description: "Show what would be done without making changes" },
      { flag: "--update", description: "Update existing items only, skip new" },
      { flag: "--all", description: "Pull all (new + existing)" },
    ],
  },
];

export const ToolsHelpView: React.FC = () => {
  const { exit } = useApp();

  useEffect(() => {
    const timer = setTimeout(() => {
      exit();
    }, 100);

    return () => clearTimeout(timer);
  }, [exit]);

  return (
    <App>
      <Box flexDirection="column" marginBottom={1}>
        <Box marginBottom={1}>
          <Text color={theme.colors.accent.primary} bold>
            elevenlabs tools
          </Text>
          <Text color={theme.colors.text.secondary}> - Tool management commands</Text>
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
            elevenlabs tools &lt;command&gt; [options]
          </Text>
        </Box>
      </Box>

      <Box flexDirection="column" marginBottom={1}>
        <Box marginBottom={1}>
          <Text color={theme.colors.accent.primary} bold>
            Commands:
          </Text>
        </Box>

        <Box flexDirection="column">
          {commands.map((cmd, index) => (
            <Box key={index} flexDirection="column" marginBottom={1}>
              <Box marginLeft={2}>
                <Box width={28}>
                  <Text color={theme.colors.text.primary}>{cmd.name}</Text>
                </Box>
                <Text color={theme.colors.text.secondary}>{cmd.description}</Text>
              </Box>
              {cmd.options && cmd.options.length > 0 && (
                <Box flexDirection="column" marginLeft={4}>
                  {cmd.options.map((opt, optIndex) => (
                    <Box key={optIndex}>
                      <Box width={26}>
                        <Text color={theme.colors.text.muted}>{opt.flag}</Text>
                      </Box>
                      <Text color={theme.colors.text.muted}>{opt.description}</Text>
                    </Box>
                  ))}
                </Box>
              )}
            </Box>
          ))}
        </Box>
      </Box>

      <Box marginTop={1}>
        <Text color={theme.colors.text.muted}>
          Disable UI mode with --no-ui flag for any command
        </Text>
      </Box>
    </App>
  );
};

export default ToolsHelpView;
