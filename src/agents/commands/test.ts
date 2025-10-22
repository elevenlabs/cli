import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import TestView from '../ui/TestView.js';
import { runAgentTests } from './test-impl.js';

export function createTestCommand(): Command {
  return new Command('test')
    .description('Run tests for an agent')
    .argument('<agent>', 'Name or ID of the agent to test')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (agentId: string, options: { ui: boolean }) => {
      try {
        if (options.ui !== false) {
          // Use Ink UI for testing
          // Note: TestView requires agentName, agentId, and testIds
          // For now, we'll use the implementation function which will gather these details
          await runAgentTests(agentId);
        } else {
          await runAgentTests(agentId);
        }
      } catch (error) {
        console.error(`Error running tests: ${error}`);
        process.exit(1);
      }
    });
}
