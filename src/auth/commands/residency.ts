import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import ResidencyView from '../ui/ResidencyView.js';
import { getResidency, setResidency, Location, LOCATIONS } from '../../shared/config.js';

export function createResidencyCommand(): Command {
  return new Command('residency')
    .description('Set the API residency location')
    .argument('[residency]', `Residency location (${LOCATIONS.join(', ')})`)
    .option('--no-ui', 'Disable interactive UI')
    .action(async (residency: string | undefined, options: { ui: boolean }) => {
      try {
        if (options.ui !== false && !residency) {
          // Use Ink UI for interactive residency selection
          const { waitUntilExit } = render(
            React.createElement(ResidencyView)
          );
          await waitUntilExit();
        } else if (residency) {
          // Direct residency setting (with or without UI)
          function isValidLocation(value: string): value is Location {
            return LOCATIONS.includes(value as Location);
          }

          if (!isValidLocation(residency)) {
            console.error(`Invalid residency: ${residency}`);
            console.error(`Valid options: ${LOCATIONS.join(', ')}`);
            process.exit(1);
          }

          if (options.ui !== false) {
            // Use UI even with direct argument
            const { waitUntilExit } = render(
              React.createElement(ResidencyView, { initialResidency: residency })
            );
            await waitUntilExit();
          } else {
            // Fallback to text-based
            await setResidency(residency);
            console.log(`Residency set to: ${residency}`);
          }
        } else {
          // No residency provided and UI disabled - show current residency
          const currentResidency = await getResidency();
          console.log(`Current residency: ${currentResidency || 'Not set (using default)'}`);
          console.log(`To set residency, use: elevenlabs auth residency <${LOCATIONS.join('|')}>`);
        }
      } catch (error) {
        console.error(`Error setting residency: ${error}`);
        process.exit(1);
      }
    });
}
