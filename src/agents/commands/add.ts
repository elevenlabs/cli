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
  outputPath?: string;
  template?: string;
  fromFile?: string;
}

export function createAddCommand(): Command {
  return new Command('add')
    .description('Add a new agent - creates config, uploads to ElevenLabs, and saves ID')
    .argument('[name]', 'Name of the agent to create')
    .option('--output-path <path>', 'Custom output path for the config file (optional)')
    .option('--template <template>', 'Template type to use (default, minimal, voice-only, text-only, customer-service, assistant)')
    .option('--from-file <path>', 'Create agent from an existing config file')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (name: string | undefined, options: AddOptions & { ui: boolean }) => {
      try {
        if (options.ui !== false && !options.outputPath && !options.fromFile) {
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

        // Validate options
        if (options.fromFile && options.template) {
          console.error('Error: Cannot use both --from-file and --template options together');
          process.exit(1);
        }

        // Non-UI path requires name to be provided (or --from-file with name in the file)
        if (!name && !options.fromFile) {
          console.error('Error: Agent name is required when using --no-ui or --output-path');
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

        // Create or load agent config
        let agentConfig: AgentConfig;
        let agentName: string;

        if (options.fromFile) {
          // Load config from existing file
          console.log(`Loading agent config from '${options.fromFile}'...`);
          try {
            agentConfig = await readConfig<AgentConfig>(options.fromFile);
            // Use provided name or name from config file
            agentName = name || agentConfig.name;
            // Override the name in config if a name was explicitly provided
            if (name) {
              agentConfig.name = name;
            }
            console.log(`Loaded config for agent: ${agentName}`);
          } catch (error) {
            console.error(`Error reading config file '${options.fromFile}': ${error}`);
            process.exit(1);
          }
        } else {
          // Create agent config using template (in memory first)
          if (!name) {
            console.error('Error: Agent name is required when using templates');
            process.exit(1);
          }
          agentName = name;
          try {
            agentConfig = getTemplateByName(name, options.template || 'default');
          } catch (error) {
            console.error(`${error}`);
            process.exit(1);
          }
        }

        // Create agent in ElevenLabs first to get ID
        console.log(`Creating agent '${agentName}' in ElevenLabs...`);

        const client = await getElevenLabsClient();

        // Extract config components
        const conversationConfig = agentConfig.conversation_config || {};
        const platformSettings = agentConfig.platform_settings;
        const workflow = agentConfig.workflow;
        const tags = agentConfig.tags || [];

        // Create new agent
        const agentId = await createAgentApi(
          client,
          agentName,
          conversationConfig,
          platformSettings,
          workflow,
          tags
        );

        console.log(`Created agent in ElevenLabs with ID: ${agentId}`);

        // Generate config path using agent name (or custom path if provided)
        let configPath = options.outputPath;
        if (!configPath) {
          configPath = await generateUniqueFilename('agent_configs', agentName);
        }

        // Create config directory and file
        const configFilePath = path.resolve(configPath);
        await fs.ensureDir(path.dirname(configFilePath));

        await writeConfig(configFilePath, agentConfig);

        if (options.fromFile) {
          console.log(`Created config file: ${configPath} (from: ${options.fromFile})`);
        } else {
          console.log(`Created config file: ${configPath} (template: ${options.template || 'default'})`);
        }

        // Store agent ID in index file
        const newAgent: AgentDefinition = {
          config: configPath,
          id: agentId
        };
        agentsConfig.agents.push(newAgent);

        // Save updated agents.json
        await writeConfig(agentsConfigPath, agentsConfig);
        console.log(`Added agent '${agentName}' to agents.json`);

        console.log(`Edit ${configPath} to customize your agent, then run 'elevenlabs agents push' to update`);

      } catch (error) {
        console.error(`Error creating agent: ${error}`);
        process.exit(1);
      }
    });
}
