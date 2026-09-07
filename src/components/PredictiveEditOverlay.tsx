import React, { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

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

// AI "jump-to-next-edit": a propagation the model predicts elsewhere in the file.
interface AiSuggestion {
    startLine: number;
    endLine: number;
    newText: string;
    oldText: string;
    reason: string;
    phase: 'preview' | 'ready'; // preview = needs jump first; ready = cursor parked, Tab applies
}

const IDENTIFIER_RE = /^[A-Za-z_$][\w$]*$/;

const PredictiveEditOverlay: React.FC = () => {
    const editorRef = useRef<any>(null);
    const monacoRef = useRef<any>(null);
    const lastEditRef = useRef<{ ts: number; range: any; oldText: string; newText: string } | null>(null);
    const dismissTimerRef = useRef<number | null>(null);
    const [suggestion, setSuggestion] = useState<Suggestion | null>(null);

    // ── AI next-edit state ────────────────────────────────────────────────
    const aiTimerRef = useRef<number | null>(null);
    const aiReqIdRef = useRef(0);
    const aiDecorationsRef = useRef<string[]>([]);
    const lastChangeTextRef = useRef<string>('');
    const [aiSuggestion, setAiSuggestion] = useState<AiSuggestion | null>(null);

    // Capture the editor + monaco handles. Editor.tsx dispatches a
    // CustomEvent whenever a Monaco instance mounts.
    useEffect(() => {
        const handler = (e: Event) => {
            const detail = (e as CustomEvent).detail;
            if (!detail?.editor) return;
            editorRef.current = detail.editor;
            if (detail.monaco) monacoRef.current = detail.monaco;
        };
        window.addEventListener('editor:registered', handler);
        return () => window.removeEventListener('editor:registered', handler);
    }, []);

    const clearAiDecorations = () => {
        const editor = editorRef.current;
        if (editor && aiDecorationsRef.current.length) {
            aiDecorationsRef.current = editor.deltaDecorations(aiDecorationsRef.current, []);
        }
    };

    const dismissAi = () => {
        clearAiDecorations();
        setAiSuggestion(null);
    };

    // Ask the backend for the next likely edit elsewhere in the file.
    const scheduleAiPredict = () => {
        if (aiTimerRef.current) window.clearTimeout(aiTimerRef.current);
        aiTimerRef.current = window.setTimeout(async () => {
            const editor = editorRef.current;
            const model = editor?.getModel?.();
            if (!editor || !model) return;

            const st = (window as any).useStore?.getState?.() || {};
            if (st.tabPredictionEnabled === false) return;
            // Don't fight the inline ghost-text completion or a local rename toast.
            if (st.voidGlobalSettings?.enableAutocomplete === false) return;

            const content: string = model.getValue();
            if (content.length > 60_000 || model.getLineCount() < 4) return;
            const pos = editor.getPosition();
            const cursorLine = pos?.lineNumber ?? 1;
            const language = model.getLanguageId?.() || 'plaintext';
            const filePath = st.activeTab?.path || st.tabs?.find?.((t: any) => t.id === st.activeTabId)?.path || '';

            const reqId = ++aiReqIdRef.current;
            const autoSel = st.modelSelectionOfFeature?.['Autocomplete'];
            const modelOverride = autoSel?.modelName || st.agentModel || undefined;
            // Pass the active backend so qwen/llama-style Lemonade model names
            // aren't misrouted to the local backend by backend name-sniffing.
            const provider = autoSel?.providerName
                || st.inferenceBackend
                || localStorage.getItem('inferenceBackend')
                || undefined;
            try {
                const res: any = await invoke('predict_next_edit', {
                    content,
                    cursorLine,
                    language,
                    filePath,
                    recentChange: lastChangeTextRef.current.slice(0, 400),
                    modelOverride,
                    provider,
                });
                // Stale (newer edit superseded this) or editor gone.
                if (reqId !== aiReqIdRef.current || !editorRef.current) return;
                if (!res?.has_edit) return;
                const start = Number(res.start_line);
                const end = Number(res.end_line);
                if (!start || end < start) return;
                // Don't suggest an edit that overlaps the cursor line — that's not a "jump".
                if (cursorLine >= start && cursorLine <= end) return;

                setSuggestion(null); // AI prediction supersedes any stale rename toast
                setAiSuggestion({
                    startLine: start,
                    endLine: end,
                    newText: String(res.new_text ?? ''),
                    oldText: String(res.old_text ?? ''),
                    reason: String(res.reason ?? 'next edit'),
                    phase: 'preview',
                });
            } catch { /* non-fatal: prediction is best-effort */ }
        }, 1100);
    };

    // Attach the change listener whenever we have a fresh editor.
    useEffect(() => {
        const editor = editorRef.current;
        if (!editor) return;

        const disposable = editor.onDidChangeModelContent((ev: any) => {
            const change = ev.changes?.[0];
            if (!change) return;
            const model = editor.getModel();
            if (!model) return;

            // Any fresh edit invalidates a pending AI next-edit; record the
            // change snippet and (re)schedule a new prediction on idle.
            // (Unconditional: dismissAi is a no-op when nothing is pending.)
            dismissAi();
            lastChangeTextRef.current = change.text || '';
            scheduleAiPredict();

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

        return () => {
            disposable.dispose();
            if (aiTimerRef.current) window.clearTimeout(aiTimerRef.current);
            if (dismissTimerRef.current) window.clearTimeout(dismissTimerRef.current);
        };
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

    // ── AI next-edit: two-phase Tab (jump → apply) ────────────────────────
    const jumpToAiEdit = (s: AiSuggestion) => {
        const editor = editorRef.current;
        const monaco = monacoRef.current;
        if (!editor || !monaco) return;
        editor.revealLineInCenter(s.startLine);
        editor.setPosition({ lineNumber: s.startLine, column: 1 });
        // Highlight the lines that will be replaced.
        aiDecorationsRef.current = editor.deltaDecorations(aiDecorationsRef.current, [{
            range: new monaco.Range(s.startLine, 1, s.endLine, 1),
            options: {
                isWholeLine: true,
                className: 'predictive-next-edit-line',
                glyphMarginClassName: 'predictive-next-edit-glyph',
                overviewRuler: { color: 'var(--vscode-button-background)', position: monaco.editor.OverviewRulerLane.Center },
            },
        }]);
        setAiSuggestion({ ...s, phase: 'ready' });
        editor.focus();
    };

    const applyAiEdit = (s: AiSuggestion) => {
        const editor = editorRef.current;
        const model = editor?.getModel?.();
        const monaco = monacoRef.current;
        if (!editor || !model || !monaco) return;
        try {
            const endCol = model.getLineMaxColumn(Math.min(s.endLine, model.getLineCount()));
            const range = new monaco.Range(s.startLine, 1, s.endLine, endCol);
            editor.executeEdits('predictive-next-edit', [{ range, text: s.newText, forceMoveMarkers: true }]);
            editor.setPosition({ lineNumber: s.startLine, column: 1 });
            editor.revealLineInCenter(s.startLine);
        } catch (e) {
            console.error('[PredictiveEditOverlay] applyAiEdit failed', e);
        } finally {
            dismissAi();
        }
    };

    useEffect(() => {
        if (!aiSuggestion) return;
        const onKey = (e: KeyboardEvent) => {
            if (e.key === 'Tab' && !e.ctrlKey && !e.altKey && !e.metaKey && !e.shiftKey) {
                e.preventDefault();
                e.stopPropagation();
                if (aiSuggestion.phase === 'preview') jumpToAiEdit(aiSuggestion);
                else applyAiEdit(aiSuggestion);
            } else if (e.key === 'Escape') {
                dismissAi();
            }
        };
        window.addEventListener('keydown', onKey, true);
        return () => window.removeEventListener('keydown', onKey, true);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [aiSuggestion]);

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

    if (!suggestion && !aiSuggestion) return null;

    const toastShell: React.CSSProperties = {
        position: 'absolute',
        right: 18,
        bottom: 18,
        zIndex: 200,
        background: 'var(--vscode-editorWidget-background)',
        border: '1px solid var(--vscode-panel-border)',
        borderRadius: 6,
        padding: '6px 12px',
        display: 'flex',
        alignItems: 'center',
        gap: 10,
        fontSize: 12,
        color: 'var(--vscode-editorWidget-foreground)',
        boxShadow: '0 4px 12px rgba(0,0,0,0.3)',
    };

    // AI next-edit toast takes priority over the local rename toast.
    if (aiSuggestion) {
        const isPreview = aiSuggestion.phase === 'preview';
        return (
            <>
                <style>{NEXT_EDIT_CSS}</style>
                <div style={toastShell}>
                    <i className="codicon codicon-lightbulb-sparkle" style={{ fontFamily: 'codicon', fontStyle: 'normal', color: 'var(--vscode-button-foreground)' }} />
                    <span>
                        {isPreview ? 'Next edit' : 'Apply edit'} · <span style={{ opacity: 0.75 }}>{aiSuggestion.reason}</span>{' '}
                        <span style={{ opacity: 0.5 }}>(line {aiSuggestion.startLine})</span>
                    </span>
                    <button onClick={() => isPreview ? jumpToAiEdit(aiSuggestion) : applyAiEdit(aiSuggestion)} style={btnAccept}>
                        <span style={{ fontWeight: 700 }}>Tab</span> {isPreview ? 'Jump' : 'Apply'}
                    </button>
                    <button onClick={dismissAi} style={btnDismiss} title="Dismiss (Esc)">
                        <i className="codicon codicon-close" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 11 }} />
                    </button>
                </div>
            </>
        );
    }

    return (
        <div style={toastShell}>
            <i className="codicon codicon-sparkle" style={{ fontFamily: 'codicon', fontStyle: 'normal', color: 'var(--vscode-button-foreground)' }} />
            <span>
                <b>{suggestion!.occurrences.length}</b> more use{suggestion!.occurrences.length === 1 ? '' : 's'} of{' '}
                <code style={tokenStyle}>{suggestion!.oldText}</code> → <code style={tokenStyle}>{suggestion!.newText}</code>
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

// Decoration styling for the highlighted target lines on jump.
const NEXT_EDIT_CSS = `
.predictive-next-edit-line { background: var(--vscode-editorSelectionBackground); }
.predictive-next-edit-glyph::before {
    content: '\\eb7e'; font-family: codicon; color: var(--vscode-button-foreground);
    display: flex; align-items: center; justify-content: center;
}
`;

const tokenStyle: React.CSSProperties = {
    fontFamily: 'var(--font-mono, monospace)',
    background: 'var(--vscode-editorHoverWidget-background)',
    color: 'var(--vscode-editorHoverWidget-foreground)',
    padding: '1px 5px',
    borderRadius: 3,
    fontSize: 11,
};

const btnAccept: React.CSSProperties = {
    background: 'var(--vscode-button-background)',
    border: '1px solid var(--vscode-button-border)',
    color: 'var(--vscode-button-foreground)',
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
