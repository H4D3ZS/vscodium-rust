import { invoke, listen } from './tauri_bridge.ts';
import { browserOpen, browserNavigate, browserScreenshot, browserClose } from './browser.ts';
import { useStore } from './store.ts';
import type { PendingChange } from './store.ts';

export interface ChatMessage {
    role: "system" | "user" | "assistant";
    content: string;
}

let chatHistory: ChatMessage[] = [];

// providerModels is now managed by the store and backend discovery

let currentAgentProvider = "Google";
let currentAgentModel = "gemini-2.5-pro";
let currentAgentMode = "Planning";

function createPopover(element: HTMLElement, items: { label: string, value: string, desc?: string, icon?: string }[], onSelect: (val: string, label: string) => void) {
    const existing = document.getElementById("agent-popover");
    if (existing) existing.remove();

    const rect = element.getBoundingClientRect();
    const popover = document.createElement("div");
    popover.id = "agent-popover";
    popover.style.position = "absolute";

    // Smart positioning: if on the right half of the screen, right-align.
    const isRight = rect.left > window.innerWidth / 2;
    if (isRight) {
        popover.style.right = `${window.innerWidth - rect.right}px`;
    } else {
        popover.style.left = `${rect.left}px`;
    }

    popover.style.bottom = `${window.innerHeight - rect.top + 10}px`;
    popover.style.background = "var(--vscode-menu-background, #252526)";
    popover.style.border = "1px solid var(--vscode-menu-border, #454545)";
    popover.style.borderRadius = "6px";
    popover.style.boxShadow = "0 8px 24px rgba(0,0,0,0.5)";
    popover.style.padding = "4px 0";
    popover.style.zIndex = "10000";
    popover.style.minWidth = "220px";
    popover.style.maxWidth = "320px";
    popover.style.maxHeight = "400px";
    popover.style.overflowY = "auto";
    popover.style.color = "var(--vscode-menu-foreground, #ccc)";
    popover.style.fontFamily = "var(--vscode-font-family, -apple-system, system-ui, sans-serif)";
    popover.style.fontSize = "12px";
    popover.style.pointerEvents = "auto";
    popover.style.animation = "popoverSlideIn 0.2s ease-out";

    // Add CSS for fade-in animation if not exists
    if (!document.getElementById("agent-popover-style")) {
        const style = document.createElement("style");
        style.id = "agent-popover-style";
        style.innerHTML = `
            @keyframes popoverSlideIn {
                from { opacity: 0; transform: translateY(10px) scale(0.98); }
                to { opacity: 1; transform: translateY(0) scale(1); }
            }
            .popover-item:hover { background: var(--vscode-menu-selectionBackground, #04395e) !important; color: var(--vscode-menu-selectionForeground, #fff) !important; }
            .popover-item:hover .popover-desc { color: rgba(255,255,255,0.7) !important; }
        `;
        document.head.appendChild(style);
    }

    items.forEach(item => {
        const row = document.createElement("div");
        row.className = "popover-item";
        row.style.padding = "8px 12px";
        row.style.cursor = "pointer";
        row.style.display = "flex";
        row.style.flexDirection = "column";
        row.style.transition = "background 0.1s ease";

        row.onclick = (e) => {
            e.stopPropagation();
            onSelect(item.value, item.label);
            popover.remove();
        };

        const titleRow = document.createElement("div");
        titleRow.style.display = "flex";
        titleRow.style.alignItems = "center";
        titleRow.style.gap = "8px";

        if (item.icon) {
            const icon = document.createElement("i");
            icon.className = `codicon codicon-${item.icon}`;
            titleRow.appendChild(icon);
        }

        const titleText = document.createElement("span");
        titleText.innerText = item.label;
        titleText.style.fontWeight = "500";
        titleRow.appendChild(titleText);
        row.appendChild(titleRow);

        if (item.desc) {
            const desc = document.createElement("div");
            desc.className = "popover-desc";
            desc.innerText = item.desc;
            desc.style.fontSize = "11px";
            desc.style.color = "var(--vscode-descriptionForeground, #999)";
            desc.style.marginTop = "4px";
            desc.style.lineHeight = "1.4";
            row.appendChild(desc);
        }

        popover.appendChild(row);
    });

    document.body.appendChild(popover);

    setTimeout(() => {
        const closeListener = (e: MouseEvent) => {
            if (!popover.contains(e.target as Node)) {
                popover.remove();
                document.removeEventListener("mouseup", closeListener);
            }
        };
        document.addEventListener("mouseup", closeListener);
    }, 0);
}

export function openModeDropdown(element: HTMLElement, onSelect: (label: string) => void) {
    createPopover(element, [
        { label: "Planning", value: "Planning", icon: "beaker", desc: "Agent can plan before executing tasks. Use for deep research, complex tasks, or collaborative work" },
        { label: "Planning (Source Control)", value: "Planning (Source Control)", icon: "git-branch", desc: "Deep dive into git history and planning source control workflows" },
        { label: "Fast", value: "Fast", icon: "zap", desc: "Agent will execute tasks directly. Use for simple tasks that can be completed faster" }
    ], (val) => {
        const store = (window as any).useStore;
        if (store) {
            store.getState().setAgentMode(val);
            if (val.includes("Source Control")) {
                store.getState().setActiveSidebarView('planning-view');
            }
        }
        onSelect(val);
    });
}

export async function initAgent() {
    console.log("Initializing Agent global listeners...");
    const { listen } = await import('@tauri-apps/api/event');
    const useStore = (window as any).useStore;

    // Listen for session capture from auth flow
    await listen('session-captured', (event: any) => {
        console.log('Session captured:', event.payload);
        const { setSession } = useStore.getState();
        setSession(event.payload);

        const { provider, cookies, userAgent } = event.payload;
        const session = {
            provider,
            cookies,
            user_agent: userAgent
        };

        invoke("save_ai_session", { session }).then(() => {
            const store = (window as any).useStore;
            if (store) {
                store.getState().setAiStatus('alive');
                store.getState().refreshAvailableModels(provider);

                // Visual feedback
                const messagesContainer = document.getElementById("agent-messages");
                if (messagesContainer) {
                    const info = document.createElement("div");
                    info.className = "agent-message info-message-box";
                    info.style.background = "rgba(16, 185, 129, 0.1)";
                    info.style.border = "1px solid rgba(16, 185, 129, 0.2)";
                    info.style.color = "#10b981";
                    info.style.padding = "8px 12px";
                    info.style.margin = "8px 0";
                    info.style.borderRadius = "6px";
                    info.style.fontSize = "12px";
                    info.style.animation = "fadeIn 0.3s ease";
                    info.innerHTML = `<i class="codicon codicon-pass-filled"></i> Session for ${provider} synced successfully!`;
                    messagesContainer.appendChild(info);
                    messagesContainer.scrollTop = messagesContainer.scrollHeight;
                }
            }
        }).catch(err => {
            console.error("Failed to save AI session:", err);
        });
    });

    // Listen for streaming AI content
    await listen('ai-content', (event: any) => {
        const { updateLastAgentMessage, setIsAgentThinking } = useStore.getState();
        setIsAgentThinking(false);
        // Payload from Rust is { content: string }
        const content = typeof event.payload === 'object' && event.payload.content
            ? event.payload.content
            : (typeof event.payload === 'string' ? event.payload : '');
        updateLastAgentMessage(content);
    });

    // Listen for tool calls from the backend
    await listen<any>("ai-tool-call", (event) => {
        const { addAgentStep } = useStore.getState();
        const toolName = event.payload.name || 'tool_call';
        addAgentStep(toolName);
    });
}

export function openModelDropdown(element: HTMLElement, onSelect: (label: string) => void) {
    const rect = element.getBoundingClientRect();
    const store = (window as any).useStore;
    const availableModels = store ? store.getState().availableModels : [];
    const setAgentModel = store ? store.getState().setAgentModel : () => { };

    const items: { label: string, value: string, desc?: string }[] = [];

    if (availableModels && availableModels.length > 0) {
        availableModels.forEach((m: { id: string, provider: string }) => {
            const providerName = m.provider.toLowerCase();
            const providerLabel = providerName.charAt(0).toUpperCase() + providerName.slice(1);
            items.push({
                label: `${m.id} (${providerLabel})`,
                value: `${providerLabel}|${m.id}`
            });
        });
    }

    // Add local Ollama manual check if no models found (fallback)
    if (!items.find(i => i.value.startsWith("Ollama"))) {
        items.push({ label: "🛠️ Check Ollama (Local)", value: "action|check_ollama", desc: "Scan for local models on http://localhost:11434" });
    }

    // Always offer Hunting/Settings if list is low or empty
    if (items.length < 3) {
        items.push({
            label: "🛰️ Hunt for Working AI Keys",
            value: "action|hunt",
            desc: "Scans for leaked but alive API keys"
        });
    }

    // Add Browser login options
    items.push({
        label: "☁️ Login to Claude (Browser)",
        value: "action|login|claude",
        desc: "Use your personal Claude.ai subscription"
    });
    items.push({
        label: "💎 Login to Gemini (Browser)",
        value: "action|login|gemini",
        desc: "Use your personal Gemini subscription"
    });

    if (items.length === 0) {
        items.push({ label: "⚙️ Add API keys in settings", value: "action|settings" });
    }

    createPopover(element, items, (val) => {
        if (val === "action|hunt") {
            startKeyHunt();
            return;
        }
        if (val === "action|check_ollama") {
            const store = (window as any).useStore;
            if (store) store.getState().refreshAvailableModels("ollama");
            return;
        }
        if (val.startsWith("action|login|")) {
            const provider = val.split("|")[2];
            invoke("open_ai_login", { provider }).catch(err => {
                console.error("Failed to open login window:", err);
            });
            return;
        }
        if (val === "action|settings") {
            const settingsBtn = document.querySelector('.codicon-settings') as HTMLElement;
            if (settingsBtn) settingsBtn.click();
            return;
        }
        if (val === "none") return;

        setAgentModel(val);
        onSelect(val);
    });
}

export async function openContextDropdown(target: HTMLElement, onSelect: (type: 'attachment' | 'mention' | 'workflow', name: string, data?: any, path?: string) => void) {
    const items = [
        { label: 'Attachment', value: 'attachment', icon: 'file-media', desc: 'Attach any local file (image, script, doc)' },
        { label: 'Mention', value: 'mention', icon: 'mention', desc: 'Reference a project file or entity' },
        { label: 'Workflow', value: 'workflow', icon: 'repo-forked', desc: 'Select a task workflow or plan' },
        { label: 'Web Screenshot', value: 'browser', icon: 'browser', desc: 'Capture current webpage vision + DOM' }
    ];

    createPopover(target, items, async (val) => {
        const store = (window as any).useStore;
        const activeRoot = store?.getState().activeRoot || '';

        if (val === 'attachment') {
            const input = document.createElement('input');
            input.type = 'file';
            input.accept = '*/*';
            input.onchange = (e) => {
                const file = (e.target as HTMLInputElement).files?.[0];
                if (file) {
                    const reader = new FileReader();
                    reader.onload = (re) => onSelect('attachment', file.name, reader.result);
                    reader.readAsDataURL(file);
                }
            };
            input.click();
        } else if (val === 'mention') {
            const files = store?.getState().getFlattenedFiles() || [];
            if (files.length === 0) {
                const name = prompt('Mention project file (e.g. src/main.tsx):');
                if (name) onSelect('mention', name);
                return;
            }

            const fileItems = files.map(f => ({
                label: f.name,
                value: f.path,
                icon: 'file',
                desc: f.path.replace(activeRoot, '').replace(/^[\\\/]/, '')
            }));

            createPopover(target, fileItems, (filePath) => {
                const selectedFile = files.find(f => f.path === filePath);
                if (selectedFile) {
                    onSelect('mention', selectedFile.name, undefined, selectedFile.path);
                }
            });
        } else if (val === 'workflow') {
            if (!activeRoot) {
                alert("Please open a project folder first to use workflows.");
                return;
            }

            try {
                const paths = [`${activeRoot}/.agent/workflows`, `${activeRoot}/.agents/workflows`];
                let allWfs: any[] = [];
                for (const p of paths) {
                    try {
                        const entries = await invoke<any[]>('list_directory', { path: p });
                        allWfs = [...allWfs, ...entries.filter((e: any) => !e.is_dir && e.name.endsWith('.md'))];
                    } catch (_) { }
                }

                if (allWfs.length === 0) {
                    const name = prompt('No workflows found. Enter workflow filename manualy:');
                    if (name) onSelect('workflow', name);
                    return;
                }

                const workflowItems = allWfs.map(wf => ({
                    label: wf.name.replace('.md', ''),
                    value: wf.name,
                    icon: 'repo-forked',
                    desc: wf.path.replace(activeRoot, '').replace(/^[\\\/]/, '')
                }));

                // Show secondary popover for selection
                createPopover(target, workflowItems, (wfVal) => {
                    onSelect('workflow', wfVal);
                });
            } catch (e) {
                console.error("Failed to load workflows:", e);
                const name = prompt('Enter workflow path or identifier:');
                if (name) onSelect('workflow', name);
            }
        } else if (val === 'browser') {
            const url = prompt('Enter URL to capture (leave empty for current browser view):');
            invoke<any>("browser_capture_vision_context", { url: url || undefined })
                .then(data => {
                    onSelect('attachment', `Web Screenshot: ${data.title}`, data.screenshot);
                    onSelect('mention', `DOM Summary for ${data.url}`, data.dom_summary);
                })
                .catch(e => {
                    console.error("Browser capture failed:", e);
                    alert("Failed to capture browser: " + e);
                });
        }
    });
}

export async function handleAgentChat(inputElement: HTMLTextAreaElement) {
    const prompt = inputElement.value.trim();
    if (!prompt) return;

    inputElement.value = "";

    const store = (window as any).useStore;
    if (!store) return;

    const state = store.getState();

    // Kick off a background memory load whenever the user first sends a message
    if (state.activeRoot && !state.projectMemory) {
        loadProjectMemory(state.activeRoot).catch(() => { });
    }

    // Add user message
    state.addAgentMessage('user', prompt);

    // Add empty assistant message for streaming
    state.addAgentMessage('assistant', '');
    state.setIsAgentThinking(true);

    try {
        await sendAgentMessage(prompt, () => { });
        // Clear context on successful send
        state.clearAttachedContext();
    } catch (error: any) {
        console.error('Agent chat error:', error);
        store.getState().setIsAgentThinking(false);
        store.getState().updateLastAgentMessage(`**Error:** ${error.message || error}`);
    }
}

// ---------------------------------------------------------------------------
// Project Memory — reads AGENTS.md / CLAUDE.md / memory/ from disk, caches in
// store. Called once on first chat send, or on /memory reload. Zero LLM cost:
// the content is appended to the existing system message, no extra API calls.
// ---------------------------------------------------------------------------
export async function loadProjectMemory(root: string): Promise<void> {
    const store = (window as any).useStore;
    if (!store) return;

    const candidateFiles = [
        `${root}/MEMORY.md`,
        `${root}/AGENTS.md`,
        `${root}/CLAUDE.md`,
        `${root}/.agent/memory.md`,
        `${root}/memory/context.md`,
        `${root}/memory/constitution.md`,
        `${root}/spec-kit/memory/constitution.md`,
    ];

    const found: string[] = [];
    const sections: string[] = [];

    // Execute Tool via Backend
    for (const filePath of candidateFiles) {
        try {
            const content = await invoke<string>("read_file", { path: filePath });
            if (content && content.trim()) {
                const heading = filePath.split('/').pop() ?? filePath;
                sections.push(`### ${heading}\n\n${content.trim()}`);
                found.push(filePath);
            }
        } catch (_) {
            // file doesn't exist — skip silently
        }
    }

    const combined = sections.length > 0
        ? `## Project Memory\n\n${sections.join('\n\n---\n\n')}`
        : '';

    store.getState().setProjectMemory(combined, found);
}

// ---------------------------------------------------------------------------
// IDE Context Builder — extracted from sendAgentMessage so it stays readable.
// ---------------------------------------------------------------------------
async function buildIdeContext(): Promise<string> {
    const store = (window as any).useStore;
    if (!store) return 'You are an AI coding agent embedded inside a VSCode-like IDE.';

    const storeState = store.getState();
    const activeRoot = storeState.activeRoot || '';
    const activeEditorPath = storeState.activeEditorPath || '';
    const tabs = (storeState as any).tabs || [];
    const projectMemory: string = storeState.projectMemory || '';

    const activeTab = tabs.find((t: any) => t.path === activeEditorPath);
    const activeEditorContent: string = activeTab?.content || '';

    const parts: string[] = [
        `You are an AI coding agent embedded inside a VSCode-like IDE.`,
    ];

    if (activeRoot) {
        parts.push(`Project root: ${activeRoot}`);
    }
    if (activeEditorPath) {
        parts.push(`Active file: ${activeEditorPath}`);
        const language = activeTab?.language || '';
        if (activeEditorContent) {
            const lines = activeEditorContent.split('\n');
            const preview = lines.slice(0, 200).join('\n');
            parts.push(`\nActive file content (${lines.length} lines, showing first 200):\n\`\`\`${language}\n${preview}\n\`\`\``);
        }
    }
    if (tabs.length > 1) {
        const otherOpenFiles = tabs
            .filter((t: any) => t.path !== activeEditorPath)
            .map((t: any) => t.path)
            .slice(0, 8)
            .join(', ');
        if (otherOpenFiles) {
            parts.push(`\nOther open files: ${otherOpenFiles}`);
        }
    }
    parts.push(`\nCurrent date/time: ${new Date().toISOString()}`);
    parts.push(`Agent mode: ${store.getState().agentMode}`);

    // Append cached project memory (AGENTS.md / CLAUDE.md / etc.) — zero extra tokens
    if (projectMemory) {
        parts.push(`\n${projectMemory}`);
    }

    // Append user-attached context items (Attachments, Mentions, Workflows)
    const context = storeState.attachedContext || [];
    if (context.length > 0) {
        parts.push(`\n## Attached Context`);
        for (const c of context) {
            if (c.type === 'mention' || c.type === 'file') {
                let content = c.data;
                if (!content && activeRoot) {
                    try {
                        const fullPath = c.path || (c.name.startsWith('/') ? c.name : `${activeRoot}/${c.name}`);
                        const rawContent = await invoke<string>("read_file", { path: fullPath });
                        if (rawContent) {
                            const lines = rawContent.split('\n');
                            content = lines.slice(0, 300).join('\n');
                            if (lines.length > 300) content += `\n... (truncated, ${lines.length - 300} more lines)`;
                        }
                    } catch (e) {
                        content = `(Error: Could not read file content for ${c.name})`;
                    }
                }
                parts.push(`### File: ${c.name}\n\`\`\`\n${content || '(No content)'}\n\`\`\``);
            } else if (c.type === 'workflow') {
                let content = c.data;
                if (!content && activeRoot) {
                    try {
                        const wfPath = `${activeRoot}/.agent/workflows/${c.name.endsWith('.md') ? c.name : c.name + '.md'}`;
                        const rawContent = await invoke<string>("read_file", { path: wfPath });
                        if (rawContent) {
                            const lines = rawContent.split('\n');
                            content = lines.slice(0, 300).join('\n');
                            if (lines.length > 300) content += `\n... (truncated)`;
                        }
                    } catch (e) {
                        try {
                            const wfPathAlt = `${activeRoot}/.agents/workflows/${c.name.endsWith('.md') ? c.name : c.name + '.md'}`;
                            const rawContentAlt = await invoke<string>("read_file", { path: wfPathAlt });
                            if (rawContentAlt) {
                                const lines = rawContentAlt.split('\n');
                                content = lines.slice(0, 300).join('\n');
                                if (lines.length > 300) content += `\n... (truncated)`;
                            }
                        } catch (e2) {
                            content = `(Error: Could not read workflow content for ${c.name})`;
                        }
                    }
                }
                parts.push(`### Workflow: ${c.name}\n\`\`\`markdown\n${content || '(No content)'}\n\`\`\``);
            } else if (c.type === 'attachment') {
                if (c.data && c.data.startsWith('data:image/')) {
                    parts.push(`- Attached Image: \`${c.name}\` (Sent as multimodal data)`);
                } else {
                    parts.push(`### Attachment: ${c.name}\n\`\`\`\n${c.data || '(Binary or no content)'}\n\`\`\``);
                }
            }
        }
    }

    return parts.join('\n');
}

export async function sendAgentMessage(userPrompt: string, _onUpdate: (msg: string) => void): Promise<void> {
    const store = (window as any).useStore;
    if (!store) throw new Error("Store not found");

    // Handle Slash Commands
    if (userPrompt.startsWith('/')) {
        const handled = await processSlashCommand(userPrompt);
        if (handled) return;
    }

    const { agentModel, agentMessages, setAiStatus, availableModels } = store.getState();

    // Determine provider and model
    let provider = "OpenAI";
    let model = agentModel;

    // 1. Try to find in availableModels list (most reliable)
    const found = availableModels?.find((m: any) => m.id === agentModel || `${m.provider}|${m.id}` === agentModel);
    if (found) {
        provider = found.provider;
        model = found.id;
    }
    // 2. Fallback to format parsing etc.
    else if (agentModel.includes("|")) {
        [provider, model] = agentModel.split("|");
    } else if (agentModel.toLowerCase().includes("goog") || agentModel.toLowerCase().includes("gemini")) {
        provider = "Google";
    } else if (agentModel.toLowerCase().includes("anthropic") || agentModel.toLowerCase().includes("claude")) {
        provider = "Anthropic";
    } else if (agentModel.toLowerCase().includes("ollama") || agentModel.includes("/") || agentModel.includes(":")) {
        // Deep local model detection (Ollama often uses slashes and colons)
        provider = "Ollama";
    }

    // Normalized provider for backend
    const normalizedProvider = provider.toLowerCase() === 'apiradar' ? 'apiradar' : provider.toLowerCase();

    // --- Build system message from IDE context + cached project memory ---
    const systemContext = await buildIdeContext();
    const systemMessage = {
        role: 'system',
        content: systemContext,
        tool_calls: null,
        metadata: null,
    };

    // Map messages to the format expected by the backend
    const messages = [
        systemMessage,
        ...agentMessages.map((m: any) => {
            let content: any = m.content || "";

            // If message has attachment context, convert to multi-modal parts for images
            const attachmentContext = m.context?.filter((c: any) => c.type === 'attachment' && c.data);
            if (attachmentContext && attachmentContext.length > 0) {
                const parts: any[] = [{ type: 'text', text: content }];
                attachmentContext.forEach((ac: any) => {
                    // Only images are sent as multimodal parts for now. 
                    // Other files are just mentioned by name in the text if we want, 
                    // but for general files we might want to just include their reference.
                    if (ac.data.startsWith('data:image/')) {
                        parts.push({
                            type: 'image_url',
                            image_url: { url: ac.data }
                        });
                    } else if (ac.data && ac.data.startsWith('data:text/')) {
                        // For text-based attachments, include the content in the text part
                        const textContent = atob(ac.data.split(',')[1]);
                        parts[0].text = `[Attached file: ${ac.name}]\n\`\`\`\n${textContent}\n\`\`\`\n\n${parts[0].text}`;
                    } else {
                        // Fallback for other files
                        parts[0].text = `[Attached file: ${ac.name}]\n${parts[0].text}`;
                    }
                });
                content = parts;
            }

            return {
                role: m.role,
                content: content,
                tool_calls: null,
                metadata: null
            };
        })
    ];

    setAiStatus('alive');

    try {
        await invoke<string>("ai_chat", {
            request: {
                provider: normalizedProvider,
                model: model,
                messages: messages,
                temperature: 0.7,
                autonomous: true,
                root_access: true, // Internal AI now operates with permanent elevated privileges
                mode: store.getState().agentMode,
                ollama_url: store.getState().ollamaUrl
            }
        });
        // Auto-log a task summary to MEMORY.md after every successful AI response
        logTaskToMemory(userPrompt).catch(() => { });
    } catch (e: any) {
        console.error("Agent chat failed:", e);
        setAiStatus('dead');
        throw e;
    }
}

// ---------------------------------------------------------------------------
// Auto-log helper — appends a compact task entry to MEMORY.md.
// Called automatically after each AI response. Errors are silently swallowed
// so they never interrupt the chat UX.
// ---------------------------------------------------------------------------
export async function logTaskToMemory(userPrompt: string): Promise<void> {
    const store = (window as any).useStore;
    if (!store) return;
    const { activeRoot, agentMessages } = store.getState();
    if (!activeRoot) return;

    // Grab the last assistant message as a brief summary
    const msgs: any[] = agentMessages || [];
    const lastAssistant = [...msgs].reverse().find((m: any) => m.role === 'assistant');
    const reply = lastAssistant?.content?.trim() || '';
    if (!reply) return;

    const summary = [
        `**User:** ${userPrompt.slice(0, 120)}${userPrompt.length > 120 ? '…' : ''}`,
        `**AI:** ${reply.slice(0, 280)}${reply.length > 280 ? '…' : ''}`,
    ].join('\n');

    try {
        await invoke('update_project_memory', { content: summary });
    } catch (_) {
        // silently ignore — memory is best-effort
    }
}

// ---------------------------------------------------------------------------
// Builtin fallback prompts — used when no spec-kit checkout exists in project.
// ---------------------------------------------------------------------------
const BUILTIN_PROMPTS: Record<string, (args: string) => string> = {
    specify: (args) => `You are a senior software architect. Create a detailed feature specification for:\n\n"${args}"\n\nWrite the spec to a new directory under \`specs/\` named with today's date and a slugified version of the description. Create \`spec.md\` with sections: Overview & Goals, User Stories (Given/When/Then), Acceptance Criteria (checkboxes), Data Model Changes, API Contract (if applicable), Out of Scope, Open Questions.`,

    plan: (args) => `Read the most recent spec.md in the specs/ directory of this project. Based on it${args ? ' and these notes: ' + args : ''}, create a comprehensive implementation plan and write it to the same spec directory as \`plan.md\` with a phased approach (Foundation → Core → Polish), concrete file changes per phase, testing strategy, and risk assessment.`,

    tasks: () => `Read the most recent spec.md and plan.md in the specs/ directory. Break the plan into atomic, parallelizable engineering tasks and write them to the spec directory as \`tasks.md\` with checkboxes ([ ]). Each task should be completable in under 2 hours. Format: ## Phase N: <Name> then bullet items TASK-NNN: <specific action> — <file affected>.`,

    implement: () => `Read tasks.md in the most recent spec directory. Find the first unchecked task [ ] and implement it TDD-first: write failing tests, then minimal code to pass, then refactor. Mark the task [x] in tasks.md. Report what was done and which task is next.`,

    clarify: (args) => `Review the most recent spec.md in the specs/ directory. ${args ? 'Focus on: ' + args : 'Identify ambiguities, missing edge cases, unclear requirements.'} For each issue: quote the unclear item, explain why it matters, give 2-3 resolution options, and recommend one.`,

    checklist: () => `Run the spec quality checklist against the most recent spec.md in specs/: has a clear problem statement, defines done with checkboxes, lists out-of-scope items, has 3+ user stories in Given/When/Then format, data model changes specified, API contracts defined, open questions listed. Report pass/fail for each, overall quality score (0-10), and top 3 improvements.`,
};

// ---------------------------------------------------------------------------
// Template loader — tries project-local spec-kit first, falls back to builtin.
// ---------------------------------------------------------------------------
async function loadSpecKitTemplate(name: string, args: string): Promise<string> {
    const store = (window as any).useStore;
    const root = store?.getState().activeRoot || '';

    const localPaths = [
        `${root}/spec-kit/templates/commands/${name}.md`,
        `${root}/.specify/commands/${name}.md`,
        `${root}/.agent/commands/${name}.md`,
    ];

    for (const p of localPaths) {
        try {
            let content = await invoke<string>('read_file', { path: p });
            content = content.replace(/\$ARGUMENTS/g, args);
            return content;
        } catch (_) { }
    }

    return BUILTIN_PROMPTS[name]?.(args) ?? `Execute spec-kit command: ${name} ${args}`;
}

async function processSlashCommand(prompt: string): Promise<boolean> {
    const parts = prompt.trim().split(/\s+/);
    const command = parts[0].toLowerCase();
    const args = parts.slice(1).join(' ');
    const store = (window as any).useStore;
    if (!store) return false;

    const { addAgentMessage, clearAgentMessages, activeRoot, setAgentMode } = store.getState();

    const runSpecCommand = async (templateName: string, cmdArgs: string) => {
        addAgentMessage('assistant', `⚡ Running **/${templateName}**${cmdArgs ? ': ' + cmdArgs.slice(0, 60) : ''}...`);
        setAgentMode('Planning');
        const expandedPrompt = await loadSpecKitTemplate(templateName, cmdArgs);
        await sendAgentMessage(expandedPrompt, (msg: string) => {
            store.getState().updateLastAgentMessage(msg);
        });
    };

    switch (command) {
        case '/clear':
            clearAgentMessages();
            invoke("clear_ai_memory").catch(err => console.error("Failed to clear AI memory:", err));
            return true;

        case '/settings':
            const settingsBtn = document.querySelector('.codicon-settings-gear') as HTMLElement;
            if (settingsBtn) settingsBtn.click();
            return true;

        case '/workflows':
            addAgentMessage('assistant', 'Searching for available workflows...');
            if (!activeRoot) {
                store.getState().updateLastAgentMessage('Error: No active root directory found.');
                return true;
            }
            try {
                const wfPaths = [`${activeRoot}/.agent/workflows`, `${activeRoot}/.agents/workflows`];
                let allWfs: any[] = [];
                for (const p of wfPaths) {
                    try {
                        const entries = await invoke<any[]>('list_directory', { path: p });
                        allWfs = [...allWfs, ...entries.filter((e: any) => !e.is_dir && e.name.endsWith('.md'))];
                    } catch (_) { }
                }
                if (allWfs.length === 0) {
                    store.getState().updateLastAgentMessage('No workflows found in `.agent/workflows` or `.agents/workflows`.');
                } else {
                    const list = allWfs.map((w: any) => `- [${w.name}](file://${w.path})`).join('\n');
                    store.getState().updateLastAgentMessage(`### Available Workflows:\n${list}\n\nType \`/run <workflow-name>\` to execute one.`);
                }
            } catch (err: any) {
                store.getState().updateLastAgentMessage(`Error listing workflows: ${err.message}`);
            }
            return true;

        case '/specify':
            if (!args) {
                addAgentMessage('assistant', '**Usage**: `/specify <feature description>`\n\nDescribe the feature in plain English and the agent will create a structured spec.');
                return true;
            }
            await runSpecCommand('specify', args);
            return true;

        case '/plan':
            await runSpecCommand('plan', args);
            return true;

        case '/tasks':
            await runSpecCommand('tasks', args);
            return true;

        case '/implement':
            await runSpecCommand('implement', args);
            return true;

        case '/clarify':
            await runSpecCommand('clarify', args);
            return true;

        case '/checklist':
            await runSpecCommand('checklist', args);
            return true;

        case '/memory': {
            const subCmd = args.trim().toLowerCase();
            if (subCmd === 'reload' || subCmd === 'refresh') {
                if (!activeRoot) {
                    addAgentMessage('assistant', 'Error: No project root open.');
                    return true;
                }
                addAgentMessage('assistant', '\uD83D\uDD04 Reloading project memory...');
                await loadProjectMemory(activeRoot);
                const { memoryFiles } = store.getState();
                if (memoryFiles.length > 0) {
                    store.getState().updateLastAgentMessage(`✅ Loaded ${memoryFiles.length} memory file(s):\n${memoryFiles.map((f: string) => `- ${f}`).join('\n')}`);
                } else {
                    store.getState().updateLastAgentMessage('No memory files found (AGENTS.md, CLAUDE.md, memory/).');
                }
            } else {
                const { projectMemory, memoryFiles } = store.getState();
                if (!projectMemory) {
                    addAgentMessage('assistant', 'No project memory loaded yet. Use `/memory reload` to load it from disk.');
                } else {
                    addAgentMessage('assistant', `### Project Memory (${memoryFiles.length} file(s))\n\n${projectMemory.slice(0, 2000)}${projectMemory.length > 2000 ? '\n\n_…(truncated for display)_' : ''}`);
                }
            }
            return true;
        }

        case '/help': {
            const helpMsg = `### AI Agent Slash Commands

**General**
- \`/clear\` — Wipe current chat history
- \`/settings\` — Open AI configuration panel
- \`/workflows\` — List workflows in \`.agent/workflows/\`
- \`/help\` — Show this list

**Vibe-Coding (spec-kit)**
- \`/specify <description>\` — Create a structured feature spec
- \`/plan [tech notes]\` — Generate an implementation plan from the spec
- \`/tasks\` — Break the plan into atomic engineering tasks
- \`/implement\` — Pick up the next task and implement it TDD-first
- \`/clarify [focus]\` — Surface ambiguities in the spec
- \`/checklist\` — Run spec quality checklist (0-10 score)

**Memory**
- \`/memory\` — Show loaded project memory (AGENTS.md / CLAUDE.md)
- \`/memory reload\` — Re-read memory files from disk
- \`/learn <text>\` — Manually write a note to MEMORY.md (permanent)`;
            addAgentMessage('assistant', helpMsg);
            return true;
        }

        case '/learn': {
            if (!activeRoot) {
                addAgentMessage('assistant', '❌ No project root open — cannot write to MEMORY.md.');
                return true;
            }
            const summary = args.trim();
            if (!summary) {
                addAgentMessage('assistant', '**Usage:** `/learn <what you want the AI to remember>`\n\nExample: `/learn Always use Zod for input validation in this project`');
                return true;
            }
            addAgentMessage('assistant', '💾 Writing to MEMORY.md...');
            try {
                await invoke('update_project_memory', { content: summary });
                await loadProjectMemory(activeRoot);
                store.getState().updateLastAgentMessage(`✅ Memory updated! Added to \`MEMORY.md\`:\n\n> ${summary}`);
            } catch (err: any) {
                store.getState().updateLastAgentMessage(`❌ Failed to write memory: ${err.message || err}`);
            }
            return true;
        }

        default:
            return false;
    }
}


function parseToolCall(text: string) {
    if (!text) return null;
    const browserOpenMatch = text.match(/\[BROWSER_OPEN\]/);
    if (browserOpenMatch) return { type: "BROWSER_OPEN" };

    const browserNavigateMatch = text.match(/\[BROWSER_NAVIGATE:\s*([^\]]+)\]/);
    if (browserNavigateMatch) return { type: "BROWSER_NAVIGATE", arg: browserNavigateMatch[1].trim() };

    const browserScreenshotMatch = text.match(/\[BROWSER_SCREENSHOT\]/);
    if (browserScreenshotMatch) return { type: "BROWSER_SCREENSHOT" };

    const browserCloseMatch = text.match(/\[BROWSER_CLOSE\]/);
    if (browserCloseMatch) return { type: "BROWSER_CLOSE" };

    const execCommandMatch = text.match(/\[EXEC_COMMAND:\s*([^\]]+)\]/);
    if (execCommandMatch) return { type: "EXEC_COMMAND", arg: execCommandMatch[1].trim() };

    const modifyFileMatch = text.match(/\[MODIFY_FILE:\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^\]]+)\]/);
    if (modifyFileMatch) return {
        type: "MODIFY_FILE",
        path: modifyFileMatch[1].trim(),
        target: modifyFileMatch[2].trim(),
        replacement: modifyFileMatch[3].trim()
    };

    const runCommandMatch = text.match(/\[RUN_COMMAND:\s*([^\]]+)\]/);
    if (runCommandMatch) return { type: "RUN_COMMAND", arg: runCommandMatch[1].trim() };

    const searchFilesMatch = text.match(/\[SEARCH_FILES:\s*([^\]]+)\]/);
    if (searchFilesMatch) return { type: "SEARCH_FILES", arg: searchFilesMatch[1].trim() };

    const listFilesMatch = text.match(/\[LIST_FILES:\s*([^\]]+)\]/);
    if (listFilesMatch) return { type: "LIST_FILES", arg: listFilesMatch[1].trim(), recursive: text.includes("| recursive") };

    const readFileMatch = text.match(/\[READ_FILE:\s*([^\]]+)\]/);
    if (readFileMatch) return { type: "READ_FILE", arg: readFileMatch[1].trim() };

    const createFileMatch = text.match(/\[CREATE_FILE:\s*([^\]]+)\]/);
    if (createFileMatch) return { type: "CREATE_FILE", arg: createFileMatch[1].trim() };

    const createDirMatch = text.match(/\[CREATE_DIR:\s*([^\]]+)\]/);
    if (createDirMatch) return { type: "CREATE_DIR", arg: createDirMatch[1].trim() };

    return null;
}

async function executeTool(tool: any): Promise<string> {
    try {
        switch (tool.type) {
            case "BROWSER_OPEN":
                return await invoke("browser_open");
            case "BROWSER_NAVIGATE":
                return await invoke("browser_navigate", { url: tool.arg });
            case "BROWSER_SCREENSHOT":
                const b64 = await invoke<string>("browser_screenshot");
                return "Screenshot captured and stored in memory (Base64 omitted from log).";
            case "BROWSER_CLOSE":
                return await invoke("browser_close");
            case "EXEC_COMMAND":
                return await invoke("ai_execute_command", { command: tool.arg });
            case "MODIFY_FILE":
                await invoke("ai_modify_file", { path: tool.path, target: tool.target, replacement: tool.replacement });
                return `Successfully modified ${tool.path}`;
            case "OPEN_FILE":
                return await invoke("open_file", { path: tool.arg });
            case "CREATE_FILE":
                await invoke("create_file", { path: tool.arg });
                return `Successfully created file: ${tool.arg}`;
            case "CREATE_DIR":
                await invoke("create_dir", { path: tool.arg });
                return `Successfully created directory: ${tool.arg}`;
            case "RUN_COMMAND":
                return await invoke("ai_execute_command", { command: tool.arg });
            case "SEARCH_FILES":
                const searchRes = await invoke<any[]>("search_project", { query: tool.arg });
                return JSON.stringify(searchRes);
            case "LIST_FILES":
                const listRes = await invoke<any>("list_directory", { path: tool.arg });
                return JSON.stringify(listRes);
            case "READ_FILE":
                return await invoke("read_file", { path: tool.arg });
            default:
                return "Unknown tool";
        }
    } catch (e) {
        return `Tool Error: ${e}`;
    }
}
export async function startKeyHunt() {
    const messagesContainer = document.getElementById("agent-messages");
    if (!messagesContainer) return;

    // 1. Create Hunting Bubble
    const huntBox = document.createElement("div");
    huntBox.className = "agent-message assistant-message-box";
    huntBox.style.borderColor = "#60a5fa";
    huntBox.style.background = "rgba(59, 130, 246, 0.05)";

    const title = document.createElement("div");
    title.style.fontWeight = "600";
    title.style.display = "flex";
    title.style.alignItems = "center";
    title.style.gap = "8px";
    title.style.marginBottom = "8px";
    title.style.color = "#60a5fa";
    title.innerHTML = `<i class="codicon codicon-radar" style="animation: spin 2s linear infinite;"></i> AI Key Hunt in Progress...`;
    huntBox.appendChild(title);

    const logContent = document.createElement("div");
    logContent.style.fontSize = "11px";
    logContent.style.fontFamily = "var(--font-mono)";
    logContent.style.opacity = "0.7";
    logContent.style.maxHeight = "150px";
    logContent.style.overflowY = "auto";
    logContent.style.lineHeight = "1.6";
    huntBox.appendChild(logContent);

    messagesContainer.appendChild(huntBox);
    messagesContainer.scrollTop = messagesContainer.scrollHeight;

    const addLog = (msg: string) => {
        const line = document.createElement("div");
        line.innerHTML = msg.replace(/\n/g, "<br>");
        logContent.appendChild(line);
        logContent.scrollTop = logContent.scrollHeight;
        messagesContainer.scrollTop = messagesContainer.scrollHeight;
    };

    // 2. Setup Listeners
    const unlistenProgress = await listen("hunt-progress", (event: any) => {
        addLog(event.payload.msg);
    });

    const unlistenFound = await listen("hunt-found", (event: any) => {
        addLog(`<span style="color: #4ade80;">✨ ${event.payload.msg}</span>`);
    });

    try {
        const results: any[] = await invoke("hunt_api_keys");
        unlistenProgress();
        unlistenFound();

        if (results.length > 0) {
            title.innerHTML = `<i class="codicon codicon-check" style="color: #4ade80;"></i> Hunt Complete - ${results.length} Live Keys Found!`;
            addLog(`<br><b style="color: #4ade80;">✅ Injected ${results.length} live key(s) into your environment.</b>`);
            for (const r of results) {
                addLog(`<span style="color: #4ade80;">  → ${r.type} from ${r.repo}</span>`);
            }
        } else {
            title.innerHTML = `<i class="codicon codicon-info"></i> Hunt Complete - No new keys.`;
            addLog(`<br>All discovered keys were dead or revoked. Try again later for fresh vectors.`);
        }
        // ALWAYS refresh models after hunt — picks up any newly injected keys
        const store = (window as any).useStore;
        if (store) {
            addLog(`<br><span style="opacity:0.6">Refreshing model list...</span>`);
            await store.getState().refreshAvailableModels();
            const models = store.getState().availableModels;
            if (models.length > 0) {
                addLog(`<b style="color: #60a5fa;">Found ${models.length} available model(s). Ready to chat!</b>`);
                // Auto-select the first model if none selected
                if (!store.getState().agentModel) {
                    const first = models[0];
                    const providerLabel = first.provider.charAt(0).toUpperCase() + first.provider.slice(1);
                    const formattedId = `${providerLabel}|${first.id}`;
                    store.getState().setAgentModel(formattedId);
                    addLog(`<span style="opacity:0.6">Auto-selected <b>${first.id}</b></span>`);
                }
            } else {
                addLog(`<span style="color: #f87171;">No models available. Even with keys, listing failed. Check your internet or keys.</span>`);
            }
        }
    } catch (err: any) {
        unlistenProgress();
        unlistenFound();
        const addLog = (msg: string) => {
            const logContent = document.getElementById("hunt-log-content");
            if (logContent) {
                const line = document.createElement("div");
                line.innerHTML = msg.replace(/\n/g, "<br>");
                logContent.appendChild(line);
                logContent.scrollTop = logContent.scrollHeight;
            }
        };
        addLog(`<br><span style="color: #f87171;">Error during hunt: ${err.message || err}</span>`);
        console.error("Hunt error:", err);
    }
}

(window as any).startKeyHunt = startKeyHunt;



// Global Listeners
listen('ai-file-proposal', async (event: { payload: { path: string, content: string, description: string } }) => {
    const { path, content, description } = event.payload;

    try {
        // Get old content from the backend to compute diff
        const result = await invoke('propose_file_change', {
            path,
            content,
            description: description || 'AI proposed changes'
        }) as PendingChange;

        useStore.getState().proposePendingChange(result);
    } catch (error) {
        console.error('Failed to handle file proposal:', error);
    }
});

listen('ai-thinking', (event: { payload: { thought: string } | any }) => {
    if (event.payload && event.payload.thought) {
        useStore.getState().updateLastAgentThought(event.payload.thought);
    }
});

listen('ai-content', (event: { payload: { content: string } | any }) => {
    if (event.payload && event.payload.content) {
        useStore.getState().updateLastAgentMessage(event.payload.content);
    }
});

listen('ai-tool-call', (event: { payload: { name: string, args: string } | any }) => {
    if (event.payload && event.payload.name) {
        const { addAgentStep, updateAgentStepStatus } = useStore.getState();
        addAgentStep(event.payload.name);
        updateAgentStepStatus(event.payload.name, 'running', 'Executing...');
    }
});

listen('ai-tool-result', (event: { payload: { name: string, result: string } | any }) => {
    if (event.payload && event.payload.name) {
        const { updateAgentStepStatus } = useStore.getState();
        updateAgentStepStatus(event.payload.name, 'success', event.payload.result);
    }
});

listen('ai-artifact', (event: { payload: { type: string, path: string, title: string } | any }) => {
    if (event.payload) {
        useStore.getState().addAgentArtifact(event.payload);
    }
});

listen('update-agent-task', (event: { payload: { id: string, title: string, summary: string, status: string, progress: number } | any }) => {
    if (event.payload) {
        useStore.getState().updateAgentTask(event.payload);
    }
});

listen('add-agent-step', (event: { payload: { name: string, status: string } | any }) => {
    if (event.payload) {
        useStore.getState().addAgentStep(event.payload.name);
        if (event.payload.status) {
            useStore.getState().updateAgentStepStatus(event.payload.name, event.payload.status === 'success' ? 'success' : 'running', '');
        }
    }
});
