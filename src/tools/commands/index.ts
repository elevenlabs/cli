import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import { createAddCommand } from './add.js';
import { createDeleteCommand } from './delete.js';
import { createPushCommand } from './push.js';
import { createPullCommand } from './pull.js';
import ToolsHelpView from '../ui/ToolsHelpView.js';

function printToolsHelp() {
  console.log('elevenlabs tools - Tool management commands\n');
  console.log('Usage:');
  console.log('  elevenlabs tools <command> [options]\n');
  console.log('Commands:');
  const commands = [
    { name: 'add <name>', description: 'Add a new tool', options: ['--type <type>  Tool type: webhook or client (default: \'webhook\')', '--config-path <path>  Custom config path'] },
    { name: 'delete [tool_id]', description: 'Delete a tool locally and from ElevenLabs', options: ['--all  Delete all tools'] },
    { name: 'push', description: 'Push tools to ElevenLabs API', options: ['--dry-run  Show what would be done without making changes'] },
    { name: 'pull', description: 'Pull tools from ElevenLabs', options: ['--tool <tool_id>  Specific tool ID to pull', '--output-dir <directory>  Output directory for configs (default: \'tool_configs\')', '--dry-run  Show what would be done without making changes', '--update  Update existing items only, skip new', '--all  Pull all (new + existing)'] },
  ];
  for (const cmd of commands) {
    console.log(`  ${cmd.name.padEnd(28)}${cmd.description}`);
    if (cmd.options) {
      for (const opt of cmd.options) {
        console.log(`    ${opt}`);
      }
    }
  }
  console.log('\nEnable interactive UI with --human-friendly flag for any command');
}

export function createToolsCommand(): Command {
  const tools = new Command('tools');
  tools.description('Manage ElevenLabs tools');

  // Disable default help
  tools.helpOption(false);
  tools.addHelpCommand(false);

  // Add custom help option
  tools.option('-h, --help', 'Display help information');
  tools.option('--human-friendly', 'Enable interactive terminal UI');

  // Custom action when tools command is run without subcommands
  tools.action(async (options: { humanFriendly?: boolean }) => {
    if (options.humanFriendly) {
      const { waitUntilExit } = render(
        React.createElement(ToolsHelpView)
      );
      await waitUntilExit();
    } else {
      printToolsHelp();
    }
    process.exit(0);
  });

  tools.addCommand(createAddCommand());
  tools.addCommand(createDeleteCommand());
  tools.addCommand(createPushCommand());
  tools.addCommand(createPullCommand());

  return tools;
}
