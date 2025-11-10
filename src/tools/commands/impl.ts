import path from 'path';
import fs from 'fs-extra';
import { readConfig, generateUniqueFilename } from '../../shared/utils.js';
import {
  getElevenLabsClient,
  createToolApi,
  updateToolApi,
  listToolsApi,
  getToolApi,
  deleteToolApi
} from '../../shared/elevenlabs-api.js';
import {
  readToolsConfig,
  writeToolsConfig,
  writeToolConfig,
  ToolsConfig,
  ToolDefinition,
  type Tool
} from '../../shared/tools.js';

const TOOLS_CONFIG_FILE = "tools.json";

// Helper function for prompting confirmation
async function promptForConfirmation(message: string): Promise<boolean> {
  // For non-UI mode, use readline
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

async function addTool(name: string, type: 'webhook' | 'client', configPath?: string): Promise<void> {
  // Check if tools.json exists, create if not
  const toolsConfigPath = path.resolve(TOOLS_CONFIG_FILE);
  let toolsConfig: ToolsConfig;
  
  try {
    toolsConfig = await readToolsConfig(toolsConfigPath);
  } catch {
    // Initialize tools.json if it doesn't exist
    toolsConfig = { tools: [] };
    await writeToolsConfig(toolsConfigPath, toolsConfig);
    console.log(`Created ${TOOLS_CONFIG_FILE}`);
  }
  
  // Create tool config using appropriate template (in memory first)
  let toolConfig;
  if (type === 'webhook') {
    toolConfig = {
      name,
      description: `${name} webhook tool`,
      type: 'webhook' as const,
      api_schema: {
        url: 'https://api.example.com/webhook',
        method: 'POST',
        request_body_schema: {
          type: 'object',
          description: 'Request body for the webhook',
          properties: {}
        },
        request_headers: {
          'Content-Type': 'application/json'
        }
      },
      response_timeout_secs: 30,
      dynamic_variables: {
        dynamic_variable_placeholders: {}
      },
      assignments: [],
      disable_interruptions: false,
      force_pre_tool_speech: false
    };
  } else {
    toolConfig = {
      name,
      description: `${name} client tool`,
      type: 'client' as const,
      expects_response: false,
      response_timeout_secs: 30,
      parameters: {
        type: 'object',
        description: 'Parameters for the client tool',
        properties: {}
      },
      dynamic_variables: {
        dynamic_variable_placeholders: {}
      }
    };
  }
  
  // Create tool in ElevenLabs first to get ID
  console.log(`Creating ${type} tool '${name}' in ElevenLabs...`);

  const client = await getElevenLabsClient();
  
  try {
    const response = await createToolApi(client, toolConfig);
    const toolId = response.id;
    
    console.log(`Created ${type} tool in ElevenLabs with ID: ${toolId}`);
    
    // Generate config path using tool name (or custom path if provided)
    if (!configPath) {
      configPath = await generateUniqueFilename('tool_configs', name);
    }
    
    // Create config directory and file
    const configFilePath = path.resolve(configPath);
    await fs.ensureDir(path.dirname(configFilePath));
    
    await writeToolConfig(configFilePath, toolConfig);
    console.log(`Created config file: ${configPath}`);
    
    // Add to tools.json
    const newTool: ToolDefinition = {
      type,
      config: configPath,
      id: toolId
    };
    toolsConfig.tools.push(newTool);
    await writeToolsConfig(toolsConfigPath, toolsConfig);
    console.log(`Added tool '${name}' to tools.json`);
    
    console.log(`Edit ${configPath} to customize your tool, then run 'elevenlabs push-tools' to update`);
    
  } catch (error) {
    console.error(`Error creating tool in ElevenLabs: ${error}`);
    process.exit(1);
  }
}
async function pushTools(toolId?: string, dryRun = false): Promise<void> {
  // Load tools configuration
  const toolsConfigPath = path.resolve(TOOLS_CONFIG_FILE);
  if (!(await fs.pathExists(toolsConfigPath))) {
    throw new Error('tools.json not found. Run \'elevenlabs add-webhook-tool\' or \'elevenlabs add-client-tool\' first.');
  }

  const toolsConfig = await readToolsConfig(toolsConfigPath);

  // Filter tools by tool ID if specified
  let toolsToProcess = toolsConfig.tools;

  if (toolId) {
    toolsToProcess = toolsToProcess.filter(tool => tool.id === toolId);
    if (toolsToProcess.length === 0) {
      throw new Error(`Tool with ID '${toolId}' not found in configuration`);
    }
  }

  console.log(`Pushing ${toolsToProcess.length} tool(s) to ElevenLabs...`);

  let changesMade = false;

  for (const toolDef of toolsToProcess) {
    const configPath = toolDef.config;

    if (!configPath) {
      console.log(`Warning: No config path specified`);
      continue;
    }

    // Check if config file exists
    if (!(await fs.pathExists(configPath))) {
      console.log(`Warning: Config file not found: ${configPath}`);
      continue;
    }

    // Load tool config
    let toolConfig: { name?: string };
    try {
      toolConfig = await readConfig(configPath);
    } catch (error) {
      console.log(`Error reading config from ${configPath}: ${error}`);
      continue;
    }

    const toolDefName = toolConfig.name || 'Unnamed Tool';

    // Get tool ID from index file
    const toolId = toolDef.id;

    // Always push (force override)
    console.log(`${toolDefName}: Will push (force override)`);

    if (dryRun) {
      console.log(`[DRY RUN] Would update tool: ${toolDefName}`);
      continue;
    }

    // Initialize ElevenLabs client
    let client;
    try {
      client = await getElevenLabsClient();
    } catch (error) {
      console.log(`Error: ${error}`);
      console.log(`Skipping tool ${toolDefName} - not configured`);
      continue;
    }

    // Perform API operation
    try {
      if (!toolId) {
        // Create new tool
        const response = await createToolApi(client, toolConfig);
        const newToolId = (response as { toolId?: string }).toolId || `tool_${Date.now()}`;
        console.log(`Created tool ${toolDefName} (ID: ${newToolId})`);

        // Store tool ID in index file
        toolDef.id = newToolId;
        changesMade = true;
      } else {
        // Update existing tool
        await updateToolApi(client, toolId, toolConfig);
        console.log(`Updated tool ${toolDefName} (ID: ${toolId})`);
      }

      changesMade = true;

    } catch (error) {
      console.log(`Error processing ${toolDefName}: ${error}`);
    }
  }
  
  // Save updated tools.json if there were changes
  if (changesMade) {
    await writeToolsConfig(toolsConfigPath, toolsConfig);
  }
}
async function pullTools(options: PullToolsOptions): Promise<void> {
  // Check if tools.json exists, create if not
  const toolsConfigPath = path.resolve(TOOLS_CONFIG_FILE);

  console.log(`Pulling tools from ElevenLabs...`);

  await pullToolsFromEnvironment(options, toolsConfigPath);
}

async function pullToolsFromEnvironment(options: PullToolsOptions, toolsConfigPath: string): Promise<void> {
  let toolsConfig: ToolsConfig;

  if (!(await fs.pathExists(toolsConfigPath))) {
    console.log(`${TOOLS_CONFIG_FILE} not found. Creating initial tools configuration...`);
    toolsConfig = { tools: [] };
    await writeToolsConfig(toolsConfigPath, toolsConfig);
  } else {
    toolsConfig = await readToolsConfig(toolsConfigPath);
  }

  const client = await getElevenLabsClient();

  let filteredTools: unknown[];

  if (options.tool) {
    // Pull specific tool by ID
    console.log(`Pulling tool with ID: ${options.tool}...`);
    try {
      const toolDetails = await getToolApi(client, options.tool);
      const toolDetailsTyped = toolDetails as { tool_id?: string; toolId?: string; id?: string; tool_config?: { name?: string } & Tool };
      const toolId = toolDetailsTyped.tool_id || toolDetailsTyped.toolId || toolDetailsTyped.id || options.tool;
      const toolName = toolDetailsTyped.tool_config?.name;
      
      if (!toolName) {
        throw new Error(`Tool with ID '${options.tool}' has no name`);
      }
      
      filteredTools = [{
        tool_id: toolId,
        toolId: toolId,
        id: toolId,
        tool_config: toolDetailsTyped.tool_config
      }];
      console.log(`Found tool: ${toolName}`);
    } catch (error) {
      throw new Error(`Failed to fetch tool with ID '${options.tool}': ${error}`);
    }
  } else {
    // Pull all tools from ElevenLabs
    console.log('Pulling all tools from ElevenLabs...');
    const toolsList = await listToolsApi(client);

    if (toolsList.length === 0) {
      console.log('No tools found in your ElevenLabs workspace.');
      return;
    }

    console.log(`Found ${toolsList.length} tool(s)`);
    filteredTools = toolsList;
  }

  // Build map of existing tools by ID for this environment
  const existingToolIds = new Map(
    toolsConfig.tools
      .map(tool => [tool.id, tool])
  );

  // Track operations for summary
  const operations = { create: 0, update: 0, skip: 0 };
  type OperationItem = {
    action: 'create' | 'update' | 'skip';
    tool: { id: string; name: string };
    existingEntry?: ToolDefinition;
  };
  const itemsToProcess: OperationItem[] = [];

  // First pass: determine what will happen
  for (const toolMeta of filteredTools) {
    const toolMetaTyped = toolMeta as { tool_id?: string; toolId?: string; id?: string; tool_config?: { name?: string } };
    const toolId = toolMetaTyped.tool_id || toolMetaTyped.toolId || toolMetaTyped.id;
    const toolName = toolMetaTyped.tool_config?.name;
    if (!toolId) {
      console.log(`Warning: Skipping tool '${toolName || 'unknown'}' - no tool ID found`);
      continue;
    }
    if (!toolName) {
      console.log(`Warning: Skipping tool with ID '${toolId}' - no tool name found`);
      continue;
    }

    let toolNameRemote = toolName;
    const existingEntry = existingToolIds.get(toolId);

    if (existingEntry) {
      // Tool with this ID already exists locally
      if (options.update || options.all) {
        // --update or --all: update existing
        itemsToProcess.push({ action: 'update', tool: { id: toolId, name: toolNameRemote }, existingEntry });
        operations.update++;
      } else {
        // Default: skip existing
        itemsToProcess.push({ action: 'skip', tool: { id: toolId, name: toolNameRemote }, existingEntry });
        operations.skip++;
      }
    } else {
      // New tool (not present locally)
      if (options.update) {
        // --update mode: skip new items (only update existing)
        itemsToProcess.push({ action: 'skip', tool: { id: toolId, name: toolNameRemote } });
        operations.skip++;
      } else {
        // Default or --all: create new items
        itemsToProcess.push({ action: 'create', tool: { id: toolId, name: toolNameRemote } });
        operations.create++;
      }
    }
  }

  // Show summary
  console.log(`\nPlan: ${operations.create} create, ${operations.update} update, ${operations.skip} skip`);
  
  if (operations.skip > 0 && !options.update && !options.all) {
    if (operations.create === 0) {
      console.log(`\n💡 Tip: Use --update to update existing tools or --all to pull everything`);
    } else {
      console.log(`\n💡 Tip: Use --all to also update existing tools`);
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
    const { action, tool, existingEntry } = item;
    
    if (action === 'skip') {
      console.log(`⊘ Skipping '${tool.name}' (already exists, use --update to overwrite)`);
      continue;
    }
    
    if (options.dryRun) {
      console.log(`[DRY RUN] Would ${action} tool: ${tool.name} (ID: ${tool.id})`);
      continue;
    }
    
    try {
      // Fetch detailed tool configuration
      console.log(`${action === 'update' ? '↻ Updating' : '+ Pulling'} config for '${tool.name}'...`);
      const toolDetails = await getToolApi(client, tool.id);

      // Extract the tool_config from the response
      const toolDetailsTyped = toolDetails as { tool_config?: Tool & { type?: string } };
      const toolConfig = toolDetailsTyped.tool_config;
      
      if (!toolConfig) {
        console.log(`  ✗ Warning: No tool_config found for '${tool.name}' - skipping`);
        continue;
      }
      
      // Determine tool type from the tool_config
      const toolType = toolConfig.type || 'unknown';
      
      if (action === 'update' && existingEntry && existingEntry.config) {
        // Update existing entry - overwrite the config file
        const configFilePath = path.resolve(existingEntry.config);
        await fs.ensureDir(path.dirname(configFilePath));
        await writeToolConfig(configFilePath, toolConfig as Tool);
        console.log(`  ✓ Updated '${tool.name}' (config: ${existingEntry.config})`);
      } else {
        // Create new entry
        const configPath = await generateUniqueFilename(options.outputDir, tool.name);
        const configFilePath = path.resolve(configPath);
        await fs.ensureDir(path.dirname(configFilePath));
        await writeToolConfig(configFilePath, toolConfig as Tool);
        
        const newTool: ToolDefinition = {
          type: toolType as 'webhook' | 'client',
          config: configPath,
          id: tool.id,
        };
        
        toolsConfig.tools.push(newTool);
        console.log(`  ✓ Added '${tool.name}' (config: ${configPath}, type: ${toolType})`);
      }
      
      itemsProcessed++;
      
    } catch (error) {
      console.log(`  ✗ Error ${action === 'update' ? 'updating' : 'pulling'} tool '${tool.name}': ${error}`);
      continue;
    }
  }

  // Save updated tools.json if there were changes
  if (!options.dryRun && itemsProcessed > 0) {
    await writeToolsConfig(toolsConfigPath, toolsConfig);
    console.log(`\nUpdated ${TOOLS_CONFIG_FILE}`);
  }

  // Final summary
  if (options.dryRun) {
    console.log(`\n[DRY RUN] Would process ${operations.create + operations.update} tool(s)`);
  } else {
    console.log(`\n✓ Summary: ${operations.create} created, ${operations.update} updated, ${operations.skip} skipped`);
    if (itemsProcessed > 0) {
      console.log(`You can now edit the config files in '${options.outputDir}/' and use them in your agents`);
    }
  }
}
async function deleteTool(toolId: string): Promise<void> {
  // Load tools configuration
  const toolsConfigPath = path.resolve(TOOLS_CONFIG_FILE);
  if (!(await fs.pathExists(toolsConfigPath))) {
    throw new Error('tools.json not found. Run \'elevenlabs agents init\' first.');
  }

  const toolsConfig = await readToolsConfig(toolsConfigPath);
  
  // Find the tool by ID
  const toolIndex = toolsConfig.tools.findIndex(tool => tool.id === toolId);
  
  if (toolIndex === -1) {
    throw new Error(`Tool with ID '${toolId}' not found in local configuration`);
  }
  
  const toolDef = toolsConfig.tools[toolIndex];
  const configPath = toolDef.config;

  // Read tool name from config if available
  let toolName = toolId;
  if (configPath && await fs.pathExists(configPath)) {
    try {
      const toolConfig = await readConfig(configPath) as { name?: string };
      toolName = toolConfig.name || toolId;
    } catch {
      // If reading fails, just use ID
    }
  }

  console.log(`Deleting tool '${toolName}' (ID: ${toolId})...`);

  // Delete from ElevenLabs
  console.log('Deleting from ElevenLabs...');
  const client = await getElevenLabsClient();
  
  try {
    await deleteToolApi(client, toolId);
    console.log('✓ Successfully deleted from ElevenLabs');
  } catch (error) {
    console.error(`Warning: Failed to delete from ElevenLabs: ${error}`);
    console.log('Continuing with local deletion...');
  }
  
  // Remove from local tools.json
  toolsConfig.tools.splice(toolIndex, 1);
  await writeToolsConfig(toolsConfigPath, toolsConfig);
  console.log(`✓ Removed '${toolName}' from ${TOOLS_CONFIG_FILE}`);
  

  // Remove from local tools.json
  toolsConfig.tools.splice(toolIndex, 1);
  await writeToolsConfig(toolsConfigPath, toolsConfig);
  console.log(`✓ Removed '${toolName}' from ${TOOLS_CONFIG_FILE}`);

  // Remove config file
  if (configPath && await fs.pathExists(configPath)) {
    await fs.remove(configPath);
    console.log(`✓ Deleted config file: ${configPath}`);
  }

  console.log(`
✓ Successfully deleted tool '${toolName}'`);
}
async function deleteAllTools(ui: boolean = true): Promise<void> {
  // Load tools configuration
  const toolsConfigPath = path.resolve(TOOLS_CONFIG_FILE);
  if (!(await fs.pathExists(toolsConfigPath))) {
    throw new Error('tools.json not found. Run \'elevenlabs agents init\' first.');
  }

  const toolsConfig = await readToolsConfig(toolsConfigPath);

  if (toolsConfig.tools.length === 0) {
    console.log('No tools found to delete');
    return;
  }

  const toolsToDelete = toolsConfig.tools;

  // Show what will be deleted
  console.log(`\nFound ${toolsToDelete.length} tool(s) to delete:`);
  for (let i = 0; i < toolsToDelete.length; i++) {
    const tool = toolsToDelete[i];
    let toolName = tool.id || 'Unknown';
    if (tool.config && await fs.pathExists(tool.config)) {
      try {
        const toolConfig = await readConfig(tool.config) as { name?: string };
        toolName = toolConfig.name || tool.id || 'Unknown';
      } catch {
        // Use ID if config read fails
      }
    }
    console.log(`  ${i + 1}. ${toolName} (${tool.id})`);
  }
  
  // Confirm deletion (skip if --no-ui)
  if (ui) {
    console.log('\nWARNING: This will delete ALL tools from both local configuration and ElevenLabs.');
    const confirmed = await promptForConfirmation('Are you sure you want to delete these tools?');
    
    if (!confirmed) {
      console.log('Deletion cancelled');
      return;
    }
  }
  
  console.log('\nDeleting tools...\n');
  
  let successCount = 0;
  let failCount = 0;
  const deletedIds = new Set<string>();
  
  // Delete each tool
  for (const toolDef of toolsToDelete) {
    try {
      // Read tool name from config if available
      let toolName = toolDef.id || 'Unknown';
      if (toolDef.config && await fs.pathExists(toolDef.config)) {
        try {
          const toolConfig = await readConfig(toolDef.config) as { name?: string };
          toolName = toolConfig.name || toolDef.id || 'Unknown';
        } catch {
          // If reading fails, just use ID
        }
      }

      console.log(`Deleting '${toolName}' (${toolDef.id})...`);
      
      // Delete from ElevenLabs
      if (toolDef.id) {
        try {
          const client = await getElevenLabsClient();
          await deleteToolApi(client, toolDef.id);
          console.log(`  ✓ Deleted from ElevenLabs`);
        } catch (error) {
          console.error(`  Warning: Failed to delete from ElevenLabs: ${error}`);
        }
      } else {
        console.log(`  Warning: No tool ID found, skipping ElevenLabs deletion`);
      }
      
      // Remove config file
      if (toolDef.config && await fs.pathExists(toolDef.config)) {
        await fs.remove(toolDef.config);
        console.log(`  ✓ Deleted config file: ${toolDef.config}`);
      }
      
      if (toolDef.id) {
        deletedIds.add(toolDef.id);
      }
      successCount++;
    } catch (error) {
      console.error(`  Failed to delete tool: ${error}`);
      failCount++;
    }
  }
  
  // Remove deleted tools from config (only the ones that were successfully deleted)
  toolsConfig.tools = toolsConfig.tools.filter(tool => !deletedIds.has(tool.id || ''));
  await writeToolsConfig(toolsConfigPath, toolsConfig);
  console.log(`\n✓ Updated ${TOOLS_CONFIG_FILE}`);
  
  // Summary
  console.log(`\n✓ Deletion complete: ${successCount} succeeded, ${failCount} failed`);
}

export { addTool, pushTools, pullTools, deleteTool, deleteAllTools };

interface PullToolsOptions {
  tool?: string;
  outputDir: string;
  dryRun: boolean;
  update?: boolean;
  all?: boolean;
  env?: string;
}
