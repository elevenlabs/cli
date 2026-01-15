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

const program = new Command();

program
  .name('elevenlabs')
  .description('ElevenLabs CLI')
  .version(version)
  .configureHelp({
    // Override the default help to use our Ink UI
    formatHelp: () => ''
  })
  .helpOption('-h, --help', 'Display help information')
  .on('option:help', async () => {
    // Show Ink-based help view
    const { waitUntilExit } = render(
      React.createElement(HelpView)
    );
    await waitUntilExit();
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
const isMainHelp = args.length === 0 ||
  (args.length === 1 && (args[0] === '--help' || args[0] === '-h'));

if (isMainHelp) {
  (async () => {
    const { waitUntilExit } = render(
      React.createElement(HelpView)
    );
    await waitUntilExit();
    process.exit(0);
  })();
} else {
  // Parse and execute
  program.parse(process.argv);
}
