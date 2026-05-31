import { invoke } from './tauri_bridge.ts';
import { addTab } from './workspace.ts';

export async function openFile(path: string, name: string) {
    try {
        await loadFileContent(path);
        addTab(path, name);
    } catch (e) {
        console.error("Failed to open file:", e);
    }
}

export async function loadFileContent(path: string) {
    const welcomeView = document.getElementById("welcome-view");
    const editorView = document.getElementById("editor");
    if (welcomeView) welcomeView.classList.add("hidden");
    if (editorView) editorView.classList.remove("hidden");

    const content = await invoke<string>("open_file", { path });
    if ((window as any).monacoEditor) (window as any).monacoEditor.setValue(content);

    const breadcrumbs = document.getElementById("breadcrumbs");
    if (breadcrumbs) {
        // Format path: vscode-rust > src > main.rs
        const parts = path.split('/').filter(p => p.length > 0);
        // Take last 3 parts for brevity if it's too long, or just showing the path relative to a root would be better.
        // For now, let's just show the last few segments.
        const displayPath = parts.slice(-3).join(" > ");
        breadcrumbs.innerText = displayPath;
    }

    invoke("ext_host_send", {
        msg: JSON.stringify({
            type: "documentOpened",
            uri: path,
            content: content,
            languageId: path.endsWith(".rs") ? "rust" : "plaintext"
        })
    });

    invoke("set_context_key", { key: "activeBuffer", value: path });
    const langId = path.endsWith(".rs") ? "rust" : "plaintext";
    invoke("check_activation_event", { event: `onLanguage:${langId}` });
}

export async function saveActiveFile() {
    const editor = (window as any).monacoEditor;
    if (!editor) return;

    const content = editor.getValue();
    try {
        await invoke("save_file", { content });
        console.log("File saved successfully");
    } catch (e) {
        console.error("Failed to save file:", e);
    }
}
