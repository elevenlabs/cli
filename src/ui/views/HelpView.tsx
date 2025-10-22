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

interface Command {
  name: string;
  description: string;
  subcommands?: Command[];
}

const commands: Command[][] = [
  [
    {
      name: "auth",
      description: "Authentication commands",
      subcommands: [
        {
          name: "login",
          description: "Login with your ElevenLabs API key",
        },
        {
          name: "logout",
          description: "Logout and remove stored API key",
        },
        {
          name: "whoami",
          description: "Show current login status",
        },
        {
          name: "residency [location]",
          description: "Set the API residency location",
        },
      ],
    },
  ],
  [
    {
      name: "agents",
      description: "Agent management commands",
      subcommands: [
        {
          name: "init [path]",
          description: "Initialize project (use --override to recreate)",
        },
        {
          name: "add <name>",
          description: "Create a new agent and push to remote",
        },
        {
          name: "list",
          description: "List all local agents",
        },
        {
          name: "delete [agent_id]",
          description: "Delete agent (use --all for all)",
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
          description: "Pull agents (--update, --all options)",
        },
        {
          name: "test <agent>",
          description: "Run tests for an agent",
        },
        {
          name: "templates",
          description: "Manage agent templates (list, show)",
        },
        {
          name: "widget <name>",
          description: "Generate HTML widget snippet",
        },
      ],
    },
  ],
  [
    {
      name: "tools",
      description: "Tool management commands",
      subcommands: [
        {
          name: "add <name>",
          description: "Add tool (--type webhook|client)",
        },
        {
          name: "delete [tool_id]",
          description: "Delete tool (use --all for all)",
        },
        {
          name: "push",
          description: "Push tools to ElevenLabs API",
        },
        {
          name: "pull",
          description: "Pull tools (--update, --all options)",
        },
      ],
    },
  ],
  [
    {
      name: "tests",
      description: "Test management commands",
      subcommands: [
        {
          name: "add <name>",
          description: "Add a new test",
        },
        {
          name: "delete [test_id]",
          description: "Delete test (use --all for all)",
        },
        {
          name: "push",
          description: "Push tests to ElevenLabs API",
        },
        {
          name: "pull",
          description: "Pull tests (--update, --all options)",
        },
      ],
    },
  ],
  [
    {
      name: "components",
      description: "UI component management",
      subcommands: [
        {
          name: "add [name]",
          description: "Add component from ElevenLabs UI registry",
        },
      ],
    },
  ],
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
            Commands:
          </Text>
        </Box>

        {commands.map((group, groupIndex) => (
          <Box key={groupIndex} flexDirection="column" marginBottom={1}>
            {group.map((cmd, cmdIndex) => (
              <Box key={cmdIndex} flexDirection="column">
                <Box marginLeft={2}>
                  <Box width={24}>
                    <Text color={theme.colors.text.primary}>{cmd.name}</Text>
                  </Box>
                  <Text color={theme.colors.text.secondary}>{cmd.description}</Text>
                </Box>
                {cmd.subcommands &&
                  cmd.subcommands.map((subcmd, subIndex) => (
                    <Box key={subIndex} marginLeft={4}>
                      <Box width={22}>
                        <Text color={theme.colors.text.muted}>{subcmd.name}</Text>
                      </Box>
                      <Text color={theme.colors.text.muted}>
                        {subcmd.description}
                      </Text>
                    </Box>
                  ))}
              </Box>
            ))}
          </Box>
        ))}
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
          For more information on a command, use: elevenlabs [command] --help
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
