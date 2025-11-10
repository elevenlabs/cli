import React, { useState, useEffect } from 'react';
import { Box, Text, useApp } from 'ink';
import App from '../../ui/App.js';
import StatusCard from '../../ui/components/StatusCard.js';
import theme from '../../ui/themes/elevenlabs.js';
import { getApiKey, getResidency } from '../../shared/config.js';

interface WhoamiViewProps {
  onComplete?: () => void;
}

export const WhoamiView: React.FC<WhoamiViewProps> = ({ onComplete }) => {
  const { exit } = useApp();
  const [loading, setLoading] = useState(true);
  const [apiKey, setApiKey] = useState<string | null>(null);
  const [residency, setResidency] = useState<string | null>(null);
  const [usingEnvVar, setUsingEnvVar] = useState(false);

  useEffect(() => {
    const loadStatus = async () => {
      try {
        const key = await getApiKey();
        setApiKey(key || null);
        setUsingEnvVar(!!process.env.ELEVENLABS_API_KEY);

        const res = await getResidency();
        setResidency(res);

        setLoading(false);
      } catch (error) {
        setLoading(false);
      }
    };

    loadStatus();

    // Auto-exit after 5 seconds
    const timer = setTimeout(() => {
      if (onComplete) {
        onComplete();
      } else {
        exit();
      }
    }, 5000);

    return () => clearTimeout(timer);
  }, [exit, onComplete]);

  return (
    <App
      title="ElevenLabs"
    >
      <Box flexDirection="column" gap={1}>
        {loading ? (
          <StatusCard
            title="Loading"
            status="loading"
            message="Checking authentication status..."
          />
        ) : apiKey ? (
          <>
            <StatusCard
              title="Authentication Status"
              status="success"
              message="Logged in to ElevenLabs"
              details={[
                `Residency: ${residency || 'Global'}`,
                usingEnvVar ? 'Source: Environment Variable' : 'Source: Stored API Key'
              ]}
            />

            <Box marginTop={1} flexDirection="column">
              <Text color={theme.colors.text.secondary}>
                • API Key: <Text color={theme.colors.text.muted}>{apiKey.slice(0, 8)}...{apiKey.slice(-4)}</Text>
              </Text>
              <Text color={theme.colors.text.secondary}>
                • Region: <Text color={theme.colors.accent.primary}>{residency || 'Global'}</Text>
              </Text>
              {usingEnvVar && (
                <Text color={theme.colors.warning}>
                  • Source: <Text color={theme.colors.accent.primary}>Environment Variable</Text>
                </Text>
              )}
            </Box>

            <Box marginTop={1}>
              <Text color={theme.colors.text.muted}>
                Use 'elevenlabs auth logout' to sign out
              </Text>
            </Box>
          </>
        ) : (
          <>
            <StatusCard
              title="Authentication Status"
              status="warning"
              message="Not logged in"
              details={[
                "No API key found"
              ]}
            />
            
            <Box marginTop={1}>
              <Text color={theme.colors.warning}>
                ⚠ You are not logged in to ElevenLabs
              </Text>
            </Box>
            
            <Box marginTop={1}>
              <Text color={theme.colors.text.secondary}>
                Run 'elevenlabs auth login' to authenticate with your API key
              </Text>
            </Box>
          </>
        )}
        
        <Box marginTop={1}>
          <Text color={theme.colors.text.muted} dimColor>
            Press Ctrl+C to exit (auto-exit in 5s)
          </Text>
        </Box>
      </Box>
    </App>
  );
};

export default WhoamiView;