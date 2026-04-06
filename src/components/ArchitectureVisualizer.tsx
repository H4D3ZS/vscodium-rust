import React, { useCallback, useMemo } from 'react';
import ReactFlow, {
    Background,
    Controls,
    Handle,
    Position,
    type Edge,
    type Node,
    MarkerType
} from 'reactflow';
import 'reactflow/dist/style.css';
import { FileCode, CheckCircle, AlertCircle, Clock, ChevronDown, ChevronRight } from 'lucide-react';

// Custom Node for Class/Module
const ClassNode = ({ data }: any) => {
    const [isExpanded, setIsExpanded] = React.useState(true);

    return (
        <div style={{
            background: 'var(--vscode-sideBar-background)',
            border: '1px solid var(--vscode-panel-border)',
            borderRadius: '8px',
            minWidth: '220px',
            boxShadow: '0 10px 15px rgba(0,0,0,0.3)',
            overflow: 'hidden',
            color: 'var(--vscode-foreground)'
        }}>
            {/* Header */}
            <div style={{
                background: 'rgba(0, 122, 204, 0.2)',
                padding: '10px 14px',
                borderBottom: '1px solid var(--vscode-panel-border)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                cursor: 'pointer'
            }} onClick={() => setIsExpanded(!isExpanded)}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <FileCode size={14} color="#007acc" />
                    <span style={{ fontWeight: 600, fontSize: '13px' }}>{data.name}</span>
                </div>
                {isExpanded ? <ChevronDown size={14} opacity={0.6} /> : <ChevronRight size={14} opacity={0.6} />}
            </div>

            {isExpanded && (
                <div style={{ padding: '8px 0' }}>
                    {/* Properties Section */}
                    {data.properties && data.properties.length > 0 && (
                        <div style={{ marginBottom: '8px' }}>
                            <div style={{ fontSize: '10px', opacity: 0.4, fontWeight: 700, padding: '4px 14px', textTransform: 'uppercase', letterSpacing: '0.5px' }}>Properties</div>
                            {data.properties.map((p: any, idx: number) => (
                                <div key={idx} style={{ padding: '4px 14px', fontSize: '12px', display: 'flex', justifyContent: 'space-between' }}>
                                    <span>{p.name}</span>
                                    <span style={{ opacity: 0.5 }}>{p.type}</span>
                                </div>
                            ))}
                        </div>
                    )}

                    {/* Methods Section */}
                    <div>
                        <div style={{ fontSize: '10px', opacity: 0.4, fontWeight: 700, padding: '4px 14px', textTransform: 'uppercase', letterSpacing: '0.5px' }}>Methods</div>
                        {data.methods.map((m: any, idx: number) => (
                            <div key={idx}
                                onClick={(e) => { e.stopPropagation(); m.onSelect?.(); }}
                                style={{
                                    padding: '6px 14px',
                                    fontSize: '12px',
                                    display: 'flex',
                                    alignItems: 'center',
                                    justifyContent: 'space-between',
                                    background: 'rgba(255,255,255,0.02)',
                                    borderBottom: '1px solid rgba(255,255,255,0.03)',
                                    cursor: 'pointer'
                                }}
                            >
                                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                    {m.status === 'Done' ? <CheckCircle size={12} color="#10b981" /> :
                                        m.status === 'Failed' ? <AlertCircle size={12} color="#f87171" /> :
                                            <Clock size={12} opacity={0.5} />}
                                    <span>{m.name}</span>
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            )}

            {/* Connection Handles */}
            <Handle type="target" position={Position.Left} style={{ background: '#555' }} />
            <Handle type="source" position={Position.Right} style={{ background: '#555' }} />
            <Handle type="target" position={Position.Top} style={{ background: '#555' }} />
            <Handle type="source" position={Position.Bottom} style={{ background: '#555' }} />
        </div>
    );
};

const nodeTypes = {
    class: ClassNode,
};

interface ArchitectureVisualizerProps {
    files: any[];
    tasks: any[];
    onSelectMethod?: (method: any) => void;
}

export const ArchitectureVisualizer: React.FC<ArchitectureVisualizerProps> = ({ files, tasks, onSelectMethod }) => {
    // Custom Node definition inside or passed via props
    const nodes: Node[] = useMemo(() => {
        return files.map((file, index) => ({
            id: `file-${file.id}`,
            type: 'class',
            position: { x: (index % 3) * 350, y: Math.floor(index / 3) * 350 },
            data: {
                name: file.path,
                properties: [],
                methods: file.functions.map((fn: any) => ({
                    ...fn,
                    onSelect: () => onSelectMethod?.(fn)
                }))
            },
        }));
    }, [files, onSelectMethod]);

    const edges: Edge[] = useMemo(() => {
        const e: Edge[] = [];
        if (nodes.length > 1) {
            for (let i = 0; i < nodes.length - 1; i++) {
                e.push({
                    id: `e${i}-${i + 1}`,
                    source: nodes[i].id,
                    target: nodes[i + 1].id,
                    animated: true,
                    markerEnd: { type: MarkerType.ArrowClosed, color: '#555' },
                    style: { stroke: '#555' }
                });
            }
        }
        return e;
    }, [nodes]);

    return (
        <div style={{ width: '100%', height: '100%', background: 'rgba(0,0,0,0.2)' }}>
            <ReactFlow
                nodes={nodes}
                edges={edges}
                nodeTypes={nodeTypes}
                fitView
            >
                <Background color="#333" gap={20} />
                <Controls />
            </ReactFlow>
        </div>
    );
};
