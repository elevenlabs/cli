import path from 'path';
import fs from 'fs-extra';
import { readConfig, writeConfig, generateUniqueFilename, toCamelCaseKeys } from '../../shared/utils.js';
import {
  getElevenLabsClient,
  createTestApi,
  updateTestApi,
  listTestsApi,
  getTestApi,
  deleteTestApi
} from '../../shared/elevenlabs-api.js';
import { listEnvironments } from '../../shared/config.js';
import { ElevenLabs } from '@elevenlabs/elevenlabs-js';

const TESTS_CONFIG_FILE = "tests.json";

interface TestDefinition {
  config: string;
  type?: string;
  id?: string;
  env?: string;
}

interface TestsConfig {
  tests: TestDefinition[];
}

// Helper function for prompting confirmation
async function promptForConfirmation(message: string): Promise<boolean> {
  const readline = await import('readline');
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
  });

  return new Promise((resolve) => {
    rl.question(`${message} (y/n): `, (answer) => {
      rl.close();
      resolve(answer.toLowerCase() === 'y' || answer.toLowerCase() === 'yes');
    });
  });
}

async function addTest(name: string, templateType: string = "basic-llm", environment?: string): Promise<void> {
  const { getTestTemplateByName } = await import('../templates.js');

  // Check if tests.json exists
  const testsConfigPath = path.resolve(TESTS_CONFIG_FILE);
  let testsConfig: TestsConfig;

  try {
    testsConfig = await readConfig<TestsConfig>(testsConfigPath);
  } catch (error) {
    // Initialize tests.json if it doesn't exist
    testsConfig = { tests: [] };
    await writeConfig(testsConfigPath, testsConfig);
    console.log(`Created ${TESTS_CONFIG_FILE}`);
  }

  // Create test config using template (in memory first)
  const testConfig = getTestTemplateByName(name, templateType);

  // Create test in ElevenLabs first to get ID
  const env = environment || 'prod';
  console.log(`Creating test '${name}' in ElevenLabs (environment: ${env})...`);

  const client = await getElevenLabsClient(env);

  try {
    const testApiConfig = toCamelCaseKeys(testConfig) as unknown as ElevenLabs.conversationalAi.CreateUnitTestRequest;
    const response = await createTestApi(client, testApiConfig);
    const testId = response.id;

    console.log(`Created test in ElevenLabs with ID: ${testId}`);

    // Generate config path using test name
    const configPath = await generateUniqueFilename('test_configs', name);

    // Create config directory and file
    const configFilePath = path.resolve(configPath);
    await fs.ensureDir(path.dirname(configFilePath));

    await writeConfig(configFilePath, testConfig);
    console.log(`Created config file: ${configPath} (template: ${templateType})`);

    // Add to tests.json if not already present
    const newTest: TestDefinition = {
      config: configPath,
      type: templateType,
      id: testId,
      env: env
    };
    testsConfig.tests.push(newTest);
    await writeConfig(testsConfigPath, testsConfig);
    console.log(`Added test '${name}' to tests.json`);

    console.log(`Edit ${configPath} to customize your test, then run 'elevenlabs push-tests' to update`);

  } catch (error) {
    console.error(`Error creating test in ElevenLabs: ${error}`);
    process.exit(1);
  }
}
async function pushTests(testId?: string, dryRun = false, environment?: string): Promise<void> {
  // Load tests configuration
  const testsConfigPath = path.resolve(TESTS_CONFIG_FILE);
  if (!(await fs.pathExists(testsConfigPath))) {
    throw new Error('tests.json not found. Run \'elevenlabs add-test\' first.');
  }

  const testsConfig = await readConfig<TestsConfig>(testsConfigPath);

  // Filter tests by environment and/or test ID
  let testsToProcess = testsConfig.tests;
  
  if (environment) {
    testsToProcess = testsToProcess.filter(test => (test.env || 'prod') === environment);
  }
  
  if (testId) {
    testsToProcess = testsToProcess.filter(test => test.id === testId);
    if (testsToProcess.length === 0) {
      throw new Error(`Test with ID '${testId}' not found in configuration`);
    }
  }
  
  if (environment && testsToProcess.length === 0) {
    console.log(`No tests found for environment '${environment}'`);
    return;
  }
  
  if (!environment) {
    const envs = [...new Set(testsToProcess.map(t => t.env || 'prod'))];
    console.log(`Pushing ${testsToProcess.length} test(s) across ${envs.length} environment(s): ${envs.join(', ')}`);
  }

  let changesMade = false;

  for (const testDef of testsToProcess) {
    const configPath = testDef.config;
    const environment = testDef.env || 'prod';

    // Check if config file exists
    if (!(await fs.pathExists(configPath))) {
      console.log(`Warning: Config file not found: ${configPath}`);
      continue;
    }

    // Load test config
    let testConfig: { name?: string };
    try {
      testConfig = await readConfig(configPath);
    } catch (error) {
      console.log(`Error reading config from ${configPath}: ${error}`);
      continue;
    }

    const testDefName = testConfig.name || 'Unnamed Test';

    // Get test ID from index file
    const testId = testDef.id;

    // Always push (force override)
    console.log(`${testDefName} [${environment}]: Will push (force override)`);

    if (dryRun) {
      console.log(`[DRY RUN] Would update test: ${testDefName} [${environment}]`);
      continue;
    }

    // Initialize ElevenLabs client for this test's environment
    let client;
    try {
      client = await getElevenLabsClient(environment);
    } catch (error) {
      console.log(`Error: ${error}`);
      console.log(`Skipping test ${testDefName} - environment '${environment}' not configured`);
      continue;
    }

    // Perform API operation
    try {
      const testApiConfig = toCamelCaseKeys(testConfig) as unknown as ElevenLabs.conversationalAi.CreateUnitTestRequest;

      if (!testId) {
        // Create new test
        const response = await createTestApi(client, testApiConfig);
        const newTestId = response.id;
        console.log(`Created test ${testDefName} (ID: ${newTestId}) [${environment}]`);
        
        // Store test ID in index file
        testDef.id = newTestId;
        changesMade = true;
      } else {
        // Update existing test
        await updateTestApi(client, testId, testApiConfig as ElevenLabs.conversationalAi.UpdateUnitTestRequest);
        console.log(`Updated test ${testDefName} (ID: ${testId}) [${environment}]`);
      }

      changesMade = true;

    } catch (error) {
      console.log(`Error processing ${testDefName}: ${error}`);
    }
  }
  
  // Save updated tests.json if there were changes
  if (changesMade) {
    await writeConfig(testsConfigPath, testsConfig);
  }
}
async function pullTests(options: { test?: string; outputDir: string; dryRun: boolean; update?: boolean; all?: boolean; env?: string }): Promise<void> {
  // Check if tests.json exists
  const testsConfigPath = path.resolve(TESTS_CONFIG_FILE);
  
  // Determine which environments to pull from
  const environmentsToPull: string[] = options.env 
    ? [options.env] 
    : await listEnvironments();
  
  if (environmentsToPull.length === 0) {
    console.log('No environments configured. Use "elevenlabs auth login" to add an environment.');
    return;
  }
  
  if (!options.env) {
    console.log(`Pulling from ${environmentsToPull.length} environment(s): ${environmentsToPull.join(', ')}`);
  }
  
  // Pull from each environment
  for (const environment of environmentsToPull) {
    console.log(`\n${'='.repeat(50)}`);
    console.log(`Environment: ${environment}`);
    console.log('='.repeat(50));
    
    await pullTestsFromEnvironment(options, environment, testsConfigPath);
  }
}

async function pullTestsFromEnvironment(options: { test?: string; outputDir: string; dryRun: boolean; update?: boolean; all?: boolean }, environment: string, testsConfigPath: string): Promise<void> {
  let testsConfig: TestsConfig;

  try {
    testsConfig = await readConfig<TestsConfig>(testsConfigPath);
  } catch (error) {
    testsConfig = { tests: [] };
    await writeConfig(testsConfigPath, testsConfig);
    console.log(`Created ${TESTS_CONFIG_FILE}`);
  }

  const client = await getElevenLabsClient(environment);

  let testsList: unknown[];

  if (options.test) {
    // Pull specific test by ID
    console.log(`Pulling test with ID: ${options.test}...`);
    try {
      const testDetails = await getTestApi(client, options.test);
      const testDetailsTyped = testDetails as { id?: string; name: string };
      const testId = testDetailsTyped.id || options.test;
      testsList = [{
        id: testId,
        name: testDetailsTyped.name
      }];
      console.log(`Found test: ${testDetailsTyped.name}`);
    } catch (error) {
      throw new Error(`Failed to fetch test with ID '${options.test}': ${error}`);
    }
  } else {
    // Fetch all tests from ElevenLabs
    console.log('Fetching all tests from ElevenLabs...');
    testsList = await listTestsApi(client, 30);

    if (testsList.length === 0) {
      console.log('No tests found in your ElevenLabs workspace.');
      return;
    }

    console.log(`Found ${testsList.length} test(s)`);
  }

  // Build map of existing tests by ID for this environment
  const existingTestIds = new Map(
    testsConfig.tests
      .filter(test => (test.env || 'prod') === environment)
      .map(test => [test.id, test])
  );

  // Track operations for summary
  const operations = { create: 0, update: 0, skip: 0 };
  type OperationItem = {
    action: 'create' | 'update' | 'skip';
    test: { id: string; name: string };
    existingEntry?: TestDefinition;
  };
  const itemsToProcess: OperationItem[] = [];

  // First pass: determine what will happen
  for (const testMeta of testsList) {
    const testMetaTyped = testMeta as { id?: string; name: string };
    const testId = testMetaTyped.id;
    if (!testId) {
      console.log(`Warning: Skipping test '${testMetaTyped.name}' - no test ID found`);
      continue;
    }

    let testNameRemote = testMetaTyped.name;
    const existingEntry = existingTestIds.get(testId);

    if (existingEntry) {
      // Test with this ID already exists locally
      if (options.update || options.all) {
        // --update or --all: update existing
        itemsToProcess.push({ action: 'update', test: { id: testId, name: testNameRemote }, existingEntry });
        operations.update++;
      } else {
        // Default: skip existing
        itemsToProcess.push({ action: 'skip', test: { id: testId, name: testNameRemote }, existingEntry });
        operations.skip++;
      }
    } else {
      // New test (not present locally)
      if (options.update) {
        // --update mode: skip new items (only update existing)
        itemsToProcess.push({ action: 'skip', test: { id: testId, name: testNameRemote } });
        operations.skip++;
      } else {
        // Default or --all: create new items
        itemsToProcess.push({ action: 'create', test: { id: testId, name: testNameRemote } });
        operations.create++;
      }
    }
  }

  // Show summary
  console.log(`\nPlan: ${operations.create} create, ${operations.update} update, ${operations.skip} skip`);
  
  if (operations.skip > 0 && !options.update && !options.all) {
    if (operations.create === 0) {
      console.log(`\n💡 Tip: Use --update to update existing tests or --all to pull everything`);
    } else {
      console.log(`\n💡 Tip: Use --all to also update existing tests`);
    }
  }

  // Prompt for confirmation if not --dry-run
  if (!options.dryRun && (operations.create > 0 || operations.update > 0)) {
    const confirmed = await promptForConfirmation('Proceed?');
    if (!confirmed) {
      console.log('Pull cancelled');
      return;
    }
  }

  // Second pass: execute operations
  let itemsProcessed = 0;
  for (const item of itemsToProcess) {
    const { action, test, existingEntry } = item;
    
    if (action === 'skip') {
      console.log(`⊘ Skipping '${test.name}' (already exists, use --update to overwrite)`);
      continue;
    }
    
    if (options.dryRun) {
      console.log(`[DRY RUN] Would ${action} test: ${test.name} (ID: ${test.id})`);
      continue;
    }
    
    try {
      // Fetch detailed test configuration
      console.log(`${action === 'update' ? '↻ Updating' : '+ Pulling'} config for '${test.name}'...`);
      const testDetails = await getTestApi(client, test.id);
      const testDetailsTyped = testDetails as { type?: string };
      
      if (action === 'update' && existingEntry) {
        // Update existing entry - overwrite the config file
        const configFilePath = path.resolve(existingEntry.config);
        await fs.ensureDir(path.dirname(configFilePath));
        await writeConfig(configFilePath, testDetails);
        console.log(`  ✓ Updated '${test.name}' (config: ${existingEntry.config})`);
      } else {
        // Create new entry
        const configPath = await generateUniqueFilename(options.outputDir, test.name);
        const configFilePath = path.resolve(configPath);
        await fs.ensureDir(path.dirname(configFilePath));
        await writeConfig(configFilePath, testDetails);
        
        const newTest: TestDefinition = {
          config: configPath,
          id: test.id,
          type: testDetailsTyped.type || 'conversational',
          env: environment
        };
        
        testsConfig.tests.push(newTest);
        console.log(`  ✓ Added '${test.name}' (config: ${configPath}) [${environment}]`);
      }
      
      itemsProcessed++;
      
    } catch (error) {
      console.log(`  ✗ Error ${action === 'update' ? 'updating' : 'pulling'} test '${test.name}': ${error}`);
      continue;
    }
  }

  // Save updated tests.json if there were changes
  if (!options.dryRun && itemsProcessed > 0) {
    await writeConfig(testsConfigPath, testsConfig);
    console.log(`\nUpdated ${TESTS_CONFIG_FILE}`);
  }

  // Final summary
  if (options.dryRun) {
    console.log(`\n[DRY RUN] Would process ${operations.create + operations.update} test(s)`);
  } else {
    console.log(`\n✓ Summary: ${operations.create} created, ${operations.update} updated, ${operations.skip} skipped`);
    if (itemsProcessed > 0) {
      console.log(`You can now edit the config files in '${options.outputDir}/' and run 'elevenlabs push-tests' to update`);
    }
  }
}
async function deleteTest(testId: string): Promise<void> {
  // Load tests configuration
  const testsConfigPath = path.resolve(TESTS_CONFIG_FILE);
  if (!(await fs.pathExists(testsConfigPath))) {
    throw new Error('tests.json not found. Run \'elevenlabs agents init\' first.');
  }

  const testsConfig = await readConfig<TestsConfig>(testsConfigPath);
  
  // Find the test by ID
  const testIndex = testsConfig.tests.findIndex(test => test.id === testId);
  
  if (testIndex === -1) {
    throw new Error(`Test with ID '${testId}' not found in local configuration`);
  }
  
  const testDef = testsConfig.tests[testIndex];
  const configPath = testDef.config;
  const environment = testDef.env || 'prod';
  
  // Read test name from config if available
  let testName = testId;
  if (configPath && await fs.pathExists(configPath)) {
    try {
      const testConfig = await readConfig(configPath) as { name?: string };
      testName = testConfig.name || testId;
    } catch {
      // If reading fails, just use ID
    }
  }
  
  console.log(`Deleting test '${testName}' (ID: ${testId}) [${environment}]...`);
  
  // Delete from ElevenLabs
  console.log('Deleting from ElevenLabs...');
  const client = await getElevenLabsClient(environment);
  
  try {
    await deleteTestApi(client, testId);
    console.log('✓ Successfully deleted from ElevenLabs');
  } catch (error) {
    console.error(`Warning: Failed to delete from ElevenLabs: ${error}`);
    console.log('Continuing with local deletion...');
  }
  
  // Remove from local tests.json
  testsConfig.tests.splice(testIndex, 1);
  await writeConfig(testsConfigPath, testsConfig);
  console.log(`✓ Removed '${testName}' from ${TESTS_CONFIG_FILE}`);
  
  // Remove config file
  if (configPath && await fs.pathExists(configPath)) {
    await fs.remove(configPath);
    console.log(`✓ Deleted config file: ${configPath}`);
  }
  
  console.log(`\n✓ Successfully deleted test '${testName}'`);
}
async function deleteAllTests(ui: boolean = true, env?: string): Promise<void> {
  // Load tests configuration
  const testsConfigPath = path.resolve(TESTS_CONFIG_FILE);
  if (!(await fs.pathExists(testsConfigPath))) {
    throw new Error('tests.json not found. Run \'elevenlabs agents init\' first.');
  }

  const testsConfig = await readConfig<TestsConfig>(testsConfigPath);
  
  if (testsConfig.tests.length === 0) {
    console.log('No tests found to delete');
    return;
  }

  // Filter tests by environment if specified
  const testsToDelete = env 
    ? testsConfig.tests.filter(test => (test.env || 'prod') === env)
    : testsConfig.tests;
  
  if (testsToDelete.length === 0) {
    console.log(env ? `No tests found in environment '${env}'` : 'No tests found to delete');
    return;
  }
  
  // Show what will be deleted
  const envInfo = env ? ` in environment '${env}'` : '';
  console.log(`\nFound ${testsToDelete.length} test(s) to delete${envInfo}:`);
  for (let i = 0; i < testsToDelete.length; i++) {
    const test = testsToDelete[i];
    let testName = test.id || 'Unknown';
    if (test.config && await fs.pathExists(test.config)) {
      try {
        const testConfig = await readConfig(test.config) as { name?: string };
        testName = testConfig.name || test.id || 'Unknown';
      } catch {
        // Use ID if config read fails
      }
    }
    const testEnv = test.env || 'prod';
    console.log(`  ${i + 1}. ${testName} (${test.id}) [${testEnv}]`);
  }
  
  // Confirm deletion (skip if --no-ui)
  if (ui) {
    const warningMsg = env 
      ? `\nWARNING: This will delete ${testsToDelete.length} test(s) from environment '${env}' in both local configuration and ElevenLabs.`
      : '\nWARNING: This will delete ALL tests from both local configuration and ElevenLabs.';
    console.log(warningMsg);
    const confirmed = await promptForConfirmation('Are you sure you want to delete these tests?');
    
    if (!confirmed) {
      console.log('Deletion cancelled');
      return;
    }
  }
  
  console.log('\nDeleting tests...\n');
  
  let successCount = 0;
  let failCount = 0;
  const deletedIds = new Set<string>();
  
  // Delete each test
  for (const testDef of testsToDelete) {
    try {
      const environment = testDef.env || 'prod';
      
      // Read test name from config if available
      let testName = testDef.id || 'Unknown';
      if (testDef.config && await fs.pathExists(testDef.config)) {
        try {
          const testConfig = await readConfig(testDef.config) as { name?: string };
          testName = testConfig.name || testDef.id || 'Unknown';
        } catch {
          // If reading fails, just use ID
        }
      }
      
      console.log(`Deleting '${testName}' (${testDef.id}) [${environment}]...`);
      
      // Delete from ElevenLabs
      if (testDef.id) {
        try {
          const client = await getElevenLabsClient(environment);
          await deleteTestApi(client, testDef.id);
          console.log(`  ✓ Deleted from ElevenLabs`);
        } catch (error) {
          console.error(`  Warning: Failed to delete from ElevenLabs: ${error}`);
        }
      } else {
        console.log(`  Warning: No test ID found, skipping ElevenLabs deletion`);
      }
      
      // Remove config file
      if (testDef.config && await fs.pathExists(testDef.config)) {
        await fs.remove(testDef.config);
        console.log(`  ✓ Deleted config file: ${testDef.config}`);
      }
      
      if (testDef.id) {
        deletedIds.add(testDef.id);
      }
      successCount++;
    } catch (error) {
      console.error(`  Failed to delete test: ${error}`);
      failCount++;
    }
  }
  
  // Remove deleted tests from config (only the ones that were successfully deleted)
  testsConfig.tests = testsConfig.tests.filter(test => !deletedIds.has(test.id || ''));
  await writeConfig(testsConfigPath, testsConfig);
  console.log(`\n✓ Updated ${TESTS_CONFIG_FILE}`);
  
  // Summary
  console.log(`\n✓ Deletion complete: ${successCount} succeeded, ${failCount} failed`);
}

export { addTest, pushTests, pullTests, deleteTest, deleteAllTests };
