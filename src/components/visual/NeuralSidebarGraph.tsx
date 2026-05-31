import React, { useCallback, useEffect, useState, useMemo } from 'react';
import ReactFlow, {
    useNodesState,
    useEdgesState,
    Handle,
    Position,
    ReactFlowProvider,
    useReactFlow,
    Controls,
    Background,
} from 'reactflow';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { motion, AnimatePresence } from 'framer-motion';
import { Maximize2, Minimize2, Cpu, Zap } from 'lucide-react';
import 'reactflow/dist/style.css';

// --- Central Drive Node ---
const DriveNode = React.memo(({ data }: any) => {
    return (
        <div style={{ position: 'relative', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
            <motion.div
                animate={{
                    scale: [1, 1.1, 1],
                    rotate: [0, 90, 180, 270, 360],
                    boxShadow: [
                        '0 0 20px rgba(168, 85, 247, 0.4)',
                        '0 0 40px rgba(168, 85, 247, 0.8)',
                        '0 0 20px rgba(168, 85, 247, 0.4)',
                    ],
                }}
                transition={{ duration: 10, repeat: Infinity, ease: "linear" }}
                style={{
                    width: '60px',
                    height: '60px',
                    borderRadius: '12px',
                    background: 'linear-gradient(135deg, #a855f7 0%, #6366f1 100%)',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    border: '2px solid rgba(255,255,255,0.3)',
                    zIndex: 2,
                }}
            >
                <Cpu size={30} color="#fff" />
            </motion.div>

            {/* Pulsing rings */}
            {[1, 2].map((i) => (
                <motion.div
                    key={i}
                    initial={{ scale: 1, opacity: 0.5 }}
                    animate={{ scale: 2.5, opacity: 0 }}
                    transition={{ duration: 3, repeat: Infinity, delay: i * 1.5, ease: "easeOut" }}
                    style={{
                        position: 'absolute',
                        width: '60px',
                        height: '60px',
                        borderRadius: '12px',
                        border: '1px solid #a855f7',
                        zIndex: 1,
                    }}
                />
            ))}
            <div style={{
                position: 'absolute',
                top: '70px',
                color: '#a855f7',
                fontSize: '10px',
                fontWeight: 800,
                textTransform: 'uppercase',
                letterSpacing: '0.1em',
                whiteSpace: 'nowrap'
            }}>
                Sentient Kernel
            </div>
            <Handle type="source" position={Position.Bottom} style={{ visibility: 'hidden' }} />
            <Handle type="target" position={Position.Top} style={{ visibility: 'hidden' }} />
        </div>
    );
});
DriveNode.displayName = 'DriveNode';

// --- Compact Neural Node ---
const NeuralNode = React.memo(({ data }: any) => {
    const isSentinel = data.category === 'fix_lessons';
    const isActive = data.isActive;
    const baseColor = isSentinel ? '#f472b6' : '#6366f1';
    const glowColor = isActive ? '#4ade80' : baseColor;

    return (
        <motion.div
            animate={isActive ? {
                scale: [1, 1.15, 1],
                boxShadow: [
                    `0 0 10px ${glowColor}aa`,
                    `0 0 30px ${glowColor}ff`,
                    `0 0 10px ${glowColor}aa`
                ],
                borderColor: [glowColor, '#fff', glowColor]
            } : {
                boxShadow: [
                    `0 0 5px ${glowColor}33`,
                    `0 0 15px ${glowColor}66`,
                    `0 0 5px ${glowColor}33`
                ],
                scale: [1, 1.02, 1]
            }}
            transition={isActive ? { duration: 0.5, repeat: Infinity } : { duration: 3, repeat: Infinity, ease: "easeInOut" }}
            style={{
                padding: '6px 12px',
                borderRadius: '20px',
                background: isActive ? 'rgba(74, 222, 128, 0.1)' : '#0a0a0a',
                border: `1.5px solid ${glowColor}`,
                color: 'var(--vscode-editor-foreground, #fff)',
                fontSize: '10px',
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
                minWidth: '100px',
                whiteSpace: 'nowrap',
                zIndex: isActive ? 10 : 1
            }}
        >
            <Handle type="target" position={Position.Top} style={{ visibility: 'hidden' }} />
            <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                {isActive ? (
                    <motion.div
                        animate={{ rotate: 360 }}
                        transition={{ duration: 2, repeat: Infinity, ease: "linear" }}
                    >
                        <Zap size={10} color="#4ade80" />
                    </motion.div>
                ) : (
                    <div style={{
                        width: '6px',
                        height: '6px',
                        borderRadius: '50%',
                        background: glowColor,
                        boxShadow: `0 0 8px ${glowColor}`
                    }} />
                )}
                <div style={{ display: 'flex', flexDirection: 'column' }}>
                    <span style={{ fontWeight: 700, overflow: 'hidden', textOverflow: 'ellipsis', maxWidth: '120px' }}>{data.label}</span>
                    {isActive && <span style={{ fontSize: '7px', color: '#4ade80', fontWeight: 800 }}>ACTIVE_CONTEXT</span>}
                </div>
            </div>
            <Handle type="source" position={Position.Bottom} style={{ visibility: 'hidden' }} />
        </motion.div>
    );
});
NeuralNode.displayName = 'NeuralNode';

const nodeTypes = {
    neuralNode: NeuralNode,
    driveNode: DriveNode,
};

const GraphCanvas = ({ isFullscreen, onToggleFullscreen }: { isFullscreen: boolean, onToggleFullscreen: () => void }) => {
    const [nodes, setNodes, onNodesChange] = useNodesState([]);
    const [edges, setEdges, onEdgesChange] = useEdgesState([]);
    const [activeIds, setActiveIds] = useState<string[]>([]);
    const { fitView } = useReactFlow();

    const refreshNeuralGraph = useCallback(async () => {
        try {
            const graph = await invoke<any>('get_neural_omni_graph');

            // 1. Add Centered Drive Node
            const newNodes: any[] = [
                {
                    id: 'drive-kernel',
                    type: 'driveNode',
                    position: { x: 0, y: 0 },
                    data: { label: 'Sentient Kernel' }
                }
            ];

            // 2. Scale and format neural nodes
            const neuralNodes = graph.nodes.map((n: any) => ({
                ...n,
                data: {
                    ...n.data,
                    isActive: activeIds.includes(n.id)
                },
                position: {
                    x: n.position.x * (isFullscreen ? 1.5 : 0.8),
                    y: n.position.y * (isFullscreen ? 1.5 : 0.8)
                }
            }));

            setNodes([...newNodes, ...neuralNodes]);

            // 3. Connect everything to the drive if no edges, or just add some core links
            const driveEdges = graph.nodes.filter((_: any, i: number) => i % 5 === 0).map((n: any) => ({
                id: `drive-link-${n.id}`,
                source: 'drive-kernel',
                target: n.id,
                animated: true,
                style: { stroke: '#a855f7', strokeWidth: 1.5, opacity: 0.4 }
            }));

            setEdges([...graph.edges, ...driveEdges]);

            setTimeout(() => fitView({ padding: 0.2, duration: 800 }), 100);
        } catch (err) {
            console.error("Neural Sidebar Graph refresh failed", err);
        }
    }, [setNodes, setEdges, fitView, activeIds, isFullscreen]);

    useEffect(() => {
        refreshNeuralGraph();

        const unlistens: any[] = [];

        const setup = async () => {
            unlistens.push(await listen<any>('memory-update', () => {
                refreshNeuralGraph();
            }));

            unlistens.push(await listen<any>('context-active', (event) => {
                const ids = event.payload.ids || [];
                setActiveIds(ids);
                // Clear highlights after 5 seconds of inactivity
                setTimeout(() => {
                    setActiveIds(prev => prev.filter(id => !ids.includes(id)));
                }, 5000);
            }));
        };

        setup();
        return () => { unlistens.forEach(fn => fn && fn()); };
    }, [refreshNeuralGraph]);

    return (
        <div style={{ width: '100%', height: '100%', position: 'relative' }}>
            <ReactFlow
                nodes={nodes}
                edges={edges}
                onNodesChange={onNodesChange}
                onEdgesChange={onEdgesChange}
                nodeTypes={nodeTypes}
                defaultEdgeOptions={{
                    animated: true,
                    style: { stroke: '#f472b6', strokeWidth: 1, opacity: 0.1 }
                }}
                fitView
                minZoom={0.05}
                maxZoom={2}
                style={{ background: 'transparent' }}
                proOptions={{ hideAttribution: true }}
            >
                {isFullscreen && (
                    <>
                        <Background color="#1a1a1a" gap={20} />
                        <Controls />
                    </>
                )}
            </ReactFlow>

            <button
                onClick={onToggleFullscreen}
                style={{
                    position: 'absolute',
                    top: '10px',
                    right: '10px',
                    background: 'rgba(255,255,255,0.05)',
                    border: '1px solid rgba(255,255,255,0.1)',
                    color: 'var(--vscode-editor-foreground, #fff)',
                    padding: '6px',
                    borderRadius: '8px',
                    cursor: 'pointer',
                    zIndex: 5,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    backdropFilter: 'blur(4px)'
                }}
                title={isFullscreen ? "Exit Fullscreen" : "Enter Fullscreen"}
            >
                {isFullscreen ? <Minimize2 size={14} /> : <Maximize2 size={14} />}
            </button>
        </div>
    );
};

const NeuralSidebarGraph: React.FC = () => {
    const [isFullscreen, setIsFullscreen] = useState(false);

    return (
        <>
            <motion.div
                layout
                style={{
                    width: '100%',
                    height: isFullscreen ? '100vh' : '280px',
                    position: isFullscreen ? 'fixed' : 'relative',
                    top: isFullscreen ? 0 : 'auto',
                    left: isFullscreen ? 0 : 'auto',
                    right: isFullscreen ? 0 : 'auto',
                    bottom: isFullscreen ? 0 : 'auto',
                    zIndex: isFullscreen ? 9999 : 1,
                    background: isFullscreen ? '#0a0a0a' : 'transparent',
                    marginBottom: isFullscreen ? 0 : '16px',
                    overflow: 'hidden',
                    borderRadius: isFullscreen ? 0 : '12px',
                    border: isFullscreen ? 'none' : '1px solid rgba(255,255,255,0.03)'
                }}
            >
                {!isFullscreen && (
                    <div style={{
                        position: 'absolute',
                        top: 0, left: 0, right: 0, bottom: 0,
                        background: 'radial-gradient(circle at center, rgba(99, 102, 241, 0.05) 0%, transparent 70%)',
                        pointerEvents: 'none',
                        zIndex: 0
                    }} />
                )}

                <ReactFlowProvider>
                    <GraphCanvas
                        isFullscreen={isFullscreen}
                        onToggleFullscreen={() => setIsFullscreen(!isFullscreen)}
                    />
                </ReactFlowProvider>

                <div style={{
                    position: 'absolute',
                    bottom: '12px',
                    left: '12px',
                    fontSize: '9px',
                    color: 'rgba(255,255,255,0.3)',
                    fontWeight: 800,
                    textTransform: 'uppercase',
                    letterSpacing: '0.15em',
                    pointerEvents: 'none',
                    zIndex: 2,
                    display: 'flex',
                    alignItems: 'center',
                    gap: '6px'
                }}>
                    <div style={{ width: '6px', height: '6px', borderRadius: '50%', background: '#4ade80', boxShadow: '0 0 8px #4ade80' }} />
                    Neural Engine Active
                </div>
            </motion.div>

            {/* Backdrop for fullscreen */}
            <AnimatePresence>
                {isFullscreen && (
                    <motion.div
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        exit={{ opacity: 0 }}
                        style={{
                            position: 'fixed',
                            inset: 0,
                            background: 'rgba(0,0,0,0.85)',
                            zIndex: 9998,
                            backdropFilter: 'blur(10px)'
                        }}
                        onClick={() => setIsFullscreen(false)}
                    />
                )}
            </AnimatePresence>
        </>
    );
};

export default NeuralSidebarGraph;
