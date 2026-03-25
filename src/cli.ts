#!/usr/bin/env node

import { Command } from 'commander';
import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { render } from 'ink';
import React from 'react';
import HelpView from './ui/views/HelpView.js';

// Import command groups
import { createAuthCommand } from './auth/commands/index.js';
import { createAgentsCommand } from './agents/commands/index.js';
import { createToolsCommand } from './tools/commands/index.js';
import { createTestsCommand } from './tests/commands/index.js';
import { createComponentsCommand } from './components/commands/index.js';
import { createCompletionCommand } from './completion/commands/index.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const packageJson = JSON.parse(readFileSync(join(__dirname, '../package.json'), 'utf-8'));
const { version } = packageJson;

function printPlainHelp() {
  console.log(`ElevenLabs CLI v${version}\n`);
  console.log('Usage:');
  console.log('  elevenlabs [command] [options]\n');
  console.log('Modules:');
  const modules = [
    { name: 'auth', description: 'Authentication commands' },
    { name: 'agents', description: 'Agent management commands' },
    { name: 'tools', description: 'Tool management commands' },
    { name: 'tests', description: 'Test management commands' },
    { name: 'components', description: 'UI component management' },
  ];
  for (const mod of modules) {
    console.log(`  ${mod.name.padEnd(16)}${mod.description}`);
  }
  console.log('\nQuick Start:');
  console.log('  1. Initialize a project: elevenlabs agents init');
  console.log('  2. Login with API key: elevenlabs auth login');
  console.log('  3. Create an agent: elevenlabs agents add "My Agent"');
  console.log('  4. Push to ElevenLabs: elevenlabs agents push');
  console.log('\nFor more information on a module, use: elevenlabs <module> --help');
  console.log('Enable interactive UI with --human-friendly flag for any command');
}

const program = new Command();

program
  .name('elevenlabs')
  .description('ElevenLabs CLI')
  .version(version)
  .option('--human-friendly', 'Enable interactive terminal UI')
  .configureHelp({
    // Override the default help to use our Ink UI
    formatHelp: () => ''
  })
  .helpOption('-h, --help', 'Display help information')
  .on('option:help', async () => {
    if (process.argv.includes('--human-friendly')) {
      const { waitUntilExit } = render(
        React.createElement(HelpView)
      );
      await waitUntilExit();
    } else {
      printPlainHelp();
    }
    process.exit(0);
  });

// Add new command groups
program.addCommand(createAuthCommand());
program.addCommand(createAgentsCommand());
program.addCommand(createToolsCommand());
program.addCommand(createTestsCommand());
program.addCommand(createComponentsCommand());
program.addCommand(createCompletionCommand());

// Show help if no arguments provided or if only help flag is provided
const args = process.argv.slice(2);
const nonFlagArgs = args.filter(a => a !== '--human-friendly' && a !== '--no-ui');
const isMainHelp = nonFlagArgs.length === 0 ||
  (nonFlagArgs.length === 1 && (nonFlagArgs[0] === '--help' || nonFlagArgs[0] === '-h'));

if (isMainHelp) {
  (async () => {
    if (args.includes('--human-friendly')) {
      const { waitUntilExit } = render(
        React.createElement(HelpView)
      );
      await waitUntilExit();
    } else {
      printPlainHelp();
    }
    process.exit(0);
  })();
} else {
  // Parse and execute
  program.parse(process.argv);
}
