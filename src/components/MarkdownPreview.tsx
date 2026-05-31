import React, { useEffect, useMemo, useState } from 'react';
import { marked } from 'marked';
import { useStore } from '../store';

// ─────────────────────────────────────────────────────────────────────────────
//  MarkdownPreview — VS Code-style side-by-side preview for .md files.
//
//  Mounted to the right of the editor when the active file is markdown
//  and the user has toggled preview on (Ctrl+Shift+V or the preview
//  button in the editor toolbar). Reads content from the active tab in
//  the store so it stays live as the user edits — no debouncing because
//  marked is fast and the document is usually small.
// ─────────────────────────────────────────────────────────────────────────────

const MarkdownPreview: React.FC = () => {
    const activeTab = useStore(s => {
        const id = s.activeTabId;
        return s.tabs.find((t: any) => t.id === id) || null;
    });
    const visible = useStore(s => (s as any).isMarkdownPreviewOpen);
    const close = useStore(s => (s as any).closeMarkdownPreview);

    // Track the rendered HTML in state so opening the preview reruns it
    // when content changes. Marked is synchronous in modern versions, but
    // we still memoize so React doesn't re-parse on unrelated re-renders.
    const html = useMemo(() => {
        if (!activeTab?.content) return '';
        try {
            return marked.parse(activeTab.content) as string;
        } catch (e) {
            return `<pre style="color:#f87171;">Markdown parse error: ${String(e)}</pre>`;
        }
    }, [activeTab?.content]);

    // Auto-close when the active file leaves markdown — saves a click.
    const isMd = !!activeTab && (
        activeTab.language === 'markdown' ||
        /\.(md|markdown|mdx?)$/i.test(activeTab.path || '')
    );
    useEffect(() => {
        if (!isMd && visible) close?.();
    }, [isMd, visible, close]);

    const [collapsedToc, setCollapsedToc] = useState(false);
    const toc = useMemo(() => {
        if (!activeTab?.content) return [] as { depth: number; text: string }[];
        const lines = activeTab.content.split('\n');
        return lines
            .map(l => l.match(/^(#{1,4})\s+(.+)$/))
            .filter(Boolean)
            .map((m: RegExpMatchArray | null) => ({
                depth: (m![1] as string).length,
                text: m![2].trim(),
            }));
    }, [activeTab?.content]);

    if (!visible || !isMd) return null;

    return (
        <div
            style={{
                flex: '0 0 50%',
                borderLeft: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.06))',
                background: 'var(--vscode-editor-background)',
                display: 'flex',
                flexDirection: 'column',
                overflow: 'hidden',
            }}
        >
            <div
                style={{
                    padding: '4px 10px',
                    borderBottom: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.06))',
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                    fontSize: 11,
                    opacity: 0.85,
                }}
            >
                <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                    <i className="codicon codicon-preview" style={iconStyle} />
                    Preview · {(activeTab?.path || '').split(/[\\/]/).pop()}
                </div>
                <div style={{ display: 'flex', gap: 6 }}>
                    {toc.length > 0 && (
                        <button onClick={() => setCollapsedToc(c => !c)} style={btnNeutral} title="Toggle table of contents">
                            <i className="codicon codicon-list-tree" style={iconStyle} />
                        </button>
                    )}
                    <button onClick={() => close?.()} style={btnNeutral} title="Close preview">
                        <i className="codicon codicon-close" style={iconStyle} />
                    </button>
                </div>
            </div>
            <div style={{ display: 'flex', flex: 1, minHeight: 0 }}>
                {!collapsedToc && toc.length > 0 && (
                    <div
                        style={{
                            flex: '0 0 180px',
                            overflowY: 'auto',
                            borderRight: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.06))',
                            padding: '8px 0',
                            background: 'rgba(0,0,0,0.15)',
                        }}
                    >
                        <div style={{ padding: '0 12px', fontSize: 10, opacity: 0.5, marginBottom: 6, textTransform: 'uppercase', letterSpacing: 0.5 }}>
                            Contents
                        </div>
                        {toc.map((h, i) => (
                            <div
                                key={i}
                                style={{
                                    padding: `2px 12px 2px ${12 + (h.depth - 1) * 10}px`,
                                    fontSize: 11,
                                    opacity: 0.8,
                                    cursor: 'pointer',
                                    whiteSpace: 'nowrap',
                                    overflow: 'hidden',
                                    textOverflow: 'ellipsis',
                                }}
                                onClick={() => {
                                    // Jump the editor to the heading line. We
                                    // re-derive the line number by scanning the
                                    // text for the heading match.
                                    const ix = (activeTab?.content || '').split('\n').findIndex(l =>
                                        l.match(new RegExp(`^#{${h.depth}}\\s+${h.text.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}\\s*$`))
                                    );
                                    if (ix >= 0) {
                                        window.dispatchEvent(new CustomEvent('editor:jump-to-line', {
                                            detail: { path: activeTab?.path, line: ix + 1, column: 1 },
                                        }));
                                    }
                                }}
                                title={h.text}
                            >
                                {h.text}
                            </div>
                        ))}
                    </div>
                )}
                <div
                    className="markdown-content"
                    style={{
                        flex: 1,
                        overflowY: 'auto',
                        padding: '14px 20px',
                        fontSize: 13,
                        lineHeight: 1.7,
                    }}
                    dangerouslySetInnerHTML={{ __html: html }}
                />
            </div>
        </div>
    );
};

const btnNeutral: React.CSSProperties = {
    background: 'transparent',
    border: 'none',
    color: 'inherit',
    cursor: 'pointer',
    padding: 2,
    fontSize: 12,
};

const iconStyle: React.CSSProperties = {
    fontFamily: 'codicon',
    fontStyle: 'normal',
};

export default MarkdownPreview;
