import { invoke } from '../../tauri_bridge';

const docVersions = new Map<string, number>();

export function getLspLanguageId(path: string): string {
    const ext = path.split('.').pop()?.toLowerCase() ?? '';
    const map: Record<string, string> = {
        rs: 'rust', ts: 'typescript', tsx: 'typescriptreact',
        js: 'javascript', jsx: 'javascriptreact', py: 'python',
        go: 'go', java: 'java', c: 'c', cpp: 'cpp', cs: 'csharp',
        json: 'json', md: 'markdown', toml: 'toml', yaml: 'yaml', yml: 'yaml',
        prisma: 'prisma', vue: 'vue', svelte: 'svelte', graphql: 'graphql', gql: 'graphql',
        rb: 'ruby', php: 'php', lua: 'lua', zig: 'zig', ex: 'elixir', exs: 'elixir',
        tf: 'terraform', hcl: 'terraform', dart: 'dart', kt: 'kotlin', kts: 'kotlin',
        sh: 'shellscript', bash: 'shellscript', zsh: 'shellscript',
    };
    return map[ext] ?? 'plaintext';
}

async function send(msg: Record<string, unknown>): Promise<void> {
    if (!(window as any).__TAURI__) return;
    await invoke('ext_host_send', { msg: JSON.stringify(msg) });
}

export async function syncDocumentOpened(path: string, content: string, languageId?: string): Promise<void> {
    if (!path || path.startsWith('vscode://')) return;
    const lang = languageId ?? getLspLanguageId(path);
    docVersions.set(path, 1);
    await send({ type: 'documentOpened', uri: path, content, languageId: lang, version: 1 });
    await invoke('check_activation_event', { event: `onLanguage:${lang}` }).catch(() => {});
}

export async function syncDocumentChanged(path: string, content: string): Promise<void> {
    if (!path || path.startsWith('vscode://')) return;
    const version = (docVersions.get(path) ?? 1) + 1;
    docVersions.set(path, version);
    await send({ type: 'documentChanged', uri: path, content, version });
}

export async function syncDocumentSaved(path: string): Promise<void> {
    if (!path || path.startsWith('vscode://')) return;
    await send({ type: 'documentSaved', uri: path });
}

export async function syncDocumentClosed(path: string): Promise<void> {
    if (!path || path.startsWith('vscode://')) return;
    docVersions.delete(path);
    await send({ type: 'documentClosed', uri: path });
}
