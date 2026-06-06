/**
 * Curated MCP Store catalog — modeled on Google Antigravity IDE's built-in MCP Store.
 *
 * Antigravity pattern: browse pre-configured servers in the agent panel, one-click Install
 * writes to mcp_config.json (`mcpServers` root), external process spawns, tools join the agent.
 * We use the same package IDs and @toolbox-sdk/server prebuilts where Antigravity docs specify them.
 *
 * @see docs/MCP_STORE.md
 * @see https://cloud.google.com/bigquery/docs/pre-built-tools-with-mcp-toolbox
 */

export type McpCatalogCategory = 'development' | 'data' | 'cloud' | 'security' | 'productivity';

export interface McpEnvField {
    key: string;
    label: string;
    placeholder?: string;
    secret?: boolean;
}

export interface McpCatalogEntry {
    id: string;
    name: string;
    description: string;
    category: McpCatalogCategory;
    /** Required for stdio servers; omit for HTTP entries. */
    command?: string;
    args?: string[];
    /** Pre-filled env keys (values filled at install time). */
    envFields?: McpEnvField[];
    type?: 'stdio' | 'http';
    serverUrl?: string;
    needsConfig?: string;
    tags?: string[];
}

export const MCP_CATALOG: McpCatalogEntry[] = [
    // ── Development ──
    { id: 'filesystem', name: 'Filesystem', category: 'development', description: 'Read and write files in allowed directories.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-filesystem', '.'], needsConfig: 'Edit the allowed path (last arg) after install.' },
    { id: 'github', name: 'GitHub', category: 'development', description: 'Repos, issues, pull requests, and code search.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-github'], envFields: [{ key: 'GITHUB_PERSONAL_ACCESS_TOKEN', label: 'GitHub PAT', secret: true }] },
    { id: 'git', name: 'Git', category: 'development', description: 'Local git operations — log, diff, blame, status.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-git'] },
    { id: 'gitlab', name: 'GitLab', category: 'development', description: 'GitLab projects, issues, and merge requests.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-gitlab'], envFields: [{ key: 'GITLAB_PERSONAL_ACCESS_TOKEN', label: 'GitLab token', secret: true }] },
    { id: 'fetch', name: 'Fetch', category: 'development', description: 'Fetch web pages and convert them to markdown.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-fetch'] },
    { id: 'puppeteer', name: 'Puppeteer', category: 'development', description: 'Headless browser automation and screenshots.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-puppeteer'] },
    { id: 'sequential-thinking', name: 'Sequential Thinking', category: 'development', description: 'Structured step-by-step reasoning for complex tasks.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-sequential-thinking'] },
    { id: 'context7', name: 'Context7', category: 'development', description: 'Up-to-date library docs injected into prompts.', command: 'npx', args: ['-y', '@upstash/context7-mcp'], envFields: [{ key: 'CONTEXT7_API_KEY', label: 'Context7 API key', secret: true }] },
    { id: 'memory', name: 'Memory', category: 'development', description: 'Persistent knowledge-graph memory across sessions.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-memory'] },

    // ── Data ──
    { id: 'postgres', name: 'PostgreSQL', category: 'data', description: 'Query Postgres databases (read-only).', command: 'npx', args: ['-y', '@modelcontextprotocol/server-postgres', 'postgresql://localhost/db'], needsConfig: 'Set your connection string (last arg).' },
    { id: 'sqlite', name: 'SQLite', category: 'data', description: 'Query a local SQLite database file.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-sqlite', './data.db'], needsConfig: 'Set the .db path (last arg).' },
    { id: 'bigquery', name: 'BigQuery', category: 'data', description: 'Query BigQuery datasets with natural language.', command: 'npx', args: ['-y', '@toolbox-sdk/server', '--prebuilt', 'bigquery', '--stdio'], envFields: [{ key: 'BIGQUERY_PROJECT', label: 'GCP project ID' }] },
    { id: 'spanner', name: 'Spanner', category: 'data', description: 'Create, manage, and query Cloud Spanner resources.', command: 'npx', args: ['-y', '@toolbox-sdk/server', '--prebuilt', 'spanner', '--stdio'], envFields: [{ key: 'SPANNER_PROJECT', label: 'GCP project ID' }] },

    // ── Cloud ──
    { id: 'firebase', name: 'Firebase', category: 'cloud', description: 'AI-powered tools for Firebase projects.', command: 'npx', args: ['-y', 'firebase-tools@latest', 'mcp'] },
    { id: 'cloud-run', name: 'Cloud Run', category: 'cloud', description: 'Deploy and manage apps on Google Cloud Run.', command: 'npx', args: ['-y', '@google-cloud/cloud-run-mcp'], envFields: [{ key: 'GOOGLE_CLOUD_PROJECT', label: 'GCP project ID' }] },
    { id: 'brave-search', name: 'Brave Search', category: 'cloud', description: 'Web search via the Brave Search API.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-brave-search'], envFields: [{ key: 'BRAVE_API_KEY', label: 'Brave API key', secret: true }] },
    { id: 'slack', name: 'Slack', category: 'cloud', description: 'Read and post Slack messages.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-slack'], envFields: [{ key: 'SLACK_BOT_TOKEN', label: 'Slack bot token', secret: true }] },

    // ── Security (optional — heavy external toolchain) ──
    {
        id: 'ida-pro-idalib',
        name: 'IDA Pro MCP (Headless)',
        category: 'security',
        description:
            '245+ live RE tools via idalib-mcp (decompile, disasm, xrefs, patches, py_eval). Requires IDA Pro 8.3+ — IDA Free not supported.',
        command: 'uvx',
        args: ['--from', 'git+https://github.com/mrexodia/ida-pro-mcp', 'idalib-mcp', '--stdio'],
        envFields: [{ key: 'IDA_MCP_MAX_WORKERS', label: 'Max worker processes (optional)', placeholder: '4' }],
        needsConfig:
            'Prereqs: Python 3.11+, uv, IDA Pro (paid). Activate idalib once (Windows): uv run "C:\\Program Files\\IDA Professional 9.x\\idalib\\python\\py-activate-idalib.py". Call idb_open() before other tools; pass database=<session_id> on every call.',
        tags: ['reverse-engineering', 'ida', 'binary-analysis', 'mcp'],
    },
    {
        id: 'ida-pro-http',
        name: 'IDA Pro MCP (GUI Plugin)',
        category: 'security',
        description:
            'Connect to IDA Pro GUI plugin MCP server (live database). Start MCP inside IDA first — for interactive RE sessions.',
        type: 'http',
        serverUrl: 'http://127.0.0.1:13337/mcp',
        needsConfig:
            'Install plugin: pip install https://github.com/mrexodia/ida-pro-mcp/archive/refs/heads/main.zip then ida-pro-mcp --install. Restart IDA. Enable MCP in IDA and ensure port 13337 is listening. SSE alternative: http://127.0.0.1:8744/sse',
        tags: ['reverse-engineering', 'ida', 'binary-analysis', 'mcp'],
    },
    {
        id: 'ghidra-pyghidra',
        name: 'Ghidra MCP (Headless)',
        category: 'security',
        description:
            'Headless Ghidra via pyghidra-mcp — decompile, xrefs, structs, batch analysis. No Ghidra GUI required.',
        command: 'uvx',
        args: [
            '--from',
            'git+https://github.com/clearbluejar/pyghidra-mcp',
            'pyghidra-mcp',
            '-t',
            'stdio',
            '--project-path',
            './ghidra-projects',
        ],
        envFields: [{ key: 'GHIDRA_INSTALL_DIR', label: 'Ghidra install path', placeholder: 'C:\\ghidra_12.0_PUBLIC' }],
        needsConfig: 'Set GHIDRA_INSTALL_DIR to your Ghidra root. Edit --project-path (last args) to a writable directory.',
        tags: ['reverse-engineering', 'ghidra', 'binary-analysis', 'mcp'],
    },
    {
        id: 'ghidra-mcp-http',
        name: 'Ghidra MCP (Plugin HTTP)',
        category: 'security',
        description:
            'Connect to GhidraMCP plugin HTTP server (200+ tools when Ghidra CodeBrowser is open with a binary loaded).',
        type: 'http',
        serverUrl: 'http://127.0.0.1:8089/',
        needsConfig:
            'Install bethington/ghidra-mcp Ghidra extension. In CodeBrowser: File → Configure → GhidraMCP, then Tools → GhidraMCP → Start MCP Server. Bridge HTTP mode: http://127.0.0.1:8081/mcp',
        tags: ['reverse-engineering', 'ghidra', 'binary-analysis', 'mcp'],
    },
    { id: 'hexstrike', name: 'HexStrike AI', category: 'security', description: '150+ pentest CLI tools via MCP. Run separately; requires Python + tool deps.', command: 'python', args: ['-m', 'hexstrike_mcp'], needsConfig: 'Clone github.com/0x4m4/hexstrike-ai and install deps first. Use absolute python path.', tags: ['bug-bounty', 'pentest'] },

    // ── Productivity ──
    { id: 'notion', name: 'Notion', category: 'productivity', description: 'Search and update Notion pages and databases.', command: 'npx', args: ['-y', '@notionhq/notion-mcp-server'], envFields: [{ key: 'NOTION_API_KEY', label: 'Notion integration token', secret: true }] },
];

export const MCP_CATEGORY_LABELS: Record<McpCatalogCategory, string> = {
    development: 'Development',
    data: 'Data & Analytics',
    cloud: 'Cloud & APIs',
    security: 'Security & Research',
    productivity: 'Productivity',
};
