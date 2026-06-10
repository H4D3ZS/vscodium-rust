/** Cursor-style tool invocation cards shown inline in the agent chat. */

import { canonicalToolName, toolsMatchForFinish } from './toolAliases';

export type AgentToolBlockKind = 'terminal' | 'read' | 'edit' | 'search' | 'todo' | 'canvas' | 'generic';
export type AgentToolBlockStatus = 'running' | 'done' | 'error';

export type AgentTodoStatus = 'pending' | 'in_progress' | 'completed' | 'cancelled';

export interface AgentTodoItem {
    id: string;
    text: string;
    status: AgentTodoStatus;
}

export interface AgentToolBlock {
    id: string;
    kind: AgentToolBlockKind;
    tool: string;
    title: string;
    status: AgentToolBlockStatus;
    ts: number;
    streamId?: string;
    command?: string;
    path?: string;
    lineRange?: string;
    pattern?: string;
    outputLines: string[];
    preview?: string;
    oldText?: string;
    newText?: string;
    todos?: AgentTodoItem[];
    exitCode?: number | null;
    /** For canvas blocks: id/title used by the "Open canvas" chip. */
    canvasId?: string;
    canvasTitle?: string;
}

export function parseToolArgs(raw: unknown): Record<string, unknown> {
    if (!raw) return {};
    if (typeof raw === 'string') {
        try {
            const p = JSON.parse(raw);
            return typeof p === 'object' && p ? (p as Record<string, unknown>) : { raw };
        } catch {
            return { raw };
        }
    }
    if (typeof raw === 'object') return raw as Record<string, unknown>;
    return {};
}

export function classifyToolKind(tool: string): AgentToolBlockKind {
    const t = canonicalToolName(tool);
    if (t === 'create_canvas' || t.includes('canvas')) return 'canvas';
    if (t.includes('todo_write') || t === 'todo_write' || t.includes('task_create')) return 'todo';
    if (t === 'run_command' || t.includes('terminal')) return 'terminal';
    if (t === 'view_file' || t.includes('file_read') || t.includes('read_file')) return 'read';
    if (t.includes('write') || t.includes('edit') || t.includes('replace') || t.includes('patch') || t.includes('apply')) return 'edit';
    if (t === 'grep' || t === 'find_by_name' || t === 'search_files' || t === 'list_files' || t === 'search_codebase' || t.includes('glob') || t.includes('grep') || t.includes('search') || t.includes('list') || t.includes('find')) return 'search';
    return 'generic';
}

// Re-export for callers that imported from this module
export { canonicalToolName, toolsMatchForFinish } from './toolAliases';

/** Quiet recon steps (glob/list/grep) collapse unless running or failed. */
export function isQuietReconBlock(block: AgentToolBlock): boolean {
    if (block.status === 'running' || block.status === 'error') return false;
    if (block.kind === 'terminal' || block.kind === 'edit' || block.kind === 'todo') return false;
    return block.kind === 'search' || block.kind === 'read' || block.kind === 'generic';
}

export interface ReconSummary {
    type: 'recon-summary';
    id: string;
    count: number;
    labels: string[];
}

export type DisplayToolBlock = AgentToolBlock | ReconSummary;

export function collapseToolBlocksForDisplay(blocks: AgentToolBlock[]): DisplayToolBlock[] {
    const out: DisplayToolBlock[] = [];
    let quiet: AgentToolBlock[] = [];

    const flushQuiet = () => {
        if (!quiet.length) return;
        if (quiet.length === 1) {
            out.push(quiet[0]);
        } else {
            out.push({
                type: 'recon-summary',
                id: `recon-${quiet[0].id}-${quiet.length}`,
                count: quiet.length,
                labels: quiet.map((b) => b.title).slice(0, 6),
            });
        }
        quiet = [];
    };

    for (const b of blocks) {
        if (isQuietReconBlock(b)) {
            quiet.push(b);
        } else {
            flushQuiet();
            out.push(b);
        }
    }
    flushQuiet();
    return out;
}

function pickString(obj: Record<string, unknown>, keys: string[]): string {
    for (const k of keys) {
        const v = obj[k];
        if (typeof v === 'string' && v.length > 0) return v;
    }
    return '';
}

export function extractEditTexts(parsed: Record<string, unknown>): { oldText?: string; newText?: string } {
    const oldText = pickString(parsed, [
        'old_string', 'search', 'old_text', 'find', 'original', 'old_content',
    ]);
    const newText = pickString(parsed, [
        'new_string', 'replace', 'new_text', 'replacement', 'content', 'new_content', 'patch',
    ]);
    return {
        oldText: oldText || undefined,
        newText: newText || undefined,
    };
}

/** Parse markdown checkbox lines into Composer todo items. */
export function parseTodoMarkdown(content: string): AgentTodoItem[] {
    const items: AgentTodoItem[] = [];
    for (const line of content.split('\n')) {
        const m = line.match(/^\s*[-*]\s+\[([ xX/~\-]|)\]\s+(.+)$/);
        if (!m) continue;
        const mark = m[1].toLowerCase();
        let status: AgentTodoStatus = 'pending';
        if (mark === 'x') status = 'completed';
        else if (mark === '/' || mark === '~') status = 'in_progress';
        else if (mark === '-') status = 'cancelled';
        items.push({
            id: `todo-${items.length}-${hashText(m[2])}`,
            text: m[2].trim(),
            status,
        });
    }
    return items;
}

function hashText(s: string): string {
    let h = 0;
    for (let i = 0; i < s.length; i++) h = ((h << 5) - h + s.charCodeAt(i)) | 0;
    return Math.abs(h).toString(36);
}

function basename(p: string): string {
    return p.replace(/\\/g, '/').split('/').pop() || p;
}

export function buildToolBlockTitle(tool: string, args: Record<string, unknown>): string {
    const kind = classifyToolKind(tool);
    const path = String(args.path || args.file_path || args.target || args.filepath || '');
    if (kind === 'read' && path) {
        const off = args.offset ?? args.start_line ?? args.startLine;
        const lim = args.limit ?? args.end_line ?? args.endLine;
        if (off != null && lim != null) return `Read \`${basename(path)}\` L${off}-${Number(off) + Number(lim) - 1}`;
        if (off != null) return `Read \`${basename(path)}\` from L${off}`;
        return `Read \`${basename(path)}\``;
    }
    if (kind === 'terminal') {
        const cmd = String(args.command || '').trim();
        if (cmd.length > 72) return cmd.slice(0, 72) + '…';
        return cmd || 'Running command';
    }
    if (kind === 'edit' && path) return `Edited \`${basename(path)}\``;
    if (kind === 'todo') {
        const fp = String(args.file_path || args.path || '');
        return fp ? `Updated tasks · ${basename(fp)}` : 'Updated task list';
    }
    if (kind === 'canvas') {
        const t = String(args.title || '');
        return t ? `Canvas · ${t}` : 'Canvas';
    }
    if (kind === 'search') {
        const pat = String(args.pattern || args.query || '');
        if (pat && path) return `Grepped \`${pat}\` in ${basename(path)}`;
        if (pat) return `Grepped \`${pat}\``;
        if (path) return `Explored ${basename(path)}`;
    }
    return tool.replace(/_/g, ' ');
}

export function createToolBlock(tool: string, args: unknown, callId?: string): AgentToolBlock {
    const parsed = parseToolArgs(args);
    const kind = classifyToolKind(tool);
    const path = String(parsed.path || parsed.file_path || parsed.target || parsed.filepath || '');
    const off = parsed.offset ?? parsed.start_line ?? parsed.startLine;
    const lim = parsed.limit ?? parsed.end_line ?? parsed.endLine;
    let lineRange: string | undefined;
    if (off != null && lim != null) lineRange = `L${off}-${Number(off) + Number(lim) - 1}`;
    else if (off != null) lineRange = `L${off}+`;

    const { oldText, newText } = extractEditTexts(parsed);
    const preview =
        kind === 'edit'
            ? (newText || String(parsed.content || '')).slice(0, 400)
            : undefined;
    const todoContent = kind === 'todo' ? String(parsed.content || '') : '';
    const todos = todoContent ? parseTodoMarkdown(todoContent) : undefined;

    return {
        id: callId || `tb-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`,
        kind,
        tool,
        title: buildToolBlockTitle(tool, parsed),
        status: 'running',
        ts: Date.now(),
        command: kind === 'terminal' ? String(parsed.command || parsed.cmd || parsed.shell_command || '') : undefined,
        path: path || String(parsed.file_path || '') || undefined,
        lineRange,
        pattern: String(parsed.pattern || parsed.query || '') || undefined,
        outputLines: [],
        preview,
        oldText,
        newText,
        todos,
        canvasId: kind === 'canvas' ? String(parsed.id || '') || undefined : undefined,
        canvasTitle: kind === 'canvas' ? String(parsed.title || '') || undefined : undefined,
    };
}

/** Pull the rendered canvas id out of a create_canvas tool result. */
export function enrichCanvasBlockFromResult(block: AgentToolBlock, result: string): AgentToolBlock {
    if (block.kind !== 'canvas') return block;
    try {
        const j = JSON.parse(result);
        const data = (j && typeof j === 'object' && typeof j.data === 'object' && j.data) || j;
        const id = data && typeof data === 'object' ? String((data as any).canvas_id || '') : '';
        if (id) return { ...block, canvasId: id };
    } catch { /* not json */ }
    return block;
}

/** Try to enrich edit blocks from a tool result payload. */
export function enrichEditBlockFromResult(block: AgentToolBlock, result: string): AgentToolBlock {
    if (block.kind !== 'edit' || (block.oldText && block.newText)) return block;
        try {
            const j = JSON.parse(result);
            if (j && typeof j === 'object') {
                const merged = { ...parseToolArgs(j), ...parseToolArgs(j.data) };
                const { oldText, newText } = extractEditTexts(merged);
                if (oldText || newText) {
                    return { ...block, oldText: oldText || block.oldText, newText: newText || block.newText };
                }
            }
        } catch { /* not json */ }
    return block;
}
