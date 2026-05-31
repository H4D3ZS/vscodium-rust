import React, { useState, useEffect } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { useStore } from '../store';

// Curated MCP server marketplace — the official @modelcontextprotocol servers
// (run via npx). One-click install registers them with the registry. Servers
// that need a path/token install with a placeholder arg the user can edit.
interface CatalogEntry {
    id: string;
    name: string;
    description: string;
    command: string;
    args: string[];
    icon: string;
    needsConfig?: string; // human note on what to fill in after install
}

const MCP_CATALOG: CatalogEntry[] = [
    { id: 'filesystem', name: 'Filesystem', description: 'Read/write files in allowed directories.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-filesystem', '.'], icon: 'folder', needsConfig: 'Set the allowed directory path (last arg).' },
    { id: 'github', name: 'GitHub', description: 'Repos, issues, PRs, code search.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-github'], icon: 'github', needsConfig: 'Set GITHUB_PERSONAL_ACCESS_TOKEN env.' },
    { id: 'git', name: 'Git', description: 'Local git repo operations (log, diff, blame).', command: 'npx', args: ['-y', '@modelcontextprotocol/server-git'], icon: 'git-merge' },
    { id: 'brave-search', name: 'Brave Search', description: 'Web search via Brave API.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-brave-search'], icon: 'search', needsConfig: 'Set BRAVE_API_KEY env.' },
    { id: 'fetch', name: 'Fetch', description: 'Fetch + convert web pages to markdown.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-fetch'], icon: 'globe' },
    { id: 'memory', name: 'Memory', description: 'Persistent knowledge-graph memory.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-memory'], icon: 'database' },
    { id: 'sequential-thinking', name: 'Sequential Thinking', description: 'Structured step-by-step reasoning tool.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-sequential-thinking'], icon: 'list-ordered' },
    { id: 'puppeteer', name: 'Puppeteer', description: 'Headless browser automation + screenshots.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-puppeteer'], icon: 'browser' },
    { id: 'postgres', name: 'PostgreSQL', description: 'Query Postgres databases (read-only).', command: 'npx', args: ['-y', '@modelcontextprotocol/server-postgres', 'postgresql://localhost/db'], icon: 'database', needsConfig: 'Set your connection string (last arg).' },
    { id: 'sqlite', name: 'SQLite', description: 'Query a local SQLite database.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-sqlite', './data.db'], icon: 'database', needsConfig: 'Set the .db path (last arg).' },
    { id: 'slack', name: 'Slack', description: 'Read/post Slack messages.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-slack'], icon: 'comment-discussion', needsConfig: 'Set SLACK_BOT_TOKEN env.' },
    { id: 'gitlab', name: 'GitLab', description: 'GitLab projects, issues, MRs.', command: 'npx', args: ['-y', '@modelcontextprotocol/server-gitlab'], icon: 'git-merge', needsConfig: 'Set GITLAB_PERSONAL_ACCESS_TOKEN env.' },
];

const McpManager: React.FC = () => {
    const { mcpServers, addMcpServer, listMcpServers } = useStore(useShallow(s => ({
        mcpServers: s.mcpServers,
        addMcpServer: s.addMcpServer,
        listMcpServers: s.listMcpServers,
    })));
    const [name, setName] = useState('');
    const [command, setCommand] = useState('');
    const [args, setArgs] = useState('');
    const [isAdding, setIsAdding] = useState(false);

    useEffect(() => {
        listMcpServers();
    }, [listMcpServers]);

    const [installing, setInstalling] = useState<string | null>(null);
    const [search, setSearch] = useState('');

    const handleAdd = async () => {
        if (!name || !command) return;
        setIsAdding(true);
        const argsArray = args.split(',').map(a => a.trim()).filter(a => a !== '');
        await addMcpServer(name, { command, args: argsArray });
        setName('');
        setCommand('');
        setArgs('');
        setIsAdding(false);
    };

    const isInstalled = (entry: CatalogEntry) =>
        mcpServers.some(s => s.name.toLowerCase() === entry.name.toLowerCase());

    const handleInstall = async (entry: CatalogEntry) => {
        setInstalling(entry.id);
        try {
            await addMcpServer(entry.name, { command: entry.command, args: entry.args });
            await listMcpServers();
        } finally {
            setInstalling(null);
        }
    };

    const filteredCatalog = MCP_CATALOG.filter(e =>
        !search || e.name.toLowerCase().includes(search.toLowerCase()) || e.description.toLowerCase().includes(search.toLowerCase())
    );

    return (
        <div className="mcp-manager">
            <div className="settings-section">
                <div className="settings-section-title">MCP Servers</div>
                <div className="settings-item-description" style={{ marginBottom: '16px' }}>
                    Connect Project Hades to external tools and data sources using the Model Context Protocol.
                </div>

                {/* ── Marketplace: browse + one-click install popular MCP servers ── */}
                <div style={{ marginBottom: '24px' }}>
                    <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '10px' }}>
                        <div style={{ fontWeight: 600, fontSize: '13px' }}>🛒 Marketplace</div>
                        <input
                            type="text"
                            placeholder="Search servers…"
                            value={search}
                            onChange={(e) => setSearch(e.target.value)}
                            style={{ fontSize: '12px', padding: '4px 8px', background: 'var(--vscode-input-background, #1e1e1e)', color: 'var(--vscode-input-foreground, #ccc)', border: '1px solid var(--vscode-panel-border, #333)', borderRadius: '4px', outline: 'none', width: '160px' }}
                        />
                    </div>
                    <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(220px, 1fr))', gap: '8px' }}>
                        {filteredCatalog.map(entry => {
                            const installed = isInstalled(entry);
                            return (
                                <div key={entry.id} style={{ padding: '10px 12px', background: 'rgba(255,255,255,0.03)', border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.08))', borderRadius: '6px', display: 'flex', flexDirection: 'column', gap: '6px' }}>
                                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                        <i className={`codicon codicon-${entry.icon}`} style={{ color: '#007acc' }}></i>
                                        <span style={{ fontSize: '13px', fontWeight: 600 }}>{entry.name}</span>
                                    </div>
                                    <div style={{ fontSize: '11px', opacity: 0.6, lineHeight: 1.4, minHeight: '30px' }}>{entry.description}</div>
                                    {entry.needsConfig && (
                                        <div style={{ fontSize: '10px', opacity: 0.45, fontStyle: 'italic' }}>⚙ {entry.needsConfig}</div>
                                    )}
                                    <button
                                        onClick={() => handleInstall(entry)}
                                        disabled={installed || installing === entry.id}
                                        style={{ marginTop: 'auto', padding: '4px 8px', fontSize: '11px', fontWeight: 600, borderRadius: '4px', cursor: installed ? 'default' : 'pointer', border: 'none', background: installed ? 'rgba(74,222,128,0.15)' : '#007acc', color: installed ? '#4ade80' : '#fff' }}
                                    >
                                        {installed ? '✓ Installed' : installing === entry.id ? 'Installing…' : '+ Install'}
                                    </button>
                                </div>
                            );
                        })}
                    </div>
                </div>

                <div className="mcp-list" style={{ marginBottom: '24px' }}>
                    {mcpServers.length === 0 ? (
                        <div style={{ opacity: 0.5, fontStyle: 'italic', fontSize: '13px', padding: '12px', background: 'rgba(255,255,255,0.03)', borderRadius: '4px' }}>
                            No MCP servers registered.
                        </div>
                    ) : (
                        mcpServers.map((server, i) => (
                            <div key={i} className="mcp-server-item" style={{
                                display: 'flex',
                                alignItems: 'center',
                                gap: '12px',
                                padding: '8px 12px',
                                background: 'rgba(255,255,255,0.05)',
                                border: '1px solid rgba(255,255,255,0.1)',
                                borderRadius: '4px',
                                marginBottom: '8px'
                            }}>
                                <i className="codicon codicon-server" style={{ color: '#007acc' }}></i>
                                <span style={{ fontSize: '13px' }}>{server.name}</span>
                                <span style={{ marginLeft: 'auto', fontSize: '11px', opacity: 0.5 }}>Connected</span>
                            </div>
                        ))
                    )}
                </div>

                <div className="add-mcp-form" style={{ background: 'rgba(255,255,255,0.02)', padding: '16px', borderRadius: '6px', border: '1px dashed rgba(255,255,255,0.1)' }}>
                    <div style={{ fontWeight: 600, fontSize: '13px', marginBottom: '12px' }}>Add New MCP Server</div>
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
                        <div className="settings-item-control" style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                            <label style={{ fontSize: '11px', opacity: 0.7 }}>Server Name</label>
                            <input
                                type="text"
                                placeholder="e.g. Memory Server"
                                value={name}
                                onChange={(e) => setName(e.target.value)}
                            />
                        </div>
                        <div className="settings-item-control" style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                            <label style={{ fontSize: '11px', opacity: 0.7 }}>Command</label>
                            <input
                                type="text"
                                placeholder="e.g. node"
                                value={command}
                                onChange={(e) => setCommand(e.target.value)}
                            />
                        </div>
                        <div className="settings-item-control" style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                            <label style={{ fontSize: '11px', opacity: 0.7 }}>Arguments (comma separated)</label>
                            <input
                                type="text"
                                placeholder="e.g. ./server.js, --db, ./data.db"
                                value={args}
                                onChange={(e) => setArgs(e.target.value)}
                            />
                        </div>
                        <button
                            onClick={handleAdd}
                            disabled={isAdding || !name || !command}
                            style={{
                                marginTop: '8px',
                                padding: '6px 12px',
                                background: '#007acc',
                                color: 'white',
                                border: 'none',
                                borderRadius: '2px',
                                cursor: 'pointer',
                                fontSize: '12px'
                            }}
                        >
                            {isAdding ? 'Adding...' : 'Add MCP Server'}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default McpManager;
