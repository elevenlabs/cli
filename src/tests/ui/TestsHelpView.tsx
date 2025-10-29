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
    description: "Add a new test",
    options: [
      { flag: "--template <template>", description: "Test template type (default: 'basic-llm')" },
    ],
  },
  {
    name: "delete [test_id]",
    description: "Delete a test locally and from ElevenLabs",
    options: [
      { flag: "--all", description: "Delete all tests" },
    ],
  },
  {
    name: "push",
    description: "Push tests to ElevenLabs API",
    options: [
      { flag: "--dry-run", description: "Show what would be done without making changes" },
    ],
  },
  {
    name: "pull",
    description: "Pull tests from ElevenLabs",
    options: [
      { flag: "--test <test_id>", description: "Specific test ID to pull" },
      { flag: "--output-dir <directory>", description: "Output directory for configs (default: 'test_configs')" },
      { flag: "--dry-run", description: "Show what would be done without making changes" },
      { flag: "--update", description: "Update existing items only, skip new" },
      { flag: "--all", description: "Pull all (new + existing)" },
    ],
  },
];

export const TestsHelpView: React.FC = () => {
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
            elevenlabs tests
          </Text>
          <Text color={theme.colors.text.secondary}> - Test management commands</Text>
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
            elevenlabs tests &lt;command&gt; [options]
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

export default TestsHelpView;
