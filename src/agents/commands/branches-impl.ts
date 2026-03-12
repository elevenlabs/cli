import { getElevenLabsClient, listBranchesApi } from '../../shared/elevenlabs-api.js';

interface BranchesListOptions {
  agent: string;
  includeArchived: boolean;
}

export async function listBranches(options: BranchesListOptions): Promise<void> {
  const client = await getElevenLabsClient();

  console.log(`Listing branches for agent: ${options.agent}...`);

  const branches = await listBranchesApi(client, options.agent, options.includeArchived);

  if (branches.length === 0) {
    console.log('No branches found for this agent.');
    return;
  }

  // Print table header
  const nameCol = 25;
  const idCol = 40;
  const statusCol = 12;
  const trafficCol = 10;

  console.log(
    `${'NAME'.padEnd(nameCol)}${'BRANCH ID'.padEnd(idCol)}${'STATUS'.padEnd(statusCol)}${'TRAFFIC'.padEnd(trafficCol)}LAST UPDATED`
  );
  console.log('─'.repeat(110));

  for (const branch of branches) {
    const name = branch.name.length > nameCol - 2 ? branch.name.slice(0, nameCol - 5) + '...' : branch.name;
    const status = branch.isArchived ? 'archived' : 'active';
    const traffic = `${branch.currentLivePercentage ?? 0}%`;
    const lastUpdated = new Date(branch.lastCommittedAt * 1000).toISOString().split('T')[0];

    console.log(
      `${name.padEnd(nameCol)}${branch.id.padEnd(idCol)}${status.padEnd(statusCol)}${traffic.padEnd(trafficCol)}${lastUpdated}`
    );
  }

  console.log(`\n${branches.length} branch(es) found`);
}
