import React, { useEffect, useMemo, useState, useCallback, useRef } from 'react';
import { Tabs, Tab, Button, Tooltip } from '@heroui/react';
import {
    IconHierarchy, IconCode, IconCopy, IconDownload, IconLayoutSidebar,
    IconFileExport, IconAlertTriangle, IconZoomIn, IconZoomOut, IconFocusCentered,
} from '@tabler/icons-react';
import { parseDiagramTabContent, repairMermaid, aggressiveRepairMermaid, type MermaidDiagram } from '../application/editor/openMermaid';
import { sanitizeHtml } from '../lib/markdown';

// ─────────────────────────────────────────────────────────────────────────────
//  MermaidView — Antigravity-style diagram canvas. Mermaid does the parse +
//  dagre layout + render (the exact engine Lucidchart-style tools and the Mermaid
//  live editor use), producing a faithful SVG. We render that SVG directly into a
//  pannable/zoomable canvas (drag to pan, ctrl/⌘+wheel or buttons to zoom). No
//  reactflow node-wrapping and no SVG scraping — those mangled dense diagrams.
//  Tabs switch diagrams; copy / export SVG / export standalone HTML in the header.
//  Mermaid is lazy-loaded so it never weighs on first paint. View-only.
// ─────────────────────────────────────────────────────────────────────────────

interface Props { code: string; id: string; }

let mermaidInitPromise: Promise<typeof import('mermaid').default> | null = null;
function getMermaid() {
    if (!mermaidInitPromise) {
        mermaidInitPromise = Promise.all([
            import('mermaid'),
            import('@mermaid-js/layout-elk'),
        ]).then(([{ default: mermaid }, elk]) => {
            // ELK layout engine — layered, orthogonal, overlap-free edge routing.
            // This is what makes diagrams read like Lucidchart/flowchart.io instead
            // of mermaid's default crossing-heavy dagre layout. Lazy-loaded here.
            try { mermaid.registerLayoutLoaders((elk as any).default ?? elk); } catch { /* fall back to dagre */ }
            mermaid.initialize({
                startOnLoad: false,
                // Clean layered orthogonal layout (Lucidchart-style).
                layout: 'elk',
                elk: { mergeEdges: true, nodePlacementStrategy: 'BRANDES_KOEPF' },
                // Dark theme to match the Antigravity-style dark classDef palette the
                // model emits (dark fills, bright strokes, light text).
                theme: 'base',
                // 'loose' is required for htmlLabels (foreignObject) to render —
                // 'strict' sanitizes them away, producing empty/mangled nodes.
                securityLevel: 'loose',
                flowchart: {
                    htmlLabels: true,
                    // 'basis' = smooth rounded connectors (flowchart.io feel); more
                    // rank/node spacing gives breathing room so dense diagrams don't
                    // overlap edges (the main "messy" cause with auto-layout).
                    curve: 'basis',
                    padding: 28,
                    nodeSpacing: 60,
                    rankSpacing: 120,
                    useMaxWidth: false,
                    diagramPadding: 16,
                },
                fontFamily: 'Inter, system-ui, sans-serif',
                themeVariables: {
                    background: 'transparent',
                    // Fallback colors for nodes the model didn't assign a class to.
                    primaryColor: '#1e293b',
                    primaryBorderColor: '#6366f1',
                    primaryTextColor: '#f8fafc',
                    secondaryColor: '#1e1b4b',
                    tertiaryColor: '#0f172a',
                    lineColor: '#94a3b8',
                    textColor: '#e2e8f0',
                    nodeTextColor: '#f8fafc',
                    clusterBkg: 'rgba(30,41,59,0.45)',
                    clusterBorder: 'rgba(99,102,241,0.55)',
                    titleColor: '#e2e8f0',
                    edgeLabelBackground: '#0f172a',
                    fontSize: '14px',
                    fontFamily: 'Inter, system-ui, sans-serif',
                },
            });
            return mermaid;
        });
    }
    return mermaidInitPromise;
}

/** Strip mermaid's inline max-width so the SVG sizes to its natural dimensions
 *  inside our own transform layer. Shadows/edge styling come from CSS
 *  (`.mmd-flow svg ...`), never an SVG `filter=` attr (which breaks rendering). */
function normalizeSvg(svg: string): string {
    return svg
        .replace(/(<svg[^>]*?)\sstyle="[^"]*"/i, '$1')
        .replace(/<svg/i, '<svg style="display:block;overflow:visible"');
}
function svgDims(svg: string): { w: number; h: number } {
    const vb = svg.match(/viewBox="([\d.\-]+)\s+([\d.\-]+)\s+([\d.\-]+)\s+([\d.\-]+)"/i);
    if (vb) return { w: Math.max(160, Math.round(+vb[3])), h: Math.max(120, Math.round(+vb[4])) };
    return { w: 1000, h: 700 };
}

interface View { x: number; y: number; k: number; }
const MIN_K = 0.15, MAX_K = 4;
const clampK = (k: number) => Math.min(MAX_K, Math.max(MIN_K, k));

const MermaidView: React.FC<Props> = ({ code, id }) => {
    const diagrams = useMemo<MermaidDiagram[]>(() => parseDiagramTabContent(code), [code]);
    const [active, setActive] = useState(0);
    const current = diagrams[Math.min(active, diagrams.length - 1)] ?? { title: 'Diagram', code: '' };

    const [svg, setSvg] = useState('');
    const [error, setError] = useState<string | null>(null);
    const [showSource, setShowSource] = useState(false);
    const [showDocs, setShowDocs] = useState(diagrams.length > 1);

    const stageRef = useRef<HTMLDivElement>(null);
    const [view, setView] = useState<View>({ x: 0, y: 0, k: 1 });
    const drag = useRef<{ sx: number; sy: number; ox: number; oy: number } | null>(null);

    const renderId = useMemo(
        () => `mmd-${id.replace(/[^a-zA-Z0-9]/g, '')}-${active}-${Math.random().toString(36).slice(2, 7)}`,
        [id, active],
    );

    // Render the active diagram → SVG.
    useEffect(() => {
        let cancelled = false;
        const src = repairMermaid((current.code || '').trim());
        if (!src) { setSvg(''); setError(null); return; }
        // Guard against pathologically large diagrams that OOM the WebView2
        // renderer. Rough node count = bracketed labels + edges; over the cap we
        // refuse to render the SVG and steer the user to the source view.
        const nodeCount = (src.match(/[\[({]/g) || []).length;
        const edgeCount = (src.match(/--+>|==+>|-\.->/g) || []).length;
        if (nodeCount > 240 || edgeCount > 300 || src.length > 60000) {
            setSvg('');
            setError(`Diagram too large to render safely (${nodeCount} nodes / ${edgeCount} edges) — it would exhaust the renderer's memory. View the source, or ask the agent to split it into smaller diagrams.`);
            return;
        }
        (async () => {
            const mermaid = await getMermaid();
            const tooBig = (svg: string) => svg.length > 4_000_000;
            // Try the faithful source; if it fails to parse (small models emit
            // invalid shapes/styling), retry once with an aggressive sanitizer that
            // keeps the structure but drops the cosmetic bits that broke it.
            const attempt = async (s: string) => {
                await mermaid.parse(s);
                return mermaid.render(renderId, s);
            };
            try {
                const { svg } = await attempt(src);
                if (tooBig(svg)) { if (!cancelled) { setSvg(''); setError('Rendered diagram is too large to display safely. View the source instead.'); } return; }
                if (!cancelled) { setSvg(normalizeSvg(svg)); setError(null); }
            } catch (e1: any) {
                try {
                    const { svg } = await attempt(aggressiveRepairMermaid(current.code || ''));
                    if (tooBig(svg)) { if (!cancelled) { setSvg(''); setError('Rendered diagram is too large to display safely. View the source instead.'); } return; }
                    if (!cancelled) { setSvg(normalizeSvg(svg)); setError(null); }
                } catch {
                    if (!cancelled) { setSvg(''); setError(String(e1?.message ?? e1 ?? 'Failed to render diagram')); }
                }
            }
        })();
        return () => { cancelled = true; };
    }, [current.code, renderId]);

    // Fit the diagram to the stage whenever a new SVG renders.
    const fit = useCallback(() => {
        const stage = stageRef.current;
        if (!stage || !svg) return;
        const { w, h } = svgDims(svg);
        const rect = stage.getBoundingClientRect();
        if (!rect.width || !rect.height) return;
        const k = clampK(Math.min((rect.width - 64) / w, (rect.height - 64) / h, 1.5));
        setView({ k, x: (rect.width - w * k) / 2, y: (rect.height - h * k) / 2 });
    }, [svg]);

    useEffect(() => {
        if (!svg) return;
        const t = setTimeout(fit, 60);
        return () => clearTimeout(t);
    }, [svg, fit]);

    // Zoom toward cursor on ctrl/⌘+wheel; plain wheel scrolls (pans) vertically.
    const onWheel = useCallback((e: React.WheelEvent) => {
        const stage = stageRef.current;
        if (!stage) return;
        if (e.ctrlKey || e.metaKey) {
            e.preventDefault();
            const rect = stage.getBoundingClientRect();
            const px = e.clientX - rect.left, py = e.clientY - rect.top;
            setView(v => {
                const k = clampK(v.k * (e.deltaY < 0 ? 1.12 : 1 / 1.12));
                const r = k / v.k;
                return { k, x: px - (px - v.x) * r, y: py - (py - v.y) * r };
            });
        } else {
            setView(v => ({ ...v, x: v.x - e.deltaX, y: v.y - e.deltaY }));
        }
    }, []);

    const onDown = useCallback((e: React.MouseEvent) => {
        if (e.button !== 0) return;
        drag.current = { sx: e.clientX, sy: e.clientY, ox: view.x, oy: view.y };
    }, [view.x, view.y]);
    useEffect(() => {
        const move = (e: MouseEvent) => {
            const d = drag.current; if (!d) return;
            setView(v => ({ ...v, x: d.ox + (e.clientX - d.sx), y: d.oy + (e.clientY - d.sy) }));
        };
        const up = () => { drag.current = null; };
        window.addEventListener('mousemove', move);
        window.addEventListener('mouseup', up);
        return () => { window.removeEventListener('mousemove', move); window.removeEventListener('mouseup', up); };
    }, []);

    const zoom = useCallback((dir: 1 | -1) => {
        const stage = stageRef.current; if (!stage) return;
        const rect = stage.getBoundingClientRect();
        const px = rect.width / 2, py = rect.height / 2;
        setView(v => {
            const k = clampK(v.k * (dir > 0 ? 1.2 : 1 / 1.2));
            const r = k / v.k;
            return { k, x: px - (px - v.x) * r, y: py - (py - v.y) * r };
        });
    }, []);

    const copySvg = useCallback(() => { if (svg) navigator.clipboard?.writeText(svg).catch(() => { }); }, [svg]);
    const exportSvg = useCallback(() => {
        if (!svg) return;
        const url = URL.createObjectURL(new Blob([svg], { type: 'image/svg+xml' }));
        const a = document.createElement('a');
        a.href = url; a.download = `${current.title || 'diagram'}.svg`; a.click();
        setTimeout(() => URL.revokeObjectURL(url), 1000);
    }, [svg, current.title]);
    const exportHtml = useCallback(async () => {
        try {
            const { invoke } = await import('../tauri_bridge');
            const { useStore } = await import('../store');
            const root = useStore.getState().activeRoot || '';
            await invoke('export_diagram_viewer', { diagrams, project: root });
        } catch (e) { console.warn('[mermaid] export HTML failed:', e); }
    }, [diagrams]);

    return (
        <div className="w-full h-full flex flex-col text-foreground min-w-0" style={{ background: '#0b1020' }}>
            {/* Header */}
            <div className="flex items-center gap-1 px-3 py-2 border-b border-white/10"
                style={{ background: 'rgba(15,23,42,0.55)', backdropFilter: 'blur(12px)' }}>
                <div className="flex items-center justify-center w-6 h-6 rounded-md" style={{ background: 'linear-gradient(135deg,#6366f1,#38bdf8)' }}>
                    <IconHierarchy size={14} className="text-white" />
                </div>
                <span className="text-small font-medium ml-1 mr-2 truncate max-w-[38%]">{current.title}</span>
                <div className="flex-1" />
                {diagrams.length > 1 && (
                    <Tooltip content="Toggle list" size="sm">
                        <Button isIconOnly size="sm" variant="light" onPress={() => setShowDocs(s => !s)}><IconLayoutSidebar size={16} /></Button>
                    </Tooltip>
                )}
                <Tooltip content="Zoom out" size="sm"><Button isIconOnly size="sm" variant="light" isDisabled={!svg} onPress={() => zoom(-1)}><IconZoomOut size={16} /></Button></Tooltip>
                <Tooltip content="Fit to view" size="sm"><Button isIconOnly size="sm" variant="light" isDisabled={!svg} onPress={fit}><IconFocusCentered size={16} /></Button></Tooltip>
                <Tooltip content="Zoom in" size="sm"><Button isIconOnly size="sm" variant="light" isDisabled={!svg} onPress={() => zoom(1)}><IconZoomIn size={16} /></Button></Tooltip>
                <Tooltip content={showSource ? 'Show diagram' : 'Show source'} size="sm">
                    <Button isIconOnly size="sm" variant="light" onPress={() => setShowSource(s => !s)}>{showSource ? <IconHierarchy size={16} /> : <IconCode size={16} />}</Button>
                </Tooltip>
                <Tooltip content="Copy SVG" size="sm"><Button isIconOnly size="sm" variant="light" isDisabled={!svg} onPress={copySvg}><IconCopy size={16} /></Button></Tooltip>
                <Tooltip content="Export .svg" size="sm"><Button isIconOnly size="sm" variant="light" isDisabled={!svg} onPress={exportSvg}><IconDownload size={16} /></Button></Tooltip>
                <Tooltip content="Export standalone HTML viewer" size="sm"><Button isIconOnly size="sm" variant="light" onPress={exportHtml}><IconFileExport size={16} /></Button></Tooltip>
            </div>

            {/* Tabs */}
            {diagrams.length > 1 && (
                <div className="px-2 pt-1.5 border-b border-white/10" style={{ background: 'rgba(15,23,42,0.4)' }}>
                    <Tabs size="sm" variant="underlined" selectedKey={String(active)}
                        onSelectionChange={(k) => setActive(Number(k))} aria-label="Diagrams">
                        {diagrams.map((d, i) => <Tab key={String(i)} title={d.title} />)}
                    </Tabs>
                </div>
            )}

            <div className="flex-1 flex min-h-0">
                {showDocs && diagrams.length > 1 && (
                    <div className="w-56 shrink-0 border-r border-white/10 overflow-auto p-2" style={{ background: 'rgba(15,23,42,0.4)' }}>
                        <div className="text-tiny uppercase tracking-wide text-default-400 px-1 mb-1">Diagrams</div>
                        {diagrams.map((d, i) => (
                            <button key={i} onClick={() => setActive(i)}
                                className={`w-full text-left px-2 py-1.5 rounded-medium text-small mb-0.5 ${i === active ? 'bg-primary/20 text-foreground' : 'text-default-500 hover:bg-white/5'}`}>
                                {d.title}
                            </button>
                        ))}
                    </div>
                )}

                {showSource ? (
                    <pre className="flex-1 m-0 p-4 overflow-auto text-small font-mono whitespace-pre-wrap">{current.code}</pre>
                ) : error ? (
                    <div className="flex-1 p-5 overflow-auto text-small text-danger font-mono">
                        <div className="font-semibold mb-1.5 flex items-center gap-1.5"><IconAlertTriangle size={16} className="text-warning" /> Mermaid syntax error</div>
                        <pre className="m-0 whitespace-pre-wrap opacity-85">{error}</pre>
                        <Button size="sm" variant="bordered" className="mt-2.5" onPress={() => setShowSource(true)}>View source</Button>
                    </div>
                ) : (
                    <div
                        ref={stageRef}
                        className="flex-1 min-w-0 relative overflow-hidden mmd-flow"
                        onWheel={onWheel}
                        onMouseDown={onDown}
                        style={{
                            cursor: drag.current ? 'grabbing' : 'grab',
                            // Antigravity-style dark canvas with a subtle dot grid +
                            // faint corner glows, so dark classDef nodes pop.
                            backgroundColor: '#0b1020',
                            backgroundImage:
                                'radial-gradient(900px 480px at 18% -8%, rgba(99,102,241,0.14) 0%, transparent 60%),' +
                                'radial-gradient(760px 420px at 102% 0%, rgba(56,189,248,0.10) 0%, transparent 55%),' +
                                'radial-gradient(rgba(148,163,184,0.10) 1px, transparent 1px)',
                            backgroundSize: 'auto, auto, 22px 22px',
                        }}
                    >
                        <div
                            style={{
                                position: 'absolute', top: 0, left: 0,
                                transform: `translate(${view.x}px, ${view.y}px) scale(${view.k})`,
                                transformOrigin: '0 0',
                                willChange: 'transform',
                            }}
                            dangerouslySetInnerHTML={{ __html: sanitizeHtml(svg) }}
                        />
                    </div>
                )}
            </div>
        </div>
    );
};

export default MermaidView;
