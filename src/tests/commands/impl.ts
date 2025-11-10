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
import { ElevenLabs } from '@elevenlabs/elevenlabs-js';

const TESTS_CONFIG_FILE = "tests.json";

interface TestDefinition {
  config: string;
  type?: string;
  id?: string;
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

async function addTest(name: string, templateType: string = "basic-llm"): Promise<void> {
  const { getTestTemplateByName } = await import('../templates.js');

  // Check if tests.json exists
  const testsConfigPath = path.resolve(TESTS_CONFIG_FILE);
  let testsConfig: TestsConfig;

  try {
    testsConfig = await readConfig<TestsConfig>(testsConfigPath);
  } catch {
    // Initialize tests.json if it doesn't exist
    testsConfig = { tests: [] };
    await writeConfig(testsConfigPath, testsConfig);
    console.log(`Created ${TESTS_CONFIG_FILE}`);
  }

  // Create test config using template (in memory first)
  const testConfig = getTestTemplateByName(name, templateType);

  // Create test in ElevenLabs first to get ID
  console.log(`Creating test '${name}' in ElevenLabs...`);

  const client = await getElevenLabsClient();

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
      id: testId
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
async function pushTests(testId?: string, dryRun = false): Promise<void> {
  // Load tests configuration
  const testsConfigPath = path.resolve(TESTS_CONFIG_FILE);
  if (!(await fs.pathExists(testsConfigPath))) {
    throw new Error('tests.json not found. Run \'elevenlabs add-test\' first.');
  }

  const testsConfig = await readConfig<TestsConfig>(testsConfigPath);

  // Filter tests by test ID if specified
  let testsToProcess = testsConfig.tests;

  if (testId) {
    testsToProcess = testsToProcess.filter(test => test.id === testId);
    if (testsToProcess.length === 0) {
      throw new Error(`Test with ID '${testId}' not found in configuration`);
    }
  }

  console.log(`Pushing ${testsToProcess.length} test(s) to ElevenLabs...`);

  let changesMade = false;

  for (const testDef of testsToProcess) {
    const configPath = testDef.config;

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
    console.log(`${testDefName}: Will push (force override)`);

    if (dryRun) {
      console.log(`[DRY RUN] Would update test: ${testDefName}`);
      continue;
    }

    // Initialize ElevenLabs client
    let client;
    try {
      client = await getElevenLabsClient();
    } catch (error) {
      console.log(`Error: ${error}`);
      console.log(`Skipping test ${testDefName} - not configured`);
      continue;
    }

    // Perform API operation
    try {
      const testApiConfig = toCamelCaseKeys(testConfig) as unknown as ElevenLabs.conversationalAi.CreateUnitTestRequest;

      if (!testId) {
        // Create new test
        const response = await createTestApi(client, testApiConfig);
        const newTestId = response.id;
        console.log(`Created test ${testDefName} (ID: ${newTestId})`);

        // Store test ID in index file
        testDef.id = newTestId;
        changesMade = true;
      } else {
        // Update existing test
        await updateTestApi(client, testId, testApiConfig as ElevenLabs.conversationalAi.UpdateUnitTestRequest);
        console.log(`Updated test ${testDefName} (ID: ${testId})`);
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
async function pullTests(options: { test?: string; outputDir: string; dryRun: boolean; update?: boolean; all?: boolean }): Promise<void> {
  // Check if tests.json exists
  const testsConfigPath = path.resolve(TESTS_CONFIG_FILE);

  console.log(`Pulling tests from ElevenLabs...`);

  await pullTestsFromEnvironment(options, testsConfigPath);
}

async function pullTestsFromEnvironment(options: { test?: string; outputDir: string; dryRun: boolean; update?: boolean; all?: boolean }, testsConfigPath: string): Promise<void> {
  let testsConfig: TestsConfig;

  try {
    testsConfig = await readConfig<TestsConfig>(testsConfigPath);
  } catch {
    testsConfig = { tests: [] };
    await writeConfig(testsConfigPath, testsConfig);
    console.log(`Created ${TESTS_CONFIG_FILE}`);
  }

  const client = await getElevenLabsClient();

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

  // Build map of existing tests by ID
  const existingTestIds = new Map(
    testsConfig.tests
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
          type: testDetailsTyped.type || 'conversational'
        };
        
        testsConfig.tests.push(newTest);
        console.log(`  ✓ Added '${test.name}' (config: ${configPath})`);
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

  console.log(`Deleting test '${testName}' (ID: ${testId})...`);

  // Delete from ElevenLabs
  console.log('Deleting from ElevenLabs...');
  const client = await getElevenLabsClient();
  
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
async function deleteAllTests(ui: boolean = true): Promise<void> {
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

  const testsToDelete = testsConfig.tests;

  // Show what will be deleted
  console.log(`\nFound ${testsToDelete.length} test(s) to delete:`);
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
    console.log(`  ${i + 1}. ${testName} (${test.id})`);
  }
  
  // Confirm deletion (skip if --no-ui)
  if (ui) {
    console.log('\nWARNING: This will delete ALL tests from both local configuration and ElevenLabs.');
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

      console.log(`Deleting '${testName}' (${testDef.id})...`);

      // Delete from ElevenLabs
      if (testDef.id) {
        try {
          const client = await getElevenLabsClient();
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
