/**
 * Cursor-style model picker dropdown — shows model info cards with
 * provider badge, context window, capabilities, and a search filter.
 */
import React, { useState, useMemo, useRef, useEffect } from 'react';

export interface ModelInfo {
    id: string;
    name: string;
    provider: string;
    providerIcon?: string;
    contextWindow?: string;
    capabilities?: string[];
    speed?: 'fast' | 'balanced' | 'powerful';
    isLocal?: boolean;
    isDefault?: boolean;
}

interface ModelPickerProps {
    models: ModelInfo[];
    selectedModel: string;
    onSelect: (modelId: string) => void;
    onClose: () => void;
}

const SPEED_COLORS: Record<string, string> = {
    fast: '#34d399',
    balanced: '#fbbf24',
    powerful: '#a78bfa',
};

const SPEED_LABELS: Record<string, string> = {
    fast: 'Fast',
    balanced: 'Balanced',
    powerful: 'Powerful',
};

const ModelPicker: React.FC<ModelPickerProps> = ({ models, selectedModel, onSelect, onClose }) => {
    const [search, setSearch] = useState('');
    const [selectedIndex, setSelectedIndex] = useState(0);
    const searchRef = useRef<HTMLInputElement>(null);
    const listRef = useRef<HTMLDivElement>(null);

    const filtered = useMemo(() => {
        if (!search) return models;
        const q = search.toLowerCase();
        return models.filter(m =>
            m.name.toLowerCase().includes(q) ||
            m.provider.toLowerCase().includes(q) ||
            m.id.toLowerCase().includes(q)
        );
    }, [models, search]);

    useEffect(() => {
        searchRef.current?.focus();
    }, []);

    useEffect(() => {
        setSelectedIndex(0);
    }, [search]);

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'ArrowDown') {
            e.preventDefault();
            setSelectedIndex(i => Math.min(i + 1, filtered.length - 1));
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            setSelectedIndex(i => Math.max(i - 1, 0));
        } else if (e.key === 'Enter') {
            e.preventDefault();
            if (filtered[selectedIndex]) {
                onSelect(filtered[selectedIndex].id);
                onClose();
            }
        } else if (e.key === 'Escape') {
            onClose();
        }
    };

    return (
        <div
            style={{
                position: 'absolute', bottom: '100%', left: 0, right: 0,
                background: 'var(--vscode-menu-background, #1e1e2e)',
                border: '1px solid var(--vscode-menu-border, rgba(255,255,255,0.12))',
                borderRadius: '10px', overflow: 'hidden',
                boxShadow: '0 -8px 30px rgba(0,0,0,0.5)',
                zIndex: 200, marginBottom: '8px',
                maxHeight: '380px', display: 'flex', flexDirection: 'column',
            }}
            onClick={(e) => e.stopPropagation()}
        >
            {/* Search */}
            <div style={{ padding: '8px 10px', borderBottom: '1px solid rgba(255,255,255,0.06)' }}>
                <div style={{
                    display: 'flex', alignItems: 'center', gap: '6px',
                    background: 'rgba(255,255,255,0.05)', borderRadius: '6px',
                    padding: '5px 8px', border: '1px solid rgba(255,255,255,0.08)',
                }}>
                    <i className="codicon codicon-search" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', opacity: 0.5 }} />
                    <input
                        ref={searchRef}
                        value={search}
                        onChange={(e) => setSearch(e.target.value)}
                        onKeyDown={handleKeyDown}
                        placeholder="Search models..."
                        style={{
                            background: 'transparent', border: 'none', outline: 'none',
                            color: 'var(--vscode-editor-foreground, #fff)', fontSize: '12px',
                            flex: 1, minWidth: 0,
                        }}
                    />
                </div>
            </div>

            {/* Model list */}
            <div ref={listRef} style={{ overflowY: 'auto', flex: 1 }}>
                {filtered.length === 0 && (
                    <div style={{ padding: '16px 12px', textAlign: 'center', color: 'rgba(255,255,255,0.4)', fontSize: '12px' }}>
                        No models found
                    </div>
                )}
                {filtered.map((model, i) => {
                    const isSelected = model.id === selectedModel;
                    const isHighlighted = i === selectedIndex;
                    return (
                        <div
                            key={model.id}
                            onClick={() => { onSelect(model.id); onClose(); }}
                            onMouseEnter={() => setSelectedIndex(i)}
                            style={{
                                padding: '8px 12px', cursor: 'pointer',
                                background: isHighlighted ? 'rgba(59,130,246,0.12)' : 'transparent',
                                borderLeft: isSelected ? '3px solid #3b82f6' : '3px solid transparent',
                                transition: 'all 0.08s',
                            }}
                        >
                            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                {/* Provider badge */}
                                <div style={{
                                    width: '22px', height: '22px', borderRadius: '5px',
                                    background: isSelected ? 'rgba(59,130,246,0.2)' : 'rgba(255,255,255,0.06)',
                                    display: 'flex', alignItems: 'center', justifyContent: 'center',
                                    fontSize: '10px', fontWeight: 700, flexShrink: 0,
                                    color: isSelected ? '#3b82f6' : 'rgba(255,255,255,0.5)',
                                }}>
                                    {model.providerIcon || model.provider.charAt(0).toUpperCase()}
                                </div>

                                <div style={{ flex: 1, minWidth: 0 }}>
                                    <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                                        <span style={{
                                            fontSize: '12px', fontWeight: isSelected ? 600 : 400,
                                            color: isSelected ? '#fff' : 'rgba(255,255,255,0.85)',
                                        }}>
                                            {model.name}
                                        </span>
                                        {model.isLocal && (
                                            <span style={{
                                                fontSize: '8px', fontWeight: 700, textTransform: 'uppercase',
                                                padding: '1px 4px', borderRadius: '3px',
                                                background: 'rgba(52,211,153,0.15)', color: '#34d399',
                                                letterSpacing: '0.05em',
                                            }}>
                                                Local
                                            </span>
                                        )}
                                        {model.isDefault && (
                                            <span style={{
                                                fontSize: '8px', fontWeight: 700, textTransform: 'uppercase',
                                                padding: '1px 4px', borderRadius: '3px',
                                                background: 'rgba(59,130,246,0.15)', color: '#3b82f6',
                                                letterSpacing: '0.05em',
                                            }}>
                                                Default
                                            </span>
                                        )}
                                    </div>
                                    <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginTop: '2px' }}>
                                        <span style={{ fontSize: '10px', opacity: 0.4 }}>
                                            {model.provider}
                                        </span>
                                        {model.contextWindow && (
                                            <>
                                                <span style={{ fontSize: '10px', opacity: 0.2 }}>·</span>
                                                <span style={{ fontSize: '10px', opacity: 0.4 }}>
                                                    {model.contextWindow}
                                                </span>
                                            </>
                                        )}
                                        {model.speed && (
                                            <>
                                                <span style={{ fontSize: '10px', opacity: 0.2 }}>·</span>
                                                <span style={{
                                                    fontSize: '9px', fontWeight: 600,
                                                    color: SPEED_COLORS[model.speed] || 'rgba(255,255,255,0.5)',
                                                }}>
                                                    {SPEED_LABELS[model.speed] || model.speed}
                                                </span>
                                            </>
                                        )}
                                    </div>
                                </div>

                                {isSelected && (
                                    <i className="codicon codicon-check" style={{
                                        fontFamily: 'codicon', fontStyle: 'normal',
                                        fontSize: '13px', color: '#3b82f6',
                                    }} />
                                )}
                            </div>

                            {/* Capabilities row */}
                            {model.capabilities && model.capabilities.length > 0 && (
                                <div style={{ display: 'flex', gap: '4px', marginTop: '4px', marginLeft: '30px', flexWrap: 'wrap' }}>
                                    {model.capabilities.map(cap => (
                                        <span key={cap} style={{
                                            fontSize: '9px', padding: '1px 5px', borderRadius: '3px',
                                            background: 'rgba(255,255,255,0.04)', color: 'rgba(255,255,255,0.45)',
                                            border: '1px solid rgba(255,255,255,0.06)',
                                        }}>
                                            {cap}
                                        </span>
                                    ))}
                                </div>
                            )}
                        </div>
                    );
                })}
            </div>

            {/* Footer hint */}
            <div style={{
                padding: '5px 12px', borderTop: '1px solid rgba(255,255,255,0.06)',
                fontSize: '9px', color: 'rgba(255,255,255,0.3)',
                display: 'flex', justifyContent: 'space-between',
            }}>
                <span>↑↓ Navigate</span>
                <span>↵ Select</span>
                <span>Esc Close</span>
            </div>
        </div>
    );
};

export default ModelPicker;
