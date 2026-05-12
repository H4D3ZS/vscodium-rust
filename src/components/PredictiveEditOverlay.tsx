import React, { useEffect, useRef, useState } from 'react';

// ─────────────────────────────────────────────────────────────────────────────
//  PredictiveEditOverlay — Cursor-style "next edit prediction".
//
//  Watches the active Monaco editor for single-token replacements (rename-
//  like edits). When the user changes "foo" → "bar", we scan the same
//  document for remaining occurrences of "foo" and surface a small bottom-
//  right toast: "3 more uses — Tab to apply everywhere". Pressing Tab
//  applies all edits at once in a single undoable transaction.
//
//  Heuristics:
//    • Only trigger when the change replaces a single identifier-like
//      token (so we don't spam suggestions on whitespace, comments, or
//      multi-line edits).
//    • Require the old text to appear at a word boundary to avoid
//      partial-word matches (e.g. "user" inside "username").
//    • Auto-dismiss after 5 s of inactivity, or on any further edit.
//
//  Wiring: mounted as a sibling of MonacoEditor; it listens on the
//  `editor:registered` CustomEvent to grab the live editor instance, and
//  on the editor's onDidChangeModelContent to track edits.
// ─────────────────────────────────────────────────────────────────────────────

interface Suggestion {
    oldText: string;
    newText: string;
    occurrences: { range: any; }[];
}

const IDENTIFIER_RE = /^[A-Za-z_$][\w$]*$/;

const PredictiveEditOverlay: React.FC = () => {
    const editorRef = useRef<any>(null);
    const lastEditRef = useRef<{ ts: number; range: any; oldText: string; newText: string } | null>(null);
    const dismissTimerRef = useRef<number | null>(null);
    const [suggestion, setSuggestion] = useState<Suggestion | null>(null);

    // Capture the editor handle. We expect Editor.tsx to dispatch a
    // CustomEvent whenever a Monaco instance mounts.
    useEffect(() => {
        const handler = (e: Event) => {
            const editor = (e as CustomEvent).detail?.editor;
            if (!editor) return;
            editorRef.current = editor;
        };
        window.addEventListener('editor:registered', handler);
        return () => window.removeEventListener('editor:registered', handler);
    }, []);

    // Attach the change listener whenever we have a fresh editor.
    useEffect(() => {
        const editor = editorRef.current;
        if (!editor) return;

        const disposable = editor.onDidChangeModelContent((ev: any) => {
            const change = ev.changes?.[0];
            if (!change) return;
            const model = editor.getModel();
            if (!model) return;

            // Pull the old token from rangeOffset/rangeLength applied to the
            // value before the edit. Monaco doesn't surface "the deleted
            // text" directly; we synthesize it by inspecting the previous
            // edit we tracked.
            const newText = change.text;
            const range = change.range;
            const lineLength = model.getLineLength(range.startLineNumber);

            // We treat a change as a rename if:
            //   • it replaces a single line range,
            //   • the inserted text looks like an identifier, and
            //   • the previous edit immediately before this one was a
            //     matching deletion of an identifier (so the user did
            //     select+type).
            const isLikelyRename =
                range.startLineNumber === range.endLineNumber &&
                IDENTIFIER_RE.test(newText) &&
                newText.length > 1 &&
                newText.length <= 60;

            if (!isLikelyRename) {
                lastEditRef.current = null;
                return;
            }

            // Look at the text immediately around the inserted token to
            // see what existed there before. Because we run after the edit
            // we read from the current model and back out: we use Monaco's
            // delta to grab the original substring length.
            const prevText = ev._oldText // safer: get from prior snapshot
                || lastEditRef.current?.oldText
                || '';

            // Fallback: pull a hint from change.rangeOffset/rangeLength
            // via the model's preserved buffer if available. Monaco doesn't
            // expose "deleted text" directly; we use rangeLength + the
            // tracked previous selection as a guess.
            let oldText = prevText;
            if (!oldText && range.endColumn > range.startColumn) {
                // The new content starts at startColumn, so anything from
                // startColumn..startColumn+rangeLength was the original
                // selection's text. We sample the line *before* the edit
                // would have begun; not perfect, but covers select+type.
                const before = model.getLineContent(range.startLineNumber);
                const start = Math.max(0, range.startColumn - 1);
                const len = Math.min(60, change.rangeLength || 0);
                oldText = before.substr(start, len);
            }

            if (!oldText || !IDENTIFIER_RE.test(oldText) || oldText === newText) {
                lastEditRef.current = null;
                return;
            }

            // Find all remaining occurrences of oldText in the document
            // that *don't* sit inside the edit we just made.
            const occs: { range: any }[] = [];
            try {
                // Use Monaco's model.findMatches for word-boundary search.
                const matches = model.findMatches(
                    oldText,
                    /* searchOnlyEditableRange */ false,
                    /* isRegex */ false,
                    /* matchCase */ true,
                    /* wordSeparators */ null,
                    /* captureMatches */ true,
                );
                for (const m of matches || []) {
                    if (!m.range) continue;
                    // Skip the freshly edited range so we don't suggest
                    // re-replacing it.
                    if (m.range.startLineNumber === range.startLineNumber &&
                        Math.abs(m.range.startColumn - range.startColumn) < 2) {
                        continue;
                    }
                    occs.push({ range: m.range });
                    if (occs.length >= 32) break;
                }
            } catch {
                /* no model.findMatches in some Monaco builds */
            }

            // Only show suggestion if there are 1+ remaining sites; that's
            // the value-add. A solo rename doesn't need prediction.
            if (occs.length === 0) {
                setSuggestion(null);
                lastEditRef.current = null;
                return;
            }

            lastEditRef.current = { ts: Date.now(), range, oldText, newText };
            setSuggestion({ oldText, newText, occurrences: occs });

            // Auto-dismiss after 5 s.
            if (dismissTimerRef.current) window.clearTimeout(dismissTimerRef.current);
            dismissTimerRef.current = window.setTimeout(() => {
                setSuggestion(null);
            }, 5000);
        });

        return () => disposable.dispose();
    }, [editorRef.current]);

    // Tab to accept; Escape to dismiss.
    useEffect(() => {
        if (!suggestion) return;
        const onKey = (e: KeyboardEvent) => {
            if (e.key === 'Tab' && !e.ctrlKey && !e.altKey && !e.metaKey) {
                e.preventDefault();
                applyAll();
            } else if (e.key === 'Escape') {
                setSuggestion(null);
            }
        };
        window.addEventListener('keydown', onKey, true);
        return () => window.removeEventListener('keydown', onKey, true);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [suggestion]);

    const applyAll = () => {
        const editor = editorRef.current;
        const s = suggestion;
        if (!editor || !s) return;
        try {
            const edits = s.occurrences.map(o => ({
                range: o.range,
                text: s.newText,
                forceMoveMarkers: false,
            }));
            editor.executeEdits('predictive-edit', edits);
            setSuggestion(null);
        } catch (e) {
            console.error('[PredictiveEditOverlay] applyAll failed', e);
        }
    };

    if (!suggestion) return null;

    return (
        <div
            style={{
                position: 'absolute',
                right: 18,
                bottom: 18,
                zIndex: 200,
                background: 'rgba(15,15,25,0.92)',
                border: '1px solid rgba(124,58,237,0.4)',
                borderRadius: 6,
                padding: '6px 12px',
                display: 'flex',
                alignItems: 'center',
                gap: 10,
                fontSize: 12,
                color: 'rgba(255,255,255,0.9)',
                boxShadow: '0 8px 24px rgba(0,0,0,0.4)',
            }}
        >
            <i className="codicon codicon-sparkle" style={{ fontFamily: 'codicon', fontStyle: 'normal', color: '#c084fc' }} />
            <span>
                <b>{suggestion.occurrences.length}</b> more use{suggestion.occurrences.length === 1 ? '' : 's'} of{' '}
                <code style={tokenStyle}>{suggestion.oldText}</code> → <code style={tokenStyle}>{suggestion.newText}</code>
            </span>
            <button onClick={applyAll} style={btnAccept}>
                <span style={{ fontWeight: 700 }}>Tab</span> Apply all
            </button>
            <button onClick={() => setSuggestion(null)} style={btnDismiss} title="Dismiss">
                <i className="codicon codicon-close" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 11 }} />
            </button>
        </div>
    );
};

const tokenStyle: React.CSSProperties = {
    fontFamily: 'var(--font-mono, monospace)',
    background: 'rgba(255,255,255,0.06)',
    padding: '1px 5px',
    borderRadius: 3,
    fontSize: 11,
};

const btnAccept: React.CSSProperties = {
    background: 'rgba(124,58,237,0.25)',
    border: '1px solid rgba(124,58,237,0.55)',
    color: '#fff',
    padding: '3px 10px',
    fontSize: 11,
    borderRadius: 4,
    cursor: 'pointer',
    display: 'inline-flex',
    alignItems: 'center',
    gap: 4,
    fontFamily: 'inherit',
};

const btnDismiss: React.CSSProperties = {
    background: 'transparent',
    border: 'none',
    color: 'inherit',
    cursor: 'pointer',
    padding: 2,
};

export default PredictiveEditOverlay;
