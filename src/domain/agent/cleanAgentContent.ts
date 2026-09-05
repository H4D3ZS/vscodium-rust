/**
 * Display-layer transforms for agent chat — strips tool JSON/XML and
 * normalizes tool names. Backend still processes full payloads.
 */

const TOOL_LABELS: Record<string, string> = {
    write_to_file: 'Writing file',
    search_replace_edit: 'Patching code',
    str_replace: 'Editing code',
    apply_shadow_patch: 'Committing edit',
    patch_file_content: 'Replacing lines',
    view_file: 'Reading file',
    run_command: 'Running command',
    bash: 'Running command',
    verify_implementation: 'Verifying',
    ghost_test: 'Running tests',
    list_files: 'Scanning directory',
    grep: 'Searching code',
    git_commit: 'Committing',
    dev_cargo_diagnostics: 'Checking Rust',
    web_search: 'Web search',
    git_diff: 'Reading diff',
    semantic_search: 'Semantic search',
    find_symbols: 'Finding symbols',
    create_directory: 'Creating directory',
    deep_security_audit: 'Security audit',
    web_security_audit: 'Web security audit',
    secrets_scan: 'Scanning secrets',
    weaponize_env: 'Assessing .env',
    browser_open: 'Opening browser',
    browser_navigate: 'Navigating',
    browser_read_dom: 'Reading page',
    browser_click: 'Clicking element',
    browser_screenshot: 'Taking screenshot',
    web_fetch: 'Web fetch',
    file_write: 'Writing file',
};

export function getToolLabel(name: string): string {
    if (TOOL_LABELS[name]) return TOOL_LABELS[name];
    return name.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
}

/**Heuristic: model tool JSON even when truncated / invalid JSON (e.g. huge file_write body). */
export function looksLikeToolCallText(text: string): boolean {
    const t = text.trim();
    if (!t.startsWith('{')) return false;
    return (
        /"(?:name|tool)"\s*:\s*"/.test(t)
        && /"(?:arguments|parameters|args|input|content)"\s*:/.test(t)
    ) || /"tool_calls"\s*:/.test(t) || /"function_call"\s*:/.test(t);
}

/**One-line human label for inline tool JSON shown in chat. */
export function summarizeToolCallText(text: string): string {
    const name = text.match(/"(?:name|tool)"\s*:\s*"([^"]+)"/)?.[1] || 'tool';
    const label = getToolLabel(name);
    const url = text.match(/"url"\s*:\s*"([^"]+)"/)?.[1];
    if (url) return `${label} · ${url}`;
    const path = text.match(/"(?:path|file_path|filename)"\s*:\s*"([^"\\]+)"/)?.[1];
    if (path) return `${label} · ${path.replace(/\\/g, '/')}`;
    const cmd = text.match(/"command"\s*:\s*"([^"]{0,80})/)?.[1];
    if (cmd) return `${label} · ${cmd}${cmd.length >= 80? '…': ''}`;
    return label;
}

/**Returns true if a JSON string looks like a tool call object */
export function isToolCallJson(text: string): boolean {
    if (looksLikeToolCallText(text)) return true;
    try {
        const t = text.trim();
        if (!t.startsWith('{') && !t.startsWith('[')) return false;
        const parsed = JSON.parse(t);
        if (Array.isArray(parsed)) return parsed.some(isToolCallJson);
        if (parsed && typeof parsed === 'object') {
            return ('name' in parsed && ('arguments' in parsed || 'parameters' in parsed || 'input' in parsed))
                || 'tool_calls' in parsed
                || 'function_call' in parsed;
        }
    } catch { /* truncated / invalid JSON from streaming models */ }
    return false;
}

/**True if a line looks like a raw tool result blob (not prose). */
export function isToolResultJson(text: string): boolean {
    const t = text.trim();
    if (!t.startsWith('{') && !t.startsWith('[')) return false;
    try {
        const parsed = JSON.parse(t);
        if (Array.isArray(parsed)) return parsed.length > 0 && typeof parsed[0] === 'object';
        if (parsed && typeof parsed === 'object') {
            return 'status' in parsed
                || 'success' in parsed
                || 'stdout' in parsed
                || 'stderr' in parsed
                || 'exit_code' in parsed
                || 'blocked' in parsed
                || 'error' in parsed;
        }
    } catch { /* not JSON */ }
    return false;
}

export function formatCursorActivityLine(
    tool: string | undefined,
    title: string,
    detail?: string,
    success?: boolean,
): string {
    const name = (tool || '').toLowerCase();
    let args: Record<string, unknown> = {};
    if (detail) {
        try {
            const parsed = JSON.parse(detail);
            if (parsed && typeof parsed === 'object' && !Array.isArray(parsed)) {
                args = parsed as Record<string, unknown>;
            }
        } catch { /* plain text detail */ }
    }

    if (name.includes('grep') || name.includes('search_codebase') || name.includes('semantic_search')) {
        const pattern = String(args.pattern || args.query || title || 'pattern');
        const path = args.path || args.file_path;
        return path? `Grepped \`${pattern}\` in ${basename(String(path))}`: `Grepped \`${pattern}\``;
    }
    if (name.includes('list_files') || name.includes('list_directory') || name === 'glob') {
        const path = args.path || args.directory_path || args.pattern || 'workspace';
        return `Explored ${basename(String(path))}`;
    }
    if (name.includes('view_file') || name.includes('file_read')) {
        return `Read ${basename(String(args.file_path || args.path || title))}`;
    }
    if (name.includes('write_to_file') || name.includes('file_write') || name.includes('search_replace')) {
        return `Edited ${basename(String(args.file_path || args.path || title))}`;
    }
    if (name.includes('run_command') || name === 'bash') {
        const cmd = String(args.command || detail || '').slice(0, 60);
        return cmd? `Ran \`${cmd}\``: 'Ran shell command';
    }
    if (name.includes('web_security_audit') || name.includes('apex_scan')) {
        const url = args.url || args.target;
        return url? `Audited ${String(url)}`: 'Web security audit';
    }
    if (name.includes('browser_navigate')) {
        return `Navigated to ${args.url || title}`;
    }
    if (name.includes('browser_open')) {
        return 'Opened stealth browser';
    }
    if (name.includes('sec_distro_inventory')) {
        return 'Inventoried Kali/Parrot tools';
    }

    const base = title || getToolLabel(tool || 'tool');
    if (success === false) return ` ${base}`;
    if (success === true) return base;
    return base;
}

function basename(p: string): string {
    const parts = p.replace(/\\/g, '/').split('/');
    return parts[parts.length - 1] || p;
}

export function formatToolSummary(name: string, args: any, result: any): string {
    try {
        const data = typeof result === 'string'? JSON.parse(result): result;
        const toolName = name.toLowerCase();

        if (toolName.includes('list_files') || toolName.includes('list_directory') || toolName.includes('ls')) {
            const count = Array.isArray(data)? data.length: (data.filenames? data.filenames.length: 0);
            return `Listed ${count} items in ${args.path || args.directory_path || 'root'}`;
        }
        if (toolName.includes('view_file') || toolName.includes('file_read') || toolName.includes('cat')) {
            return `Read ${args.file_path || args.path} (${data.numLines || 'all'} lines)`;
        }
        if (toolName.includes('run_command') || toolName.includes('bash') || toolName.includes('sh')) {
            const cmd = args.command || '';
            const shortCmd = cmd.length > 40? cmd.substring(0, 40) + '…': cmd;
            return shortCmd? `Ran \`${shortCmd}\``: 'Ran command';
        }
        if (toolName.includes('grep') || toolName.includes('search')) {
            const count = Array.isArray(data)? data.length: 0;
            return `Found ${count} matches for "${args.pattern || args.query}"`;
        }
        if (toolName.includes('write_to_file') || toolName.includes('file_write')) {
            return `Wrote ${args.file_path || args.path || args.filename || 'file'}`;
        }
        if (toolName.includes('file_edit') || toolName.includes('modify_file')) {
            return `Edited ${args.file_path || args.path}`;
        }
        if (toolName.includes('git_status')) {
            return 'Checked git status';
        }
        if (data && typeof data === 'object' && 'status' in data) {
            const st = String(data.status);
            if (st === 'failed' || st === 'error') return `${getToolLabel(name)} failed`;
            if (st === 'blocked') return `${getToolLabel(name)} blocked`;
            if (st === 'success' || st === 'ok') return getToolLabel(name);
        }
    } catch {
        /* fall through */
    }
    return getToolLabel(name);
}

/**
 * Strip tool-call markup and raw JSON from assistant message text.
 * Used by chat panel — keep prose only (Cursor-style).
 */
export function cleanAgentContent(raw: string): string {
    if (!raw) return '';
    let s = raw;

    s = s.replace(/<tool_call>[\s\S]*?<\/tool_call>/g, '');
    s = s.replace(/<function_calls>[\s\S]*?<\/function_calls>/g, '');
    s = s.replace(/<function>[\s\S]*?<\/function>/g, '');
    s = s.replace(/<invoke>[\s\S]*?<\/invoke>/g, '');

    s = s.replace(/```[a-z0-9_:.-]*\s*\n?([\s\S]*?)```/gi, (match, inner) => {
        const trimmed = inner.trim();
        return isToolCallJson(trimmed) || isToolResultJson(trimmed)? '': match;
    });
    s = s.replace(/```\s*(\{[\s\S]*?\})\s*```/g, (match, inner) => {
        return isToolCallJson(inner) || isToolResultJson(inner)? '': match;
    });

    // Unclosed streaming fences that are clearly tool JSON
    s = s.replace(/```(?:json)?\s*\n(\{\s*"name"\s*:[\s\S]*)$/gi, (match, inner) => {
        return looksLikeToolCallText(inner) || isToolResultJson(inner)? '': match;
    });

    s = s.split('\n').filter((line) => {
        const t = line.trim();
        if (!t) return true;
        if (/^Executing tool:/i.test(t)) return false;
        if (looksLikeToolCallText(t) || isToolCallJson(t) || isToolResultJson(t)) return false;
        if (/^\{"status"\s*:/.test(t)) return false;
        return true;
    }).join('\n');

    s = s.replace(/<<<< SEARCH[\s\S]*?>>>>/g, '');
    s = s.replace(/<<<<<<[\s\S]*?>>>>>>>/g, '');
    s = s.replace(/MISSION_ACCOMPLISHED/g, '');
    s = s.replace(/TASK_COMPLETE/g, '');

    s = s.replace(/\$\s*\\(?:text|mathit|mathrm|mathbf|mathcal|mathsf|mathtt)\{([^{}]{1,6})\}\s*\$/g,
        (_m, inner) => String(inner));
    s = s.replace(/(?:^|[^\w])\$([A-Za-z0-9])\$(?=[^\w]|$)/g, (_m, ch) => ch);
    s = s.replace(/(?:\b[A-Za-z]\b\s+){3,}/g, (m) => m.replace(/\s+/g, ''));

    s = s.replace(/\n{3,}/g, '\n\n').trim();
    return s;
}

/**Avoid wiping streamed markdown when the final payload is empty or tool-only. */
export function shouldReplaceAgentContent(existing: string, incoming: string): boolean {
    const ex = (existing || '').trim();
    const inc = (incoming || '').trim();
    if (!inc) return false;
    if (!ex) return true;
    if (inc.length < 160 && ex.length > inc.length * 2) return false;
    return true;
}

/**One-line summary for activity terminal tool results. */
export function summarizeToolResult(name: string, result: string, args?: any): string {
    let parsedArgs = args;
    if (!parsedArgs) {
        parsedArgs = {};
    }
    const summary = formatToolSummary(name, parsedArgs, result);
    if (summary !== getToolLabel(name)) return summary;

    try {
        const data = typeof result === 'string'? JSON.parse(result): result;
        if (data && typeof data === 'object') {
            if (typeof data.message === 'string' && data.message.trim()) {
                return data.message.trim().slice(0, 200);
            }
            if (typeof data.stderr === 'string' && data.stderr.trim()) {
                return data.stderr.trim().split(/\r?\n/)[0].slice(0, 200);
            }
            if (typeof data.stdout === 'string' && data.stdout.trim()) {
                return data.stdout.trim().split(/\r?\n/)[0].slice(0, 200);
            }
            if (data.status === 'failed' || data.status === 'error') {
                return `${getToolLabel(name)} failed`;
            }
        }
    } catch {
        /* plain text */
    }

    const lines = result.split(/\r?\n/).filter(Boolean);
    const head = (lines[0] ?? '').trim();
    if (head.startsWith('{') && isToolResultJson(head)) {
        return formatToolSummary(name, parsedArgs, head);
    }
    return head.slice(0, 200) || getToolLabel(name);
}
