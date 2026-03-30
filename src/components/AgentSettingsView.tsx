import React, { useEffect, useState } from 'react';
import { useStore } from '../store';
import { getThemes, applyTheme } from '../theme_engine';
import type { VscodeTheme } from '../theme_engine';

const AgentSettingsView: React.FC = () => {
    const ollamaUrl = useStore(state => state.ollamaUrl);
    const setOllamaUrl = useStore(state => state.setOllamaUrl);
    const ollamaStatus = useStore(state => state.ollamaStatus);
    const refreshModels = useStore(state => state.refreshAvailableModels);
    const agentModel = useStore(state => state.agentModel);
    const setAgentModel = useStore(state => state.setAgentModel);
    const availableModels = useStore(state => state.availableModels);
    const setTheme = useStore(state => state.setTheme);
    const mcpServers = useStore(state => state.mcpServers);
    const addMcpServer = useStore(state => state.addMcpServer);
    const removeMcpServer = useStore(state => state.removeMcpServer);
    const listMcpServers = useStore(state => state.listMcpServers);
    const isPullingModel = useStore(state => state.isPullingModel);
    const pullOllamaModel = useStore(state => state.pullOllamaModel);
    const [pullInput, setPullInput] = useState('');

    const [newMcpName, setNewMcpName] = useState('');
    const [newMcpType, setNewMcpType] = useState<'command' | 'http'>('command');
    const [newMcpCommand, setNewMcpCommand] = useState('');
    const [newMcpArgs, setNewMcpArgs] = useState('');
    const [newMcpUrl, setNewMcpUrl] = useState('');
    const [isAddingMcp, setIsAddingMcp] = useState(false);

    useEffect(() => {
        listMcpServers().catch(console.error);
    }, []);

    return (
        <div className="agent-settings-view" style={{ padding: '12px', display: 'flex', flexDirection: 'column', gap: '20px', height: '100%', overflowY: 'auto' }}>
            <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', marginBottom: '10px', padding: '20px 0', background: 'rgba(255,255,255,0.02)', borderRadius: '12px', border: '1px solid rgba(255,255,255,0.05)' }}>
                <div style={{ position: 'relative', marginBottom: '12px' }}>
                    <div style={{
                        width: '64px', height: '64px',
                        background: 'linear-gradient(135deg, #3b82f6 0%, #2563eb 100%)',
                        borderRadius: '20px',
                        display: 'flex', alignItems: 'center', justifyContent: 'center',
                        boxShadow: '0 8px 24px rgba(37, 99, 235, 0.4)'
                    }}>
                        <i className="codicon codicon-sparkle" style={{ fontSize: '32px', color: '#fff' }}></i>
                    </div>
                    {ollamaStatus === 'running' && (
                        <div style={{
                            position: 'absolute', bottom: '-4px', right: '-4px',
                            width: '16px', height: '16px', borderRadius: '50%',
                            background: '#4ade80',
                            border: '3px solid var(--vscode-sideBar-background)',
                            boxShadow: '0 0 12px rgba(74, 222, 128, 0.6)'
                        }}></div>
                    )}
                </div>
                <div style={{ fontSize: '18px', fontWeight: 800, color: '#fff', letterSpacing: '-0.5px', display: 'flex', alignItems: 'center', gap: '8px' }}>
                    Antigravity Engine
                    <div style={{
                        padding: '2px 8px',
                        background: 'rgba(255,255,255,0.05)',
                        borderRadius: '100px',
                        fontSize: '9px',
                        fontWeight: 700,
                        border: '1px solid rgba(255,255,255,0.1)',
                        color: ollamaStatus === 'running' ? '#4ade80' : 'rgba(255,255,255,0.6)'
                    }}>
                        {(agentModel.split('|').pop() || '').split(':')[0].toUpperCase()}
                    </div>
                </div>
                <div style={{ fontSize: '11px', opacity: 0.5, fontWeight: 600, textTransform: 'uppercase', letterSpacing: '0.1em' }}>Core Configuration</div>
            </div>

            <section>
                <div style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-sideBarSectionHeader-foreground)', marginBottom: '12px', textTransform: 'uppercase' }}>
                    Model Configuration
                </div>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                    <label style={{ fontSize: '11px', opacity: 0.8 }}>Active Model</label>
                    <select
                        value={agentModel}
                        onChange={(e) => setAgentModel(e.target.value)}
                        style={{ background: 'var(--vscode-dropdown-background)', color: 'var(--vscode-dropdown-foreground)', border: '1px solid var(--vscode-dropdown-border)', padding: '4px', fontSize: '12px', cursor: 'pointer', position: 'relative', zIndex: 1 }}
                    >
                        {availableModels.map(m => (
                            <option key={`${m.provider}|${m.id}`} value={`${m.provider}|${m.id}`}>
                                {m.provider.toUpperCase()} - {m.id}
                            </option>
                        ))}
                    </select>
                </div>
            </section>

            <section>
                <div style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-sideBarSectionHeader-foreground)', marginBottom: '12px', textTransform: 'uppercase' }}>
                    Ollama Integration
                </div>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                        <label style={{ fontSize: '11px', opacity: 0.8 }}>Self-Hosted URL</label>
                        <input
                            type="text"
                            value={ollamaUrl}
                            onChange={(e) => setOllamaUrl(e.target.value)}
                            style={{ background: 'var(--vscode-input-background)', color: 'var(--vscode-input-foreground)', border: '1px solid var(--vscode-input-border)', padding: '4px 8px', fontSize: '12px' }}
                            placeholder="http://localhost:11434"
                        />
                    </div>

                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <div style={{
                            width: '8px',
                            height: '8px',
                            borderRadius: '50%',
                            background: ollamaStatus === 'running' ? '#4ade80' : ollamaStatus === 'error' ? '#f87171' : '#fbbf24'
                        }}></div>
                        <span style={{ fontSize: '12px' }}>
                            {ollamaStatus === 'running' ? 'Connected' : ollamaStatus === 'error' ? 'Error' : 'Checking...'}
                        </span>
                        <button
                            onClick={() => refreshModels('ollama')}
                            style={{ marginLeft: 'auto', background: 'var(--vscode-button-secondaryBackground)', color: 'var(--vscode-button-secondaryForeground)', border: 'none', padding: '2px 8px', fontSize: '10px', cursor: 'pointer', borderRadius: '4px' }}
                        >
                            Reconnect
                        </button>
                    </div>

                    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px', marginTop: '4px', padding: '10px', background: 'var(--vscode-sideBar-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: '2px' }}>
                        <label style={{ fontSize: '11px', fontWeight: 600, opacity: 0.7 }}>Pull New Model</label>
                        <div style={{ display: 'flex', gap: '8px' }}>
                            <input
                                type="text"
                                placeholder="e.g. deepseek-v3"
                                value={pullInput}
                                onChange={(e) => setPullInput(e.target.value)}
                                style={{ flex: 1, background: 'var(--vscode-input-background)', color: 'var(--vscode-input-foreground)', border: '1px solid var(--vscode-input-border)', padding: '4px 8px', fontSize: '11px' }}
                                disabled={isPullingModel}
                            />
                            <button
                                onClick={() => {
                                    if (pullInput) {
                                        pullOllamaModel(pullInput);
                                        setPullInput('');
                                    }
                                }}
                                disabled={isPullingModel || !pullInput}
                                style={{
                                    background: isPullingModel ? 'var(--vscode-button-secondaryBackground)' : 'var(--vscode-button-background)',
                                    color: 'var(--vscode-button-foreground)',
                                    border: 'none',
                                    padding: '4px 12px',
                                    fontSize: '11px',
                                    cursor: isPullingModel ? 'wait' : 'pointer',
                                    borderRadius: '2px',
                                    fontWeight: 600
                                }}
                            >
                                {isPullingModel ? 'Pulling...' : 'Pull'}
                            </button>
                        </div>
                        {isPullingModel && (
                            <div style={{ height: '2px', background: 'rgba(255,255,255,0.1)', borderRadius: '1px', marginTop: '4px', overflow: 'hidden' }}>
                                <div style={{ height: '100%', width: '100%', background: '#3b82f6', animation: 'progressIndeterminate 1.5s infinite linear' }}></div>
                            </div>
                        )}
                    </div>
                </div>
            </section>

            <section>
                <div style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-sideBarSectionHeader-foreground)', marginBottom: '12px', textTransform: 'uppercase', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    MCP Servers
                    <button
                        onClick={() => setIsAddingMcp(!isAddingMcp)}
                        style={{ background: 'transparent', border: 'none', color: '#3b82f6', cursor: 'pointer', fontSize: '10px' }}
                    >
                        {isAddingMcp ? 'Cancel' : '+ Add Server'}
                    </button>
                </div>

                {isAddingMcp && (
                    <div style={{ marginBottom: '16px', padding: '10px', background: 'var(--vscode-sideBar-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: '2px', display: 'flex', flexDirection: 'column', gap: '8px' }}>
                        <input
                            placeholder="Server Name (e.g. filesystem)"
                            value={newMcpName}
                            onChange={e => setNewMcpName(e.target.value)}
                            style={{ background: 'var(--vscode-input-background)', color: 'var(--vscode-input-foreground)', border: '1px solid var(--vscode-input-border)', padding: '4px 8px', fontSize: '11px' }}
                        />
                        <select
                            value={newMcpType}
                            onChange={e => setNewMcpType(e.target.value as any)}
                            style={{ background: 'rgba(0,0,0,0.2)', color: '#fff', border: '1px solid rgba(255,255,255,0.1)', padding: '4px 8px', fontSize: '11px', borderRadius: '4px' }}
                        >
                            <option value="command">Stdio Command</option>
                            <option value="http">HTTP Transport</option>
                        </select>
                        {newMcpType === 'command' ? (
                            <>
                                <input
                                    placeholder="Command (e.g. npx)"
                                    value={newMcpCommand}
                                    onChange={e => setNewMcpCommand(e.target.value)}
                                    style={{ background: 'rgba(0,0,0,0.2)', color: '#fff', border: '1px solid rgba(255,255,255,0.1)', padding: '4px 8px', fontSize: '11px', borderRadius: '4px' }}
                                />
                                <input
                                    placeholder="Args (comma separated)"
                                    value={newMcpArgs}
                                    onChange={e => setNewMcpArgs(e.target.value)}
                                    style={{ background: 'rgba(0,0,0,0.2)', color: '#fff', border: '1px solid rgba(255,255,255,0.1)', padding: '4px 8px', fontSize: '11px', borderRadius: '4px' }}
                                />
                            </>
                        ) : (
                            <input
                                placeholder="URL (e.g. http://localhost:3000/sse)"
                                value={newMcpUrl}
                                onChange={e => setNewMcpUrl(e.target.value)}
                                style={{ background: 'rgba(0,0,0,0.2)', color: '#fff', border: '1px solid rgba(255,255,255,0.1)', padding: '4px 8px', fontSize: '11px', borderRadius: '4px' }}
                            />
                        )}
                        <button
                            onClick={async () => {
                                let config: any = {};
                                if (newMcpType === 'command') {
                                    config = { command: newMcpCommand, args: newMcpArgs.split(',').map(a => a.trim()).filter(Boolean) };
                                } else {
                                    config = { url: newMcpUrl };
                                }
                                await addMcpServer(newMcpName, config);
                                setIsAddingMcp(false);
                                setNewMcpName(''); setNewMcpCommand(''); setNewMcpArgs(''); setNewMcpUrl('');
                            }}
                            style={{ background: '#3b82f6', color: '#fff', border: 'none', padding: '6px', fontSize: '11px', borderRadius: '4px', cursor: 'pointer', fontWeight: 600 }}
                        >
                            Connect Server
                        </button>
                    </div>
                )}

                <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                    {mcpServers.length === 0 && !isAddingMcp && (
                        <div style={{ fontSize: '11px', opacity: 0.4, fontStyle: 'italic', textAlign: 'center', padding: '10px' }}>No MCP servers configured.</div>
                    )}
                    {mcpServers.map(server => (
                        <div key={server.name} style={{ display: 'flex', alignItems: 'center', gap: '8px', padding: '6px 10px', background: 'var(--vscode-sideBar-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: '2px' }}>
                            <i className="codicon codicon-server" style={{ fontSize: '14px', color: '#89d185', opacity: 0.8 }}></i>
                            <div style={{ display: 'flex', flexDirection: 'column', flex: 1, overflow: 'hidden' }}>
                                <span style={{ fontSize: '12px', fontWeight: 600 }}>{server.name}</span>
                                <span style={{ fontSize: '10px', opacity: 0.5, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                    {server.config.command ? `${server.config.command} ${server.config.args.join(' ')}` : server.config.url}
                                </span>
                            </div>
                            <i
                                className="codicon codicon-trash"
                                onClick={() => removeMcpServer(server.name)}
                                style={{ fontSize: '14px', opacity: 0.4, cursor: 'pointer' }}
                                title="Remove Server"
                            ></i>
                        </div>
                    ))}
                </div>
            </section>

        </div>
    );
};

export default AgentSettingsView;
