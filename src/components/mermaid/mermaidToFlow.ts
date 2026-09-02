import type { Node, Edge } from 'reactflow';
import { MarkerType } from 'reactflow';

// ─────────────────────────────────────────────────────────────────────────────
//  mermaidToFlow — convert a Mermaid-rendered flowchart SVG into reactflow
//  nodes + edges. Mermaid does the hard part (parse + dagre layout); we scrape
//  the laid-out SVG and rebuild it as native vector nodes so it renders like
//  Lucidchart (discrete, styled, selectable boxes) while keeping Mermaid as the
//  authoring syntax. View-only — nodes are not draggable.
//
//  Returns null when the SVG isn't a node/edge graph (sequence/ER/gantt/pie) or
//  scraping finds nothing — the caller then falls back to the raw SVG render.
// ─────────────────────────────────────────────────────────────────────────────

export interface FlowNodeData {
    label: string;
    fill: string;
    stroke: string;
    w: number;
    h: number;
    shape: 'rect' | 'diamond' | 'round' | 'data';
    isGroup?: boolean;
}

export interface FlowResult {
    nodes: Node<FlowNodeData>[];
    edges: Edge[];
}

function nodeOrigId(domId: string): string {
    // mermaid v11 node id: "flowchart-<origId>-<counter>"
    return domId.replace(/^flowchart-/, '').replace(/-\d+$/, '');
}

function shapeOf(el: Element): FlowNodeData['shape'] {
    if (el.querySelector('polygon')) return 'diamond';
    if (el.querySelector('circle, ellipse')) return 'round';
    // cylinders / parallelograms render as <path>
    if (el.querySelector('path') && !el.querySelector('rect')) return 'data';
    return 'rect';
}

function readColor(el: Element): { fill: string; stroke: string } {
    const shape = el.querySelector('rect, polygon, circle, ellipse, path');
    if (!shape) return { fill: '', stroke: '' };
    const style = (shape as SVGElement).getAttribute('style') || '';
    const fillM = style.match(/fill:\s*([^;]+)/i);
    const strokeM = style.match(/stroke:\s*([^;]+)/i);
    const fill = fillM?.[1]?.trim() || shape.getAttribute('fill') || '';
    const stroke = strokeM?.[1]?.trim() || shape.getAttribute('stroke') || '';
    const bad = (c: string) => !c || c === 'none' || c === 'transparent';
    return { fill: bad(fill) ? '' : fill, stroke: bad(stroke) ? '' : stroke };
}

/** Parse a Mermaid flowchart SVG string into reactflow nodes/edges, or null. */
export function mermaidToFlow(svg: string): FlowResult | null {
    if (!svg) return null;
    const host = document.createElement('div');
    host.style.cssText = 'position:absolute;left:-99999px;top:-99999px;visibility:hidden;width:4000px;height:4000px;';
    host.innerHTML = svg;
    document.body.appendChild(host);
    try {
        const svgEl = host.querySelector('svg') as SVGSVGElement | null;
        if (!svgEl) return null;
        const nodeEls = Array.from(svgEl.querySelectorAll('g.node')) as SVGGElement[];
        if (nodeEls.length === 0) return null;

        const nodes: Node<FlowNodeData>[] = [];
        const origToRf = new Map<string, string>();
        const centers = new Map<string, { x: number; y: number }>();

        // First pass: collect all centers to compute bounding box
        const allCenters: { x: number; y: number }[] = [];
        for (const el of nodeEls) {
            const tr = el.getAttribute('transform') || '';
            const m = tr.match(/translate\(\s*([-\d.]+)[ ,]+([-\d.]+)\s*\)/);
            if (m) allCenters.push({ x: +m[1], y: +m[2] });
        }
        if (allCenters.length === 0) return null;

        // Compute bounding box and scale factor
        const minX = Math.min(...allCenters.map(c => c.x));
        const minY = Math.min(...allCenters.map(c => c.y));
        const maxX = Math.max(...allCenters.map(c => c.x));
        const maxY = Math.max(...allCenters.map(c => c.y));
        const rangeX = maxX - minX || 1;
        const rangeY = maxY - minY || 1;

                // Scale to fit within a reasonable reactflow viewport (800x600 base)
        const targetW = 800;
        const targetH = 600;
        const scale = 1; // Keep 1:1 scale to prevent text wrapping and sizing bugs
        const padX = 0;
        const padY = 0;

        // Subgraph clusters first
        const clusterEls = Array.from(svgEl.querySelectorAll('g.cluster')) as SVGGElement[];
        clusterEls.forEach((el, i) => {
            let bbox: DOMRect;
            try { bbox = el.getBBox(); } catch { return; }
            const label = (el.querySelector('.cluster-label, .nodeLabel') as Element | null)?.textContent?.trim() || '';
            nodes.push({
                id: `cluster-${i}`,
                type: 'mmGroup',
                position: { x: (bbox.x - minX) * scale + padX, y: (bbox.y - minY) * scale + padY },
                data: { label, fill: '', stroke: '', w: bbox.width * scale, h: bbox.height * scale, shape: 'rect', isGroup: true },
                draggable: false, selectable: false, focusable: false,
                zIndex: 0,
                style: { width: bbox.width * scale, height: bbox.height * scale },
            });
        });

        for (const el of nodeEls) {
            const tr = el.getAttribute('transform') || '';
            const m = tr.match(/translate\(\s*([-\d.]+)[ ,]+([-\d.]+)\s*\)/);
            const cx = m ? +m[1] : 0;
            const cy = m ? +m[2] : 0;
            let bbox: DOMRect;
            try { bbox = el.getBBox(); } catch { continue; }
            const w = Math.max(40, bbox.width);
            const h = Math.max(28, bbox.height);
            const label = (el.querySelector('.nodeLabel, .label, foreignObject') as Element | null)?.textContent?.trim() || '';
            const { fill, stroke } = readColor(el);
            const rfId = el.id || `n-${nodes.length}`;
            origToRf.set(nodeOrigId(el.id || rfId), rfId);

            // Scale coordinates to reactflow viewport
            const scaledX = (cx - minX) * scale + padX;
            const scaledY = (cy - minY) * scale + padY;
            centers.set(rfId, { x: scaledX, y: scaledY });

            nodes.push({
                id: rfId,
                type: 'mmNode',
                position: { x: scaledX - (w * scale) / 2, y: scaledY - (h * scale) / 2 },
                data: { label, fill, stroke, w: w * scale, h: h * scale, shape: shapeOf(el) },
                draggable: false, selectable: true,
                zIndex: 1,
                style: { width: w * scale, height: h * scale },
            });
        }

        // Edges — source/dest come from the reliable LS-/LE- classes mermaid adds.
        const edgeEls = Array.from(
            svgEl.querySelectorAll('g.edgePaths path, .edgePaths path, path.flowchart-link'),
        ) as SVGPathElement[];
        const labelEls = Array.from(svgEl.querySelectorAll('g.edgeLabels .edgeLabel')) as Element[];
        const edges: Edge[] = [];
        const seen = new Set<string>();
        edgeEls.forEach((p, i) => {
            const cls = p.getAttribute('class') || '';
            const ls = cls.match(/\bLS-([^\s]+)/);
            const le = cls.match(/\bLE-([^\s]+)/);
            const src = ls ? origToRf.get(ls[1]) : undefined;
            const dst = le ? origToRf.get(le[1]) : undefined;
            if (!src || !dst) return;
            const id = `${p.id || `e-${i}`}`;
            if (seen.has(id)) return;
            seen.add(id);
            const label = labelEls[i]?.textContent?.trim() || '';
            // Pick handle sides from geometry so smoothstep follows the TD/LR flow.
            const a = centers.get(src), b = centers.get(dst);
            let sourceHandle = 's-bottom', targetHandle = 't-top';
            if (a && b) {
                const dx = b.x - a.x, dy = b.y - a.y;
                if (Math.abs(dy) >= Math.abs(dx)) {
                    if (dy >= 0) { sourceHandle = 's-bottom'; targetHandle = 't-top'; }
                    else { sourceHandle = 's-top'; targetHandle = 't-bottom'; }
                } else {
                    if (dx >= 0) { sourceHandle = 's-right'; targetHandle = 't-left'; }
                    else { sourceHandle = 's-left'; targetHandle = 't-right'; }
                }
            }
            edges.push({
                id,
                source: src,
                target: dst,
                sourceHandle,
                targetHandle,
                label: label || undefined,
                type: 'smoothstep',
                markerEnd: { type: MarkerType.ArrowClosed, color: '#94a3b8', width: 20, height: 20 },
                style: { stroke: '#64748b', strokeWidth: 2 },
                labelStyle: { fill: '#e2e8f0', fontSize: 12, fontWeight: 500 },
                labelBgStyle: { fill: '#0f172a', fillOpacity: 0.9, rx: 6 },
                labelBgPadding: [8, 4],
            });
        });

        return { nodes, edges };
    } catch {
        return null;
    } finally {
        document.body.removeChild(host);
    }
}
