import React, { useState, useMemo, useCallback } from 'react';
import { marked } from 'marked';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';

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
    const out: Segment[] = [];
    const fence = /```([^\n]*)\n([\s\S]*?)```/g;
    let last = 0;
    let m: RegExpExecArray | null;
    while ((m = fence.exec(input))) {
        if (m.index > last) {
            out.push({ kind: 'prose', text: input.slice(last, m.index) });
        }
        const { lang, path } = parseFenceHeader(m[1] || '');
        out.push({ kind: 'code', lang, path, body: m[2] });
        last = m.index + m[0].length;
    }
    if (last < input.length) {
        out.push({ kind: 'prose', text: input.slice(last) });
    }
    return out;
}

const CodeBlock: React.FC<{
    lang: string;
    path: string | null;
    body: string;
    allowApply: boolean;
}> = ({ lang, path, body, allowApply }) => {
    const [copied, setCopied] = useState(false);
    const [applied, setApplied] = useState<'ok' | 'err' | null>(null);
    const [busy, setBusy] = useState(false);

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
                    {allowApply && (
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

const MessageBody: React.FC<MessageBodyProps> = ({ content, allowApply = true }) => {
    const segments = useMemo(() => splitSegments(content || ''), [content]);
    return (
        <div className="markdown-content" style={{ fontSize: 13, lineHeight: 1.6 }}>
            {segments.map((seg, i) => {
                if (seg.kind === 'prose') {
                    const trimmed = seg.text.replace(/^\s+|\s+$/g, '');
                    if (!trimmed) return null;
                    return (
                        <div
                            key={i}
                            dangerouslySetInnerHTML={{ __html: marked.parse(trimmed) as string }}
                        />
                    );
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
};

export default MessageBody;
