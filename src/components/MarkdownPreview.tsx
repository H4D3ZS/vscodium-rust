import React, { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { useStore } from '../store';
import { parseMarkdown, isMarkdownPath } from '../lib/markdown';

// ─────────────────────────────────────────────────────────────────────────────
//  MarkdownPreview — resizable side-by-side preview for .md files.
//  Ctrl+Shift+V or toolbar button toggles visibility. Drag the left edge to
//  resize; layout adapts when the pane is narrow.
// ─────────────────────────────────────────────────────────────────────────────

const MarkdownPreview: React.FC = () => {
    const activeTab = useStore(s => {
        const id = s.activeTabId;
        return s.tabs.find((t: any) => t.id === id) || null;
    });
    const visible = useStore(s => (s as any).isMarkdownPreviewOpen);
    const close = useStore(s => (s as any).closeMarkdownPreview);
    const widthPct = useStore(s => (s as any).markdownPreviewWidthPct ?? 42);
    const setWidthPct = useStore(s => (s as any).setMarkdownPreviewWidthPct);

    const html = useMemo(() => parseMarkdown(activeTab?.content || ''), [activeTab?.content]);
    const isMd = !!activeTab && isMarkdownPath(activeTab.path || '');

    useEffect(() => {
        if (!isMd && visible) close?.();
    }, [isMd, visible, close]);

    const [collapsedToc, setCollapsedToc] = useState(false);
    const [narrow, setNarrow] = useState(false);
    const paneRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const el = paneRef.current;
        if (!el) return;
        const ro = new ResizeObserver(([entry]) => {
            setNarrow(entry.contentRect.width < 380);
        });
        ro.observe(el);
        return () => ro.disconnect();
    }, [visible]);

    const toc = useMemo(() => {
        if (!activeTab?.content) return [] as { depth: number; text: string }[];
        return activeTab.content.split('\n')
            .map(l => l.match(/^(#{1,4})\s+(.+)$/))
            .filter(Boolean)
            .map((m: RegExpMatchArray | null) => ({
                depth: (m![1] as string).length,
                text: m![2].trim(),
            }));
    }, [activeTab?.content]);

    const startResize = useCallback((e: React.MouseEvent) => {
        e.preventDefault();
        document.body.style.cursor = 'col-resize';
        document.body.classList.add('resizing');

        const onMove = (ev: MouseEvent) => {
            const row = paneRef.current?.parentElement;
            if (!row) return;
            const rect = row.getBoundingClientRect();
            const pct = ((rect.right - ev.clientX) / rect.width) * 100;
            setWidthPct?.(pct);
        };
        const onUp = () => {
            document.body.style.cursor = '';
            document.body.classList.remove('resizing');
            window.removeEventListener('mousemove', onMove);
            window.removeEventListener('mouseup', onUp);
        };
        window.addEventListener('mousemove', onMove);
        window.addEventListener('mouseup', onUp);
    }, [setWidthPct]);

    const fileName = (activeTab?.path || '').split(/[\\/]/).pop() || 'document.md';

    const handleExportPdf = () => {
        void import('../lib/markdownPdf').then(({ exportMarkdownToPdf }) => {
            exportMarkdownToPdf(fileName.replace(/\.md$/i, ''), html);
        });
    };

    if (!visible || !isMd) return null;

    const showToc = toc.length > 0 && !collapsedToc && !narrow;

    return (
        <>
            <div
                className="resizer-v markdown-preview-resizer"
                onMouseDown={startResize}
                title="Drag to resize preview"
            />
            <div
                ref={paneRef}
                className={`markdown-preview-pane${narrow ? ' markdown-preview-pane--narrow' : ''}`}
                style={{
                    flex: `0 0 ${widthPct}%`,
                    minWidth: 220,
                    maxWidth: '70%',
                }}
            >
                <div className="markdown-preview-toolbar">
                    <div className="markdown-preview-toolbar__title">
                        <i className="codicon codicon-open-preview" style={iconStyle} />
                        Preview · {fileName}
                    </div>
                    <div className="markdown-preview-toolbar__actions">
                        {toc.length > 0 && !narrow && (
                            <button onClick={() => setCollapsedToc(c => !c)} style={btnNeutral} title="Toggle table of contents">
                                <i className="codicon codicon-list-tree" style={iconStyle} />
                            </button>
                        )}
                        <button onClick={handleExportPdf} style={btnNeutral} title="Export as PDF (system print dialog — no extra bundle size)">
                            <i className="codicon codicon-printer" style={iconStyle} />
                        </button>
                        <button onClick={() => close?.()} style={btnNeutral} title="Close preview">
                            <i className="codicon codicon-close" style={iconStyle} />
                        </button>
                    </div>
                </div>
                <div className="markdown-preview-body">
                    {showToc && (
                        <nav className="markdown-preview-toc" aria-label="Table of contents">
                            <div className="markdown-preview-toc__label">Contents</div>
                            {toc.map((h, i) => (
                                <button
                                    key={i}
                                    type="button"
                                    className="markdown-preview-toc__item"
                                    style={{ paddingLeft: `${12 + (h.depth - 1) * 10}px` }}
                                    onClick={() => {
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
                                </button>
                            ))}
                        </nav>
                    )}
                    <div
                        className="markdown-content markdown-preview-content"
                        dangerouslySetInnerHTML={{ __html: html }}
                    />
                </div>
            </div>
        </>
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
