import React from 'react';
import { useStore } from '../../store';

// Antigravity-style completion card: shows after a run finishes, with an
// "N files changed" chip and one-click walkthrough generation.

export function messageIsCompletion(content: string): boolean {
    if (!content) return false;
    return /MISSION_ACCOMPLISHED|TASK_COMPLETE/i.test(content);
}

const WalkthroughCard: React.FC = () => {
    const summary = useStore(s => s.lastRunSummary);
    const isAgentThinking = useStore(s => s.isAgentThinking);
    const activeRoot = useStore(s => s.activeRoot);
    const filesChanged = summary?.filesChanged ?? 0;

    const [busy, setBusy] = React.useState(false);
    const wpath = activeRoot ? `${activeRoot}/walkthrough.md` : '';

    const openPreview = async (): Promise<boolean> => {
        if (!wpath) return false;
        try {
            const { invoke } = await import('../../tauri_bridge');
            const content = await invoke<string>('read_file', { path: wpath });
            if (!content || !content.trim()) return false;
            const { openFileWithMarkdownPreview } = await import('../../application/editor/openFile');
            await openFileWithMarkdownPreview(wpath); // rendered, Antigravity-style
            return true;
        } catch { return false; }
    };

    const genWalkthrough = async () => {
        if (!activeRoot || busy) return;
        setBusy(true);
        try {
            const { sendAgentMessage } = await import('../../agent');
            const prompt =
                `[ANTIGRAVITY WALKTHROUGH] Write a polished walkthrough.md for the changes just made. ` +
                `Markdown sections: ## Overview, ## Architecture (key files + their roles as a bullet list), ` +
                `## Step-by-Step, ## Testing, ## Gotchas. Keep it concise and skimmable. ` +
                `You MUST save it to \`${wpath}\` using the write_to_file tool — this is the only required action.`;
            await sendAgentMessage(prompt, () => {});
            // Auto-open the freshly written file in rendered preview.
            await openPreview();
        } finally { setBusy(false); }
    };

    const openWalkthrough = async () => {
        // Open the rendered preview; if it doesn't exist yet, generate it.
        if (!(await openPreview())) await genWalkthrough();
    };

    return (
        <div className="ac-card">
            <div className="ac-card__title">
                <i className="codicon codicon-pass-filled" style={{ color: 'var(--ac-green)' }} />
                Task complete
                {filesChanged > 0 && (
                    <span className="ac-chip ac-chip--ok" style={{ marginLeft: 'auto' }}>
                        <i className="codicon codicon-diff" />
                        {filesChanged} file{filesChanged > 1 ? 's' : ''} changed
                    </span>
                )}
            </div>
            <div className="ac-card__actions">
                <button className="ac-btn ac-btn--primary" onClick={genWalkthrough} disabled={isAgentThinking || busy}>
                    <i className={`codicon codicon-${busy ? 'sync~spin' : 'book'}`} /> {busy ? 'Generating…' : 'Generate walkthrough'}
                </button>
                <button className="ac-btn ac-btn--ghost" onClick={openWalkthrough} disabled={busy}>
                    <i className="codicon codicon-go-to-file" /> Open walkthrough
                </button>
            </div>
        </div>
    );
};

export default WalkthroughCard;
