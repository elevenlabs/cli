import React, { useState, useEffect } from 'react';
import { Box, Text, useApp } from 'ink';
import App from '../../ui/App.js';
import theme from '../../ui/themes/elevenlabs.js';
import { getElevenLabsClient, listBranchesApi } from '../../shared/elevenlabs-api.js';
import type { ElevenLabs } from '@elevenlabs/elevenlabs-js';

interface BranchesListViewProps {
  agent: string;
  includeArchived: boolean;
  onComplete?: () => void;
}

export const BranchesListView: React.FC<BranchesListViewProps> = ({
  agent,
  includeArchived,
  onComplete
}) => {
  const { exit } = useApp();
  const [branches, setBranches] = useState<ElevenLabs.AgentBranchSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchBranches = async () => {
      try {
        const client = await getElevenLabsClient();
        const results = await listBranchesApi(client, agent, includeArchived);
        setBranches(results);
        setLoading(false);

        setTimeout(() => {
          if (onComplete) onComplete();
          else exit();
        }, 2000);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to list branches');
        setLoading(false);
        setTimeout(() => {
          if (onComplete) onComplete();
          else exit();
        }, 2000);
      }
    };

    fetchBranches();
  }, []);

  return (
    <App title="ElevenLabs">
      <Box flexDirection="column">
        {error ? (
          <Text color={theme.colors.error}>✗ {error}</Text>
        ) : loading ? (
          <Text color={theme.colors.text.muted}>Loading branches...</Text>
        ) : branches.length === 0 ? (
          <Text color={theme.colors.text.muted}>No branches found for this agent.</Text>
        ) : (
          <>
            <Text color={theme.colors.text.primary} bold>
              Branches:
            </Text>

            {/* Table Header */}
            <Box marginTop={1}>
              <Box width={25}>
                <Text color={theme.colors.text.muted} bold>NAME</Text>
              </Box>
              <Box width={35}>
                <Text color={theme.colors.text.muted} bold>BRANCH ID</Text>
              </Box>
              <Box width={12}>
                <Text color={theme.colors.text.muted} bold>STATUS</Text>
              </Box>
              <Box width={10}>
                <Text color={theme.colors.text.muted} bold>TRAFFIC</Text>
              </Box>
              <Box>
                <Text color={theme.colors.text.muted} bold>LAST UPDATED</Text>
              </Box>
            </Box>

            <Box marginY={0}>
              <Text color={theme.colors.text.muted}>{'─'.repeat(100)}</Text>
            </Box>

            {branches.map((branch, index) => {
              const status = branch.isArchived ? 'archived' : 'active';
              const statusColor = branch.isArchived ? theme.colors.text.muted : theme.colors.success;
              const traffic = `${branch.currentLivePercentage ?? 0}%`;
              const lastUpdated = new Date(branch.lastCommittedAt * 1000).toISOString().split('T')[0];

              return (
                <Box key={index}>
                  <Box width={25}>
                    <Text color={theme.colors.text.primary}>{branch.name}</Text>
                  </Box>
                  <Box width={35}>
                    <Text color={theme.colors.text.muted}>{branch.id}</Text>
                  </Box>
                  <Box width={12}>
                    <Text color={statusColor}>{status}</Text>
                  </Box>
                  <Box width={10}>
                    <Text color={theme.colors.text.secondary}>{traffic}</Text>
                  </Box>
                  <Box>
                    <Text color={theme.colors.text.muted}>{lastUpdated}</Text>
                  </Box>
                </Box>
              );
            })}

            <Box marginTop={1}>
              <Text color={theme.colors.text.secondary}>
                {branches.length} branch(es) found
              </Text>
            </Box>
          </>
        )}
      </Box>
    </App>
  );
};

export default BranchesListView;
