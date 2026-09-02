import React, { useState, useMemo, useCallback } from 'react';
import { marked } from 'marked';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import { isToolCallJson, isToolResultJson, looksLikeToolCallText, summarizeToolCallText } from '../../domain/agent/cleanAgentContent';
import { sanitizeHtml } from '../../lib/markdown';

let markedConfigured = false;
function ensureMarkedConfig() {
    if (markedConfigured) return;
    marked.setOptions({ gfm: true, breaks: true });
    markedConfigured = true;
}

const TREE_LINE_RE = /^\s*[│├└─║╔╗╚╝╠╣╦╩╬══├└│─┬┤┘┌└╔╗╚╝├┤│╔═╗║╚╝╠╣╦╩╬═║╠╣╦╩╬]\s*/;
function isTreeBlock(text: string): boolean {
    const lines = text.split('\n').filter(l => l.trim());
    if (lines.length < 2) return false;
    let treeCount = 0;
    for (const line of lines) {
        if (TREE_LINE_RE.test(line) || /[├└│─╦╩╠╣╔╗╚╝]/.test(line)) treeCount++;
    }
    return treeCount >= Math.ceil(lines.length * 0.4);
}

function preProcessTreeBlocks(input: string): string {
    const lines = input.split('\n');
    const result: string[] = [];
    let i = 0;
    while (i < lines.length) {
        const line = lines[i];
        if (/[├└│─╦╩╠╣╔╗╚╝]/.test(line) && line.trim()) {
            const blockStart = i;
            while (i < lines.length && (lines[i].trim() === '' || /[├└│─╦╩╠╣╔╗╚╝]/.test(lines[i]))) i++;
            const block = lines.slice(blockStart, i).join('\n');
            if (isTreeBlock(block)) {
                result.push('```');
                result.push(block);
                result.push('```');
            } else {
                result.push(...lines.slice(blockStart, i));
            }
        } else {
            result.push(line);
            i++;
        }
    }
    return result.join('\n');
}

// ── Custom Interactive Blocks ────────────────────────────────────────────────
const ClarifyingQuestionBlock: React.FC<{ data: any }> = ({ data }) => {
    const [selected, setSelected] = useState<number | null>(null);
    return (
        <div style={{
            background: 'var(--vscode-editor-background)',
            border: '1px solid var(--vscode-widget-border)',
            borderRadius: '6px',
            margin: '12px 0',
            overflow: 'hidden',
            fontFamily: 'var(--vscode-font-family)'
        }}>
            <div style={{
                padding: '12px 16px',
                borderBottom: '1px solid var(--vscode-widget-border)',
                fontWeight: 600,
                fontSize: '13px'
            }}>
                {data.title || 'Clarifying Question'}
            </div>
            <div style={{ padding: '8px' }}>
                {(data.options || []).map((opt: string, idx: number) => (
                    <div 
                        key={idx}
                        onClick={() => setSelected(idx)}
                        style={{
                            padding: '8px 12px',
                            margin: '4px',
                            borderRadius: '4px',
                            background: selected === idx ? 'var(--vscode-button-background)' : 'rgba(255,255,255,0.05)',
                            color: selected === idx ? 'var(--vscode-button-foreground)' : 'var(--vscode-foreground)',
                            cursor: 'pointer',
                            fontSize: '12px',
                            display: 'flex',
                            alignItems: 'center',
                            gap: '8px'
                        }}
                    >
                        <span style={{ 
                            background: 'rgba(0,0,0,0.2)', 
                            borderRadius: '50%', 
                            width: '18px', 
                            height: '18px', 
                            display: 'flex', 
                            alignItems: 'center', 
                            justifyContent: 'center',
                            fontSize: '10px'
                        }}>{idx + 1}</span>
                        {opt}
                    </div>
                ))}
            </div>
            <div style={{
                padding: '8px 12px',
                borderTop: '1px solid var(--vscode-widget-border)',
                display: 'flex',
                justifyContent: 'flex-end',
                gap: '8px'
            }}>
                <button style={{ ...chipBtn, background: 'transparent', border: 'none' }}>Skip</button>
                <button style={{ ...chipBtn, background: 'var(--vscode-button-background)', color: 'var(--vscode-button-foreground)', border: 'none' }}>Continue</button>
            </div>
        </div>
    );
};

const SubagentsBlock: React.FC<{ data: any[] }> = ({ data }) => {
    return (
        <div style={{
            background: 'var(--vscode-editor-background)',
            border: '1px solid var(--vscode-widget-border)',
            borderRadius: '6px',
            margin: '12px 0',
            padding: '12px',
            fontFamily: 'var(--vscode-font-family)'
        }}>
            <div style={{ fontSize: '11px', color: 'var(--vscode-descriptionForeground)', marginBottom: '8px', display: 'flex', alignItems: 'center', gap: '6px' }}>
                <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontSize: '12px' }} />
                Started {data.length} subagents
            </div>
            {data.map((agent: any, idx: number) => (
                <div key={idx} style={{
                    padding: '8px 12px',
                    margin: '4px 0',
                    background: 'rgba(255,255,255,0.03)',
                    border: '1px solid rgba(255,255,255,0.05)',
                    borderRadius: '6px',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '12px'
                }}>
                    <i className={agent.status === 'running' ? 'codicon codicon-loading codicon-modifier-spin' : agent.status === 'done' ? 'codicon codicon-check' : 'codicon codicon-circle-large-outline'} 
                       style={{ color: agent.status === 'done' ? '#10b981' : agent.status === 'running' ? '#3b82f6' : 'inherit', fontSize: '14px' }} />
                    <div style={{ flex: 1 }}>
                        <div style={{ fontSize: '12px', fontWeight: 600 }}>{agent.task}</div>
                        <div style={{ fontSize: '10px', color: 'var(--vscode-descriptionForeground)', display: 'flex', alignItems: 'center', gap: '4px', marginTop: '2px' }}>
                            <i className="codicon codicon-hubot" style={{ fontSize: '10px' }} />
                            {agent.model}
                        </div>
                    </div>
                </div>
            ))}
        </div>
    );
};

// ─────────────────────────────────────────────────────────────────────────────
//  MessageBody — Cursor-style chat message renderer.
//
//  Splits an assistant message into alternating prose/code segments and
//  renders each code block with a header bar (language + optional target
//  path) plus three actions:
//    • Copy        — clipboard.writeText
//    • Apply       — write the block straight to the target file (or the
//                    active tab if the block didn't pin a path)
//    • Insert at cursor — paste at the active editor cursor (best-effort;
//                    requires the editor to expose a cursor.write event)
//
//  Code blocks can declare a target file with Cursor-style fences:
//      ```tsx:src/components/Foo.tsx
//      …
//      ```
//  We honour that hint so a model that writes patches across multiple
//  files lands each block in the right place.
// ─────────────────────────────────────────────────────────────────────────────

interface MessageBodyProps {
    content: string;
    /** Allow the parent to opt out of Apply actions on read-only messages
     *  (e.g. while the agent is still streaming). Defaults to `true`. */
    allowApply?: boolean;
}

// Parse one fence header line. The body of the regex tolerates several
// vendor flavours seen in the wild:
//   ```ts
//   ```tsx:src/foo.ts
//   ```ts src/foo.ts
//   ```tsx file=src/foo.ts
function parseFenceHeader(raw: string): { lang: string; path: string | null } {
    const header = raw.trim();
    if (!header) return { lang: 'text', path: null };
    // 1) lang:path  (Cursor's preferred form)
    const colon = header.match(/^([\w+-]+)\s*[:|]\s*(.+)$/);
    if (colon) return { lang: colon[1], path: colon[2].trim() };
    // 2) lang file=path
    const eq = header.match(/^([\w+-]+)\s+file=(.+)$/i);
    if (eq) return { lang: eq[1], path: eq[2].trim() };
    // 3) lang path/with/slash.ext
    const space = header.match(/^([\w+-]+)\s+(\S+\.[\w]+)$/);
    if (space) return { lang: space[1], path: space[2].trim() };
    return { lang: header, path: null };
}

type Segment =
    | { kind: 'prose'; text: string }
    | { kind: 'code'; lang: string; path: string | null; body: string };

// Split a markdown string into ordered prose/code segments. We do this with
// a stateful single pass because using marked's lexer round-trips through
// HTML tokens which is overkill — the only structure we care about is the
// fence boundary.
function splitSegments(input: string): Segment[] {
    ensureMarkedConfig();
    const processed = preProcessTreeBlocks(input);
    const out: Segment[] = [];
    const fence = /```([^\n]*)\n?([\s\S]*?)```/g;
    let last = 0;
    let m: RegExpExecArray | null;
    while ((m = fence.exec(processed))) {
        if (m.index > last) {
            out.push({ kind: 'prose', text: processed.slice(last, m.index) });
        }
        const { lang, path } = parseFenceHeader(m[1] || '');
        out.push({ kind: 'code', lang, path, body: m[2] });
        last = m.index + m[0].length;
    }
    if (last < processed.length) {
        out.push({ kind: 'prose', text: processed.slice(last) });
    }
    return out;
}

const ToolCallChip: React.FC<{ body: string }> = ({ body }) => {
    const summary = useMemo(() => summarizeToolCallText(body), [body]);
    const toolName = useMemo(
        () => body.match(/"(?:name|tool)"\s*:\s*"([^"]+)"/)?.[1] || 'tool',
        [body],
    );
    const icon =
        toolName.includes('browser') ? 'globe'
            : toolName.includes('write') || toolName.includes('file') ? 'file'
                : toolName.includes('grep') || toolName.includes('search') ? 'search'
                    : toolName.includes('bash') || toolName.includes('command') ? 'terminal'
                        : toolName.includes('web') || toolName.includes('audit') ? 'shield'
                            : 'tools';

    return (
        <div
            style={{
                display: 'flex',
                alignItems: 'center',
                gap: 8,
                margin: '6px 0',
                padding: '6px 10px',
                borderRadius: 8,
                border: '1px solid rgba(56,189,248,0.2)',
                background: 'rgba(56,189,248,0.06)',
                fontSize: 11,
                color: 'rgba(255,255,255,0.78)',
            }}
        >
            <i className={`codicon codicon-${icon}`} style={{ fontSize: 12, opacity: 0.7, color: '#38bdf8' }} />
            <span style={{ flex: 1, lineHeight: 1.45 }}>{summary}</span>
            <span style={{ fontSize: 9, opacity: 0.35, fontFamily: 'var(--font-mono, monospace)' }}>executed</span>
        </div>
    );
};

const CodeBlock: React.FC<{
    lang: string;
    path: string | null;
    body: string;
    allowApply: boolean;
}> = ({ lang, path, body, allowApply }) => {
    const [copied, setCopied] = useState(false);
    const [applied, setApplied] = useState<'ok' | 'err' | null>(null);
    const [busy, setBusy] = useState(false);
    const isToolPayload = useMemo(
        () => isToolCallJson(body.trim()) || isToolResultJson(body.trim()),
        [body],
    );
    const showApply = allowApply && !isToolPayload;

    const onCopy = useCallback(() => {
        navigator.clipboard.writeText(body).then(() => {
            setCopied(true);
            setTimeout(() => setCopied(false), 1500);
        }).catch(() => {});
    }, [body]);

    const onApply = useCallback(async () => {
        if (busy) return;
        setBusy(true);
        setApplied(null);
        try {
            // Prefer the fence-declared path; fall back to whichever tab is
            // active right now so the user can apply a code block to the
            // file they're staring at when the model forgot to pin a path.
            const target = path ||
                (useStore.getState().tabs.find((t: any) => t.id === useStore.getState().activeTabId)?.path);
            if (!target) {
                setApplied('err');
                return;
            }
            // Snapshot the workspace so the user can roll the apply back
            // via the checkpoint banner if they don't like the result.
            try {
                await invoke<any>('git_auto_checkpoint', { reason: `apply chat block → ${target}` }).catch(() => null);
            } catch { /* ignore — checkpoint is best-effort */ }
            await invoke('write_file_content', { path: target, content: body });
            setApplied('ok');
            // Tell the editor to reload the file if it's the active tab.
            window.dispatchEvent(new CustomEvent('editor:reload-file', { detail: { path: target } }));
            setTimeout(() => setApplied(null), 2500);
        } catch (e) {
            console.error('[MessageBody] Apply failed:', e);
            setApplied('err');
            setTimeout(() => setApplied(null), 3500);
        } finally {
            setBusy(false);
        }
    }, [body, busy, path]);

    const onInsert = useCallback(() => {
        // Pass to the editor; if no listener picks it up the apply button
        // is the fallback.
        window.dispatchEvent(new CustomEvent('editor:insert-at-cursor', { detail: { text: body } }));
    }, [body]);

    return (
        <div
            style={{
                border: '1px solid rgba(255,255,255,0.08)',
                borderRadius: 6,
                margin: '8px 0',
                background: 'rgba(0,0,0,0.25)',
                overflow: 'hidden',
            }}
        >
            <div
                style={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    padding: '4px 10px',
                    background: 'rgba(255,255,255,0.04)',
                    borderBottom: '1px solid rgba(255,255,255,0.06)',
                    fontSize: 10,
                    fontFamily: 'var(--font-mono, monospace)',
                    color: 'rgba(255,255,255,0.75)',
                }}
            >
                <div style={{ display: 'flex', gap: 8, alignItems: 'center', minWidth: 0 }}>
                    <span style={{ opacity: 0.6 }}>{lang || 'text'}</span>
                    {path && (
                        <span
                            style={{ opacity: 0.85, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}
                            title={path}
                        >
                            {path}
                        </span>
                    )}
                </div>
                <div style={{ display: 'flex', gap: 6 }}>
                    <button
                        onClick={onCopy}
                        style={chipBtn}
                        title="Copy to clipboard"
                    >
                        <i className={`codicon codicon-${copied ? 'check' : 'copy'}`} style={iconStyle} />
                        {copied ? 'Copied' : 'Copy'}
                    </button>
                    {showApply && (
                        <>
                            <button
                                onClick={onInsert}
                                style={chipBtn}
                                title="Insert at editor cursor"
                            >
                                <i className="codicon codicon-arrow-down" style={iconStyle} />
                                Insert
                            </button>
                            <button
                                onClick={onApply}
                                disabled={busy}
                                style={{
                                    ...chipBtn,
                                    background:
                                        applied === 'ok'  ? 'rgba(16,185,129,0.18)' :
                                        applied === 'err' ? 'rgba(239,68,68,0.18)' :
                                                            'rgba(124,58,237,0.18)',
                                    border:
                                        applied === 'ok'  ? '1px solid rgba(16,185,129,0.4)' :
                                        applied === 'err' ? '1px solid rgba(239,68,68,0.4)' :
                                                            '1px solid rgba(124,58,237,0.4)',
                                    color:
                                        applied === 'ok'  ? '#10b981' :
                                        applied === 'err' ? '#ef4444' :
                                                            '#c084fc',
                                }}
                                title={path ? `Apply to ${path}` : 'Apply to active file'}
                            >
                                <i
                                    className={`codicon codicon-${
                                        busy ? 'sync' : applied === 'ok' ? 'check' : applied === 'err' ? 'error' : 'check-all'
                                    }`}
                                    style={iconStyle}
                                />
                                {busy ? 'Applying…' : applied === 'ok' ? 'Applied' : applied === 'err' ? 'Failed' : 'Apply'}
                            </button>
                        </>
                    )}
                </div>
            </div>
            <pre style={{
                margin: 0,
                padding: 10,
                overflowX: 'auto',
                fontSize: 12,
                lineHeight: 1.55,
                fontFamily: 'var(--font-mono, monospace)',
                whiteSpace: 'pre',
                color: 'rgba(255,255,255,0.92)',
            }}>
                <code>{body}</code>
            </pre>
        </div>
    );
};

const chipBtn: React.CSSProperties = {
    background: 'rgba(255,255,255,0.06)',
    border: '1px solid rgba(255,255,255,0.1)',
    color: 'rgba(255,255,255,0.85)',
    padding: '2px 8px',
    borderRadius: 4,
    fontSize: 10,
    cursor: 'pointer',
    display: 'inline-flex',
    alignItems: 'center',
    gap: 4,
    fontFamily: 'inherit',
};

const iconStyle: React.CSSProperties = {
    fontFamily: 'codicon',
    fontStyle: 'normal',
    fontSize: 10,
};

// Memoized: props are primitives (content, allowApply), so a message only
// re-renders when its own content changes. Critical during streaming — without
// this, every token re-rendered the whole chat history and re-ran marked.parse
// on every prior message.
const MessageBody = React.memo(({ content, allowApply = true }: MessageBodyProps) => {
    const segments = useMemo(() => splitSegments(content || ''), [content]);
    return (
        <div className="markdown-content" style={{ fontSize: 13, lineHeight: 1.6 }}>
            {segments.map((seg, i) => {
                if (seg.kind === 'prose') {
                    const trimmed = seg.text.replace(/^\s+|\s+$/g, '');
                    if (!trimmed) return null;
                    let html: string;
                    try {
                        html = sanitizeHtml(marked.parse(trimmed, { async: false }) as string);
                    } catch {
                        html = trimmed.replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/\n/g, '<br/>');
                    }
                    return (
                        <div
                            key={i}
                            dangerouslySetInnerHTML={{ __html: html }}
                        />
                    );
                }
                if (seg.kind === 'code') {
                    if (seg.lang === 'json:question') {
                        try {
                            const data = JSON.parse(seg.body);
                            return <ClarifyingQuestionBlock key={i} data={data} />;
                        } catch (e) {
                            // fallback
                        }
                    }
                    if (seg.lang === 'json:subagents') {
                        try {
                            const data = JSON.parse(seg.body);
                            return <SubagentsBlock key={i} data={data} />;
                        } catch (e) {
                            // fallback
                        }
                    }
                    const trimmedBody = seg.body.trim();
                    if (
                        looksLikeToolCallText(trimmedBody)
                        || isToolCallJson(trimmedBody)
                        || isToolResultJson(trimmedBody)
                        || (seg.lang === 'json' && trimmedBody.startsWith('{') && /"name"\s*:/.test(trimmedBody))
                    ) {
                        return <ToolCallChip key={i} body={trimmedBody} />;
                    }
                }

                return (
                    <CodeBlock
                        key={i}
                        lang={seg.lang}
                        path={seg.path}
                        body={seg.body}
                        allowApply={allowApply}
                    />
                );
            })}
        </div>
    );
});

MessageBody.displayName = 'MessageBody';

export default MessageBody;
