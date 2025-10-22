import { Command } from 'commander';
import { getTemplateOptions, getTemplateByName } from '../templates.js';

export function createTemplatesCommand(): Command {
  const templates = new Command('templates');
  templates.description('Manage agent templates');

  templates
    .command('list')
    .description('List available agent templates')
    .action(() => {
      const templates = getTemplateOptions();
      console.log('Available agent templates:');
      console.log('='.repeat(50));
      for (const [templateName, description] of Object.entries(templates)) {
        console.log(`\n${templateName}`);
        console.log(`   ${description}`);
      }
      console.log('\nUse \'elevenlabs agents add <name> --template <template_name>\' to create an agent with a specific template');
    });

  templates
    .command('show')
    .description('Show template configuration')
    .argument('<template>', 'Template name to display')
    .action((templateName: string) => {
      try {
        const template = getTemplateByName('Example', templateName);
        console.log(`Template: ${templateName}`);
        console.log('='.repeat(50));
        console.log(JSON.stringify(template, null, 2));
      } catch (error) {
        console.error(`Error loading template: ${error}`);
        process.exit(1);
      }
    });

  return templates;
}
