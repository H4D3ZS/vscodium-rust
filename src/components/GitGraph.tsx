import React, { useEffect, useState, useRef, useCallback } from 'react';
import { invoke } from '../tauri_bridge';
import { useStore } from '../store';
import yggdrasilImg from '../assets/yggdrasil.png';
import cherryBlossomImg from '../assets/cherry_blossom.png';

interface GitCommit {
    hash: string;
    author: string;
    date: string;
    message: string;
    parents: string[];
}

// ── Force-directed graph types ──
interface GraphNode {
    id: string;
    commit: GitCommit;
    x: number;
    y: number;
    vx: number;
    vy: number;
    color: string;
    radius: number;
    pinned: boolean;
}

interface GraphEdge {
    source: string;
    target: string;
    color: string;
}

const COLORS = [
    '#22c55e', '#3b82f6', '#f59e0b', '#ec4899', '#8b5cf6',
    '#06b6d4', '#f43f5e', '#84cc16', '#6366f1', '#14b8a6',
    '#e879f9', '#fb923c', '#38bdf8', '#a3e635', '#c084fc'
];

function strHash(s: string): number {
    let h = 0;
    for (let i = 0; i < s.length; i++) h = s.charCodeAt(i) + ((h << 5) - h);
    return Math.abs(h);
}

function initials(name: string): string {
    return name.split(/\s+/).map(w => w[0] || '').join('').toUpperCase().slice(0, 2);
}

function timeAgo(d: string): string {
    try {
        if (!d) return '';
        // git's %ai is "2026-03-24 14:32:01 +0800"
        // Chrome/Safari handles this well, but let's be safe
        const dt = new Date(d.replace(' ', 'T'));
        const diff = Date.now() - dt.getTime();
        if (isNaN(diff)) return '';

        const s = Math.floor(diff / 1000);
        if (s < 60) return `${s}s ago`;
        const m = Math.floor(s / 60);
        if (m < 60) return `${m}m ago`;
        const h = Math.floor(m / 60);
        if (h < 24) return `${h}h ago`;
        const days = Math.floor(h / 24);
        if (days === 0) return `${h}h ago`;
        if (days < 30) return `${days}d ${h % 24}h ago`;
        return `${Math.floor(days / 30)}mo ago`;
    } catch { return ''; }
}

// ── Force simulation ──
function runForceSimulation(
    nodes: GraphNode[],
    edges: GraphEdge[],
    width: number,
    height: number,
    iterations: number = 120,
    style: 'force' | 'maltego' | 'tree' = 'force'
) {
    const nodeMap = new Map(nodes.map(n => [n.id, n]));
    const REPULSION = style === 'maltego' ? 5000 : 3500;
    const ATTRACTION = style === 'maltego' ? 0.015 : 0.008;
    const DAMPING = 0.85;
    const CENTER_GRAVITY = 0.01;
    const cx = width / 2;
    const cy = height / 2;

    for (let iter = 0; iter < iterations; iter++) {
        const temp = 1 - iter / iterations; // cooling

        // Repulsion (all pairs)
        for (let i = 0; i < nodes.length; i++) {
            for (let j = i + 1; j < nodes.length; j++) {
                const a = nodes[i], b = nodes[j];
                let dx = b.x - a.x;
                let dy = b.y - a.y;
                let dist = Math.sqrt(dx * dx + dy * dy) || 1;
                const force = (REPULSION * temp) / (dist * dist);
                const fx = (dx / dist) * force;
                const fy = (dy / dist) * force;
                if (!a.pinned) { a.vx -= fx; a.vy -= fy; }
                if (!b.pinned) { b.vx += fx; b.vy += fy; }
            }
        }

        // Attraction (edges)
        for (const edge of edges) {
            const a = nodeMap.get(edge.source);
            const b = nodeMap.get(edge.target);
            if (!a || !b) continue;
            let dx = b.x - a.x;
            let dy = b.y - a.y;
            let dist = Math.sqrt(dx * dx + dy * dy) || 1;
            const force = dist * ATTRACTION * temp;
            const fx = (dx / dist) * force;
            const fy = (dy / dist) * force;
            if (!a.pinned) { a.vx += fx; a.vy += fy; }
            if (!b.pinned) { b.vx -= fx; b.vy -= fy; }
        }

        // Center gravity
        for (const n of nodes) {
            if (n.pinned) continue;
            n.vx += (cx - n.x) * CENTER_GRAVITY * temp;
            n.vy += (cy - n.y) * CENTER_GRAVITY * temp;
        }

        // Apply velocities
        for (const n of nodes) {
            if (n.pinned) continue;
            n.vx *= DAMPING;
            n.vy *= DAMPING;
            n.x += n.vx;
            n.y += n.vy;
            // Keep in bounds
            n.x = Math.max(40, Math.min(width - 40, n.x));
            n.y = Math.max(40, Math.min(height - 40, n.y));
        }
    }
}

const GitGraph: React.FC = () => {
    const [history, setHistory] = useState<GitCommit[]>([]);
    const [loading, setLoading] = useState(true);
    const [selectedHash, setSelectedHash] = useState<string | null>(null);
    const [hoveredHash, setHoveredHash] = useState<string | null>(null);
    const [dragNode, setDragNode] = useState<string | null>(null);
    const [graphNodes, setGraphNodes] = useState<GraphNode[]>([]);
    const [graphEdges, setGraphEdges] = useState<GraphEdge[]>([]);
    const [pan, setPan] = useState({ x: 0, y: 0 });
    const [zoom, setZoom] = useState(1);
    const [isPanning, setIsPanning] = useState(false);
    const [panStart, setPanStart] = useState({ x: 0, y: 0 });
    const [layoutState, setLayoutState] = useState({ centerX: 0, centerY: 0, radius: 0 });
    const [diffContent, setDiffContent] = useState<string | null>(null);
    const [showDiff, setShowDiff] = useState(false);
    // Remove manual layout state, use adaptive logic
    const svgRef = useRef<SVGSVGElement>(null);
    const containerRef = useRef<HTMLDivElement>(null);
    const activeRoot = useStore(state => state.activeRoot);
    const agentModel = useStore(state => state.agentModel);
    const agentMode = useStore(state => state.agentMode);
    const addAgentMessage = useStore(state => state.addAgentMessage);

    const fetchHistory = useCallback(async () => {
        try {
            setLoading(true);
            const data = await invoke<GitCommit[]>('git_get_history', { path: activeRoot || "." });
            // Support larger history for World Tree demonstration
            setHistory(data?.slice(0, 500) || []);
        } catch (e) {
            console.error("Git history error:", e);
        } finally {
            setLoading(false);
        }
    }, [activeRoot]);

    useEffect(() => { fetchHistory(); }, [fetchHistory]);

    // Build graph when history or layout changes
    useEffect(() => {
        if (history.length === 0) return;
        const rect = containerRef.current?.getBoundingClientRect();
        const w = rect?.width || 500;
        const h = rect?.height || 600;

        // Create nodes
        let nodes: GraphNode[] = history.map((commit, i) => {
            const authorColor = COLORS[strHash(commit.author) % COLORS.length];
            return {
                id: commit.hash,
                commit,
                x: w / 2, y: h / 2,
                vx: 0, vy: 0,
                color: authorColor,
                radius: commit.parents.length > 1 ? 18 : 14,
                pinned: false
            };
        });

        // Create edges
        const edges: GraphEdge[] = [];
        for (const node of nodes) {
            for (const parentHash of node.commit.parents) {
                const parent = nodes.find(n =>
                    n.id.startsWith(parentHash) || parentHash.startsWith(n.id.substring(0, 7))
                );
                if (parent) {
                    edges.push({
                        source: node.id,
                        target: parent.id,
                        color: node.color
                    });
                }
            }
        }

        // ── Phase 27: Asset-Backed Yggdrasil & Cherry Blossom Engine ──
        const isWorld = nodes.length > 50;
        const cx = w / 2;
        const cy = h / 2;
        const r_base = Math.min(w, h) / 1.7; // Taller tree for immersive view
        setLayoutState({ centerX: cx, centerY: cy, radius: r_base });

        const childMap = new Map<string, string[]>();
        edges.forEach(e => {
            const children = childMap.get(e.target) || [];
            children.push(e.source);
            childMap.set(e.target, children);
        });

        const positions = new Map<string, { x: number, y: number, color: string, r: number }>();
        const assignPos = (id: string, startAngle: number, endAngle: number, depth: number) => {
            const children = childMap.get(id) || [];
            const midAngle = (startAngle + endAngle) / 2;

            // Phase 31: Curvilinear "Poof" Canopy Silhouette
            const maxExpectedDepth = 25;
            const progress = Math.min(1, depth / maxExpectedDepth);

            // Non-linear radius to form a rounded "Mushroom" canopy
            const poofFactor = Math.sin(progress * Math.PI / 1.1); // Rounds at the top
            const baseR = isWorld ? 110 : 80;
            const r = (depth === 0) ? 0 : baseR + (depth * 45 * poofFactor);

            const spreadFactor = 1.2 + (depth * 0.3 * (1 - progress)); // Widen early, taper late
            const spiralOffset = Math.sin(depth * 1.9) * 0.4 * (1 - progress); // More stable at top
            const finalAngle = midAngle + spiralOffset;

            // Phase 31: Canopy-Bounded Jitter
            const anatomicalJitter = Math.sin(depth * 1.6) * (depth * 12) * (1 - progress);
            const x = cx + Math.cos(finalAngle - Math.PI / 2) * r + anatomicalJitter;
            const y = (cy + (isWorld ? 460 : 430)) + Math.sin(finalAngle - Math.PI / 2) * r * 0.85; // Slightly flattened y

            const color = isWorld ? (depth < 5 ? '#f472b6' : '#fbcfe8') : (depth < 5 ? '#db2777' : '#fbcfe8');
            positions.set(id, { x, y, color, r: isWorld ? Math.max(8, 20 - depth) : Math.max(9, 22 - depth) });

            if (children.length === 0) return;
            const sector = (endAngle - startAngle) / children.length;
            children.forEach((childId, idx) => {
                // Wide sectoring for anatomical filling
                const nextStart = finalAngle - (spreadFactor / 2) + (idx * spreadFactor / children.length);
                const nextEnd = finalAngle - (spreadFactor / 2) + ((idx + 1) * spreadFactor / children.length);
                assignPos(childId, nextStart, nextEnd, depth + 1);
            });
        };

        const rootNode = nodes[nodes.length - 1];
        if (rootNode) assignPos(rootNode.id, -Math.PI / 6, Math.PI / 6, 0); // Vertical trunk start

        nodes = nodes.map(n => {
            const pos = positions.get(n.id);
            if (!pos) return { ...n, x: layoutState.centerX, y: layoutState.centerY, pinned: true, color: '#fbbf24', radius: 6 };
            return { ...n, x: pos.x, y: pos.y, pinned: true, color: pos.color, radius: pos.r };
        });

        setGraphNodes(nodes);
        setGraphEdges(edges);
        setLoading(false);
        setPan({ x: 0, y: 0 });
        setZoom(nodes.length > 200 ? 0.35 : 0.85); // Show entire tree canopy initially
    }, [history]);

    // ── Drag handling ──
    const handleMouseDown = (e: React.MouseEvent, nodeId: string) => {
        e.stopPropagation();
        setDragNode(nodeId);
        setSelectedHash(nodeId);
    };

    const handleSvgMouseDown = (e: React.MouseEvent) => {
        if (dragNode) return;
        setIsPanning(true);
        setPanStart({ x: e.clientX - pan.x, y: e.clientY - pan.y });
    };

    const handleMouseMove = useCallback((e: React.MouseEvent) => {
        if (dragNode) {
            const svg = svgRef.current;
            if (!svg) return;
            const rect = svg.getBoundingClientRect();
            const x = (e.clientX - rect.left - pan.x) / zoom;
            const y = (e.clientY - rect.top - pan.y) / zoom;
            setGraphNodes(prev => prev.map(n =>
                n.id === dragNode ? { ...n, x, y, pinned: true } : n
            ));
        } else if (isPanning) {
            setPan({
                x: e.clientX - panStart.x,
                y: e.clientY - panStart.y
            });
        }
    }, [dragNode, isPanning, pan, zoom, panStart]);

    const handleMouseUp = useCallback(() => {
        setDragNode(null);
        setIsPanning(false);
    }, []);

    const handleWheel = useCallback((e: React.WheelEvent) => {
        e.preventDefault();
        const delta = e.deltaY > 0 ? 0.92 : 1.08;
        setZoom(z => Math.max(0.3, Math.min(3, z * delta)));
    }, []);

    const selectedNode = graphNodes.find(n => n.id === selectedHash);
    const nodeMap = new Map(graphNodes.map(n => [n.id, n]));

    if (loading) return (
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100%', gap: 10, fontSize: 12, opacity: 0.6 }}>
            <div style={{ width: 16, height: 16, border: '2px solid var(--vscode-focusBorder)', borderTop: '2px solid transparent', borderRadius: '50%', animation: 'spin 1s linear infinite' }} />
            Building graph…
        </div>
    );

    if (history.length === 0) return (
        <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', height: '100%', opacity: 0.4 }}>
            <i className="codicon codicon-git-commit" style={{ fontSize: 36, marginBottom: 10 }} />
            <div style={{ fontSize: 12 }}>No commits found</div>
        </div>
    );

    return (
        <div ref={containerRef} style={{
            display: 'flex', flexDirection: 'column', height: '100%',
            background: 'var(--vscode-editor-background)', position: 'relative', overflow: 'hidden'
        }}>
            {/* Adaptive layout indicator */}
            <div style={{
                position: 'absolute', top: 8, left: 8, zIndex: 10,
                display: 'flex', gap: 6, background: 'rgba(0,0,0,0.5)',
                border: '1px solid rgba(255,255,255,0.1)', color: 'rgba(255,255,255,0.8)',
                fontSize: 10, fontWeight: 700, letterSpacing: '1px', textTransform: 'uppercase',
                boxShadow: `0 0 15px ${graphNodes.length > 200 ? 'rgba(251, 191, 36, 0.2)' : 'rgba(244, 114, 182, 0.2)'}`
            }}>
                <i className={`codicon codicon-${graphNodes.length > 200 ? 'hubot' : 'sparkle'}`}
                    style={{ color: graphNodes.length > 200 ? '#fbbf24' : '#f472b6', marginRight: 4 }}></i>
                {graphNodes.length > 200 ? 'Yggdrasil World Tree' : 'Cherry Blossom Binary Tree'}
            </div>

            <div style={{
                position: 'absolute', top: 8, right: 8, zIndex: 10,
                display: 'flex', gap: 4, background: 'rgba(0,0,0,0.5)',
                padding: '4px 6px', borderRadius: 8, backdropFilter: 'blur(10px)',
                border: '1px solid rgba(255,255,255,0.1)'
            }}>
                <button onClick={() => setZoom(z => Math.min(3, z * 1.2))}
                    title="Zoom In"
                    style={{ background: 'rgba(255,255,255,0.1)', border: 'none', color: 'var(--vscode-editor-foreground, #fff)', width: 24, height: 24, borderRadius: 4, cursor: 'pointer', fontSize: 14 }}>+</button>
                <button onClick={() => setZoom(z => Math.max(0.3, z * 0.8))}
                    title="Zoom Out"
                    style={{ background: 'rgba(255,255,255,0.1)', border: 'none', color: 'var(--vscode-editor-foreground, #fff)', width: 24, height: 24, borderRadius: 4, cursor: 'pointer', fontSize: 14 }}>−</button>
                <button onClick={() => { setZoom(1); setPan({ x: 0, y: 0 }); }}
                    title="Reset View"
                    style={{ background: 'rgba(255,255,255,0.1)', border: 'none', color: 'var(--vscode-editor-foreground, #fff)', width: 24, height: 24, borderRadius: 4, cursor: 'pointer', fontSize: 11 }}>⟲</button>
            </div>

            {/* ── SVG Canvas ── */}
            <svg ref={svgRef}
                style={{ flex: 1, cursor: isPanning ? 'grabbing' : dragNode ? 'grabbing' : 'grab', width: '100%' }}
                onMouseDown={handleSvgMouseDown}
                onMouseMove={handleMouseMove}
                onMouseUp={handleMouseUp}
                onMouseLeave={handleMouseUp}
                onWheel={handleWheel}
            >
                {/* Background grid */}
                <defs>
                    <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
                        <circle cx="20" cy="20" r="0.5" fill="var(--vscode-editorGroup-border, rgba(255,255,255,0.06))" />
                    </pattern>
                    <filter id="glow" x="-20%" y="-20%" width="140%" height="140%">
                        <feGaussianBlur stdDeviation="3" result="blur" />
                        <feComposite in="SourceGraphic" in2="blur" operator="over" />
                    </filter>
                    <filter id="trunkShadow" x="-10%" y="-10%" width="120%" height="120%">
                        <feDropShadow dx="0" dy="2" stdDeviation="3" floodColor="#000" floodOpacity="0.5" />
                    </filter>
                    <linearGradient id="trunkGradient" x1="0%" y1="0%" x2="0%" y2="100%">
                        <stop offset="0%" stopColor="#d97706" />
                        <stop offset="100%" stopColor="#78350f" />
                    </linearGradient>
                    <marker id="arrowhead" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="4" markerHeight="4" orient="auto">
                        <polygon points="0 0, 8 3, 0 6" fill="rgba(255,255,255,0.2)" />
                    </marker>
                </defs>

                <g transform={`translate(${pan.x}, ${pan.y}) scale(${zoom})`}>
                    <image
                        href={graphNodes.length > 50 ? yggdrasilImg : cherryBlossomImg}
                        x={layoutState.centerX - layoutState.radius * 2}
                        y={layoutState.centerY - layoutState.radius * 2}
                        width={layoutState.radius * 4}
                        height={layoutState.radius * 4}
                        style={{ opacity: 0.25, pointerEvents: 'none', filter: 'brightness(1.3) contrast(1.2)' }}
                    />

                    {/* ── Edges (Structural Bark) ── */}
                    {graphEdges.map((edge, i) => {
                        const source = nodeMap.get(edge.source);
                        const target = nodeMap.get(edge.target);
                        if (!source || !target) return null;

                        const sx = source.x;
                        const sy = source.y;
                        const tx = target.x;
                        const ty = target.y;

                        const isHighlighted = selectedHash === edge.source || selectedHash === edge.target;

                        // Phase 30: Deep Bark Texture logic
                        const sourceIdx = graphNodes.indexOf(source!);
                        const depthFactor = Math.max(0, 1 - (sourceIdx / 35));
                        const woodenWidth = 1.3 + (depthFactor * 5); // Realistic tapering

                        // High-Curvature Bio-mimetic Bezier
                        const midX = (sx + tx) / 2 + (Math.sin(sourceIdx * 1.4) * 55 * (1 - depthFactor));
                        const midY = (sy + ty) / 2 + (Math.cos(sourceIdx * 1.4) * 25);
                        const d = `M${sx},${sy} Q${midX},${midY} ${tx},${ty}`;

                        return (
                            <g key={i}>
                                {/* Bark shadow/glow */}
                                <path
                                    d={d}
                                    stroke="rgba(0,0,0,0.4)"
                                    strokeWidth={woodenWidth + 1}
                                    fill="none"
                                />
                                <path
                                    d={d}
                                    stroke="#2b1810" // Deep Dark Brown bark
                                    strokeWidth={isHighlighted ? woodenWidth + 2.5 : woodenWidth}
                                    opacity={isHighlighted ? 1 : 0.7}
                                    fill="none"
                                    strokeLinecap="round"
                                    style={{ transition: 'all 0.5s' }}
                                />
                            </g>
                        );
                    })}

                    {/* ── Nodes ── */}
                    {graphNodes.map(node => {
                        const isSelected = selectedHash === node.id;
                        const isHovered = hoveredHash === node.id;
                        const isConnected = graphEdges.some(e =>
                            (e.source === selectedHash && e.target === node.id) ||
                            (e.target === selectedHash && e.source === node.id)
                        );
                        const r = node.radius;
                        const active = isSelected || isHovered || isConnected;

                        return (
                            <g key={node.id}
                                onMouseDown={(e) => handleMouseDown(e, node.id)}
                                onMouseEnter={() => setHoveredHash(node.id)}
                                onMouseLeave={() => setHoveredHash(null)}
                                style={{ cursor: 'pointer', transition: 'all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275)' }}
                            >
                                {/* Outer glow */}
                                {isSelected && (
                                    <circle cx={node.x} cy={node.y} r={r + 10}
                                        fill="none" stroke={node.color} strokeWidth="1" opacity="0.3" />
                                )}

                                {/* Phase 31: High-Density Blossom Cluster (5 Petals) */}
                                <g transform={`translate(${node.x}, ${node.y}) scale(${r / 10 * (isSelected ? 1.7 : 1)})`}
                                    style={{ transition: 'transform 0.6s cubic-bezier(0.175, 0.885, 0.32, 1.275)' }}>
                                    {/* Petal 1 (Core) */}
                                    <path d="M0,-8 C4,-12 10,-8 10,0 C10,8 4,12 0,8 C-4,12 -10,8 -10,0 C-10,-8 -4,-12 0,-8 Z"
                                        fill={isSelected ? '#fff' : node.color}
                                        filter={active ? "url(#glow)" : ""}
                                        opacity={active ? 1 : 0.9} />
                                    {/* Petal 2-5 (Cluster) */}
                                    <path d="M0,-8 C4,-12 10,-8 10,0 C10,8 4,12 0,8 C-4,12 -10,8 -10,0 C-10,-8 -4,-12 0,-8 Z"
                                        transform="rotate(72) scale(0.8) translate(4, 2)"
                                        fill={node.color} opacity="0.7" />
                                    <path d="M0,-8 C4,-12 10,-8 10,0 C10,8 4,12 0,8 C-4,12 -10,8 -10,0 C-10,-8 -4,-12 0,-8 Z"
                                        transform="rotate(144) scale(0.7) translate(-3, 5)"
                                        fill="#fbcfe8" opacity="0.6" />
                                    <path d="M0,-8 C4,-12 10,-8 10,0 C10,8 4,12 0,8 C-4,12 -10,8 -10,0 C-10,-8 -4,-12 0,-8 Z"
                                        transform="rotate(216) scale(0.9) translate(-5, -2)"
                                        fill="#f9a8d4" opacity="0.5" />
                                    <path d="M0,-8 C4,-12 10,-8 10,0 C10,8 4,12 0,8 C-4,12 -10,8 -10,0 C-10,-8 -4,-12 0,-8 Z"
                                        transform="rotate(288) scale(0.85) translate(2, -6)"
                                        fill="#fff1f2" opacity="0.8" />
                                </g>

                                {/* Author initials */}
                                <text x={node.x} y={node.y + 1}
                                    textAnchor="middle" dominantBaseline="middle"
                                    fontSize={r > 16 ? 10 : 8} fontWeight="700"
                                    fill={isSelected ? '#fff' : node.color}
                                    opacity={active || !selectedHash ? 1 : 0.4}
                                    style={{ pointerEvents: 'none', userSelect: 'none', fontFamily: '-apple-system, system-ui, sans-serif' }}
                                >
                                    {initials(node.commit.author)}
                                </text>

                                {/* Hash label */}
                                {(active || zoom > 1.2) && (
                                    <text x={node.x} y={node.y + r + 12}
                                        textAnchor="middle" fontSize="9"
                                        fill={node.color} opacity="0.7"
                                        style={{ pointerEvents: 'none', fontFamily: 'monospace' }}
                                    >
                                        {node.commit.hash.substring(0, 7)}
                                    </text>
                                )}

                                {/* Commit message tooltip on hover */}
                                {isHovered && !isSelected && (
                                    <g>
                                        <rect x={node.x + r + 8} y={node.y - 20}
                                            width={Math.min(node.commit.message.split('\n')[0].length * 6.5 + 16, 220)}
                                            height={36} rx={6}
                                            fill="rgba(0,0,0,0.85)" stroke={node.color} strokeWidth="1"
                                        />
                                        <text x={node.x + r + 16} y={node.y - 5}
                                            fontSize="10" fill="#fff"
                                            style={{ pointerEvents: 'none', fontFamily: '-apple-system, system-ui, sans-serif' }}
                                        >
                                            {node.commit.message.split('\n')[0].substring(0, 32)}
                                            {node.commit.message.split('\n')[0].length > 32 ? '…' : ''}
                                        </text>
                                        <text x={node.x + r + 16} y={node.y + 8}
                                            fontSize="9" fill="rgba(255,255,255,0.4)"
                                            style={{ pointerEvents: 'none', fontFamily: 'monospace' }}
                                        >
                                            {node.commit.author.split(' ')[0]} · {timeAgo(node.commit.date)}
                                        </text>
                                    </g>
                                )}

                                {/* Merge badge */}
                                {node.commit.parents.length > 1 && (
                                    <g>
                                        <circle cx={node.x + r - 2} cy={node.y - r + 2}
                                            r={5} fill="#a78bfa" stroke="#1a1a2e" strokeWidth="1.5" />
                                        <text x={node.x + r - 2} y={node.y - r + 3}
                                            textAnchor="middle" dominantBaseline="middle"
                                            fontSize="7" fill="#fff" fontWeight="700"
                                            style={{ pointerEvents: 'none' }}
                                        >M</text>
                                    </g>
                                )}
                            </g>
                        );
                    })}
                </g>
            </svg>

            {/* ── Selected node detail panel ── */}
            {selectedNode && (
                <div style={{
                    position: 'absolute', bottom: 0, left: 0, right: 0,
                    background: 'var(--vscode-sideBar-background)',
                    backdropFilter: 'blur(12px)',
                    borderTop: `2px solid ${selectedNode.color}`,
                    padding: '12px 14px',
                    maxHeight: '40%',
                    overflowY: 'auto',
                    zIndex: 20,
                    animation: 'slideUp 0.2s ease-out'
                }}>
                    <style>{`@keyframes slideUp { from { transform: translateY(20px); opacity: 0; } to { transform: translateY(0); opacity: 1; } }`}</style>

                    {/* Header */}
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 10 }}>
                        <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
                            <div style={{
                                width: 32, height: 32, borderRadius: '50%',
                                background: `linear-gradient(135deg, ${selectedNode.color}, ${selectedNode.color}80)`,
                                display: 'flex', alignItems: 'center', justifyContent: 'center',
                                fontSize: 12, fontWeight: 700, color: 'var(--vscode-editor-foreground, #fff)',
                                boxShadow: `0 0 12px ${selectedNode.color}40`
                            }}>
                                {initials(selectedNode.commit.author)}
                            </div>
                            <div>
                                <div style={{ fontSize: 13, fontWeight: 600, color: 'var(--vscode-editor-foreground, #fff)' }}>{selectedNode.commit.author}</div>
                                <div style={{ fontSize: 10, opacity: 0.4 }}>{timeAgo(selectedNode.commit.date)}</div>
                            </div>
                        </div>
                        <div style={{ display: 'flex', gap: 6, alignItems: 'center' }}>
                            <code style={{
                                background: `${selectedNode.color}25`, color: selectedNode.color,
                                padding: '3px 10px', borderRadius: 12,
                                fontSize: 10, fontWeight: 700, fontFamily: 'monospace',
                                border: `1px solid ${selectedNode.color}30`
                            }}>
                                {selectedNode.commit.hash.substring(0, 12)}
                            </code>
                            <button onClick={() => setSelectedHash(null)}
                                style={{
                                    background: 'rgba(255,255,255,0.1)', border: 'none', color: 'var(--vscode-editor-foreground, #fff)',
                                    width: 22, height: 22, borderRadius: 4, cursor: 'pointer', fontSize: 12
                                }}>×</button>
                        </div>
                    </div>

                    {/* Message */}
                    <div style={{
                        fontSize: 12, lineHeight: 1.6, color: 'rgba(255,255,255,0.85)',
                        whiteSpace: 'pre-wrap', wordBreak: 'break-word',
                        padding: '10px 12px',
                        background: 'rgba(255,255,255,0.03)',
                        borderRadius: 6,
                        borderLeft: `3px solid ${selectedNode.color}`
                    }}>
                        {selectedNode.commit.message}
                    </div>

                    {/* Actions Toolbar */}
                    <div style={{ display: 'flex', gap: 8, marginTop: 14 }}>
                        <button
                            onClick={async () => {
                                if (confirm("Revert this commit?")) {
                                    try {
                                        await invoke('git_revert', { hash: selectedNode.id });
                                        fetchHistory();
                                    } catch (e) {
                                        alert(e);
                                    }
                                }
                            }}
                            className="scm-action-btn"
                            style={{
                                background: 'rgba(244, 63, 94, 0.2)', color: '#fb7185',
                                border: '1px solid rgba(244, 63, 94, 0.4)',
                                padding: '6px 12px', borderRadius: 4, cursor: 'pointer',
                                fontSize: 11, display: 'flex', alignItems: 'center', gap: 6
                            }}>
                            <i className="codicon codicon-history"></i> Revert
                        </button>
                        <button
                            onClick={async () => {
                                try {
                                    const commitMsg = selectedNode.commit.message;
                                    const hash = selectedNode.id;
                                    const provider = agentModel.includes(':') ? agentModel.split(':')[0] : 'anthropic';
                                    const model = agentModel.includes(':') ? agentModel.split(':')[1] : agentModel;

                                    addAgentMessage('user', `Please analyze this commit: ${hash} - ${commitMsg}`);

                                    await invoke('ai_chat', {
                                        request: {
                                            provider,
                                            model,
                                            messages: [
                                                {
                                                    role: 'user',
                                                    content: `Explain this commit in depth, analyzing the changes and suggesting potential improvements: \n\nHash: ${hash}\nMessage: ${commitMsg}\n\nPlease use your tools to fetch the diff if needed.`
                                                }
                                            ],
                                            autonomous: true,
                                            mode: agentMode
                                        }
                                    });
                                } catch (e) {
                                    alert(`AI Explain failed: ${e}`);
                                }
                            }}
                            className="scm-action-btn"
                            style={{
                                background: 'rgba(59, 130, 246, 0.2)', color: '#60a5fa',
                                border: '1px solid rgba(59, 130, 246, 0.4)',
                                padding: '6px 12px', borderRadius: 4, cursor: 'pointer',
                                fontSize: 11, display: 'flex', alignItems: 'center', gap: 6
                            }}>
                            <i className="codicon codicon-sparkle"></i> AI Explain
                        </button>
                        <button
                            title="Show full diff"
                            onClick={async () => {
                                try {
                                    const diff = await invoke<string>('git_diff', {
                                        path: activeRoot || '.',
                                        hash: selectedNode.id
                                    });
                                    setDiffContent(diff);
                                    setShowDiff(true);
                                } catch (e) {
                                    alert(`Failed to fetch diff: ${e}`);
                                }
                            }}
                            className="scm-action-btn"
                            style={{
                                background: 'rgba(255, 255, 255, 0.05)', color: 'rgba(255,255,255,0.7)',
                                border: '1px solid rgba(255,255,255,0.1)',
                                padding: '6px 12px', borderRadius: 4, cursor: 'pointer',
                                fontSize: 11, display: 'flex', alignItems: 'center', gap: 6
                            }}>
                            <i className="codicon codicon-diff"></i> View Diff
                        </button>
                    </div>

                    {/* Parents */}
                    {selectedNode.commit.parents.length > 0 && (
                        <div style={{ marginTop: 8, display: 'flex', flexWrap: 'wrap', gap: 6, alignItems: 'center', fontSize: 10, opacity: 0.4 }}>
                            <span>{selectedNode.commit.parents.length > 1 ? '⤴ Merge →' : 'Parent →'}</span>
                            {selectedNode.commit.parents.map((p, i) => (
                                <code key={i}
                                    onClick={() => setSelectedHash(p)}
                                    style={{
                                        background: 'rgba(255,255,255,0.06)', padding: '2px 8px',
                                        borderRadius: 4, cursor: 'pointer', fontFamily: 'monospace'
                                    }}
                                >{p.substring(0, 7)}</code>
                            ))}
                        </div>
                    )}
                </div>
            )}
            {/* Diff Modal Overlay */}
            {showDiff && diffContent !== null && (
                <div style={{
                    position: 'absolute', top: 0, left: 0, right: 0, bottom: 0,
                    background: 'rgba(0,0,0,0.8)', backdropFilter: 'blur(8px)',
                    zIndex: 100, display: 'flex', flexDirection: 'column',
                    animation: 'fadeIn 0.2s ease-out'
                }}>
                    <style>{`@keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }`}</style>
                    <div style={{
                        display: 'flex', justifyContent: 'space-between', alignItems: 'center',
                        padding: '12px 16px', background: 'var(--vscode-sideBar-background)',
                        borderBottom: '1px solid rgba(255,255,255,0.1)'
                    }}>
                        <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
                            <i className="codicon codicon-diff" style={{ color: '#60a5fa' }}></i>
                            <span style={{ fontSize: 13, fontWeight: 600 }}>Commit Diff: {selectedNode?.id.substring(0, 8)}</span>
                        </div>
                        <button
                            onClick={() => setShowDiff(false)}
                            style={{
                                background: 'rgba(255,255,255,0.1)', border: 'none', color: 'var(--vscode-editor-foreground, #fff)',
                                padding: '4px 12px', borderRadius: 4, cursor: 'pointer', fontSize: 11
                            }}>Close</button>
                    </div>
                    <div style={{ flex: 1, overflow: 'auto', padding: 16 }}>
                        <pre style={{
                            margin: 0, fontSize: 11, fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace',
                            lineHeight: 1.5, color: '#d1d5db', whiteSpace: 'pre-wrap'
                        }}>
                            {diffContent.split('\n').map((line, i) => {
                                let color = '#d1d5db';
                                if (line.startsWith('+')) color = '#4ade80';
                                else if (line.startsWith('-')) color = '#f87171';
                                else if (line.startsWith('@@')) color = '#818cf8';
                                return <div key={i} style={{ color }}>{line}</div>;
                            })}
                        </pre>
                    </div>
                </div>
            )}
        </div>
    );
};

export default GitGraph;
