import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import path from 'path';
import fs from 'fs-extra';
import InitView from '../ui/InitView.js';
import { writeConfig } from '../../shared/utils.js';
import { writeToolsConfig, ToolsConfig } from '../../shared/tools.js';

const AGENTS_CONFIG_FILE = "agents.json";
const TOOLS_CONFIG_FILE = "tools.json";
const TESTS_CONFIG_FILE = "tests.json";

interface AgentsConfig {
  agents: Array<{
    config: string;
    id?: string;
  }>;
}

interface TestsConfig {
  tests: Array<{
    config: string;
    id?: string;
  }>;
}

export function createInitCommand(): Command {
  return new Command('init')
    .description('Initialize a new agent management project')
    .argument('[path]', 'Path to initialize the project in', '.')
    .option('--no-ui', 'Disable interactive UI')
    .option('--override', 'Override existing files and recreate from scratch', false)
    .action(async (projectPath: string, options: { ui: boolean; override: boolean }) => {
      try {
        if (options.ui !== false) {
          // Use Ink UI for initialization
          const { waitUntilExit } = render(
            React.createElement(InitView, { projectPath, override: options.override })
          );
          await waitUntilExit();
        } else {
          // Fallback to original implementation
          const fullPath = path.resolve(projectPath);
          console.log(`Initializing project in ${fullPath}`);
          if (options.override) {
            console.log('⚠ Override mode: existing files will be overwritten');
          }

          // Create directory if it doesn't exist
          await fs.ensureDir(fullPath);

          // Create agents.json file
          const agentsConfigPath = path.join(fullPath, AGENTS_CONFIG_FILE);
          if (!options.override && await fs.pathExists(agentsConfigPath)) {
            console.log(`${AGENTS_CONFIG_FILE} already exists (skipped)`);
          } else {
            const initialConfig: AgentsConfig = {
              agents: []
            };
            await writeConfig(agentsConfigPath, initialConfig);
            console.log(`Created ${AGENTS_CONFIG_FILE}`);
          }

          // Create tools.json file
          const toolsConfigPath = path.join(fullPath, TOOLS_CONFIG_FILE);
          if (!options.override && await fs.pathExists(toolsConfigPath)) {
            console.log(`${TOOLS_CONFIG_FILE} already exists (skipped)`);
          } else {
            const initialToolsConfig: ToolsConfig = {
              tools: []
            };
            await writeToolsConfig(toolsConfigPath, initialToolsConfig);
            console.log(`Created ${TOOLS_CONFIG_FILE}`);
          }

          // Create tests.json file
          const testsConfigPath = path.join(fullPath, TESTS_CONFIG_FILE);
          if (!options.override && await fs.pathExists(testsConfigPath)) {
            console.log(`${TESTS_CONFIG_FILE} already exists (skipped)`);
          } else {
            const initialTestsConfig: TestsConfig = {
              tests: []
            };
            await writeConfig(testsConfigPath, initialTestsConfig);
            console.log(`Created ${TESTS_CONFIG_FILE}`);
          }

          // Create directory structure
          const configDirs = ['agent_configs', 'tool_configs', 'test_configs'];
          for (const dir of configDirs) {
            const dirPath = path.join(fullPath, dir);
            if (options.override && await fs.pathExists(dirPath)) {
              await fs.remove(dirPath);
            }
            await fs.ensureDir(dirPath);
            const existed = await fs.pathExists(dirPath);
            console.log(`Created directory: ${dir}${!options.override && existed ? ' (already existed)' : ''}`);
          }

          // Create .env.example file
          const envExamplePath = path.join(fullPath, '.env.example');
          if (!options.override && await fs.pathExists(envExamplePath)) {
            console.log('.env.example already exists (skipped)');
          } else {
            const envExample = `# ElevenLabs API Key
ELEVENLABS_API_KEY=your_api_key_here
`;
            await fs.writeFile(envExamplePath, envExample);
            console.log('Created .env.example');
          }

          console.log('\nProject initialized successfully!');
          console.log('Next steps:');
          console.log('1. Set your ElevenLabs API key: elevenlabs auth login');
          console.log('2. Create an agent: elevenlabs agents add "My Agent" --template default');
          console.log('3. Create tools: elevenlabs tools add "My Webhook" --type webhook');
          console.log('4. Create tests: elevenlabs tests add "My Test" --template basic-llm');
          console.log('5. Push to ElevenLabs: elevenlabs agents push && elevenlabs tools push && elevenlabs tests push');
          console.log('6. Run tests: elevenlabs agents test "My Agent"');
        }
      } catch (error) {
        console.error(`Error initializing project: ${error}`);
        process.exit(1);
      }
    });
}
