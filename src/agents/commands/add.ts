import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import path from 'path';
import fs from 'fs-extra';
import AddAgentView from '../ui/AddAgentView.js';
import { readConfig, writeConfig, generateUniqueFilename } from '../../shared/utils.js';
import { getTemplateByName, AgentConfig } from '../templates.js';
import { getElevenLabsClient, createAgentApi } from '../../shared/elevenlabs-api.js';

const AGENTS_CONFIG_FILE = "agents.json";

interface AgentsConfig {
  agents: Array<{
    config: string;
    id?: string;
  }>;
}

interface AgentDefinition {
  config: string;
  id?: string;
}

interface AddOptions {
  configPath?: string;
  template?: string;
}

export function createAddCommand(): Command {
  return new Command('add')
    .description('Add a new agent - creates config, uploads to ElevenLabs, and saves ID')
    .argument('[name]', 'Name of the agent to create')
    .option('--config-path <path>', 'Custom config path (optional)')
    .option('--template <template>', 'Template type to use')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (name: string | undefined, options: AddOptions & { ui: boolean }) => {
      try {
        if (options.ui !== false && !options.configPath) {
          // Use Ink UI for agent creation
          const { waitUntilExit } = render(
            React.createElement(AddAgentView, {
              initialName: name,
              template: options.template
            })
          );
          await waitUntilExit();
          return;
        }

        // Non-UI path requires name to be provided
        if (!name) {
          console.error('Error: Agent name is required when using --no-ui or --config-path');
          process.exit(1);
        }

        // Check if agents.json exists
        const agentsConfigPath = path.resolve(AGENTS_CONFIG_FILE);
        if (!(await fs.pathExists(agentsConfigPath))) {
          console.error('agents.json not found. Run \'elevenlabs agents init\' first.');
          process.exit(1);
        }

        // Load existing config
        const agentsConfig = await readConfig<AgentsConfig>(agentsConfigPath);

        // Create agent config using template (in memory first)
        let agentConfig: AgentConfig;
        try {
          agentConfig = getTemplateByName(name, options.template || 'default');
        } catch (error) {
          console.error(`${error}`);
          process.exit(1);
        }

        // Create agent in ElevenLabs first to get ID
        console.log(`Creating agent '${name}' in ElevenLabs...`);

        const client = await getElevenLabsClient();

        // Extract config components
        const conversationConfig = agentConfig.conversation_config || {};
        const platformSettings = agentConfig.platform_settings;
        const tags = agentConfig.tags || [];

        // Create new agent
        const agentId = await createAgentApi(
          client,
          name,
          conversationConfig,
          platformSettings,
          tags
        );

        console.log(`Created agent in ElevenLabs with ID: ${agentId}`);

        // Generate config path using agent name (or custom path if provided)
        let configPath = options.configPath;
        if (!configPath) {
          configPath = await generateUniqueFilename('agent_configs', name);
        }

        // Create config directory and file
        const configFilePath = path.resolve(configPath);
        await fs.ensureDir(path.dirname(configFilePath));

        await writeConfig(configFilePath, agentConfig);
        console.log(`Created config file: ${configPath} (template: ${options.template || 'default'})`);

        // Store agent ID in index file
        const newAgent: AgentDefinition = {
          config: configPath,
          id: agentId
        };
        agentsConfig.agents.push(newAgent);

        // Save updated agents.json
        await writeConfig(agentsConfigPath, agentsConfig);
        console.log(`Added agent '${name}' to agents.json`);

        console.log(`Edit ${configPath} to customize your agent, then run 'elevenlabs agents push' to update`);

      } catch (error) {
        console.error(`Error creating agent: ${error}`);
        process.exit(1);
      }
    });
}
