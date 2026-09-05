import { useStore } from '../../store';
import type { EditorTab } from '../../store/types';

// Stable path so repeated diagrams from the agent reuse one tab instead of
// piling up. (Virtual tab — never written to disk.)
const MERMAID_TAB_PATH = 'mermaid://diagram';

export interface MermaidDiagram {
    title: string;
    code: string;
    description?: string;
}

/**
 * Extract the first ```mermaid block (legacy single-diagram helper).
 */
export function extractMermaid(text: string): string | null {
    if (!text) return null;
    const m = text.match(/```(?:mermaid)\s*\n([\s\S]*?)```/i);
    const code = m?.[1]?.trim();
    return code && code.length > 0 ? code : null;
}

/**
 * Extract ALL ```mermaid blocks from markdown/text, with a best-effort title and
 * description pulled from the nearest preceding markdown heading (e.g. "### 1.
 * High-Level System Architecture"). Powers the tabbed multi-diagram viewer.
 */
export function extractAllMermaid(text: string): MermaidDiagram[] {
    if (!text) return [];
    const out: MermaidDiagram[] = [];
    const re = /```(?:mermaid)\s*\n([\s\S]*?)```/gi;
    let m: RegExpExecArray | null;
    let idx = 0;
    while ((m = re.exec(text)) !== null) {
        const code = (m[1] || '').trim();
        if (!code) continue;
        idx += 1;
        // Look back at the text before this block for the closest heading.
        const before = text.slice(0, m.index);
        const headings = [...before.matchAll(/^#{1,6}\s+(.+)$/gm)];
        const lastHeading = headings.length ? headings[headings.length - 1][1].trim() : '';
        const title = (lastHeading || `Diagram ${idx}`)
            .replace(/^\d+[.)]\s*/, '') // strip leading "1. "
            .slice(0, 80);
        out.push({ title, code });
    }
    return out;
}

function writeDiagramTab(diagrams: MermaidDiagram[]): void {
    if (!diagrams.length) return;
    const content = JSON.stringify(diagrams);
    const filename = diagrams.length > 1 ? `Diagrams (${diagrams.length}).mmd` : `${diagrams[0].title}.mmd`;
    const st = useStore.getState();
    const existing = st.tabs.find((t) => t.path === MERMAID_TAB_PATH);
    if (existing) {
        useStore.setState((state) => ({
            tabs: state.tabs.map((t) =>
                t.path === MERMAID_TAB_PATH ? { ...t, content, filename } : t,
            ),
            activeTabId: existing.id,
        }));
        return;
    }
    const id = `tab-mermaid-${Date.now()}`;
    const tab = {
        id, filename, path: MERMAID_TAB_PATH, content,
        isModified: false, language: 'markdown', type: 'mermaid',
    } as unknown as EditorTab;
    useStore.setState((state) => {
        const history = state.tabHistory.slice(0, state.tabHistoryIndex + 1);
        history.push(id);
        return {
            tabs: [...state.tabs, tab],
            activeTabId: id,
            tabHistory: history,
            tabHistoryIndex: history.length - 1,
        };
    });
}

/** Open (or refresh + focus) the viewer with multiple diagrams as tabs. */
export function openMermaidDiagrams(diagrams: MermaidDiagram[]): void {
    const clean = diagrams.filter(d => d.code && d.code.trim().length > 0);
    if (clean.length) writeDiagramTab(clean);
}

/** Legacy single-diagram entry — delegates to the multi-diagram tab. */
export function openMermaidDiagram(code: string, title = 'Diagram'): void {
    const src = (code || '').trim();
    if (!src) return;
    writeDiagramTab([{ title: title.replace(/\.mmd$/, ''), code: src }]);
}

/**
 * Repair common mistakes small models make in Mermaid so it renders instead of
 * throwing a parse error. Conservative — only well-known invalid patterns.
 */
export function repairMermaid(code: string): string {
    if (!code) return code;
    let c = code;
    // Strip stray markdown fences the model sometimes leaves inside the block.
    c = c.replace(/^\s*```(?:mermaid)?\s*$/gim, '');
    // Strip the model's inline `%%{init: {...}}%%` / `%%{ theme: ... }%%` directive.
    // The viewer owns the theme (dark, Antigravity-style); a model-injected light
    // theme renders washed-out light nodes on our dark canvas. Removing it also
    // avoids a class of init-directive parse errors. `[\s\S]` spans multi-line.
    c = c.replace(/%%\{[\s\S]*?\}%%/g, '');
    // `classDef name [shape:diamond]` is invalid — classDef takes fill/stroke, not
    // a bracketed shape. Convert to a valid styled classDef so it parses + styles.
    c = c.replace(
        /(^\s*classDef\s+\S+)\s*\[[^\]]*\]\s*;?/gim,
        '$1 fill:#1e293b,stroke:#6366f1,color:#e5e9f0;',
    );
    // `style name [..]` similar invalid bracket form → drop the bracket.
    c = c.replace(/(^\s*style\s+\S+)\s*\[[^\]]*\]\s*;?/gim, '$1 fill:#1e293b,stroke:#6366f1;');
    // `title`/`subtitle` are NOT valid inside flowchart/graph — models add them as
    // bare lines. Convert to %% comments so the diagram parses.
    if (/^\s*(flowchart|graph)\b/i.test(c)) {
        c = c.replace(/^\s*(title|subtitle)\s+(.+)$/gim, '%% $1: $2');
    }
    // `DB[(( ... ))]` — models sometimes double the parens on the database/cylinder
    // shape. Valid Mermaid is single-paren `DB[( ... )]`. Collapse the extras.
    // (Standalone circles `X((label))` are NOT touched — those have no leading `[`.)
    c = c.replace(/\[\(\(+/g, '[(').replace(/\)\)+\]/g, ')]');
    // `subgraph ID ["Title"]` — a SPACE between the id and `[` makes Mermaid read
    // the bracket as an unterminated node shape and throw a parse error on the next
    // node. Mermaid wants `subgraph ID["Title"]`. Remove the space.
    c = c.replace(/^(\s*subgraph\s+[A-Za-z0-9_]+)\s+(\[)/gim, '$1$2');
    // `subgraph "Bare Title"` (quoted, no id) → give it an id so it parses cleanly.
    c = c.replace(/^(\s*subgraph)\s+("(?:[^"]+)")\s*$/gim, (_m, p1, p2, off, str) =>
        `${p1} sg_${(str.slice(0, off).match(/subgraph/gi)?.length ?? 0)}[${p2}]`);
    // Strip stray HTML tags inside labels (models occasionally emit junk like
    // `<정/>` or `<x>`), keeping `<br>`/`<br/>`. CRITICAL: only match real tags
    // (`<` + letter/`/`), never Mermaid link syntax `<--`, `<-->`, `<==`,
    // `<-.->` — those start `<` + `-`/`=`/`.` and must survive untouched.
    c = c.replace(/<\/?(?!br\b)\p{L}[^>]*>/giu, '');
    return c.trim();
}

/**
 * Last-resort sanitizer used only after a normal `repairMermaid` + parse still
 * fails. Strips everything cosmetic that small/quantized models tend to mangle —
 * class assignments, classDef/class/style lines, and exotic node shapes — leaving
 * a plain connected graph. Loses styling but renders the STRUCTURE instead of
 * showing a parse error. Edges and labels are preserved.
 */
export function aggressiveRepairMermaid(code: string): string {
    let c = repairMermaid(code);
    // Drop styling lines entirely.
    c = c.replace(/^\s*(classDef|class|style|linkStyle)\b.*$/gim, '');
    // Drop `:::className` assignments stuck on nodes.
    c = c.replace(/:::[A-Za-z0-9_]+/g, '');
    // Collapse exotic node shapes to plain rectangles so a bad delimiter can't
    // break the parse: ([x])→[x], [(x)]→[x], ((x))→[x], {{x}}→[x], {x}→[x], >x]→[x].
    c = c.replace(/\(\[([^\]]*?)\]\)/g, '[$1]');   // stadium  ([x])
    c = c.replace(/\[\(([^)]*?)\)\]/g, '[$1]');     // cylinder [(x)]
    c = c.replace(/\(\(([^)]*?)\)\)/g, '[$1]');     // circle   ((x))
    c = c.replace(/\{\{([^}]*?)\}\}/g, '[$1]');     // hexagon  {{x}}
    c = c.replace(/\{([^}]*?)\}/g, '[$1]');         // diamond  {x}
    return c.trim();
}

/** Parse a mermaid tab's stored content into diagrams (handles legacy raw source). */
export function parseDiagramTabContent(content: string): MermaidDiagram[] {
    const raw = (content || '').trim();
    if (!raw) return [];
    if (raw.startsWith('[')) {
        try {
            const arr = JSON.parse(raw);
            if (Array.isArray(arr)) return arr as MermaidDiagram[];
        } catch { /* fall through to raw */ }
    }
    return [{ title: 'Diagram', code: raw }];
}
