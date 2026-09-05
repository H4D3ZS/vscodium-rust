/**
 * CanvasRenderer — renders a declarative CanvasSpec into a rich dashboard.
 *
 * Zero chart dependencies: bar/line/pie are hand-rolled SVG (~potato-friendly,
 * works fully offline). Markdown reuses lib/markdown (marked + DOMPurify).
 */
import React, { useMemo, useState } from 'react';
import type {
    CanvasBlock,
    CanvasSpec,
    CanvasTone,
    CanvasChartSeries,
} from '../../domain/canvas/CanvasSpec';
import { parseMarkdown } from '../../lib/markdown';

// ── Tone palette (matches IDE dark theme) ────────────────────────────────────

const TONE_COLOR: Record<CanvasTone, string> = {
    neutral: '#9da5b4',
    success: '#4ade80',
    warning: '#fbbf24',
    danger: '#f87171',
    info: '#60a5fa',
    accent: '#c084fc',
};

const SERIES_FALLBACK = ['#60a5fa', '#4ade80', '#fbbf24', '#f87171', '#c084fc', '#22d3ee'];

function seriesColor(s: CanvasChartSeries, i: number): string {
    return s.tone? TONE_COLOR[s.tone]: SERIES_FALLBACK[i % SERIES_FALLBACK.length];
}

const card: React.CSSProperties = {
    background: 'rgba(255,255,255,0.03)',
    border: '1px solid rgba(255,255,255,0.08)',
    borderRadius: 8,
    padding: '14px 16px',
};

const blockTitleStyle: React.CSSProperties = {
    fontSize: 12,
    fontWeight: 600,
    textTransform: 'uppercase',
    letterSpacing: '0.06em',
    opacity: 0.75,
    marginBottom: 10,
};

function BlockTitle({ text }: { text?: string }) {
    if (!text) return null;
    return <div style={blockTitleStyle}>{text}</div>;
}

// ── Stats ────────────────────────────────────────────────────────────────────

function StatsBlock({ block }: { block: Extract<CanvasBlock, { type: 'stats' }> }) {
    return (
        <div style={{ display: 'grid', gridTemplateColumns: `repeat(auto-fit, minmax(140px, 1fr))`, gap: 10 }}>
            {block.items.map((it, i) => (
                <div key={i} style={{ ...card, textAlign: 'center' }}>
                    <div style={{
                        fontSize: 26,
                        fontWeight: 700,
                        color: it.tone? TONE_COLOR[it.tone]: 'var(--vscode-foreground, #e5e7eb)',
                        lineHeight: 1.2,
                        wordBreak: 'break-word',
                    }}>
                        {it.value}
                    </div>
                    <div style={{ fontSize: 11, opacity: 0.7, marginTop: 4 }}>{it.label}</div>
                    {it.hint && <div style={{ fontSize: 10, opacity: 0.45, marginTop: 2 }}>{it.hint}</div>}
                </div>
            ))}
        </div>
    );
}

// ── Table (sortable) ─────────────────────────────────────────────────────────

function TableBlock({ block }: { block: Extract<CanvasBlock, { type: 'table' }> }) {
    const [sortCol, setSortCol] = useState<number | null>(null);
    const [sortDir, setSortDir] = useState<1 | -1>(1);

    const rows = useMemo(() => {
        if (sortCol === null) return block.rows;
        return [...block.rows].sort((a, b) => {
            const av = a[sortCol]; const bv = b[sortCol];
            const an = typeof av === 'number'? av: parseFloat(String(av));
            const bn = typeof bv === 'number'? bv: parseFloat(String(bv));
            if (Number.isFinite(an) && Number.isFinite(bn)) return (an - bn) * sortDir;
            return String(av).localeCompare(String(bv)) * sortDir;
        });
    }, [block.rows, sortCol, sortDir]);

    const onHeaderClick = (i: number) => {
        if (sortCol === i) setSortDir((d) => (d === 1? -1: 1));
        else { setSortCol(i); setSortDir(1); }
    };

    return (
        <div style={{ ...card, padding: 0, overflow: 'hidden' }}>
            {block.title && <div style={{ ...blockTitleStyle, margin: 0, padding: '12px 16px 0' }}>{block.title}</div>}
            <div style={{ overflowX: 'auto' }}>
                <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 12 }}>
                    <thead>
                        <tr>
                            {block.columns.map((c, i) => (
                                <th
                                    key={i}
                                    onClick={() => onHeaderClick(i)}
                                    style={{
                                        textAlign: 'left', padding: '10px 14px', cursor: 'pointer',
                                        borderBottom: '1px solid rgba(255,255,255,0.1)',
                                        fontWeight: 600, fontSize: 11, textTransform: 'uppercase',
                                        letterSpacing: '0.04em', opacity: 0.8, userSelect: 'none',
                                        whiteSpace: 'nowrap',
                                    }}
                                    title="Click to sort"
                                >
                                    {c}{sortCol === i? (sortDir === 1? ' ▲': ' ▼'): ''}
                                </th>
                            ))}
                        </tr>
                    </thead>
                    <tbody>
                        {rows.map((r, ri) => (
                            <tr key={ri} style={{ background: ri % 2? 'rgba(255,255,255,0.02)': 'transparent' }}>
                                {block.columns.map((_, ci) => (
                                    <td key={ci} style={{ padding: '8px 14px', borderBottom: '1px solid rgba(255,255,255,0.04)', verticalAlign: 'top' }}>
                                        {r[ci] ?? ''}
                                    </td>
                                ))}
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    );
}

// ── Charts (hand-rolled SVG) ─────────────────────────────────────────────────

const CHART_W = 560;
const CHART_H = 220;
const PAD = { top: 12, right: 12, bottom: 28, left: 44 };

function niceMax(v: number): number {
    if (v <= 0) return 1;
    const exp = Math.pow(10, Math.floor(Math.log10(v)));
    const f = v / exp;
    const nf = f <= 1? 1: f <= 2? 2: f <= 5? 5: 10;
    return nf * exp;
}

function ChartLegend({ series }: { series: CanvasChartSeries[] }) {
    const named = series.filter((s) => s.name);
    if (named.length < 1 || series.length < 2) return null;
    return (
        <div style={{ display: 'flex', gap: 14, flexWrap: 'wrap', marginTop: 8, fontSize: 11 }}>
            {series.map((s, i) => (
                <span key={i} style={{ display: 'inline-flex', alignItems: 'center', gap: 5, opacity: 0.85 }}>
                    <span style={{ width: 10, height: 10, borderRadius: 2, background: seriesColor(s, i), display: 'inline-block' }} />
                    {s.name || `Series ${i + 1}`}
                </span>
            ))}
        </div>
    );
}

function axisTicks(max: number): number[] {
    return [0, 0.25, 0.5, 0.75, 1].map((f) => max * f);
}

function BarChart({ block }: { block: Extract<CanvasBlock, { type: 'chart' }> }) {
    const { labels, series } = block;
    const max = niceMax(Math.max(...series.flatMap((s) => s.values), 0));
    const innerW = CHART_W - PAD.left - PAD.right;
    const innerH = CHART_H - PAD.top - PAD.bottom;
    const groupW = innerW / Math.max(labels.length, 1);
    const barW = Math.min(34, (groupW * 0.7) / series.length);

    return (
        <svg viewBox={`0 0 ${CHART_W} ${CHART_H}`} style={{ width: '100%', height: 'auto', maxHeight: 280 }}>
            {axisTicks(max).map((t, i) => {
                const y = PAD.top + innerH - (t / max) * innerH;
                return (
                    <g key={i}>
                        <line x1={PAD.left} x2={CHART_W - PAD.right} y1={y} y2={y} stroke="rgba(255,255,255,0.07)" />
                        <text x={PAD.left - 6} y={y + 3} textAnchor="end" fontSize={9} fill="rgba(255,255,255,0.45)">
                            {Number.isInteger(t)? t: t.toFixed(1)}
                        </text>
                    </g>
                );
            })}
            {labels.map((lab, li) => {
                const cx = PAD.left + groupW * li + groupW / 2;
                const totalBarW = barW * series.length;
                return (
                    <g key={li}>
                        {series.map((s, si) => {
                            const v = s.values[li] ?? 0;
                            const h = max > 0? (v / max) * innerH: 0;
                            const x = cx - totalBarW / 2 + si * barW;
                            return (
                                <rect
                                    key={si}
                                    x={x} y={PAD.top + innerH - h}
                                    width={Math.max(barW - 2, 1)} height={Math.max(h, 0)}
                                    rx={2} fill={seriesColor(s, si)} opacity={0.9}
                                >
                                    <title>{`${lab}${s.name? ` · ${s.name}`: ''}: ${v}`}</title>
                                </rect>
                            );
                        })}
                        <text x={cx} y={CHART_H - 8} textAnchor="middle" fontSize={9} fill="rgba(255,255,255,0.55)">
                            {lab.length > 12? `${lab.slice(0, 11)}…`: lab}
                        </text>
                    </g>
                );
            })}
        </svg>
    );
}

function LineChart({ block }: { block: Extract<CanvasBlock, { type: 'chart' }> }) {
    const { labels, series } = block;
    const max = niceMax(Math.max(...series.flatMap((s) => s.values), 0));
    const innerW = CHART_W - PAD.left - PAD.right;
    const innerH = CHART_H - PAD.top - PAD.bottom;
    const xAt = (i: number) => PAD.left + (labels.length > 1? (i / (labels.length - 1)) * innerW: innerW / 2);
    const yAt = (v: number) => PAD.top + innerH - (max > 0? (v / max) * innerH: 0);

    return (
        <svg viewBox={`0 0 ${CHART_W} ${CHART_H}`} style={{ width: '100%', height: 'auto', maxHeight: 280 }}>
            {axisTicks(max).map((t, i) => {
                const y = yAt(t);
                return (
                    <g key={i}>
                        <line x1={PAD.left} x2={CHART_W - PAD.right} y1={y} y2={y} stroke="rgba(255,255,255,0.07)" />
                        <text x={PAD.left - 6} y={y + 3} textAnchor="end" fontSize={9} fill="rgba(255,255,255,0.45)">
                            {Number.isInteger(t)? t: t.toFixed(1)}
                        </text>
                    </g>
                );
            })}
            {series.map((s, si) => {
                const pts = s.values.slice(0, labels.length).map((v, i) => `${xAt(i)},${yAt(v)}`);
                const color = seriesColor(s, si);
                const areaPath = `M ${xAt(0)},${yAt(0) + 0} L ${pts.join(' L ')} L ${xAt(s.values.length - 1)},${PAD.top + innerH} L ${xAt(0)},${PAD.top + innerH} Z`;
                return (
                    <g key={si}>
                        {series.length === 1 && (
                            <path d={areaPath} fill={color} opacity={0.12} />
                        )}
                        <polyline points={pts.join(' ')} fill="none" stroke={color} strokeWidth={2} strokeLinejoin="round" />
                        {s.values.slice(0, labels.length).map((v, i) => (
                            <circle key={i} cx={xAt(i)} cy={yAt(v)} r={2.5} fill={color}>
                                <title>{`${labels[i]}${s.name? ` · ${s.name}`: ''}: ${v}`}</title>
                            </circle>
                        ))}
                    </g>
                );
            })}
            {labels.map((lab, i) => (
                (labels.length <= 12 || i % Math.ceil(labels.length / 12) === 0) && (
                    <text key={i} x={xAt(i)} y={CHART_H - 8} textAnchor="middle" fontSize={9} fill="rgba(255,255,255,0.55)">
                        {lab.length > 10? `${lab.slice(0, 9)}…`: lab}
                    </text>
                )
            ))}
        </svg>
    );
}

function PieChart({ block }: { block: Extract<CanvasBlock, { type: 'chart' }> }) {
    const values = block.series[0]?.values ?? [];
    const total = values.reduce((a, b) => a + Math.max(b, 0), 0) || 1;
    const cx = 110, cy = 110, r = 88, inner = 52;
    let angle = -Math.PI / 2;

    const arcs = values.map((v, i) => {
        const frac = Math.max(v, 0) / total;
        const a0 = angle;
        const a1 = angle + frac * Math.PI * 2;
        angle = a1;
        const large = a1 - a0 > Math.PI? 1: 0;
        const p0 = [cx + r * Math.cos(a0), cy + r * Math.sin(a0)];
        const p1 = [cx + r * Math.cos(a1), cy + r * Math.sin(a1)];
        const q1 = [cx + inner * Math.cos(a1), cy + inner * Math.sin(a1)];
        const q0 = [cx + inner * Math.cos(a0), cy + inner * Math.sin(a0)];
        const d = `M ${p0} A ${r} ${r} 0 ${large} 1 ${p1} L ${q1} A ${inner} ${inner} 0 ${large} 0 ${q0} Z`;
        return { d, color: SERIES_FALLBACK[i % SERIES_FALLBACK.length], label: block.labels[i] ?? `#${i + 1}`, value: v, frac };
    });

    return (
        <div style={{ display: 'flex', gap: 20, alignItems: 'center', flexWrap: 'wrap' }}>
            <svg viewBox="0 0 220 220" style={{ width: 180, height: 180, flexShrink: 0 }}>
                {arcs.map((a, i) => (
                    <path key={i} d={a.d} fill={a.color} opacity={0.9}>
                        <title>{`${a.label}: ${a.value} (${(a.frac * 100).toFixed(1)}%)`}</title>
                    </path>
                ))}
            </svg>
            <div style={{ display: 'flex', flexDirection: 'column', gap: 6, fontSize: 12 }}>
                {arcs.map((a, i) => (
                    <span key={i} style={{ display: 'inline-flex', alignItems: 'center', gap: 6 }}>
                        <span style={{ width: 10, height: 10, borderRadius: 2, background: a.color, display: 'inline-block' }} />
                        <span style={{ opacity: 0.85 }}>{a.label}</span>
                        <span style={{ opacity: 0.5 }}>{a.value} · {(a.frac * 100).toFixed(1)}%</span>
                    </span>
                ))}
            </div>
        </div>
    );
}

function ChartBlock({ block }: { block: Extract<CanvasBlock, { type: 'chart' }> }) {
    return (
        <div style={card}>
            <BlockTitle text={block.title} />
            {block.chart === 'pie'? <PieChart block={block} />
: block.chart === 'line'? <LineChart block={block} />
: <BarChart block={block} />}
            {block.chart !== 'pie' && <ChartLegend series={block.series} />}
        </div>
    );
}

// ── Remaining blocks ─────────────────────────────────────────────────────────

function CalloutBlock({ block }: { block: Extract<CanvasBlock, { type: 'callout' }> }) {
    const tone = block.tone ?? 'info';
    const color = TONE_COLOR[tone];
    return (
        <div style={{
            ...card,
            borderLeft: `3px solid ${color}`,
            background: `${color}14`,
        }}>
            {block.title && <div style={{ fontWeight: 600, fontSize: 13, color, marginBottom: 4 }}>{block.title}</div>}
            <div
                style={{ fontSize: 12.5, lineHeight: 1.55 }}
                dangerouslySetInnerHTML={{ __html: parseMarkdown(block.content) }}
            />
        </div>
    );
}

function ProgressBlock({ block }: { block: Extract<CanvasBlock, { type: 'progress' }> }) {
    return (
        <div style={card}>
            <BlockTitle text={block.title} />
            <div style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
                {block.items.map((it, i) => {
                    const max = it.max ?? 100;
                    const pct = Math.min(100, Math.max(0, (it.value / (max || 1)) * 100));
                    const color = it.tone? TONE_COLOR[it.tone]: (pct >= 100? TONE_COLOR.success: TONE_COLOR.info);
                    return (
                        <div key={i}>
                            <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: 11.5, marginBottom: 4 }}>
                                <span style={{ opacity: 0.85 }}>{it.label}</span>
                                <span style={{ opacity: 0.6 }}>{it.value}{it.max != null? ` / ${it.max}`: '%'}</span>
                            </div>
                            <div style={{ height: 6, borderRadius: 3, background: 'rgba(255,255,255,0.08)', overflow: 'hidden' }}>
                                <div style={{ width: `${pct}%`, height: '100%', background: color, borderRadius: 3, transition: 'width var(--duration-base, 150ms) ease' }} />
                            </div>
                        </div>
                    );
                })}
            </div>
        </div>
    );
}

function TodoBlock({ block }: { block: Extract<CanvasBlock, { type: 'todo' }> }) {
    const done = block.items.filter((i) => i.done).length;
    return (
        <div style={card}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'baseline' }}>
                <BlockTitle text={block.title ?? 'Tasks'} />
                <span style={{ fontSize: 11, opacity: 0.55 }}>{done}/{block.items.length}</span>
            </div>
            <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
                {block.items.map((it, i) => (
                    <div key={i} style={{ display: 'flex', alignItems: 'flex-start', gap: 8, fontSize: 12.5 }}>
                        <span style={{
                            width: 14, height: 14, borderRadius: 7, flexShrink: 0, marginTop: 1,
                            border: `1.5px solid ${it.done? TONE_COLOR.success: 'rgba(255,255,255,0.3)'}`,
                            background: it.done? TONE_COLOR.success: 'transparent',
                            display: 'inline-flex', alignItems: 'center', justifyContent: 'center',
                            fontSize: 9, color: '#0b0e14', fontWeight: 700,
                        }}>
                            {it.done? '': ''}
                        </span>
                        <span style={{ opacity: it.done? 0.55: 0.9, textDecoration: it.done? 'line-through': 'none' }}>
                            {it.text}
                        </span>
                    </div>
                ))}
            </div>
        </div>
    );
}

function KvBlock({ block }: { block: Extract<CanvasBlock, { type: 'kv' }> }) {
    return (
        <div style={card}>
            <BlockTitle text={block.title} />
            <div style={{ display: 'grid', gridTemplateColumns: 'minmax(110px, max-content) 1fr', gap: '6px 18px', fontSize: 12.5 }}>
                {block.pairs.map((p, i) => (
                    <React.Fragment key={i}>
                        <span style={{ opacity: 0.55 }}>{p.key}</span>
                        <span style={{ wordBreak: 'break-word' }}>{p.value}</span>
                    </React.Fragment>
                ))}
            </div>
        </div>
    );
}

function CodeBlock({ block }: { block: Extract<CanvasBlock, { type: 'code' }> }) {
    return (
        <div style={{ ...card, padding: 0, overflow: 'hidden' }}>
            {(block.title || block.language) && (
                <div style={{
                    padding: '8px 14px', fontSize: 11, opacity: 0.65,
                    borderBottom: '1px solid rgba(255,255,255,0.07)',
                    display: 'flex', justifyContent: 'space-between',
                }}>
                    <span>{block.title ?? ''}</span>
                    <span style={{ fontFamily: 'monospace' }}>{block.language ?? ''}</span>
                </div>
            )}
            <pre style={{
                margin: 0, padding: '12px 14px', fontSize: 12, lineHeight: 1.5,
                overflowX: 'auto', fontFamily: 'var(--monaco-monospace-font, "SF Mono", Consolas, monospace)',
            }}>
                {block.content}
            </pre>
        </div>
    );
}

function TimelineBlock({ block }: { block: Extract<CanvasBlock, { type: 'timeline' }> }) {
    return (
        <div style={card}>
            <BlockTitle text={block.title} />
            <div style={{ display: 'flex', flexDirection: 'column' }}>
                {block.items.map((it, i) => {
                    const color = it.status === 'done'? TONE_COLOR.success
: it.status === 'active'? TONE_COLOR.info: 'rgba(255,255,255,0.3)';
                    return (
                        <div key={i} style={{ display: 'flex', gap: 12 }}>
                            <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
                                <span style={{
                                    width: 10, height: 10, borderRadius: 5, background: color, flexShrink: 0, marginTop: 4,
                                    boxShadow: it.status === 'active'? `0 0 6px ${color}`: 'none',
                                }} />
                                {i < block.items.length - 1 && (
                                    <span style={{ width: 1.5, flex: 1, minHeight: 14, background: 'rgba(255,255,255,0.12)' }} />
                                )}
                            </div>
                            <div style={{ paddingBottom: i < block.items.length - 1? 14: 0 }}>
                                <div style={{ fontSize: 12.5, fontWeight: 600, opacity: 0.9 }}>{it.title}</div>
                                {it.detail && <div style={{ fontSize: 11.5, opacity: 0.6, marginTop: 2 }}>{it.detail}</div>}
                            </div>
                        </div>
                    );
                })}
            </div>
        </div>
    );
}

function MarkdownBlock({ block }: { block: Extract<CanvasBlock, { type: 'markdown' }> }) {
    return (
        <div
            className="canvas-markdown"
            style={{ fontSize: 13, lineHeight: 1.6 }}
            dangerouslySetInnerHTML={{ __html: parseMarkdown(block.content) }}
        />
    );
}

// ── Root ─────────────────────────────────────────────────────────────────────

function renderBlock(block: CanvasBlock, i: number): React.ReactNode {
    switch (block.type) {
        case 'stats': return <StatsBlock key={i} block={block} />;
        case 'table': return <TableBlock key={i} block={block} />;
        case 'chart': return <ChartBlock key={i} block={block} />;
        case 'markdown': return <MarkdownBlock key={i} block={block} />;
        case 'callout': return <CalloutBlock key={i} block={block} />;
        case 'progress': return <ProgressBlock key={i} block={block} />;
        case 'todo': return <TodoBlock key={i} block={block} />;
        case 'kv': return <KvBlock key={i} block={block} />;
        case 'code': return <CodeBlock key={i} block={block} />;
        case 'timeline': return <TimelineBlock key={i} block={block} />;
        default: return null;
    }
}

const CanvasRenderer: React.FC<{ spec: CanvasSpec }> = ({ spec }) => {
    return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 14, maxWidth: 920, margin: '0 auto', width: '100%' }}>
            <div>
                <h1 style={{ fontSize: 22, fontWeight: 700, margin: 0 }}>{spec.title}</h1>
                {spec.subtitle && <div style={{ fontSize: 12.5, opacity: 0.6, marginTop: 4 }}>{spec.subtitle}</div>}
                <div style={{ fontSize: 10.5, opacity: 0.4, marginTop: 4 }}>
                    Updated {new Date(spec.updatedAt).toLocaleString()}
                </div>
            </div>
            {spec.blocks.map(renderBlock)}
        </div>
    );
};

export default CanvasRenderer;
