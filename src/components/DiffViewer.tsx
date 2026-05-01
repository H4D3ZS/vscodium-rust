import React, { useState, useMemo } from 'react';
import { useStore } from '../store';
import type { PendingChange } from '../store';
import { Sparkles, Check, X, CheckCheck, XCircle, ChevronLeft, ChevronRight } from 'lucide-react';

// ---------------------------------------------------------------------------
// Minimal Myers-style line diff — no deps, runs in the browser
// ---------------------------------------------------------------------------
type DiffLine = { type: 'add' | 'del' | 'same'; text: string };

function computeLineDiff(oldText: string, newText: string): DiffLine[] {
    const oldLines = oldText.split('\n');
    const newLines = newText.split('\n');
    const result: DiffLine[] = [];

    // Simple LCS-based diff — fast enough for files up to ~2000 lines
    const m = oldLines.length;
    const n = newLines.length;

    // Build LCS table
    const dp: number[][] = Array.from({ length: m + 1 }, () => new Array(n + 1).fill(0));
    for (let i = 1; i <= m; i++) {
        for (let j = 1; j <= n; j++) {
            dp[i][j] = oldLines[i - 1] === newLines[j - 1]
                ? dp[i - 1][j - 1] + 1
                : Math.max(dp[i - 1][j], dp[i][j - 1]);
        }
    }

    // Traceback
    const trace: DiffLine[] = [];
    let i = m, j = n;
    while (i > 0 || j > 0) {
        if (i > 0 && j > 0 && oldLines[i - 1] === newLines[j - 1]) {
            trace.push({ type: 'same', text: oldLines[i - 1] });
            i--; j--;
        } else if (j > 0 && (i === 0 || dp[i][j - 1] >= dp[i - 1][j])) {
            trace.push({ type: 'add', text: newLines[j - 1] });
            j--;
        } else {
            trace.push({ type: 'del', text: oldLines[i - 1] });
            i--;
        }
    }
    return trace.reverse();
}

// Collapse unchanged runs longer than CONTEXT lines into a fold
const CONTEXT = 3;
function withFolds(lines: DiffLine[]): Array<DiffLine | { type: 'fold'; count: number }> {
    const out: Array<DiffLine | { type: 'fold'; count: number }> = [];
    let sameRun = 0;
    let buffer: DiffLine[] = [];

    const flush = () => {
        if (sameRun <= CONTEXT * 2) {
            out.push(...buffer);
        } else {
            out.push(...buffer.slice(0, CONTEXT));
            out.push({ type: 'fold', count: sameRun - CONTEXT * 2 });
            out.push(...buffer.slice(buffer.length - CONTEXT));
        }
        buffer = [];
        sameRun = 0;
    };

    for (const line of lines) {
        if (line.type === 'same') {
            buffer.push(line);
            sameRun++;
        } else {
            flush();
            out.push(line);
        }
    }
    flush();
    return out;
}

// ---------------------------------------------------------------------------
// Single-file diff panel
// ---------------------------------------------------------------------------
export const FileDiff: React.FC<{ change: PendingChange }> = ({ change }) => {
    const lines = useMemo(
        () => withFolds(computeLineDiff(change.originalContent || '', change.proposedContent || '')),
        [change.originalContent, change.proposedContent]
    );

    const additions = lines.filter(l => l.type === 'add').length;
    const deletions = lines.filter(l => l.type === 'del').length;

    return (
        <div style={{ flex: 1, overflow: 'auto', fontSize: '12px', fontFamily: "var(--vscode-editor-font-family, 'Cascadia Code', monospace)" }}>
            <div style={{ padding: '4px 12px', background: 'rgba(255,255,255,0.03)', borderBottom: '1px solid rgba(255,255,255,0.06)', display: 'flex', gap: '12px', fontSize: '11px', opacity: 0.7 }}>
                <span style={{ color: '#4ec994' }}>+{additions}</span>
                <span style={{ color: '#f87171' }}>-{deletions}</span>
                <span style={{ opacity: 0.5 }}>{change.description || ''}</span>
            </div>
            {lines.map((line, idx) => {
                if ((line as any).type === 'fold') {
                    return (
                        <div key={idx} style={{ padding: '2px 16px', background: 'rgba(255,255,255,0.02)', color: 'rgba(255,255,255,0.3)', fontSize: '11px' }}>
                            ··· {(line as any).count} unchanged lines ···
                        </div>
                    );
                }
                const l = line as DiffLine;
                const bg = l.type === 'add' ? 'rgba(78,201,148,0.10)' : l.type === 'del' ? 'rgba(248,113,113,0.10)' : 'transparent';
                const prefix = l.type === 'add' ? '+' : l.type === 'del' ? '-' : ' ';
                const prefixColor = l.type === 'add' ? '#4ec994' : l.type === 'del' ? '#f87171' : 'rgba(255,255,255,0.2)';
                return (
                    <div key={idx} style={{ background: bg, display: 'flex', minHeight: '18px' }}>
                        <span style={{ color: prefixColor, padding: '0 8px', userSelect: 'none', minWidth: '20px' }}>{prefix}</span>
                        <span style={{ color: 'var(--vscode-editor-foreground, #ccc)', whiteSpace: 'pre', flex: 1, padding: '0 4px' }}>{l.text}</span>
                    </div>
                );
            })}
        </div>
    );
};

// ---------------------------------------------------------------------------
// Main DiffViewer
// ---------------------------------------------------------------------------
const DiffViewer: React.FC = () => {
    const pendingChanges = useStore(state => state.pendingChanges);
    const acceptPendingChange = useStore(state => state.acceptPendingChange);
    const rejectPendingChange = useStore(state => state.rejectPendingChange);
    const acceptAllPendingChanges = useStore(state => state.acceptAllPendingChanges);
    const rejectAllPendingChanges = useStore(state => state.rejectAllPendingChanges);
    const [activeIdx, setActiveIdx] = useState(0);
    const [accepting, setAccepting] = useState(false);

    if (pendingChanges.length === 0) return null;

    const safeIdx = Math.min(activeIdx, pendingChanges.length - 1);
    const change = pendingChanges[safeIdx];

    const handleAcceptAll = async () => {
        setAccepting(true);
        await acceptAllPendingChanges();
        setAccepting(false);
        setActiveIdx(0);
    };

    const shortPath = (p: string) => p.replace(/\\/g, '/').split('/').slice(-2).join('/');

    return (
        <div style={{
            position: 'absolute', top: 0, left: 0, right: 0, zIndex: 1000,
            background: 'var(--vscode-editor-background, #1e1e1e)',
            borderBottom: '2px solid var(--terminator-accent, #00c6ff)',
            boxShadow: '0 4px 24px rgba(0,0,0,0.6)',
            display: 'flex', flexDirection: 'column',
            maxHeight: '50vh',
            animation: 'slideDown 0.2s cubic-bezier(0,0,0.2,1)'
        }}>
            {/* Header */}
            <div style={{ display: 'flex', alignItems: 'center', padding: '8px 14px', gap: '10px', background: 'var(--vscode-editorWidget-background, #252526)', borderBottom: '1px solid rgba(255,255,255,0.07)', flexShrink: 0 }}>
                <Sparkles size={14} style={{ color: 'var(--terminator-accent, #00c6ff)', flexShrink: 0 }} />
                <span style={{ fontSize: '12px', fontWeight: 600, color: 'rgba(255,255,255,0.9)' }}>
                    AI Changes
                </span>
                <span style={{ fontSize: '11px', color: 'rgba(255,255,255,0.4)', background: 'rgba(255,255,255,0.07)', borderRadius: '10px', padding: '1px 8px' }}>
                    {pendingChanges.length} file{pendingChanges.length > 1 ? 's' : ''}
                </span>
                <div style={{ flex: 1 }} />
                <button onClick={handleAcceptAll} disabled={accepting} style={{ display: 'flex', alignItems: 'center', gap: '5px', padding: '4px 12px', borderRadius: '4px', border: 'none', background: 'var(--vscode-button-background, #0e639c)', color: 'white', fontSize: '12px', fontWeight: 600, cursor: 'pointer', opacity: accepting ? 0.6 : 1 }}>
                    <CheckCheck size={13} /> Accept All
                </button>
                <button onClick={() => { rejectAllPendingChanges(); setActiveIdx(0); }} style={{ display: 'flex', alignItems: 'center', gap: '5px', padding: '4px 10px', borderRadius: '4px', border: '1px solid rgba(248,113,113,0.5)', background: 'transparent', color: '#f87171', fontSize: '12px', fontWeight: 600, cursor: 'pointer' }}>
                    <XCircle size={13} /> Reject All
                </button>
            </div>

            {/* File tabs (if multiple) */}
            {pendingChanges.length > 1 && (
                <div style={{ display: 'flex', alignItems: 'center', overflowX: 'auto', background: 'rgba(0,0,0,0.2)', borderBottom: '1px solid rgba(255,255,255,0.07)', flexShrink: 0, gap: '2px', padding: '0 6px' }}>
                    <button onClick={() => setActiveIdx(i => Math.max(0, i - 1))} style={{ background: 'none', border: 'none', color: 'rgba(255,255,255,0.4)', cursor: 'pointer', padding: '4px 2px' }}><ChevronLeft size={14} /></button>
                    {pendingChanges.map((c, i) => (
                        <button key={c.id} onClick={() => setActiveIdx(i)} style={{
                            background: i === safeIdx ? 'rgba(255,255,255,0.08)' : 'none',
                            border: 'none', borderBottom: i === safeIdx ? '2px solid var(--terminator-accent, #00c6ff)' : '2px solid transparent',
                            color: i === safeIdx ? 'rgba(255,255,255,0.9)' : 'rgba(255,255,255,0.4)',
                            fontSize: '11px', padding: '5px 10px', cursor: 'pointer', whiteSpace: 'nowrap', transition: 'all 0.15s'
                        }}>
                            {shortPath(c.path)}
                        </button>
                    ))}
                    <button onClick={() => setActiveIdx(i => Math.min(pendingChanges.length - 1, i + 1))} style={{ background: 'none', border: 'none', color: 'rgba(255,255,255,0.4)', cursor: 'pointer', padding: '4px 2px' }}><ChevronRight size={14} /></button>
                </div>
            )}

            {/* Diff content */}
            {change && <FileDiff change={change} />}

            {/* Per-file actions */}
            <div style={{ display: 'flex', gap: '8px', padding: '8px 14px', background: 'rgba(0,0,0,0.15)', borderTop: '1px solid rgba(255,255,255,0.06)', flexShrink: 0, alignItems: 'center' }}>
                <span style={{ fontSize: '11px', color: 'rgba(255,255,255,0.4)', flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                    {change?.path}
                </span>
                <button onClick={() => { rejectPendingChange(change.id); if (safeIdx >= pendingChanges.length - 1) setActiveIdx(Math.max(0, safeIdx - 1)); }} style={{ display: 'flex', alignItems: 'center', gap: '4px', padding: '4px 10px', borderRadius: '4px', border: '1px solid rgba(248,113,113,0.5)', background: 'transparent', color: '#f87171', fontSize: '12px', cursor: 'pointer' }}>
                    <X size={12} /> Reject
                </button>
                <button onClick={async () => { await acceptPendingChange(change.id); if (safeIdx >= pendingChanges.length - 1) setActiveIdx(Math.max(0, safeIdx - 1)); }} style={{ display: 'flex', alignItems: 'center', gap: '4px', padding: '4px 12px', borderRadius: '4px', border: 'none', background: 'var(--vscode-button-background, #0e639c)', color: 'white', fontSize: '12px', fontWeight: 600, cursor: 'pointer' }}>
                    <Check size={12} /> Accept
                </button>
            </div>

            <style>{`
                @keyframes slideDown { from { transform: translateY(-100%); } to { transform: translateY(0); } }
            `}</style>
        </div>
    );
};

export default DiffViewer;
