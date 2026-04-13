import React, { useState, useEffect } from 'react';
import { useStore } from '../store';

const McpManager: React.FC = () => {
    const { mcpServers, addMcpServer, listMcpServers } = useStore();
    const [name, setName] = useState('');
    const [command, setCommand] = useState('');
    const [args, setArgs] = useState('');
    const [isAdding, setIsAdding] = useState(false);

    useEffect(() => {
        listMcpServers();
    }, [listMcpServers]);

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

    return (
        <div className="mcp-manager">
            <div className="settings-section">
                <div className="settings-section-title">MCP Servers</div>
                <div className="settings-item-description" style={{ marginBottom: '16px' }}>
                    Connect Project Hades to external tools and data sources using the Model Context Protocol.
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
