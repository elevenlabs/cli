import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import { createInitCommand } from './init.js';
import { createAddCommand } from './add.js';
import { createListCommand } from './list.js';
import { createDeleteCommand } from './delete.js';
import { createStatusCommand } from './status.js';
import { createPushCommand } from './push.js';
import { createPullCommand } from './pull.js';
import { createTemplatesCommand } from './templates.js';
import { createWidgetCommand } from './widget.js';
import { createTestCommand } from './test.js';
import { createBranchesCommand } from './branches.js';
import AgentsHelpView from '../ui/AgentsHelpView.js';

function printAgentsHelp() {
  console.log('elevenlabs agents - Agent management commands\n');
  console.log('Usage:');
  console.log('  elevenlabs agents <command> [options]\n');
  console.log('Commands:');
  const commands = [
    { name: 'init [path]', description: 'Initialize project', options: ['--override  Recreate existing project'] },
    { name: 'add [name]', description: 'Create a new agent and push to remote', options: ['--output-path <path>  Custom output path for config file', '--from-file <path>  Create agent from existing config file', '--template <template>  Template type to use (default, minimal, voice-only, text-only, customer-service, assistant)'] },
    { name: 'list', description: 'List all local agents' },
    { name: 'delete [agent_id]', description: 'Delete agent', options: ['--all  Delete all agents'] },
    { name: 'status', description: 'Show the status of agents' },
    { name: 'push', description: 'Push agents to ElevenLabs', options: ['--branch <branch>  Push to a specific branch'] },
    { name: 'pull', description: 'Pull agents from ElevenLabs', options: ['--branch <branch>  Pull from a specific branch', '--all-branches  Pull all branches for each agent', '--update  Update existing agents', '--all  Pull all agents'] },
    { name: 'branches list', description: 'List branches for an agent', options: ['--agent <agent_id>  Agent ID (required)', '--include-archived  Include archived branches'] },
    { name: 'test <agent>', description: 'Run tests for an agent' },
    { name: 'templates [action]', description: 'Manage agent templates', options: ['list  List available templates', 'show <name>  Show template details'] },
    { name: 'widget <name>', description: 'Generate HTML widget snippet' },
  ];
  for (const cmd of commands) {
    console.log(`  ${cmd.name.padEnd(28)}${cmd.description}`);
    if (cmd.options) {
      for (const opt of cmd.options) {
        const [flag, ...descParts] = opt.split('  ');
        console.log(`      ${flag.padEnd(26)}${descParts.join('  ')}`);
      }
    }
  }
  console.log('\nEnable interactive UI with --human-friendly flag for any command');
}

export function createAgentsCommand(): Command {
  const agents = new Command('agents');
  agents.description('Manage ElevenLabs agents');

  agents.helpOption('-h, --help', 'Display help information');
  agents.addHelpCommand(false);

  agents.option('--human-friendly', 'Enable interactive terminal UI');

  // Custom action when agents command is run without subcommands
  agents.action(async () => {
    if (process.argv.includes('--human-friendly')) {
      const { waitUntilExit } = render(
        React.createElement(AgentsHelpView)
      );
      await waitUntilExit();
    } else {
      printAgentsHelp();
    }
    process.exit(0);
  });

  agents.addCommand(createInitCommand());
  agents.addCommand(createAddCommand());
  agents.addCommand(createListCommand());
  agents.addCommand(createDeleteCommand());
  agents.addCommand(createStatusCommand());
  agents.addCommand(createPushCommand());
  agents.addCommand(createPullCommand());
  agents.addCommand(createTemplatesCommand());
  agents.addCommand(createWidgetCommand());
  agents.addCommand(createTestCommand());
  agents.addCommand(createBranchesCommand());

  return agents;
}
