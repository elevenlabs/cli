// @ts-nocheck
import React, { useEffect } from "react";
import { Box, Text, useApp } from "ink";
import App from "../../ui/App.js";
import theme from "../../ui/themes/elevenlabs.js";

interface Command {
  name: string;
  description: string;
}

const commands: Command[] = [
  {
    name: "add [name]",
    description: "Add a component from the ElevenLabs UI registry",
  },
];

export const ComponentsHelpView: React.FC = () => {
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
            elevenlabs components
          </Text>
          <Text color={theme.colors.text.secondary}> - UI component management</Text>
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
            elevenlabs components &lt;command&gt; [options]
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
            <Box key={index} marginLeft={2}>
              <Box width={28}>
                <Text color={theme.colors.text.primary}>{cmd.name}</Text>
              </Box>
              <Text color={theme.colors.text.secondary}>{cmd.description}</Text>
            </Box>
          ))}
        </Box>
      </Box>

      <Box marginTop={1}>
        <Text color={theme.colors.text.muted}>
          Components are sourced from https://ui.elevenlabs.io
        </Text>
      </Box>
    </App>
  );
};

export default ComponentsHelpView;
