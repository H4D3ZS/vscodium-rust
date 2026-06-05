import React, { useState, useCallback, useMemo, useEffect } from 'react';
import ReactFlow, {
    Background,
    Controls,
    Panel,
    useNodesState,
    useEdgesState,
    addEdge,
    ReactFlowProvider,
    Handle,
    Position,
    MarkerType,
    useReactFlow,
    type Connection,
    type Edge,
    type Node,
} from 'reactflow';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import 'reactflow/dist/style.css';
import { motion, AnimatePresence } from 'framer-motion';
import {
    X,
    Database,
    Share2,
    FileJson,
    Brain,
    Download,
    Plus,
    Play,
    Settings,
    RefreshCw,
    Maximize2,
    Eraser,
    Upload,
    Columns,
    Cpu,
    Activity
} from 'lucide-react';
import { useStore } from '../../store';
import NeuralSummaryView from './NeuralSummaryView';

// --- Custom Nodes ---

const JsonNode = React.memo(({ data }: any) => (
    <div style={{
        padding: '10px 14px',
        borderRadius: '6px',
        background: 'var(--vscode-editor-background, #141414)',
        border: '1px solid rgba(167, 139, 250, 0.5)',
        color: 'var(--vscode-editor-foreground, #fff)',
        minWidth: '160px',
        position: 'relative',
        boxShadow: '0 4px 12px rgba(0,0,0,0.4)',
    }}>
        <Handle type="target" position={Position.Left} style={{ background: '#7c3aed', width: '6px', height: '6px', border: 'none' }} />
        <div style={{
            color: '#a78bfa',
            fontSize: '9px',
            fontWeight: 800,
            textTransform: 'uppercase',
            letterSpacing: '0.1em',
            marginBottom: '4px'
        }}>{data.type}</div>
        <div style={{ fontWeight: 600, fontSize: '13px' }}>{data.label}</div>
        {data.value !== undefined && (
            <div style={{ color: '#4ade80', marginTop: '6px', fontFamily: 'monospace', fontSize: '12px' }}>{String(data.value)}</div>
        )}
        <Handle type="source" position={Position.Right} style={{ background: '#7c3aed', width: '6px', height: '6px', border: 'none' }} />
    </div>
));
JsonNode.displayName = 'JsonNode';

const FlowNode = React.memo(({ data }: any) => (
    <div style={{
        padding: '16px 28px',
        borderRadius: '32px',
        background: data.color || '#3b82f6',
        color: 'var(--vscode-editor-foreground, #fff)',
        fontWeight: 700,
        fontSize: '15px',
        boxShadow: '0 8px 24px rgba(0,0,0,0.3)',
        border: '3px solid rgba(255,255,255,0.3)',
        textAlign: 'center',
        position: 'relative',
        minWidth: '140px'
    }}>
        <Handle type="target" position={Position.Top} style={{ background: '#fff', border: 'none' }} />
        {data.label}
        <Handle type="source" position={Position.Bottom} style={{ background: '#fff', border: 'none' }} />
    </div>
));
FlowNode.displayName = 'FlowNode';

const ErdNode = React.memo(({ data }: any) => (
    <div style={{
        background: 'var(--vscode-editor-background, #141414)',
        border: '1px solid rgba(16, 185, 129, 0.6)',
        borderRadius: '6px',
        minWidth: '190px',
        overflow: 'hidden',
        boxShadow: '0 5px 15px rgba(0,0,0,0.3)'
    }}>
        <div style={{
            background: 'rgba(16, 185, 129, 0.2)',
            padding: '8px 14px',
            borderBottom: '1px solid rgba(16, 185, 129, 0.3)',
            display: 'flex',
            alignItems: 'center',
            gap: '10px'
        }}>
            <Database size={15} color="#10b981" />
            <span style={{ color: 'var(--vscode-editor-foreground, #fff)', fontWeight: 700, fontSize: '13px', textTransform: 'uppercase', letterSpacing: '0.05em' }}>{data.label}</span>
        </div>
        <div style={{ padding: '6px 0' }}>
            {data.columns?.map((col: any, i: number) => (
                <div key={i} style={{
                    padding: '5px 14px',
                    display: 'flex',
                    justifyContent: 'space-between',
                    fontSize: '11px',
                    borderBottom: i === data.columns.length - 1 ? 'none' : '1px solid rgba(255,255,255,0.03)'
                }}>
                    <span style={{ color: col.isPk ? '#fbbf24' : '#e2e8f0', fontWeight: col.isPk ? 600 : 400 }}>{col.name}</span>
                    <span style={{ color: 'rgba(255,255,255,0.4)', fontSize: '10px' }}>{col.type}</span>
                </div>
            ))}
        </div>
        <Handle type="target" position={Position.Top} style={{ background: '#10b981', border: 'none' }} />
        <Handle type="source" position={Position.Bottom} style={{ background: '#10b981', border: 'none' }} />
    </div>
));
ErdNode.displayName = 'ErdNode';

const NeuralNode = React.memo(({ data }: any) => {
    const isSentinel = data.category === 'fix_lessons';
    const glowColor = isSentinel ? '#f472b6' : '#6366f1';

    return (
        <motion.div
            animate={{
                boxShadow: [
                    `0 0 10px ${glowColor}44`,
                    `0 0 25px ${glowColor}88`,
                    `0 0 10px ${glowColor}44`
                ],
                scale: [1, 1.05, 1]
            }}
            transition={{
                duration: 3,
                repeat: Infinity,
                ease: "easeInOut"
            }}
            style={{
                padding: '12px 18px',
                borderRadius: '50px',
                background: 'var(--vscode-editor-background, #0a0a0a)',
                border: `2px solid ${glowColor}`,
                color: 'var(--vscode-editor-foreground, #fff)',
                minWidth: '120px',
                display: 'flex',
                alignItems: 'center',
                gap: '10px',
                position: 'relative'
            }}
        >
            <Handle type="target" position={Position.Top} style={{ visibility: 'hidden' }} />
            <div style={{
                width: '10px',
                height: '10px',
                borderRadius: '50%',
                background: glowColor,
                boxShadow: `0 0 10px ${glowColor}`
            }} />
            <div style={{ display: 'flex', flexDirection: 'column' }}>
                <span style={{ fontSize: '12px', fontWeight: 700 }}>{data.label}</span>
                <span style={{ fontSize: '9px', opacity: 0.5, textTransform: 'uppercase' }}>{data.category}</span>
            </div>
            <Handle type="source" position={Position.Bottom} style={{ visibility: 'hidden' }} />
        </motion.div>
    );
});
NeuralNode.displayName = 'NeuralNode';

const nodeTypes = {
    jsonNode: JsonNode,
    flowNode: FlowNode,
    erdNode: ErdNode,
    neuralNode: NeuralNode,
    // Legacy support for older node types
    json: JsonNode,
    process: FlowNode,
    input: FlowNode,
    decision: FlowNode,
    output: FlowNode,
    table: FlowNode
};

const DraggableNode = ({ type, label, color, onClick }: any) => {
    const onDragStart = (event: React.DragEvent, nodeType: string) => {
        event.dataTransfer.setData('application/reactflow', nodeType);
        event.dataTransfer.setData('application/reactflow-label', label);
        event.dataTransfer.setData('application/reactflow-color', color);
        event.dataTransfer.effectAllowed = 'move';
    };

    return (
        <div
            onDragStart={(event) => onDragStart(event, type)}
            onClick={() => onClick(type, label, color)}
            draggable
            style={{
                padding: '12px 16px',
                borderRadius: '10px',
                background: 'rgba(255,255,255,0.03)',
                border: `1px solid ${color}33`,
                color: 'var(--vscode-editor-foreground, #fff)',
                fontSize: '13px',
                fontWeight: 500,
                cursor: 'pointer',
                display: 'flex',
                alignItems: 'center',
                gap: '12px',
                transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                userSelect: 'none'
            }}
            onMouseEnter={(e) => {
                e.currentTarget.style.background = 'rgba(255,255,255,0.08)';
                e.currentTarget.style.borderColor = `${color}88`;
                e.currentTarget.style.transform = 'translateY(-2px)';
            }}
            onMouseLeave={(e) => {
                e.currentTarget.style.background = 'rgba(255,255,255,0.03)';
                e.currentTarget.style.borderColor = `${color}33`;
                e.currentTarget.style.transform = 'translateY(0)';
            }}
        >
            <div style={{ width: '10px', height: '10px', borderRadius: '50%', background: color, boxShadow: `0 0 10px ${color}` }}></div>
            {label}
        </div>
    );
};

// --- Dependency Graph View ---

const DependencyGraphView: React.FC = () => {
    const [depNodes, setDepNodes, onDepNodesChange] = useNodesState([]);
    const [depEdges, setDepEdges, onDepEdgesChange] = useEdgesState([]);
    const [loading, setLoading] = useState(false);
    const { fitView } = useReactFlow();
    const activeRoot = (useStore.getState() as any).activeRoot ?? '';

    const loadDeps = useCallback(async () => {
        setLoading(true);
        try {
            const files = await invoke<string[]>('list_project_files', { root: activeRoot, extensions: ['ts', 'tsx', 'js', 'jsx', 'rs'] }).catch(() => []);
            const importMap: Record<string, Set<string>> = {};
            for (const f of (files ?? []).slice(0, 80)) {
                const content = await invoke<string>('read_file', { path: f }).catch(() => '');
                const imports = [...content.matchAll(/(?:import|from)\s+['"]([^'"]+)['"]/g)]
                    .map(m => m[1])
                    .filter(p => p.startsWith('.'));
                const short = f.replace(activeRoot, '').replace(/\\/g, '/').replace(/^\//, '');
                importMap[short] = new Set(imports.map(p => {
                    const parts = short.split('/');
                    parts.pop();
                    const base = [...parts, p].join('/').replace(/\/\.\.\//g, '/').replace(/\/\.\//g, '/');
                    return base.split('/').slice(-1)[0];
                }));
            }
            const allFiles = Object.keys(importMap);
            const cols = 5;
            const newNodes = allFiles.map((f, i) => ({
                id: f,
                type: 'default',
                position: { x: (i % cols) * 220, y: Math.floor(i / cols) * 80 },
                data: { label: f.split('/').pop() ?? f },
                style: {
                    background: '#1a1a2e', border: '1px solid rgba(0,198,255,0.3)',
                    color: 'var(--vscode-editor-foreground, #fff)', fontSize: '10px', padding: '4px 8px', borderRadius: '4px',
                },
            }));
            const newEdges: any[] = [];
            for (const [src, deps] of Object.entries(importMap)) {
                for (const dep of deps) {
                    const target = allFiles.find(f => f.endsWith(dep) || f.endsWith(dep + '.ts') || f.endsWith(dep + '.tsx'));
                    if (target && target !== src) {
                        newEdges.push({
                            id: `${src}->${target}`,
                            source: src, target,
                            markerEnd: { type: MarkerType.ArrowClosed, color: 'rgba(0,198,255,0.5)' },
                            style: { stroke: 'rgba(0,198,255,0.25)', strokeWidth: 1 },
                        });
                    }
                }
            }
            setDepNodes(newNodes);
            setDepEdges(newEdges);
            setTimeout(() => fitView({ padding: 0.1, duration: 600 }), 100);
        } finally {
            setLoading(false);
        }
    }, [activeRoot, setDepNodes, setDepEdges, fitView]);

    useEffect(() => { loadDeps(); }, []);

    return (
        <div style={{ flex: 1, background: '#090909', overflow: 'hidden', position: 'relative' }}>
            {loading && (
                <div style={{ position: 'absolute', top: '12px', left: '50%', transform: 'translateX(-50%)', zIndex: 10, background: 'rgba(0,198,255,0.15)', border: '1px solid rgba(0,198,255,0.3)', color: '#00c6ff', padding: '4px 16px', borderRadius: '20px', fontSize: '11px', fontWeight: 600 }}>
                    Parsing imports…
                </div>
            )}
            <button
                onClick={loadDeps}
                style={{ position: 'absolute', top: '10px', right: '10px', zIndex: 10, background: 'rgba(255,255,255,0.06)', border: '1px solid rgba(255,255,255,0.15)', color: 'var(--vscode-editor-foreground, #fff)', padding: '4px 12px', borderRadius: '6px', fontSize: '11px', cursor: 'pointer' }}
            >
                Refresh
            </button>
            <ReactFlow nodes={depNodes} edges={depEdges} onNodesChange={onDepNodesChange} onEdgesChange={onDepEdgesChange} fitView style={{ background: '#090909' }}>
                <Background color="#1a1a1a" gap={20} />
                <Controls />
            </ReactFlow>
        </div>
    );
};

// --- Main Component ---

const VisualLabInner: React.FC<{ isInline?: boolean; editorOverlay?: boolean }> = ({ isInline, editorOverlay }) => {
    const isVisualLabOpen = useStore(state => state.isVisualLabOpen);
    const toggleVisualLab = useStore(state => state.toggleVisualLab);
    const visualLabMode = useStore(state => state.visualLabMode);
    const setVisualLabMode = useStore(state => state.setVisualLabMode);
    const visualLabData = useStore(state => state.visualLabData);
    const isFullScreen = useStore(state => state.isVisualLabFullScreen);
    const setIsFullScreen = useStore(state => state.setIsVisualLabFullScreen);
    const isSplitView = useStore(state => state.isVisualLabSplitView);
    const setIsSplitView = useStore(state => state.setIsVisualLabSplitView);

    const [nodes, setNodes, onNodesChange] = useNodesState([]);
    const [edges, setEdges, onEdgesChange] = useEdgesState([]);
    const [isAiModalOpen, setIsAiModalOpen] = useState(false);
    const [parseError, setParseError] = useState<string | null>(null);
    const { fitView } = useReactFlow();

    const activeTabId = useStore(state => state.activeTabId);
    const tabs = useStore(state => state.tabs);
    const activeJsonTab = useMemo(() => {
        const tab = tabs.find(t => t.id === activeTabId);
        if (!tab || tab.type === 'settings' || tab.type === 'mcp-store') return null;
        if (tab.language === 'json' || tab.path.toLowerCase().endsWith('.json')) return tab;
        return null;
    }, [tabs, activeTabId]);

    const isLargeGraph = nodes.length > 50;

    const onConnect = useCallback(
        (params: Connection | Edge) => setEdges((eds) => addEdge({
            ...params,
            animated: false, // Dotted edges look better static for performance
            style: {
                strokeWidth: 1.5,
                stroke: '#3b82f6',
                strokeDasharray: '4 4',
                opacity: 0.6
            },
        }, eds)),
        [setEdges]
    );

    const addNode = useCallback((type: string, label: string, color: string) => {
        const id = `node-${Date.now()}`;
        const newNode = {
            id,
            type,
            position: { x: 400, y: 300 }, // Center-ish
            data: { label, color },
        };
        setNodes((nds) => nds.concat(newNode));
    }, [setNodes]);

    const handleAiGenerate = useCallback(async (prompt: string) => {
        setIsAiModalOpen(false);
        try {
            const graph: any = await invoke('generate_visual_graph', { prompt });
            setNodes(graph.nodes);
            setEdges(graph.edges);
            setVisualLabMode(graph.nodes.some((n: any) => n.type === 'erdNode') ? 'erd' : 'flow');
            setTimeout(() => fitView({ padding: 0.2, duration: 800 }), 100);
        } catch (error) {
            console.error("AI Generation failed:", error);
        }
    }, [setNodes, setEdges, setVisualLabMode, fitView]);

    const onDragOver = useCallback((event: React.DragEvent) => {
        event.preventDefault();
        event.dataTransfer.dropEffect = 'move';
    }, []);

    const onDrop = useCallback(
        (event: React.DragEvent) => {
            event.preventDefault();

            const type = event.dataTransfer.getData('application/reactflow');
            const label = event.dataTransfer.getData('application/reactflow-label');
            const color = event.dataTransfer.getData('application/reactflow-color');

            if (typeof type === 'undefined' || !type) return;

            const position = { x: event.clientX - 300, y: event.clientY - 100 }; // Rough offset
            const newNode = {
                id: `node-${Date.now()}`,
                type,
                position,
                data: { label: `${label}`, color },
            };

            setNodes((nds) => nds.concat(newNode));
        },
        [setNodes]
    );

    const handleImport = useCallback((event: React.ChangeEvent<HTMLInputElement>) => {
        const file = event.target.files?.[0];
        if (!file) return;

        const reader = new FileReader();
        reader.onload = (e) => {
            const content = e.target?.result as string;
            // Use the global store to set experimental visual data
            const store = (window as any).useStore?.getState();
            if (store) {
                store.setVisualLabData(content);
                // Auto-detect mode based on extension or content
                if (file.name.endsWith('.sql')) {
                    store.setVisualLabMode('erd');
                } else {
                    store.setVisualLabMode('json');
                }
            }
        };
        reader.readAsText(file);
    }, []);

    const refreshNeuralGraph = useCallback(async () => {
        if ((visualLabMode as any) === 'neural') {
            try {
                const graph = await invoke<any>('get_neural_omni_graph');
                setNodes(graph.nodes);
                setEdges(graph.edges);
                setTimeout(() => fitView({ padding: 0.2, duration: 800 }), 100);
            } catch (err) {
                console.error("Neural Graph refresh failed", err);
            }
        }
    }, [visualLabMode, setNodes, setEdges, fitView]);

    const jsonSource = activeJsonTab?.content ?? visualLabData ?? '';

    const loadJsonGraph = useCallback((content: string) => {
        if (!content.trim()) {
            setNodes([]);
            setEdges([]);
            setParseError(null);
            return;
        }
        try {
            const parsed = JSON.parse(content);
            setParseError(null);
            invoke('get_visual_graph', { data: parsed, format: 'json' })
                .then((graph: any) => {
                    setNodes(graph.nodes ?? []);
                    setEdges(graph.edges ?? []);
                    window.setTimeout(() => fitView({ padding: 0.15, duration: 600 }), 80);
                })
                .catch((err) => {
                    console.error('JSON visualizer parse failed', err);
                    setParseError('Could not build graph from this JSON.');
                    setNodes([]);
                    setEdges([]);
                });
        } catch {
            setParseError('Invalid JSON — fix syntax in the editor to visualize.');
            setNodes([]);
            setEdges([]);
        }
    }, [setNodes, setEdges, fitView]);

    // Live JSON sync in editor overlay (debounced).
    useEffect(() => {
        if (!editorOverlay || isInline || visualLabMode !== 'json') return;
        const handle = window.setTimeout(() => loadJsonGraph(jsonSource), 280);
        return () => window.clearTimeout(handle);
    }, [editorOverlay, isInline, visualLabMode, jsonSource, activeJsonTab?.id, loadJsonGraph]);

    // Handle initial load and mode changes (overlay / inline only)
    useEffect(() => {
        if (editorOverlay && visualLabMode === 'json') return;
        if ((visualLabMode as any) === 'neural') {
            refreshNeuralGraph();
            return;
        }

        if (visualLabData && visualLabMode !== 'none') {
            const store = (window as any).useStore?.getState();
            const activeTab = store?.tabs.find((t: any) => t.id === store.activeTabId);

            let format = visualLabMode === 'erd' ? 'sql' : 'json';
            if (activeTab?.path?.endsWith('.mongodb')) {
                format = 'mongodb';
            }

            invoke('get_visual_graph', {
                data: visualLabMode === 'erd' ? visualLabData : (format === 'mongodb' ? visualLabData : JSON.parse(visualLabData)),
                format
            })
                .then((graph: any) => {
                    setNodes(graph.nodes);
                    setEdges(graph.edges);
                    setTimeout(() => fitView({ padding: 0.2, duration: 800 }), 100);
                })
                .catch(err => {
                    console.error("Backend parse failed, falling back to basic view", err);
                    setNodes([{
                        id: 'error',
                        type: 'flowNode',
                        data: { label: 'Parse Error', color: '#ef4444' },
                        position: { x: 0, y: 0 }
                    }]);
                });
        }
    }, [visualLabData, visualLabMode, setNodes, setEdges, fitView, refreshNeuralGraph, editorOverlay]);

    // Listen for real-time memory updates from the AI Cortex (overlay modes only)
    useEffect(() => {
        let unlisten: any;
        const setup = async () => {
            unlisten = await listen<any>('memory-update', () => {
                refreshNeuralGraph();
            });
        };
        setup();
        return () => { if (unlisten) unlisten(); };
    }, [refreshNeuralGraph]);

    if (!isInline && !isVisualLabOpen) return null;

    const isJsonOverlay = editorOverlay && visualLabMode === 'json';
    return (
        <motion.div
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.95 }}
            transition={{ duration: 0.2, ease: "easeOut" }}
            style={{
                position: isFullScreen ? 'fixed' : (isInline || editorOverlay ? 'absolute' : 'fixed'),
                top: isFullScreen ? '0' : (isInline || editorOverlay ? '0' : '40px'),
                left: isFullScreen ? '0' : (isInline || editorOverlay ? '0' : '60px'),
                right: isFullScreen ? '0' : (isInline || editorOverlay ? '0' : '12px'),
                bottom: isFullScreen ? '0' : (isInline || editorOverlay ? '0' : '12px'),
                zIndex: isFullScreen ? 99999 : (editorOverlay ? 200 : 5000),
                background: 'var(--vscode-editor-background, #0f0f0f)',
                borderRadius: (isFullScreen || isInline) ? '0' : '12px',
                border: isFullScreen ? 'none' : '1px solid rgba(255, 255, 255, 0.05)',
                display: 'flex',
                flexDirection: 'column',
                boxShadow: isInline ? 'none' : '0 20px 50px rgba(0,0,0,0.5)',
                overflow: 'hidden',
                width: isInline ? '100%' : 'auto',
                height: isInline ? '100%' : 'auto'
            }}
        >
            {/* Header / Toolbar */}
            <div style={{
                height: '50px',
                padding: '0 20px',
                display: 'flex',
                alignItems: 'center',
                background: 'rgba(255,255,255,0.02)',
                borderBottom: '1px solid rgba(255,255,255,0.05)',
                gap: '20px'
            }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
                    <div style={{
                        width: '32px', height: '32px',
                        background: 'linear-gradient(135deg, #a855f7 0%, #6366f1 100%)',
                        borderRadius: '8px',
                        display: 'flex', alignItems: 'center', justifyContent: 'center'
                    }}>
                        <Brain size={18} color="#fff" />
                    </div>
                    <span style={{ fontWeight: 700, fontSize: '14px', letterSpacing: '-0.2px' }}>
                        {isJsonOverlay ? 'JSON Visualizer' : 'Visual Lab'}
                    </span>
                </div>

                <div style={{ flex: 1, display: 'flex', justifyContent: 'center', gap: '8px' }}>
                    {!isJsonOverlay && (
                        <>
                    <ModeToggle
                        active={visualLabMode === 'json'}
                        onClick={() => setVisualLabMode('json')}
                        icon={<FileJson size={14} />}
                        label="JSON Flow"
                    />
                    <ModeToggle
                        active={visualLabMode === 'flow'}
                        onClick={() => setVisualLabMode('flow')}
                        icon={<Share2 size={14} />}
                        label="Flowchart"
                    />
                    <ModeToggle
                        active={visualLabMode === 'erd'}
                        onClick={() => setVisualLabMode('erd')}
                        icon={<Database size={14} />}
                        label="ERD/Schema"
                    />
                    <ModeToggle
                        active={(visualLabMode as any) === 'summary'}
                        onClick={() => setVisualLabMode('summary' as any)}
                        icon={<Activity size={14} />}
                        label="Summary"
                    />
                    <ModeToggle
                        active={(visualLabMode as any) === 'deps'}
                        onClick={() => setVisualLabMode('deps' as any)}
                        icon={<Cpu size={14} />}
                        label="Dep Graph"
                    />
                        </>
                    )}
                    {isJsonOverlay && (
                        <span style={{ fontSize: 11, opacity: 0.45, alignSelf: 'center' }}>
                            Pan and zoom to explore — graph updates as you edit JSON
                        </span>
                    )}
                </div>

                <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                    <button
                        onClick={() => {
                            const store = (window as any).useStore?.getState();
                            const activeTab = store.tabs.find((t: any) => t.id === store.activeTabId);
                            if (activeTab) store.setVisualLabData(activeTab.content);
                        }}
                        title="Reload from Editor"
                        style={{
                            background: 'transparent', border: 'none', color: 'rgba(255,255,255,0.4)',
                            cursor: 'pointer', display: 'flex', alignItems: 'center', gap: '6px',
                            padding: '4px 8px', borderRadius: '4px', transition: 'all 0.2s'
                        }}
                        onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.05)'}
                        onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}
                    >
                        <RefreshCw size={16} /> <span style={{ fontSize: '12px' }}>Refresh</span>
                    </button>
                    <button
                        onClick={() => fitView({ padding: 0.2, duration: 800 })}
                        title="Fit All Nodes"
                        style={{
                            background: 'transparent', border: 'none', color: 'rgba(255,255,255,0.4)',
                            cursor: 'pointer', display: 'flex', alignItems: 'center', gap: '6px',
                            padding: '4px 8px', borderRadius: '4px', transition: 'all 0.2s'
                        }}
                        onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.05)'}
                        onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}
                    >
                        <Maximize2 size={16} /> <span style={{ fontSize: '12px' }}>Focus</span>
                    </button>
                    <button
                        onClick={() => setIsFullScreen(!isFullScreen)}
                        title={isFullScreen ? "Exit Full Screen" : "Enter Full Screen"}
                        style={{
                            background: isFullScreen ? 'rgba(59, 130, 246, 0.2)' : 'transparent',
                            border: 'none', color: isFullScreen ? '#3b82f6' : 'rgba(255,255,255,0.4)',
                            cursor: 'pointer', display: 'flex', alignItems: 'center', gap: '6px',
                            padding: '4px 8px', borderRadius: '4px', transition: 'all 0.2s'
                        }}
                        onMouseEnter={(e) => e.currentTarget.style.background = isFullScreen ? 'rgba(59, 130, 246, 0.3)' : 'rgba(255,255,255,0.05)'}
                        onMouseLeave={(e) => e.currentTarget.style.background = isFullScreen ? 'rgba(59, 130, 246, 0.2)' : 'transparent'}
                    >
                        <Maximize2 size={16} style={{ transform: isFullScreen ? 'rotate(180deg)' : 'none' }} />
                        <span style={{ fontSize: '12px' }}>{isFullScreen ? "Window" : "Full"}</span>
                    </button>
                    <button
                        onClick={() => setIsSplitView(!isSplitView)}
                        title={isSplitView ? "Exit Split View" : "Enter Split View"}
                        style={{
                            background: isSplitView ? 'rgba(59, 130, 246, 0.2)' : 'transparent',
                            border: 'none', color: isSplitView ? '#3b82f6' : 'rgba(255,255,255,0.4)',
                            cursor: 'pointer', display: 'flex', alignItems: 'center', gap: '6px',
                            padding: '4px 8px', borderRadius: '4px', transition: 'all 0.2s'
                        }}
                        onMouseEnter={(e) => e.currentTarget.style.background = isSplitView ? 'rgba(59, 130, 246, 0.3)' : 'rgba(255,255,255,0.05)'}
                        onMouseLeave={(e) => e.currentTarget.style.background = isSplitView ? 'rgba(59, 130, 246, 0.2)' : 'transparent'}
                    >
                        <Columns size={16} />
                        <span style={{ fontSize: '12px' }}>{isSplitView ? "Merge" : "Split"}</span>
                    </button>
                    <button
                        onClick={() => {
                            if (confirm('Clear entire canvas?')) {
                                setNodes([]);
                                setEdges([]);
                            }
                        }}
                        title="Clear Canvas"
                        style={{
                            background: 'transparent', border: 'none', color: 'rgba(239, 68, 68, 0.5)',
                            cursor: 'pointer', display: 'flex', alignItems: 'center', gap: '6px',
                            padding: '4px 8px', borderRadius: '4px', transition: 'all 0.2s'
                        }}
                        onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(239, 68, 68, 0.1)'}
                        onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}
                    >
                        <Eraser size={16} /> <span style={{ fontSize: '12px' }}>Clear</span>
                    </button>
                    <div style={{ width: '1px', height: '20px', background: 'rgba(255,255,255,0.1)' }}></div>
                    <input
                        type="file"
                        id="visual-lab-import"
                        style={{ display: 'none' }}
                        accept=".json,.sql"
                        onChange={handleImport}
                    />
                    <button
                        onClick={() => document.getElementById('visual-lab-import')?.click()}
                        style={{
                            background: 'transparent', border: 'none', color: 'rgba(255,255,255,0.4)',
                            cursor: 'pointer', display: 'flex', alignItems: 'center', gap: '6px',
                            padding: '4px 8px', borderRadius: '4px', transition: 'all 0.2s'
                        }}
                        onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.05)'}
                        onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}
                    >
                        <Upload size={16} /> <span style={{ fontSize: '12px' }}>Import</span>
                    </button>
                    <button style={{
                        background: 'transparent', border: 'none', color: 'rgba(255,255,255,0.4)',
                        cursor: 'pointer', display: 'flex', alignItems: 'center', gap: '6px'
                    }}>
                        <Download size={16} /> <span style={{ fontSize: '12px' }}>Export</span>
                    </button>
                    <div style={{ width: '1px', height: '20px', background: 'rgba(255,255,255,0.1)' }}></div>
                    <button
                        onClick={() => toggleVisualLab(false)}
                        style={{
                            background: 'rgba(255,255,255,0.05)', border: 'none', color: 'var(--vscode-editor-foreground, #fff)',
                            width: '28px', height: '28px', borderRadius: '50%',
                            display: 'flex', alignItems: 'center', justifyContent: 'center',
                            cursor: 'pointer'
                        }}
                    >
                        <X size={16} />
                    </button>
                </div>
            </div>

            {/* Main Content Area */}
            <div style={{ flex: 1, position: 'relative', display: 'flex', overflow: 'hidden' }}>
                {/* Mode-Specific Rendering */}
                {['json', 'flow', 'erd'].includes(visualLabMode as string) ? (
                    <>
                        {/* Sidebar - only for graph modes */}
                        {visualLabMode !== 'json' && (
                            <div style={{
                                width: '280px',
                                borderRight: '1px solid rgba(255,255,255,0.05)',
                                padding: '24px',
                                background: 'rgba(0,0,0,0.3)',
                                display: 'flex',
                                flexDirection: 'column',
                                gap: '20px'
                            }}>
                                <div style={{ fontSize: '13px', fontWeight: 700, opacity: 0.5, textTransform: 'uppercase', letterSpacing: '0.1em' }}>Components</div>
                                <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
                                    <DraggableNode type="input" label="Trigger / Input" color="#3b82f6" onClick={addNode} />
                                    <DraggableNode type="process" label="Action / Process" color="#8b5cf6" onClick={addNode} />
                                    <DraggableNode type="decision" label="Condition / If" color="#f59e0b" onClick={addNode} />
                                    <DraggableNode type="output" label="Goal / Output" color="#10b981" onClick={addNode} />
                                </div>
                                {visualLabMode === 'erd' && (
                                    <>
                                        <div style={{ fontSize: '13px', fontWeight: 700, opacity: 0.5, textTransform: 'uppercase', letterSpacing: '0.1em', marginTop: '20px' }}>Database</div>
                                        <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
                                            <DraggableNode type="table" label="Table / Entity" color="#ec4899" onClick={addNode} />
                                        </div>
                                    </>
                                )}
                            </div>
                        )}

                        {/* Graph Canvas */}
                        <div style={{ flex: 1, position: 'relative' }}>
                            {parseError && (
                                <div style={{
                                    position: 'absolute', top: 8, left: 8, right: 8, zIndex: 15,
                                    padding: '6px 10px', borderRadius: 6, fontSize: 11,
                                    background: 'rgba(239,68,68,0.12)', border: '1px solid rgba(239,68,68,0.35)', color: '#f87171',
                                }}>
                                    {parseError}
                                </div>
                            )}
                            {nodes.length === 0 && !parseError && (
                                <div style={{
                                    position: 'absolute', inset: 0, display: 'flex', flexDirection: 'column',
                                    alignItems: 'center', justifyContent: 'center', zIndex: 10,
                                    color: 'rgba(255,255,255,0.2)', gap: '15px'
                                }}>
                                    <FileJson size={48} strokeWidth={1} />
                                    <p style={{ margin: 0, fontSize: '16px', fontWeight: 500, color: 'rgba(255,255,255,0.4)' }}>No Data Detected</p>
                                </div>
                            )}
                            <ReactFlow
                                nodes={nodes} edges={edges}
                                onNodesChange={onNodesChange} onEdgesChange={onEdgesChange} onConnect={onConnect}
                                nodeTypes={nodeTypes} fitView onDrop={onDrop} onDragOver={onDragOver}
                                style={{ background: '#090909' }}
                            >
                                <Background color="#1a1a1a" gap={20} />
                                <Controls />
                                <Panel position="bottom-right">
                                    <div style={{ background: 'rgba(30,30,30,0.8)', backdropFilter: 'blur(10px)', padding: '10px 15px', borderRadius: '20px', border: '1px solid rgba(255,255,255,0.1)' }}>
                                        <AiCommandButton onClick={() => setIsAiModalOpen(true)} />
                                    </div>
                                </Panel>
                            </ReactFlow>
                        </div>
                    </>
                ) : visualLabMode === ('summary' as any) ? (
                    <div style={{ flex: 1, background: '#090909', overflow: 'hidden' }}>
                        <NeuralSummaryView />
                    </div>
                ) : (visualLabMode as any) === 'deps' ? (
                    <DependencyGraphView />
                ) : null}
            </div>

            <AnimatePresence>
                {isAiModalOpen && (
                    <AiBuilderModal
                        isOpen={isAiModalOpen}
                        onClose={() => setIsAiModalOpen(false)}
                        onGenerate={handleAiGenerate}
                    />
                )}
            </AnimatePresence>

            <div style={{
                height: '24px',
                padding: '0 15px',
                display: 'flex',
                alignItems: 'center',
                background: 'rgba(0,0,0,0.2)',
                fontSize: '10px',
                color: 'rgba(255,255,255,0.3)',
                justifyContent: 'space-between'
            }}>
                <span>VISUAL_LAB_ACTIVE ({String(visualLabMode).toUpperCase()})</span>
                <span>{nodes.length} Nodes | {edges.length} Connections</span>
            </div>
        </motion.div>
    );
};

const ModeToggle = ({ active, onClick, icon, label }: any) => (
    <button
        onClick={onClick}
        style={{
            padding: '6px 14px',
            borderRadius: '20px',
            background: active ? 'rgba(255,255,255,0.08)' : 'transparent',
            border: active ? '1px solid rgba(255,255,255,0.1)' : '1px solid transparent',
            color: active ? '#fff' : 'rgba(255,255,255,0.4)',
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            fontSize: '12px',
            fontWeight: 500,
            cursor: 'pointer',
            transition: 'all 0.2s'
        }}
    >
        {icon}
        {label}
    </button>
);

const AiCommandButton = ({ onClick }: any) => (
    <button
        onClick={onClick}
        style={{
            background: 'linear-gradient(90deg, #3b82f6, #8b5cf6)',
            border: 'none',
            color: 'var(--vscode-editor-foreground, #fff)',
            padding: '6px 16px',
            borderRadius: '100px',
            fontSize: '12px',
            fontWeight: 600,
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            cursor: 'pointer',
            boxShadow: '0 4px 15px rgba(59, 130, 246, 0.4)'
        }}>
        <Plus size={14} /> AI Builder
    </button>
);

const AiBuilderModal = ({ isOpen, onClose, onGenerate }: any) => {
    const [prompt, setPrompt] = useState('');

    if (!isOpen) return null;

    return (
        <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            style={{
                position: 'absolute',
                inset: 0,
                background: 'rgba(0,0,0,0.8)',
                backdropFilter: 'blur(5px)',
                zIndex: 6000,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                padding: '20px'
            }}
            onClick={onClose}
        >
            <motion.div
                initial={{ scale: 0.9, y: 20 }}
                animate={{ scale: 1, y: 0 }}
                className="ai-modal"
                style={{
                    width: '100%',
                    maxWidth: '500px',
                    background: 'var(--vscode-editor-background, #1a1a1a)',
                    borderRadius: '16px',
                    border: '1px solid rgba(255,255,255,0.1)',
                    padding: '24px',
                    boxShadow: '0 20px 50px rgba(0,0,0,0.5)'
                }}
                onClick={e => e.stopPropagation()}
            >
                <div style={{ display: 'flex', alignItems: 'center', gap: '12px', marginBottom: '20px' }}>
                    <div style={{
                        width: '36px', height: '36px',
                        background: 'linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%)',
                        borderRadius: '10px',
                        display: 'flex', alignItems: 'center', justifyContent: 'center'
                    }}>
                        <Brain size={20} color="#fff" />
                    </div>
                    <div>
                        <h3 style={{ margin: 0, fontSize: '16px' }}>AI Flow Builder</h3>
                        <p style={{ margin: 0, fontSize: '12px', opacity: 0.5 }}>Describe the process or architecture</p>
                    </div>
                </div>

                <textarea
                    autoFocus
                    value={prompt}
                    onChange={(e) => setPrompt(e.target.value)}
                    placeholder="e.g., 'Create a login flow with social auth' or 'Design a database for a blog with users, posts, and comments'"
                    style={{
                        width: '100%',
                        height: '120px',
                        background: 'rgba(0,0,0,0.3)',
                        border: '1px solid rgba(255,255,255,0.1)',
                        borderRadius: '12px',
                        padding: '12px',
                        color: 'var(--vscode-editor-foreground, #fff)',
                        fontSize: '14px',
                        resize: 'none',
                        outline: 'none',
                        marginBottom: '20px',
                        fontFamily: 'inherit'
                    }}
                />

                <div style={{ display: 'flex', gap: '12px', justifyContent: 'flex-end' }}>
                    <button
                        onClick={onClose}
                        style={{
                            padding: '10px 20px',
                            borderRadius: '10px',
                            background: 'transparent',
                            border: '1px solid rgba(255,255,255,0.1)',
                            color: 'rgba(255,255,255,0.6)',
                            cursor: 'pointer'
                        }}
                    >
                        Cancel
                    </button>
                    <button
                        onClick={async () => {
                            const btn = document.getElementById('ai-gen-btn');
                            if (btn) {
                                btn.innerText = 'Generating...';
                                (btn as HTMLButtonElement).disabled = true;
                            }
                            await onGenerate(prompt);
                        }}
                        id="ai-gen-btn"
                        disabled={!prompt.trim()}
                        style={{
                            padding: '10px 24px',
                            borderRadius: '10px',
                            background: 'linear-gradient(90deg, #3b82f6, #8b5cf6)',
                            border: 'none',
                            color: 'var(--vscode-editor-foreground, #fff)',
                            fontWeight: 600,
                            cursor: prompt.trim() ? 'pointer' : 'not-allowed',
                            opacity: prompt.trim() ? 1 : 0.5,
                            minWidth: '160px'
                        }}
                    >
                        Generate Diagram
                    </button>
                </div>
            </motion.div>
        </motion.div>
    );
};

const VisualLab: React.FC<{ isInline?: boolean; editorOverlay?: boolean }> = ({ isInline, editorOverlay }) => (
    <ReactFlowProvider>
        <VisualLabInner isInline={isInline} editorOverlay={editorOverlay} />
    </ReactFlowProvider>
);

export default VisualLab;
