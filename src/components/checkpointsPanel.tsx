import React, { useState, useEffect } from 'react';
import { listCheckpoints, rollbackToCheckpoint, deleteCheckpoint, type Checkpoint, getCheckpointDiff } from '../services/gitCheckpoints';

const CheckpointsPanel: React.FC = () => {
    const [checkpoints, setCheckpoints] = useState<Checkpoint[]>([]);
    const [loading, setLoading] = useState(true);
    const [selectedCheckpoint, setSelectedCheckpoint] = useState<string | null>(null);
    const [diff, setDiff] = useState<any>(null);
    const [rollingBack, setRollingBack] = useState(false);

    const fetchCheckpoints = async () => {
        setLoading(true);
        try {
            const cps = await listCheckpoints(50);
            setCheckpoints(cps);
        } catch (e) {
            console.error('Failed to fetch checkpoints:', e);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchCheckpoints();
    }, []);

    const handleRollback = async (checkpointId: string) => {
        if (!confirm(`Rollback to checkpoint "${checkpointId}"? This will create a new commit reverting all changes.`)) {
            return;
        }

        setRollingBack(true);
        try {
            const result = await rollbackToCheckpoint(checkpointId);
            alert(result);
            await fetchCheckpoints();
        } catch (e: any) {
            alert(`Failed to rollback: ${e}`);
        } finally {
            setRollingBack(false);
        }
    };

    const handleDelete = async (checkpointId: string) => {
        if (!confirm(`Delete checkpoint "${checkpointId}"? This only removes the checkpoint tag, not the commit.`)) {
            return;
        }

        try {
            await deleteCheckpoint(checkpointId);
            await fetchCheckpoints();
            if (selectedCheckpoint === checkpointId) {
                setSelectedCheckpoint(null);
                setDiff(null);
            }
        } catch (e: any) {
            alert(`Failed to delete: ${e}`);
        }
    };

    const handleViewDiff = async (checkpointId: string) => {
        setSelectedCheckpoint(checkpointId);
        try {
            const checkpointDiff = await getCheckpointDiff(checkpointId);
            setDiff(checkpointDiff);
        } catch (e: any) {
            console.error('Failed to get diff:', e);
        }
    };

    if (loading) {
        return (
            <div style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                height: '100%',
                color: 'var(--vscode-foreground)',
                fontSize: '12px',
            }}>
                <i className="codicon codicon-loading" style={{ animation: 'spin 1s linear infinite', marginRight: '8px' }}></i>
                Loading checkpoints...
            </div>
        );
    }

    return (
        <div style={{
            display: 'flex',
            flexDirection: 'column',
            height: '100%',
            background: 'var(--vscode-editor-background)',
            color: 'var(--vscode-foreground)',
        }}>
            {/* Header */}
            <div style={{
                height: '35px',
                display: 'flex',
                alignItems: 'center',
                padding: '0 12px',
                borderBottom: '1px solid var(--vscode-panel-border)',
                background: 'var(--vscode-panel-background)',
                justifyContent: 'space-between',
            }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <i className="codicon codicon-history" style={{ fontSize: '14px' }}></i>
                    <span style={{ fontSize: '11px', fontWeight: 600 }}>CHECKPOINTS</span>
                    <span style={{
                        background: 'var(--vscode-badge-background)',
                        color: 'var(--vscode-badge-foreground)',
                        padding: '1px 6px',
                        borderRadius: '10px',
                        fontSize: '10px',
                    }}>{checkpoints.length}</span>
                </div>
                <button
                    onClick={fetchCheckpoints}
                    style={{
                        background: 'transparent',
                        border: 'none',
                        color: 'var(--vscode-foreground)',
                        cursor: 'pointer',
                        padding: '4px',
                        borderRadius: '2px',
                    }}
                    title="Refresh"
                >
                    <i className="codicon codicon-refresh" style={{ fontSize: '14px' }}></i>
                </button>
            </div>

            {/* Body */}
            <div style={{ flex: 1, display: 'flex', overflow: 'hidden' }}>
                {/* Checkpoints List */}
                <div style={{
                    width: '100%',
                    overflowY: 'auto',
                    borderRight: selectedCheckpoint ? '1px solid var(--vscode-panel-border)' : 'none',
                }}>
                    {checkpoints.length === 0 ? (
                        <div style={{
                            padding: '24px',
                            textAlign: 'center',
                            opacity: 0.5,
                            fontSize: '12px',
                        }}>
                            <i className="codicon codicon-history" style={{ fontSize: '32px', marginBottom: '8px', display: 'block', opacity: 0.3 }}></i>
                            No checkpoints yet.<br />
                            They are created automatically before AI edits.
                        </div>
                    ) : (
                        checkpoints.map(cp => (
                            <div
                                key={cp.id}
                                style={{
                                    padding: '8px 12px',
                                    borderBottom: '1px solid var(--vscode-panel-border)',
                                    cursor: 'pointer',
                                    background: selectedCheckpoint === cp.id
                                        ? 'var(--vscode-list-activeSelectionBackground)'
                                        : 'transparent',
                                }}
                            >
                                <div style={{
                                    display: 'flex',
                                    justifyContent: 'space-between',
                                    alignItems: 'flex-start',
                                    marginBottom: '4px',
                                }}>
                                    <div style={{ flex: 1 }}>
                                        <div style={{
                                            fontSize: '12px',
                                            fontWeight: 500,
                                            marginBottom: '2px',
                                        }}>
                                            {cp.is_ai_generated && (
                                                <i
                                                    className="codicon codicon-robot"
                                                    style={{ fontSize: '12px', marginRight: '4px', opacity: 0.6 }}
                                                ></i>
                                            )}
                                            {cp.description || cp.name}
                                        </div>
                                        <div style={{
                                            fontSize: '10px',
                                            opacity: 0.6,
                                        }}>
                                            {cp.datetime}
                                        </div>
                                    </div>
                                    <div style={{
                                        fontSize: '10px',
                                        background: 'var(--vscode-badge-background)',
                                        color: 'var(--vscode-badge-foreground)',
                                        padding: '2px 6px',
                                        borderRadius: '10px',
                                    }}>
                                        {cp.files_changed} files
                                    </div>
                                </div>
                                <div style={{
                                    display: 'flex',
                                    gap: '4px',
                                    marginTop: '6px',
                                }}>
                                    <button
                                        onClick={(e) => { e.stopPropagation(); handleViewDiff(cp.id); }}
                                        style={{
                                            background: 'transparent',
                                            border: '1px solid var(--vscode-button-background)',
                                            color: 'var(--vscode-button-background)',
                                            padding: '2px 8px',
                                            fontSize: '10px',
                                            borderRadius: '2px',
                                            cursor: 'pointer',
                                        }}
                                    >
                                        View Diff
                                    </button>
                                    <button
                                        onClick={(e) => { e.stopPropagation(); handleRollback(cp.id); }}
                                        disabled={rollingBack}
                                        style={{
                                            background: 'transparent',
                                            border: '1px solid var(--vscode-testing-iconPassed)',
                                            color: 'var(--vscode-testing-iconPassed)',
                                            padding: '2px 8px',
                                            fontSize: '10px',
                                            borderRadius: '2px',
                                            cursor: rollingBack ? 'wait' : 'pointer',
                                        }}
                                    >
                                        Rollback
                                    </button>
                                    <button
                                        onClick={(e) => { e.stopPropagation(); handleDelete(cp.id); }}
                                        style={{
                                            background: 'transparent',
                                            border: '1px solid var(--vscode-testing-iconFailed)',
                                            color: 'var(--vscode-testing-iconFailed)',
                                            padding: '2px 8px',
                                            fontSize: '10px',
                                            borderRadius: '2px',
                                            cursor: 'pointer',
                                        }}
                                    >
                                        Delete
                                    </button>
                                </div>
                            </div>
                        ))
                    )}
                </div>

                {/* Diff Preview */}
                {selectedCheckpoint && diff && (
                    <div style={{
                        width: '100%',
                        overflowY: 'auto',
                        padding: '12px',
                    }}>
                        <div style={{
                            fontSize: '11px',
                            fontWeight: 600,
                            marginBottom: '8px',
                        }}>
                            DIFF: {checkpoints.find(cp => cp.id === selectedCheckpoint)?.description}
                        </div>
                        <div style={{
                            fontSize: '11px',
                            marginBottom: '12px',
                            opacity: 0.7,
                        }}>
                            {diff.total_additions} additions, {diff.total_deletions} deletions across {diff.files.length} files
                        </div>
                        {diff.files.map((file: any, idx: number) => (
                            <div
                                key={idx}
                                style={{
                                    padding: '8px',
                                    background: 'var(--vscode-textBlockQuote-background)',
                                    borderRadius: '4px',
                                    marginBottom: '8px',
                                    fontSize: '12px',
                                }}
                            >
                                <div style={{ fontWeight: 500, marginBottom: '4px' }}>
                                    {file.path}
                                </div>
                                <div style={{ fontSize: '10px', opacity: 0.6 }}>
                                    +{file.additions} -{file.deletions}
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </div>
        </div>
    );
};

export default CheckpointsPanel;
