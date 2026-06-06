import { marked } from 'marked';
import DOMPurify from 'dompurify';

let configured = false;

function ensureMarkedConfig() {
    if (configured) return;
    marked.setOptions({
        gfm: true,
        breaks: true,
    });
    configured = true;
}

/** VS Code–style GFM parse + sanitize for preview panes. */
export function parseMarkdown(source: string): string {
    ensureMarkedConfig();
    if (!source?.trim()) return '';
    try {
        const raw = marked.parse(source, { async: false }) as string;
        return DOMPurify.sanitize(raw, {
            ADD_ATTR: ['target'],
            ALLOWED_URI_REGEXP: /^(?:(?:https?|mailto|data):|[^a-z]|[a-z+.\-]+(?:[^a-z+.\-:]|$))/i,
        });
    } catch (e) {
        return `<pre style="color:#f87171;">Markdown parse error: ${String(e)}</pre>`;
    }
}

export function isMarkdownPath(path: string): boolean {
    return /\.(md|markdown|mdx?)$/i.test(path || '');
}

export function isReportMarkdownPath(path: string): boolean {
    if (!isMarkdownPath(path)) return false;
    const norm = path.replace(/\\/g, '/').toLowerCase();
    return norm.includes('/reports/') || norm.includes('/recon/') || norm.endsWith('-report.md');
}
