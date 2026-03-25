import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import { createAddCommand } from './add.js';
import { createDeleteCommand } from './delete.js';
import { createPushCommand } from './push.js';
import { createPullCommand } from './pull.js';
import TestsHelpView from '../ui/TestsHelpView.js';

function printTestsHelp() {
  console.log('elevenlabs tests - Test management commands\n');
  console.log('Usage:');
  console.log('  elevenlabs tests <command> [options]\n');
  console.log('Commands:');
  const commands = [
    { name: 'add <name>', description: 'Add a new test', options: ['--template <template>  Test template type (default: \'basic-llm\')'] },
    { name: 'delete [test_id]', description: 'Delete a test locally and from ElevenLabs', options: ['--all  Delete all tests'] },
    { name: 'push', description: 'Push tests to ElevenLabs API', options: ['--dry-run  Show what would be done without making changes'] },
    { name: 'pull', description: 'Pull tests from ElevenLabs', options: ['--test <test_id>  Specific test ID to pull', '--output-dir <directory>  Output directory for configs (default: \'test_configs\')', '--dry-run  Show what would be done without making changes', '--update  Update existing items only, skip new', '--all  Pull all (new + existing)'] },
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

export function createTestsCommand(): Command {
  const tests = new Command('tests');
  tests.description('Manage ElevenLabs tests');

  // Disable default help
  tests.helpOption(false);
  tests.addHelpCommand(false);

  // Add custom help option
  tests.option('-h, --help', 'Display help information');
  tests.option('--human-friendly', 'Enable interactive terminal UI');

  // Custom action when tests command is run without subcommands
  tests.action(async (options: { humanFriendly?: boolean }) => {
    if (options.humanFriendly) {
      const { waitUntilExit } = render(
        React.createElement(TestsHelpView)
      );
      await waitUntilExit();
    } else {
      printTestsHelp();
    }
    process.exit(0);
  });

  tests.addCommand(createAddCommand());
  tests.addCommand(createDeleteCommand());
  tests.addCommand(createPushCommand());
  tests.addCommand(createPullCommand());

  return tests;
}
