// Architecture graph for the standalone module panel — styled after the
// Visual Lab JSON visualizer: compact typed cards, left→right hierarchy
// (directory → file), deterministic stacking with measured node heights so
// nodes never overlap. Files collapse by default; expanding re-layouts.

import React, { useMemo, useState, useCallback } from 'react';
import ReactFlow, {
    Background,
    Controls,
    Handle,
    Position,
    type Edge,
    type Node,
    MarkerType,
} from 'reactflow';
import 'reactflow/dist/style.css';
import type { ArchitectureFileNode } from '../../infrastructure/workspaceArchitecture';

const MAX_METHODS_SHOWN = 8;
const COLLAPSED_H = 64;
const ROW_H = 22;
const GAP = 14;
const DIR_W = 220;
const FILE_W = 300;
const FILE_X = DIR_W + 140;

// ── Nodes (JsonNode aesthetic: tiny uppercase type, name, accent border) ──

const DirNode = React.memo(({ data }: any) => (
    <div style={{
        padding: '10px 14px',
        borderRadius: 6,
        background: 'var(--vscode-editor-background, #141414)',
        border: '1px solid rgba(59, 130, 246, 0.5)',
        color: 'var(--vscode-editor-foreground, #fff)',
        width: DIR_W,
        boxShadow: '0 4px 12px rgba(0,0,0,0.4)',
    }}>
        <div style={{ color: '#60a5fa', fontSize: 9, fontWeight: 800, textTransform: 'uppercase', letterSpacing: '0.1em', marginBottom: 4 }}>
            Directory
        </div>
        <div style={{ fontWeight: 600, fontSize: 13, wordBreak: 'break-all' }}>{data.label}</div>
        <div style={{ opacity: 0.5, fontSize: 11, marginTop: 4 }}>{data.fileCount} files</div>
        <Handle type="source" position={Position.Right} style={{ background: '#3b82f6', width: 6, height: 6, border: 'none' }} />
    </div>
));
DirNode.displayName = 'DirNode';

const FileNode = React.memo(({ data }: any) => {
    const expanded: boolean = data.expanded;
    const methods = data.functions as Array<{ name: string; type: string; line: number }>;
    const shown = expanded ? methods.slice(0, MAX_METHODS_SHOWN) : [];
    const hidden = methods.length - shown.length;

    return (
        <div style={{
            borderRadius: 6,
            background: 'var(--vscode-editor-background, #141414)',
            border: '1px solid rgba(167, 139, 250, 0.5)',
            color: 'var(--vscode-editor-foreground, #fff)',
            width: FILE_W,
            boxShadow: '0 4px 12px rgba(0,0,0,0.4)',
            overflow: 'hidden',
        }}>
            <Handle type="target" position={Position.Left} style={{ background: '#7c3aed', width: 6, height: 6, border: 'none' }} />
            <div
                style={{ padding: '10px 14px', cursor: 'pointer' }}
                onClick={() => data.onToggle()}
            >
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    <div style={{ color: '#a78bfa', fontSize: 9, fontWeight: 800, textTransform: 'uppercase', letterSpacing: '0.1em' }}>
                        File
                    </div>
                    <span style={{ fontSize: 10, opacity: 0.6 }}>
                        {methods.length} symbols {expanded ? '▾' : '▸'}
                    </span>
                </div>
                <div style={{ fontWeight: 600, fontSize: 12, marginTop: 4, wordBreak: 'break-all' }}>{data.label}</div>
            </div>
            {expanded && (
                <div style={{ borderTop: '1px solid rgba(255,255,255,0.06)' }}>
                    {shown.map((m, i) => (
                        <div key={i} style={{
                            height: ROW_H,
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'space-between',
                            padding: '0 14px',
                            fontSize: 11,
                            fontFamily: 'var(--font-mono, monospace)',
                            background: i % 2 ? 'transparent' : 'rgba(255,255,255,0.02)',
                        }}>
                            <span style={{ color: '#4ade80', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{m.name}</span>
                            <span style={{ opacity: 0.4, fontSize: 9, textTransform: 'uppercase' }}>{m.type}</span>
                        </div>
                    ))}
                    {hidden > 0 && (
                        <div style={{ height: ROW_H, display: 'flex', alignItems: 'center', padding: '0 14px', fontSize: 10, opacity: 0.5 }}>
                            +{hidden} more…
                        </div>
                    )}
                </div>
            )}
        </div>
    );
});
FileNode.displayName = 'FileNode';

const nodeTypes = { archDir: DirNode, archFile: FileNode };

// ── Layout: measured heights → exact stacking, no overlap ─────────────────

function fileHeight(f: ArchitectureFileNode, expanded: boolean): number {
    if (!expanded) return COLLAPSED_H;
    const rows = Math.min(f.functions.length, MAX_METHODS_SHOWN)
        + (f.functions.length > MAX_METHODS_SHOWN ? 1 : 0);
    return COLLAPSED_H + rows * ROW_H + 8;
}

function dirOf(relPath: string): string {
    const parts = relPath.replace(/\\/g, '/').split('/');
    return parts.length > 1 ? parts.slice(0, Math.min(2, parts.length - 1)).join('/') : '.';
}

export const ArchitectureGraph: React.FC<{ files: ArchitectureFileNode[] }> = ({ files }) => {
    const [expandedIds, setExpandedIds] = useState<Set<number>>(new Set());

    const toggle = useCallback((id: number) => {
        setExpandedIds((prev) => {
            const next = new Set(prev);
            if (next.has(id)) next.delete(id);
            else next.add(id);
            return next;
        });
    }, []);

    const { nodes, edges } = useMemo(() => {
        const groups = new Map<string, ArchitectureFileNode[]>();
        for (const f of files) {
            const dir = dirOf(f.path);
            if (!groups.has(dir)) groups.set(dir, []);
            groups.get(dir)!.push(f);
        }

        const nodes: Node[] = [];
        const edges: Edge[] = [];
        let y = 0;

        for (const [dir, members] of [...groups.entries()].sort(([a], [b]) => a.localeCompare(b))) {
            const blockTop = y;
            for (const f of members) {
                const expanded = expandedIds.has(f.id);
                const fileName = f.path.replace(/\\/g, '/').split('/').pop() ?? f.path;
                nodes.push({
                    id: `file-${f.id}`,
                    type: 'archFile',
                    position: { x: FILE_X, y },
                    data: { label: fileName, functions: f.functions, expanded, onToggle: () => toggle(f.id) },
                });
                edges.push({
                    id: `e-${dir}-${f.id}`,
                    source: `dir-${dir}`,
                    target: `file-${f.id}`,
                    style: { stroke: '#3b82f6', strokeWidth: 1.5, strokeDasharray: '4 4', opacity: 0.5 },
                    markerEnd: { type: MarkerType.ArrowClosed, color: '#3b82f6' },
                });
                y += fileHeight(f, expanded) + GAP;
            }
            const blockH = y - blockTop - GAP;
            nodes.push({
                id: `dir-${dir}`,
                type: 'archDir',
                position: { x: 0, y: blockTop + Math.max(0, blockH / 2 - 45) },
                data: { label: dir, fileCount: members.length },
            });
            y += GAP * 2; // breathing room between directory blocks
        }
        return { nodes, edges };
    }, [files, expandedIds, toggle]);

    return (
        <ReactFlow
            nodes={nodes}
            edges={edges}
            nodeTypes={nodeTypes}
            fitView
            minZoom={0.1}
            proOptions={{ hideAttribution: true }}
        >
            <Background color="#333" gap={20} />
            <Controls />
        </ReactFlow>
    );
};

export default ArchitectureGraph;
