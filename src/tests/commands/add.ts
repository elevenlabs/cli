import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import AddTestView from '../ui/AddTestView.js';
import { addTest } from './impl.js';

export function createAddCommand(): Command {
  return new Command('add')
    .description('Add a new test')
    .argument('<name>', 'Name of the test to create')
    .option('--template <template>', 'Test template type', 'basic-llm')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (name: string, options: { template: string; ui: boolean }) => {
      try {
        if (options.ui !== false) {
          // Use Ink UI for test creation
          const { waitUntilExit } = render(
            React.createElement(AddTestView, {
              initialName: name
            })
          );
          await waitUntilExit();
        } else {
          await addTest(name, options.template);
        }
      } catch (error) {
        console.error(`Error creating test: ${error}`);
        process.exit(1);
      }
    });
}
