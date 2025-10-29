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
];

export const AuthHelpView: React.FC = () => {
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
            elevenlabs auth
          </Text>
          <Text color={theme.colors.text.secondary}> - Authentication commands</Text>
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
            elevenlabs auth &lt;command&gt; [options]
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

export default AuthHelpView;
