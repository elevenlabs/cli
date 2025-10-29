import path from 'path';
import fs from 'fs-extra';
import { readConfig } from '../../shared/utils.js';
import { getElevenLabsClient } from '../../shared/elevenlabs-api.js';
import { AgentConfig } from '../templates.js';
import { getAgentName } from './utils.js';

const AGENTS_CONFIG_FILE = "agents.json";

interface AgentDefinition {
  config: string;
  id?: string;
}

interface AgentsConfig {
  agents: AgentDefinition[];
}

export async function runAgentTests(agentId: string): Promise<void> {
  // Load agents configuration and get agent details
  const agentsConfigPath = path.resolve(AGENTS_CONFIG_FILE);
  if (!(await fs.pathExists(agentsConfigPath))) {
    throw new Error('agents.json not found. Run \'elevenlabs agents init\' first.');
  }

  const agentsConfig = await readConfig<AgentsConfig>(agentsConfigPath);
  const agentDef = agentsConfig.agents.find(agent => agent.id === agentId);

  if (!agentDef) {
    throw new Error(`Agent with ID '${agentId}' not found in configuration`);
  }

  // Get agent config to find attached tests
  const configPath = agentDef.config;

  if (!configPath || !(await fs.pathExists(configPath))) {
    const agentName = await getAgentName(configPath);
    throw new Error(`Config file not found for agent '${agentName}': ${configPath}`);
  }

  const agentConfig = await readConfig<AgentConfig>(configPath);
  const attachedTests = agentConfig.platform_settings?.testing?.attached_tests || [];
  const agentName = agentConfig.name || 'Unnamed Agent';

  if (attachedTests.length === 0) {
    throw new Error(`No tests attached to agent '${agentName}'. Add tests to the agent's testing configuration.`);
  }

  const testIds = attachedTests.map(test => test.test_id);

  // Get agent environment
  const environment = 'prod';

  console.log(`Running ${testIds.length} test(s) for agent '${agentName}' [${environment}]...`);
  console.log('');

  // Run tests without UI
  const client = await getElevenLabsClient(environment);

  try {
    // Import the API functions we need
    const { runTestsOnAgentApi, getTestInvocationApi } = await import('../../shared/elevenlabs-api.js');

    // Start the test run
    const invocationResponse = await runTestsOnAgentApi(client, agentId, testIds) as { id: string };
    const invocationId = invocationResponse.id;

    console.log(`Test invocation started (ID: ${invocationId})`);
    console.log('Waiting for tests to complete...');
    console.log('');

    // Poll for test completion
    let allComplete = false;
    let attempts = 0;
    const maxAttempts = 60; // 5 minutes with 5 second intervals

    while (!allComplete && attempts < maxAttempts) {
      await new Promise(resolve => setTimeout(resolve, 5000)); // Wait 5 seconds
      attempts++;

      const invocationStatus = await getTestInvocationApi(client, invocationId) as { test_runs?: Array<{ status: string; test_name?: string; test_id?: string }> };
      const testRuns = invocationStatus.test_runs || [];

      allComplete = testRuns.every((run: { status: string }) =>
        run.status === 'passed' || run.status === 'failed'
      );

      if (allComplete) {
        console.log('Test Results:');
        console.log('='.repeat(50));

        let passedCount = 0;
        let failedCount = 0;

        for (const testRun of testRuns) {
          const status = testRun.status === 'passed' ? '✓' : '✗';
          const testName = testRun.test_name || testRun.test_id || 'Unknown';
          console.log(`${status} ${testName}: ${testRun.status}`);

          if (testRun.status === 'passed') {
            passedCount++;
          } else {
            failedCount++;
          }
        }

        console.log('='.repeat(50));
        console.log(`Total: ${testRuns.length} | Passed: ${passedCount} | Failed: ${failedCount}`);

        if (failedCount > 0) {
          process.exit(1);
        }
      }
    }

    if (!allComplete) {
      console.error('Tests did not complete within the timeout period.');
      process.exit(1);
    }

  } catch (error) {
    console.error(`Error running tests: ${error}`);
    process.exit(1);
  }
}
