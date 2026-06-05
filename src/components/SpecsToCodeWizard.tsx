import React, { useState, useEffect } from 'react';
import MonacoEditor from '@monaco-editor/react';
import { useStore } from '../store';
import { invoke } from '../tauri_bridge';
import { X, ChevronRight, Play, CheckCircle, Clock, AlertCircle, FileCode, Layout, Zap } from 'lucide-react';

const SpecsToCodeWizard: React.FC = () => {
    const isSpecsWizardOpen = useStore(state => state.isSpecsWizardOpen);
    const setSpecsWizardOpen = useStore(state => state.setSpecsWizardOpen);
    const specsWizardStep = useStore(state => state.specsWizardStep);
    const setSpecsWizardStep = useStore(state => state.setSpecsWizardStep);
    const currentSpecProjectId = useStore(state => state.currentSpecProjectId);
    const setCurrentSpecProjectId = useStore(state => state.setCurrentSpecProjectId);
    const theme = useStore(state => state.theme);

    if (!isSpecsWizardOpen) return null;

    return (
        <div className="specs-wizard-overlay" style={{
            position: 'fixed',
            inset: 0,
            zIndex: 9999,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            background: 'rgba(0, 0, 0, 0.7)',
            backdropFilter: 'blur(8px)'
        }}>
            <div className="specs-wizard-container" style={{
                width: '90vw',
                height: '85vh',
                background: 'var(--vscode-sideBar-background)',
                border: '1px solid var(--vscode-panel-border)',
                borderRadius: '12px',
                display: 'flex',
                flexDirection: 'column',
                overflow: 'hidden',
                boxShadow: '0 20px 50px rgba(0,0,0,0.5)'
            }}>
                {/* Header */}
                <div className="wizard-header" style={{
                    padding: '16px 24px',
                    borderBottom: '1px solid var(--vscode-panel-border)',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    background: 'rgba(255, 255, 255, 0.03)'
                }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                        <div style={{
                            width: '32px',
                            height: '32px',
                            background: 'var(--terminator-accent)',
                            borderRadius: '8px',
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'center'
                        }}>
                            <Layout size={18} color="white" />
                        </div>
                        <div>
                            <h2 style={{ margin: 0, fontSize: '16px', fontWeight: 600 }}>Specs-to-Code Pipeline</h2>
                            <div style={{ fontSize: '11px', opacity: 0.5 }}>Autonomous Project Generation Engine</div>
                        </div>
                    </div>
                    <button
                        onClick={() => setSpecsWizardOpen(false)}
                        style={{ background: 'transparent', border: 'none', color: 'inherit', cursor: 'pointer', opacity: 0.6 }}
                    >
                        <X size={20} />
                    </button>
                </div>

                {/* Content */}
                <div className="wizard-content" style={{ flex: 1, overflow: 'hidden', position: 'relative' }}>
                    {specsWizardStep === 'generator' && <SpecsGenerator />}
                    {specsWizardStep === 'status' && <ProjectStatusView projectId={currentSpecProjectId!} />}
                    {specsWizardStep === 'project' && <GeneratedProjectView projectId={currentSpecProjectId!} />}
                </div>
            </div>
        </div>
    );
};

const SpecsGenerator: React.FC = () => {
    const [projectName, setProjectName] = useState("");
    const [specs, setSpecs] = useState("// Define your project requirements here...\n\n# Project: \n\n## Core Logic\n- \n\n## Data Structures\n- \n");
    const [preferredProvider, setPreferredProvider] = useState('API Cloud');
    const [ollamaModel, setOllamaModel] = useState('');
    const [availableModels, setAvailableModels] = useState<string[]>([]);
    const setSpecsWizardStep = useStore(state => state.setSpecsWizardStep);
    const setCurrentSpecProjectId = useStore(state => state.setCurrentSpecProjectId);
    const theme = useStore(state => state.theme);

    useEffect(() => {
        if (preferredProvider === 'Ollama') {
            const fetchModels = async () => {
                try {
                    const models = await invoke<string[]>("list_provider_models", { provider: 'ollama' });
                    setAvailableModels(models);
                    if (models.length > 0 && !ollamaModel) {
                        setOllamaModel(models[0]);
                    }
                } catch (e) {
                    console.error("Failed to fetch Ollama models:", e);
                }
            };
            fetchModels();
        }
    }, [preferredProvider]);

    const [guiding, setGuiding] = useState(false);
    const [guideStatus, setGuideStatus] = useState('');

    // Kiro-style guided pipeline: idea → requirements → design, generated by the
    // real model, then dropped into the spec for review before tasks are created.
    const handleGuided = async () => {
        const idea = `${projectName}\n${specs}`.trim();
        if (!idea || idea.length < 4) { alert('Enter a project name / idea first.'); return; }
        setGuiding(true);
        const isOllama = preferredProvider.toLowerCase().includes('ollama');
        const provider = isOllama ? 'ollama' : 'google';
        const model = isOllama ? (ollamaModel || '') : 'gemini-2.0-flash';
        const ask = async (sys: string, user: string): Promise<string> => {
            const r = await invoke<string>('ai_chat_fast', {
                request: {
                    provider, model,
                    messages: [{ role: 'system', content: sys }, { role: 'user', content: user }],
                    temperature: 0.4, autonomous: false, mode: 'Chat', tools: [],
                },
            });
            return (r || '').trim();
        };
        try {
            setGuideStatus('Generating requirements…');
            const requirements = await ask(
                'You are a senior product engineer. Write a concise REQUIREMENTS document (markdown only): user stories, functional requirements, non-functional requirements, and acceptance criteria. No preamble.',
                idea);
            setGuideStatus('Generating design…');
            const design = await ask(
                'You are a senior software architect. Given these requirements, write a concise DESIGN document (markdown only): architecture overview, components, data models, key APIs, and the file/module layout. No preamble.',
                requirements);
            setSpecs(`# Requirements\n\n${requirements}\n\n# Design\n\n${design}\n`);
            setGuideStatus('✓ Requirements + design generated — review, then Generate.');
        } catch (e) {
            setGuideStatus(`Guided generation failed: ${e}`);
        } finally {
            setGuiding(false);
        }
    };

    const handleCreate = async () => {
        if (!projectName) {
            alert("Please enter a project name");
            return;
        }
        try {
            const id = await invoke<number>("cmd_specs_create_project", {
                name: projectName,
                specs,
                provider: preferredProvider.toLowerCase().includes('ollama') ? `ollama:${ollamaModel}` : 'google'
            });
            setCurrentSpecProjectId(id);
            setSpecsWizardStep('status');
            // Trigger layout generation in background
            invoke("cmd_specs_generate_layout", { projectId: id }).catch(console.error);
        } catch (e) {
            alert(`Error: ${e}`);
        }
    };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', padding: '24px', gap: '20px' }}>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                <label style={{ fontSize: '12px', fontWeight: 600, opacity: 0.7 }}>PROJECT IDENTITY</label>
                <input
                    type="text"
                    placeholder="Project Name (e.g. NeuralNetworkCore)"
                    value={projectName}
                    onChange={(e) => setProjectName(e.target.value)}
                    style={{
                        background: 'var(--vscode-input-background)',
                        color: 'var(--vscode-input-foreground)',
                        border: '1px solid var(--vscode-input-border)',
                        padding: '12px 16px',
                        borderRadius: '8px',
                        fontSize: '14px',
                        outline: 'none'
                    }}
                />
            </div>

            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    <label style={{ fontSize: '12px', fontWeight: 600, opacity: 0.7 }}>ASSEMBLY PROVIDER</label>
                    <button
                        onClick={async () => {
                            if (confirm("Clear all project history?")) {
                                await invoke("cmd_specs_clear_history");
                                window.location.reload();
                            }
                        }}
                        style={{
                            background: 'transparent',
                            border: 'none',
                            color: '#f44336',
                            fontSize: '10px',
                            cursor: 'pointer',
                            opacity: 0.6,
                            padding: '4px'
                        }}
                    >
                        CLEAR HISTORY
                    </button>
                </div>
                <div style={{ display: 'flex', gap: '10px' }}>
                    {['API Cloud', 'Ollama'].map(p => (
                        <button
                            key={p}
                            onClick={() => setPreferredProvider(p)}
                            style={{
                                flex: 1,
                                padding: '10px',
                                background: preferredProvider === p ? 'rgba(0, 122, 204, 0.2)' : 'rgba(255,255,255,0.03)',
                                border: `1px solid ${preferredProvider === p ? '#007acc' : 'var(--vscode-panel-border)'}`,
                                borderRadius: '8px',
                                color: preferredProvider === p ? '#fff' : 'rgba(255,255,255,0.6)',
                                fontSize: '12px',
                                cursor: 'pointer',
                                transition: 'all 0.2s',
                                fontWeight: preferredProvider === p ? 600 : 400
                            }}
                        >
                            {p}
                        </button>
                    ))}
                </div>
                {preferredProvider === 'Ollama' && (
                    <div style={{ marginTop: '8px' }}>
                        <select
                            value={ollamaModel}
                            onChange={(e) => setOllamaModel(e.target.value)}
                            style={{
                                width: '100%',
                                padding: '8px 12px',
                                background: 'rgba(255,255,255,0.03)',
                                border: '1px solid var(--vscode-panel-border)',
                                borderRadius: '6px',
                                color: 'var(--vscode-editor-foreground, #eee)',
                                fontSize: '12px',
                                cursor: 'pointer',
                                outline: 'none'
                            }}
                        >
                            {availableModels.length === 0 ? (
                                <option disabled>No local models found / Loading...</option>
                            ) : (
                                availableModels.map(m => (
                                    <option key={m} value={m}>{m}</option>
                                ))
                            )}
                        </select>
                    </div>
                )}
            </div>

            <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: '8px', minHeight: 0 }}>
                <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: 8 }}>
                    <label style={{ fontSize: '12px', fontWeight: 600, opacity: 0.7 }}>TECHNICAL SPECIFICATIONS</label>
                    <button
                        onClick={handleGuided}
                        disabled={guiding}
                        title="Generate a requirements doc → design doc from your idea, then review before generating tasks."
                        style={{ fontSize: 11, fontWeight: 600, padding: '4px 10px', borderRadius: 6, cursor: guiding ? 'wait' : 'pointer', border: '1px solid rgba(168,85,247,0.3)', background: 'rgba(168,85,247,0.12)', color: '#c084fc' }}
                    >
                        {guiding ? '⟳ Generating…' : '✨ Guided (Requirements → Design)'}
                    </button>
                </div>
                {guideStatus && <div style={{ fontSize: 10, opacity: 0.6 }}>{guideStatus}</div>}
                <div style={{ flex: 1, border: '1px solid var(--vscode-panel-border)', borderRadius: '8px', overflow: 'hidden' }}>
                    <MonacoEditor
                        height="100%"
                        language="markdown"
                        theme={theme}
                        value={specs}
                        onChange={(val) => setSpecs(val ?? "")}
                        options={{
                            minimap: { enabled: false },
                            fontSize: 14,
                            padding: { top: 16 }
                        }}
                    />
                </div>
            </div>

            <div style={{ display: 'flex', justifyContent: 'flex-end' }}>
                <button
                    onClick={handleCreate}
                    style={{
                        background: 'var(--terminator-accent)',
                        color: 'white',
                        border: 'none',
                        padding: '12px 24px',
                        borderRadius: '8px',
                        fontWeight: 600,
                        display: 'flex',
                        alignItems: 'center',
                        gap: '8px',
                        cursor: 'pointer',
                        boxShadow: '0 4px 15px rgba(0, 198, 255, 0.3)'
                    }}
                >
                    INITIALIZE PROJECT <ChevronRight size={18} />
                </button>
            </div>
        </div>
    );
};

const ProjectStatusView: React.FC<{ projectId: number }> = ({ projectId }) => {
    const [tasks, setTasks] = useState<any[]>([]);
    const [expandedErrorId, setExpandedErrorId] = useState<number | null>(null);
    const [expandedTasks, setExpandedTasks] = useState<Record<number, boolean>>({});
    const setSpecsWizardStep = useStore(state => state.setSpecsWizardStep);

    const toggleExpand = (taskId: number) => {
        setExpandedTasks(prev => ({ ...prev, [taskId]: !prev[taskId] }));
    };

    useEffect(() => {
        const poll = async () => {
            try {
                const res = await invoke<any[]>("cmd_specs_get_project_tasks", { projectId });
                setTasks(res);
                // Auto-expand new processing tasks
                setExpandedTasks(prev => {
                    const next = { ...prev };
                    res.forEach(t => {
                        if (t.status === 'Processing' && prev[t.id] === undefined) {
                            next[t.id] = true;
                        }
                    });
                    return next;
                });
            } catch (e) {
                console.error(e);
            }
        };
        poll();
        const interval = setInterval(poll, 3000);
        return () => clearInterval(interval);
    }, [projectId]);

    const doneCount = tasks.filter(t => t.status === 'Done').length;

    const handleDelete = async () => {
        if (confirm("Are you sure you want to delete this project and all its tasks?")) {
            await invoke("cmd_specs_delete_project", { id: projectId });
            setSpecsWizardStep('generator');
        }
    };

    const handleTaskDelete = async (taskId: number) => {
        try {
            await invoke("cmd_specs_delete_task", { id: taskId });
            setTasks(prev => prev.filter(t => t.id !== taskId));
        } catch (e) {
            console.error(e);
        }
    };

    return (
        <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '24px', height: '100%', overflow: 'hidden' }}>
            <style>{`
                @keyframes pulse-blue {
                    0% { opacity: 0.6; }
                    50% { opacity: 1; }
                    100% { opacity: 0.6; }
                }
                @keyframes ping-small {
                    0% { transform: scale(1); opacity: 1; }
                    70%, 100% { transform: scale(2); opacity: 0; }
                }
            `}</style>

            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                    <h3 style={{ margin: 0 }}>Assembly Progress</h3>
                    <div style={{ fontSize: '11px', opacity: 0.5 }}>PROJECT ID: {projectId}</div>
                </div>
                <div style={{ display: 'flex', gap: '16px', alignItems: 'center' }}>
                    <button
                        onClick={handleDelete}
                        style={{
                            background: 'rgba(244, 67, 54, 0.1)',
                            border: '1px solid rgba(244, 67, 54, 0.3)',
                            color: '#f44336',
                            padding: '6px 12px',
                            borderRadius: '6px',
                            fontSize: '11px',
                            cursor: 'pointer',
                            fontWeight: 600
                        }}
                    >
                        DELETE PROJECT
                    </button>
                    <div style={{ display: 'flex', gap: '16px', fontSize: '12px', background: 'rgba(255,255,255,0.05)', padding: '6px 12px', borderRadius: '20px' }}>
                        <span style={{ color: 'var(--terminator-success)' }}>DONE: {doneCount}</span>
                        <span style={{ color: 'var(--terminator-accent)' }}>TOTAL: {tasks.length}</span>
                    </div>
                </div>
            </div>

            <div className="task-list" style={{ flex: 1, overflowY: 'auto', display: 'flex', flexDirection: 'column', gap: '10px' }}>
                {tasks.length === 0 ? (
                    <div style={{ padding: '40px', textAlign: 'center', opacity: 0.5 }}>
                        <Clock size={32} style={{ margin: '0 auto 12px' }} />
                        <p>Waking up background workers...</p>
                    </div>
                ) : (
                    tasks.map(task => (
                        <div key={task.id} style={{
                            background: 'rgba(255,255,255,0.03)',
                            borderRadius: '8px',
                            border: '1px solid var(--vscode-panel-border)',
                            overflow: 'hidden'
                        }}>
                            <div
                                onClick={() => toggleExpand(task.id)}
                                style={{
                                    padding: '12px 16px',
                                    display: 'flex',
                                    alignItems: 'center',
                                    justifyContent: 'space-between',
                                    cursor: 'pointer',
                                    borderBottom: expandedTasks[task.id] ? '1px solid rgba(255,255,255,0.05)' : 'none',
                                    background: expandedTasks[task.id] ? 'rgba(255,255,255,0.02)' : 'transparent'
                                }}
                            >
                                <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                                    {task.status === 'Done' ? <CheckCircle size={14} color="#10b981" /> :
                                        task.status === 'Failed' ? <AlertCircle size={14} color="#f87171" /> :
                                            task.status === 'Processing' ? <div className="spinner" style={{ width: 14, height: 14, border: '2px solid rgba(255,255,255,0.2)', borderTop: '2px solid #007acc', borderRadius: '50%', animation: 'spin 1s linear infinite' }} /> :
                                                <Clock size={14} style={{ opacity: 0.5 }} />}
                                    <div style={{ display: 'flex', flexDirection: 'column' }}>
                                        <span style={{ fontSize: '13px', fontWeight: 500 }}>{task.work_type}</span>
                                        <span style={{ fontSize: '10px', opacity: 0.4 }}>TASK ID: {task.id}</span>
                                    </div>
                                </div>
                                <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                                    {task.status === 'Failed' && (
                                        <button
                                            onClick={(e) => { e.stopPropagation(); setExpandedErrorId(expandedErrorId === task.id ? null : task.id); }}
                                            style={{
                                                background: 'transparent',
                                                color: '#f87171',
                                                border: '1px solid rgba(248, 113, 113, 0.3)',
                                                padding: '4px 10px',
                                                borderRadius: '4px',
                                                fontSize: '10px',
                                                cursor: 'pointer',
                                                fontWeight: 600
                                            }}
                                        >
                                            {expandedErrorId === task.id ? 'Hide Error' : 'View Error'}
                                        </button>
                                    )}
                                    <span style={{ fontSize: '11px', opacity: 0.5, textTransform: 'uppercase' }}>{task.status}</span>
                                    {task.status === 'Failed' && (
                                        <button
                                            onClick={(e) => { e.stopPropagation(); invoke("cmd_specs_retry_task", { id: task.id }).catch(console.error); }}
                                            style={{
                                                background: 'rgba(248, 113, 113, 0.1)',
                                                color: '#f87171',
                                                border: '1px solid #f87171',
                                                padding: '4px 10px',
                                                borderRadius: '4px',
                                                fontSize: '10px',
                                                cursor: 'pointer',
                                                textTransform: 'uppercase',
                                                fontWeight: 600
                                            }}
                                        >
                                            Retry
                                        </button>
                                    )}
                                    <button
                                        onClick={(e) => { e.stopPropagation(); handleTaskDelete(task.id); }}
                                        style={{
                                            background: 'transparent',
                                            border: 'none',
                                            color: '#f44336',
                                            opacity: 0.5,
                                            cursor: 'pointer',
                                            padding: '4px'
                                        }}
                                        title="Delete Task Record"
                                    >
                                        <X size={14} />
                                    </button>
                                </div>
                            </div>

                            {expandedTasks[task.id] && (
                                <>
                                    {(task.current_log || task.logs) && (
                                        <div style={{
                                            padding: '10px 16px',
                                            background: task.status === 'Processing' ? 'rgba(0, 122, 204, 0.05)' : 'rgba(255,255,255,0.01)',
                                            display: 'flex',
                                            alignItems: 'start',
                                            gap: '12px',
                                            fontSize: '12px',
                                            color: task.status === 'Processing' ? '#60a5fa' : 'rgba(255,255,255,0.7)',
                                            fontWeight: task.status === 'Processing' ? 500 : 400,
                                            borderTop: '1px solid rgba(255,255,255,0.03)',
                                            animation: task.status === 'Processing' ? 'pulse-blue 2s infinite' : 'none'
                                        }}>
                                            <div style={{ position: 'relative', width: 10, height: 10, marginTop: '4px' }}>
                                                <div style={{
                                                    position: 'absolute',
                                                    inset: 0,
                                                    background: task.status === 'Done' ? '#10b981' : task.status === 'Failed' ? '#f87171' : '#007acc',
                                                    borderRadius: '50%',
                                                    opacity: 0.8
                                                }} />
                                                {task.status === 'Processing' && (
                                                    <div style={{
                                                        position: 'absolute',
                                                        inset: 0,
                                                        background: '#007acc',
                                                        borderRadius: '50%',
                                                        animation: 'ping-small 1.5s cubic-bezier(0, 0, 0.2, 1) infinite'
                                                    }} />
                                                )}
                                            </div>
                                            <div style={{
                                                letterSpacing: '0.3px',
                                                wordBreak: 'break-word',
                                                whiteSpace: 'pre-wrap',
                                                lineHeight: '1.5',
                                                flex: 1,
                                                maxHeight: '150px',
                                                overflowY: 'auto'
                                            }}>
                                                {task.current_log || task.logs}
                                            </div>
                                        </div>
                                    )}

                                    {expandedErrorId === task.id && task.logs && (
                                        <div style={{
                                            padding: '12px 16px',
                                            background: 'rgba(244, 67, 54, 0.05)',
                                            borderTop: '1px solid rgba(244, 67, 54, 0.1)',
                                            fontFamily: 'monospace',
                                            fontSize: '11px',
                                            color: '#fca5a5',
                                            whiteSpace: 'pre-wrap',
                                            maxHeight: '200px',
                                            overflow: 'auto'
                                        }}>
                                            <div style={{ fontWeight: 600, marginBottom: '8px', opacity: 0.8, fontSize: '10px' }}>BACKEND ERROR LOG:</div>
                                            {task.logs}
                                        </div>
                                    )}

                                    {task.preview_code && (
                                        <div style={{
                                            padding: '0 16px 16px',
                                            background: 'rgba(0,0,0,0.1)'
                                        }}>
                                            <div style={{
                                                background: 'rgba(30,30,30,0.5)',
                                                borderRadius: '6px',
                                                border: '1px solid rgba(255,255,255,0.05)',
                                                padding: '12px',
                                                fontFamily: 'var(--vscode-editor-font-family, monospace)',
                                                fontSize: '12px',
                                                maxHeight: '200px',
                                                overflow: 'auto',
                                                whiteSpace: 'pre-wrap',
                                                color: 'var(--vscode-editor-foreground, #d4d4d4)',
                                                position: 'relative'
                                            }}>
                                                <div style={{
                                                    position: 'absolute',
                                                    top: '8px',
                                                    right: '12px',
                                                    fontSize: '10px',
                                                    opacity: 0.3,
                                                    fontWeight: 600,
                                                    letterSpacing: '1px'
                                                }}>LIVE PREVIEW</div>
                                                {task.preview_code}
                                            </div>
                                        </div>
                                    )}
                                </>
                            )}
                        </div>
                    ))
                )}
            </div>

            <div style={{ display: 'flex', justifyContent: 'center' }}>
                <button
                    onClick={() => setSpecsWizardStep('project')}
                    disabled={tasks.length === 0}
                    style={{
                        background: tasks.length > 0 ? 'var(--terminator-accent)' : 'transparent',
                        border: '1px solid var(--vscode-panel-border)',
                        color: tasks.length > 0 ? 'white' : 'inherit',
                        padding: '10px 24px',
                        borderRadius: '8px',
                        cursor: tasks.length > 0 ? 'pointer' : 'not-allowed',
                        opacity: tasks.length > 0 ? 1 : 0.5,
                        display: 'flex',
                        alignItems: 'center',
                        gap: '8px',
                        fontWeight: 600
                    }}
                >
                    PREVIEW GENERATED CODE <ChevronRight size={16} />
                </button>
            </div>
        </div>
    );
};

import { ArchitectureVisualizer } from './ArchitectureVisualizer';

const GeneratedProjectView: React.FC<{ projectId: number }> = ({ projectId }) => {
    const [projectData, setProjectData] = useState<any[]>([]);
    const [tasks, setTasks] = useState<any[]>([]);
    const [loading, setLoading] = useState(true);
    const setSpecsWizardStep = useStore(state => state.setSpecsWizardStep);

    useEffect(() => {
        const fetchData = async () => {
            try {
                const [layout, projectTasks] = await Promise.all([
                    invoke<any[]>("cmd_specs_get_extended_project_layout", { projectId }),
                    invoke<any[]>("cmd_specs_get_project_tasks", { projectId })
                ]);
                setProjectData(layout);
                setTasks(projectTasks);
            } catch (e) {
                console.error(e);
            } finally {
                setLoading(false);
            }
        };
        fetchData();
    }, [projectId]);

    return (
        <div style={{ display: 'flex', height: '100%', flexDirection: 'column' }}>
            <div style={{ flex: 1, position: 'relative', overflow: 'hidden' }}>
                {loading ? (
                    <div style={{ position: 'absolute', inset: 0, display: 'flex', alignItems: 'center', justifyContent: 'center', background: 'rgba(0,0,0,0.2)' }}>
                        <div className="spinner" style={{ width: 40, height: 40, border: '4px solid rgba(255,255,255,0.1)', borderTop: '4px solid #007acc', borderRadius: '50%', animation: 'spin 1s linear infinite' }} />
                    </div>
                ) : (
                    <ArchitectureVisualizer files={projectData} tasks={tasks} />
                )}
            </div>

            <div style={{
                height: '60px',
                borderTop: '1px solid var(--vscode-panel-border)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                padding: '0 24px',
                background: 'rgba(0,0,0,0.1)'
            }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <div style={{ width: 10, height: 10, borderRadius: '50%', background: '#10b981' }} />
                        <span style={{ fontSize: '11px', opacity: 0.6 }}>IMPLEMENTED</span>
                    </div>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <div style={{ width: 10, height: 10, borderRadius: '50%', background: '#f87171' }} />
                        <span style={{ fontSize: '11px', opacity: 0.6 }}>FAILED</span>
                    </div>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <div style={{ width: 10, height: 10, borderRadius: '50%', background: '#666' }} />
                        <span style={{ fontSize: '11px', opacity: 0.6 }}>PENDING</span>
                    </div>
                </div>

                <div style={{ display: 'flex', gap: '12px' }}>
                    <button
                        onClick={() => setSpecsWizardStep('status')}
                        style={{
                            background: 'rgba(255,255,255,0.05)',
                            border: '1px solid var(--vscode-panel-border)',
                            color: 'var(--vscode-editor-foreground, #eee)',
                            padding: '8px 20px',
                            borderRadius: '6px',
                            cursor: 'pointer',
                            fontSize: '12px',
                            fontWeight: 600,
                            display: 'flex',
                            alignItems: 'center',
                            gap: '8px'
                        }}
                    >
                        <ChevronRight size={14} style={{ transform: 'rotate(180deg)' }} /> BACK TO PROGRESS
                    </button>
                    <button
                        onClick={() => window.location.reload()} // Simplified refresh for VFS updates
                        style={{
                            background: 'var(--terminator-accent)',
                            border: 'none',
                            color: 'white',
                            padding: '8px 20px',
                            borderRadius: '6px',
                            cursor: 'pointer',
                            fontSize: '12px',
                            fontWeight: 600
                        }}
                    >
                        REFRESH MAP
                    </button>
                </div>
            </div>
        </div>
    );
};

export default SpecsToCodeWizard;
