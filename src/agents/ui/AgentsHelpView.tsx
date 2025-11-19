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
    name: "init [path]",
    description: "Initialize project",
    options: [
      { flag: "--override", description: "Recreate existing project" },
    ],
  },
  {
    name: "add [name]",
    description: "Create a new agent and push to remote",
    options: [
      { flag: "--output-path <path>", description: "Custom output path for config file" },
      { flag: "--from-file <path>", description: "Create agent from existing config file" },
      { flag: "--template <template>", description: "Template type to use (default, minimal, voice-only, text-only, customer-service, assistant)" },
    ],
  },
  {
    name: "list",
    description: "List all local agents",
  },
  {
    name: "delete [agent_id]",
    description: "Delete agent",
    options: [
      { flag: "--all", description: "Delete all agents" },
    ],
  },
  {
    name: "status",
    description: "Show the status of agents",
  },
  {
    name: "push",
    description: "Push agents to ElevenLabs",
  },
  {
    name: "pull",
    description: "Pull agents from ElevenLabs",
    options: [
      { flag: "--update", description: "Update existing agents" },
      { flag: "--all", description: "Pull all agents" },
    ],
  },
  {
    name: "test <agent>",
    description: "Run tests for an agent",
  },
  {
    name: "templates [action]",
    description: "Manage agent templates",
    options: [
      { flag: "list", description: "List available templates" },
      { flag: "show <name>", description: "Show template details" },
    ],
  },
  {
    name: "widget <name>",
    description: "Generate HTML widget snippet",
  },
];

export const AgentsHelpView: React.FC = () => {
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
            elevenlabs agents
          </Text>
          <Text color={theme.colors.text.secondary}> - Agent management commands</Text>
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
            elevenlabs agents &lt;command&gt; [options]
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

export default AgentsHelpView;
