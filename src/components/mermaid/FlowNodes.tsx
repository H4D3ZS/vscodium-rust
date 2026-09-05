import React from 'react';
import { Handle, Position, type NodeProps } from 'reactflow';
import type { FlowNodeData } from './mermaidToFlow';

// View-only reactflow node types for the Mermaid→reactflow vector flowchart.
// Glass surfaces, colored accents lifted from the source diagram, cheap CSS
// box-shadow (never an SVG filter — that OOM'd WebView2 before).

const HANDLE_STYLE: React.CSSProperties = { opacity: 0, width: 1, height: 1, border: 'none', minWidth: 0, minHeight: 0 };

// Hidden source+target handles on all four sides so edges can route by geometry.
const Handles: React.FC = () => (
    <>
        {(['top', 'right', 'bottom', 'left'] as const).map((side) => {
            const pos = side === 'top' ? Position.Top : side === 'right' ? Position.Right : side === 'bottom' ? Position.Bottom : Position.Left;
            return (
                <React.Fragment key={side}>
                    <Handle id={`s-${side}`} type="source" position={pos} style={HANDLE_STYLE} isConnectable={false} />
                    <Handle id={`t-${side}`} type="target" position={pos} style={HANDLE_STYLE} isConnectable={false} />
                </React.Fragment>
            );
        })}
    </>
);

function isLightColor(color: string): boolean {
    let r = 0, g = 0, b = 0;
    const c = color.trim().toLowerCase();
    if (c.startsWith('#')) {
        const hex = c.slice(1);
        if (hex.length === 3) {
            r = parseInt(hex[0] + hex[0], 16);
            g = parseInt(hex[1] + hex[1], 16);
            b = parseInt(hex[2] + hex[2], 16);
        } else if (hex.length >= 6) {
            r = parseInt(hex.slice(0, 2), 16);
            g = parseInt(hex.slice(2, 4), 16);
            b = parseInt(hex.slice(4, 6), 16);
        }
    } else if (c.startsWith('rgb')) {
        const m = c.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)/);
        if (m) {
            r = parseInt(m[1], 10);
            g = parseInt(m[2], 10);
            b = parseInt(m[3], 10);
        }
    }
    const brightness = (r * 299 + g * 587 + b * 114) / 1000;
    return brightness > 165;
}

/** Tint a source fill into a soft glass surface; fall back to the IDE slate. */
function surface(fill: string): { bg: string; border: string; text: string } {
    if (fill && /^#|rgb|hsl/i.test(fill)) {
        const isLight = isLightColor(fill);
        // Use CSS color-mix to create a glassy background color that scales alpha safely
        // regardless of hex, rgb, rgba, or hsl input formats.
        const bg = `linear-gradient(160deg, ${fill}, color-mix(in srgb, ${fill} 80%, transparent))`;
        return {
            bg,
            border: isLight ? 'rgba(0,0,0,0.15)' : 'rgba(255,255,255,0.22)',
            text: isLight ? '#0f172a' : '#f8fafc',
        };
    }
    return { bg: 'linear-gradient(160deg, #1e293b, #172033)', border: 'rgba(255,255,255,0.14)', text: '#e5e9f0' };
}

export const MmNode: React.FC<NodeProps<FlowNodeData>> = ({ data, selected }) => {
    const s = surface(data.fill);
    const radius = data.shape === 'round' ? 9999 : data.shape === 'diamond' ? 8 : 16;
    const clip = data.shape === 'diamond'
        ? 'polygon(50% 0%, 100% 50%, 50% 100%, 0% 50%)'
        : data.shape === 'data'
            ? 'polygon(8% 0, 100% 0, 92% 100%, 0% 100%)'
            : undefined;
    return (
        <div
            style={{
                width: data.w, height: data.h,
                background: s.bg,
                border: `2px solid ${selected ? '#38bdf8' : s.border}`,
                borderRadius: radius,
                clipPath: clip,
                color: s.text,
                display: 'flex', alignItems: 'center', justifyContent: 'center',
                textAlign: 'center', padding: '8px 16px', boxSizing: 'border-box',
                fontSize: 14, fontWeight: 600, lineHeight: 1.3,
                boxShadow: selected
                    ? '0 0 0 3px rgba(56,189,248,0.6), 0 12px 32px rgba(0,0,0,0.6)'
                    : '0 6px 20px rgba(0,0,0,0.4), 0 2px 8px rgba(0,0,0,0.2)',
                transition: 'box-shadow 0.15s, border-color 0.15s, transform 0.15s',
                cursor: 'pointer',
            }}
            onMouseEnter={(e) => { (e.currentTarget as HTMLElement).style.transform = 'scale(1.02)'; }}
            onMouseLeave={(e) => { (e.currentTarget as HTMLElement).style.transform = 'scale(1)'; }}
        >
            <Handles />
            <div style={{ wordBreak: 'break-word', whiteSpace: 'normal', width: '100%' }}>{data.label}</div>
        </div>
    );
};

export const MmGroup: React.FC<NodeProps<FlowNodeData>> = ({ data }) => (
    <div
        style={{
            width: data.w, height: data.h,
            background: 'rgba(30,41,59,0.4)',
            border: '2px dashed rgba(99,102,241,0.4)',
            borderRadius: 16,
            boxSizing: 'border-box',
            backdropFilter: 'blur(4px)',
        }}
    >
        {data.label && (
            <div style={{
                position: 'absolute', top: 10, left: 14,
                fontSize: 12, fontWeight: 700, letterSpacing: '0.05em',
                textTransform: 'uppercase', color: 'rgba(148,163,184,0.8)',
            }}>{data.label}</div>
        )}
    </div>
);

export const flowNodeTypes = { mmNode: MmNode, mmGroup: MmGroup };
