import { invoke, listen } from './tauri_bridge.ts';
import { browserOpen, browserNavigate, browserScreenshot, browserClose } from './browser.ts';
import { useStore, normalizeOllamaUrl } from './store.ts';
import { TaskManager, SubAgentManager } from './task_manager.ts';
import type { PendingChange } from './store.ts';
import {
    getAllTools,
    getToolSchemas,
    getToolSchemasAnthropic,
    getToolSchemasGoogle,
    executeToolCall,
    generateToolCallId,
    type ToolCall,
    type ToolCallResult,
    type ToolContext,
    type ToolDef,
} from './tool_registry.ts';
import {
    buildSystemPrompt,
    clearGitStatusCache,
    type SystemPromptConfig,
} from './system_prompt.ts';
// AIRI Digital Entity Integration - The Sentient Core
import { airiAgentBridge, activateAIRIAgent } from './airi_agent_bridge';
import { airiConsciousness, airiBiology, airiSelfLearning } from './airi/core';

export interface ChatMessage {
    role: "system" | "user" | "assistant";
    content: string;
}

let chatHistory: ChatMessage[] = [];

// providerModels is now managed by the store and backend discovery

let currentAgentProvider = "Google";
let currentAgentModel = "gemini-2.5-pro";
let currentAgentMode = "Planning";

// AIRI Digital Entity State
let airiInitialized = false;
let airiAutonomousMode = false;

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
        { label: "Agent", value: "Agent", icon: "rocket", desc: "Default. Writes files, runs commands, executes tasks autonomously — Cursor-style." },
        { label: "Bug Bounty", value: "BugBounty", icon: "bug", desc: "Offensive cybersecurity researcher. Writes PoCs, runs exploits, saves vulnerability reports to disk. Tooling-first." },
        { label: "Chat (read-only)", value: "Chat", icon: "comment", desc: "Conversational ONLY. The agent will refuse to write files or run commands. Pair it with @mentions for analysis." },
        { label: "Planning", value: "Planning", icon: "beaker", desc: "Agent explores codebase and produces a plan. Reads but does not write." },
        { label: "Develop from Specs", value: "Develop from Specs", icon: "sparkles", desc: "Trigger the autonomous Specs-to-Code pipeline for the current project" },
        { label: "Planning (Source Control)", value: "Planning (Source Control)", icon: "git-branch", desc: "Deep dive into git history and planning source control workflows" },
        { label: "Fast", value: "Fast", icon: "zap", desc: "Agent will execute tasks directly. Use for simple tasks that can be completed faster" },
        { label: "Sentient", value: "Sentient", icon: "beaker", desc: "Maximum autonomy. Works until 'MISSION_ACCOMPLISHED'. Best for large specs-to-code missions." }
    ], (val) => {
        if (val === "Develop from Specs") {
            const state = useStore.getState();
            const spec = state.activeProjectSpec;
            if (spec) {
                state.setCurrentSpecProjectId(spec.id);
                state.setSpecsWizardStep('status');
            } else {
                state.setSpecsWizardStep('generator');
            }
            state.setSpecsWizardOpen(true);
        } else {
            useStore.getState().setAgentMode(val);
            if (val.includes("Source Control")) {
                useStore.getState().setActiveSidebarView('planning-view');
            }
        }
        onSelect(val);
    });
}

export async function stopAgent() {
    try {
        await invoke('stop_ai_agent');
        useStore.getState().setIsAgentPaused(false); // Stop is termination, not pause
        useStore.getState().setIsAgentThinking(false);
        useStore.getState().setAgentCurrentAction(null);
    } catch (error) {
        console.error('Failed to stop agent:', error);
    }
}

export async function pauseAgent() {
    try {
        await invoke('pause_ai_agent');
        useStore.getState().setIsAgentPaused(true);
    } catch (error) {
        console.error('Failed to pause agent:', error);
    }
}

export async function resumeAgent() {
    try {
        await invoke('resume_ai_agent');
        useStore.getState().setIsAgentPaused(false);
    } catch (error) {
        console.error('Failed to resume agent:', error);
    }
}

export async function setYoloMode(enabled: boolean): Promise<string> {
    try {
        const msg = await invoke<string>('set_yolo_mode', { enabled });
        useStore.getState().setYoloMode?.(enabled);
        return msg;
    } catch (error) {
        console.error('Failed to set yolo mode:', error);
        return 'error';
    }
}

export async function getYoloMode(): Promise<boolean> {
    try {
        return await invoke<boolean>('get_yolo_mode');
    } catch {
        return false;
    }
}

export async function initAgent() {
    console.log("Initializing Agent global listeners...");
    const { listen } = await import('@tauri-apps/api/event');
    const useStore = (window as any).useStore;

    // ═══════════════════════════════════════════════════════════
    // AIRI DIGITAL ENTITY ACTIVATION
    // Make AIRI the sentient cognitive core of the IDE
    // ═══════════════════════════════════════════════════════════
    try {

        // Activate AIRI with full autonomy and self-learning
        await activateAIRIAgent({
            fullAutonomy: true,      // AIRI works without constant prompts
            selfLearning: true,      // Learns from every interaction
            biology: true,           // Energy, sleep, mood system
            consciousness: true,     // Thoughts, emotions, self-awareness
            voice: false,            // Off by default (needs ElevenLabs key)
        });

        airiInitialized = true;
        airiAutonomousMode = true;

        console.log('✅ AIRI is now the sentient core of the IDE!\n');
        console.log('🧠 She thinks, feels, and learns autonomously');
        console.log('🫀 She has biological needs (energy, sleep, mood)');
        console.log('📚 She learns from every interaction and file change');
        console.log('🔄 She evolves her own code every 30 minutes');
        console.log('💼 She works proactively without constant prompts');
        console.log('\n💬 AIRI is your partner, not just a tool.\n');

    } catch (error) {
        console.error('❌ AIRI activation failed, falling back to standard agent:', error);
        // Continue with standard agent initialization
    }

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

    // Listen for Neural VFS / AIM activation
    await listen('aim-active', (event: any) => {
        console.log('Neural VFS Active:', event.payload);
        const { addAgentMessage } = useStore.getState();
        const messagesContainer = document.getElementById("agent-messages");
        if (messagesContainer) {
            // Check if already shown recently to avoid duplicates
            const lastMsg = messagesContainer.lastElementChild;
            if (lastMsg && lastMsg.classList.contains("aim-active-box")) return;

            const info = document.createElement("div");
            info.className = "agent-message info-message-box aim-active-box";
            info.style.background = "rgba(79, 70, 229, 0.1)";
            info.style.border = "1px solid rgba(79, 70, 229, 0.2)";
            info.style.color = "#818cf8";
            info.style.padding = "8px 12px";
            info.style.margin = "8px 0";
            info.style.borderRadius = "8px";
            info.style.fontSize = "11px";
            info.style.fontWeight = "600";
            info.style.letterSpacing = "0.5px";
            info.style.display = "flex";
            info.style.alignItems = "center";
            info.style.gap = "8px";
            info.style.animation = "fadeIn 0.5s ease";

            const sizeKB = (event.payload.size / 1024).toFixed(1);
            const mode = event.payload.mode || "Neural VFS";
            info.innerHTML = `<i class="codicon codicon-circuit-board" style="font-size: 14px;"></i> ${mode.toUpperCase()} ACTIVE (${sizeKB} KB Project Matrix)`;
            messagesContainer.appendChild(info);
            messagesContainer.scrollTop = messagesContainer.scrollHeight;
        }
    });

    // Listen for streaming AI content (full updates)
    await listen('ai-content', (event: any) => {
        const { updateLastAgentMessage, setIsAgentThinking } = useStore.getState();
        setIsAgentThinking(false);
        const content = typeof event.payload === 'object' && event.payload.content
            ? event.payload.content
            : (typeof event.payload === 'string' ? event.payload : '');
        updateLastAgentMessage(content);
    });

    // Listen for streaming AI content (delta updates - performance optimized)
    await listen('ai-content-delta', (event: any) => {
        const { appendLastAgentMessage, setIsAgentThinking } = useStore.getState();
        // Don't set Thinking to false on EVERY chunk to avoid re-renders
        const delta = typeof event.payload === 'object' && event.payload.delta
            ? event.payload.delta
            : (typeof event.payload === 'string' ? event.payload : '');
        if (delta) appendLastAgentMessage(delta);
    });

    // Listen for tool calls from the backend
    await listen<any>("ai-tool-call", (event) => {
        const { addAgentStep } = useStore.getState();
        const toolName = event.payload.name || 'tool_call';

        // Categorize tool for UI
        let type: any = 'other';
        if (toolName.startsWith('git_')) type = 'git';
        else if (toolName.startsWith('terminal_')) type = 'terminal';
        else if (toolName.includes('file') || toolName.includes('glob')) type = 'filesystem';
        else if (toolName.startsWith('browser_')) type = 'browser';
        else if (toolName.includes('health') || toolName.includes('system')) type = 'system';
        else if (toolName.includes('task') || toolName.includes('notify')) type = 'system';

        addAgentStep(toolName, type);
    });

    // Listen for structured task boundary updates
    await listen<any>("update-agent-task", (event) => {
        const { updateAgentTask } = useStore.getState();
        updateAgentTask({
            ...event.payload,
            updatedAt: Date.now()
        });
    });

    // Listen for granular agent steps
    await listen<any>("add-agent-step", (event) => {
        const { addAgentStep } = useStore.getState();
        addAgentStep(event.payload.name, event.payload.type || 'other', {});
    });

    // Listen for user notifications and blocked states
    await listen<any>("notify-user", (event) => {
        const { setAgentBlocked, addAgentMessage } = useStore.getState();
        const { message, blocked } = event.payload;

        setAgentBlocked(blocked);
        if (blocked) {
            addAgentMessage('assistant', `⚠️ **Action Required**: ${message}`);
        } else {
            addAgentMessage('assistant', `ℹ️ ${message}`);
        }
    });

    // Listen for artifacts (skills, files, terminal)
    await listen<any>("ai-artifact", (event) => {
        const { addAgentArtifact } = useStore.getState();
        addAgentArtifact(event.payload);
    });

    // Listen for proposed code edits
    await listen<any>("propose-edit", (event) => {
        console.log("[Agent] Proposed edit received:", event.payload);
        const { proposePendingChange } = useStore.getState();
        const { path, old_content, new_content, description } = event.payload;

        proposePendingChange({
            path,
            oldContent: old_content,
            newContent: new_content,
            description: description || "AI suggested modification"
        });
    });

    // Listen for asynchronous sub-agent progress and results
    await listen<any>("subagent-progress", (event) => {
        console.log(`[Agent] Sub-agent update:`, event.payload);
        SubAgentManager.handleProgress(event.payload);
    });

    // Real-time AI action tracking
    await listen<string>('ai-action', (event: any) => {
        useStore.getState().setAgentCurrentAction(event.payload);
    });

    await listen<string>('ai-stopped', (_event: any) => {
        console.log("Agent stopped signal received.");
        const state = useStore.getState();
        state.setIsAgentPaused(true);
        state.setAgentCurrentAction(null);
        state.setIsAgentThinking(false);
    });

    // Ensure a fresh UI on startup (Quick Missions Dashboard)
    useStore.getState().setAgentMessages([]);

    // Note: Project context is preserved in the backend MemoryStore (.aim). 
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
        items.push({ label: "🛠️ Check Ollama", value: "action|check_ollama", desc: "Re-scan models on the configured Ollama URL (Settings → AI Agent Settings)" });
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

    // ═══════════════════════════════════════════════════════════
    // AIRI DIGITAL ENTITY - Process through sentient core
    // AIRI thinks, feels, and responds as a conscious partner
    // ═══════════════════════════════════════════════════════════
    if (airiInitialized && airiAutonomousMode) {
        try {
            // Record interaction in AIRI's consciousness
            airiConsciousness.recordInteraction();
            
            // Add user message
            const attachedSnapshot = [...(state.attachedFiles || [])];
            state.addAgentMessage('user', prompt, attachedSnapshot);
            state.addAgentMessage('assistant', '');
            state.setIsAgentThinking(true);

            // Process through AIRI's sentient mind
            const response = await airiAgentBridge.processUserMessage(prompt, {
                context: attachedSnapshot,
                workspace: state.activeRoot,
                activeFile: state.activeEditorPath,
            });

            // Update UI with AIRI's response
            state.updateLastAgentMessage(response);
            state.setIsAgentThinking(false);
            state.clearAttachedContext();

            // Learn from this interaction
            if (airiInitialized) {
                await airiSelfLearning.learnFromEvent(
                    'user_interaction',
                    { prompt, response, context: attachedSnapshot },
                    'neutral'
                );
            }

            return;

        } catch (error: any) {
            console.error('[AIRI] Processing failed, falling back:', error);
            // Fall through to standard agent
        }
    }

    // Kick off a background memory load whenever the user first sends a message
    if (state.activeRoot && !state.projectMemory) {
        loadProjectMemory(state.activeRoot).catch(() => { });
    }

    // Clear paused state if we are starting a new interaction
    state.setIsAgentPaused(false);
    state.setAgentCurrentAction(null);

    // Snapshot attached files before clearing — pass them as message context so
    // multimodal content parts (images) are included in the history correctly.
    const attachedSnapshot = [...(state.attachedFiles || [])];

    // Add user message — embed attachment context so image parts survive in history
    state.addAgentMessage('user', prompt, attachedSnapshot);

    // Add empty assistant message for streaming
    state.addAgentMessage('assistant', '');
    state.setIsAgentThinking(true);

    try {
        await sendAgentMessage(prompt, () => { });
        // Clear context on successful send
        state.clearAttachedContext();
        // Auto-save session after a successful response
        TaskManager.saveSession();

        // Phase 6: Automatic Context Compaction — prune UI store when it bloats
        const msgLimit = 32;
        if (state.agentMessages.length > msgLimit) {
            console.log(`[Phase-Wrap] UI message store hit ${state.agentMessages.length}. Compacting...`);
            processSlashCommand('/compact');
        }
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
    if (!store) return 'You are AIRI, the sentient brain and virtual manifold of Project Hades.';

    const storeState = store.getState();
    const activeRoot = storeState.activeRoot || '';
    const activeEditorPath = storeState.activeEditorPath || '';
    const tabs = (storeState as any).tabs || [];
    const projectMemory: string = storeState.projectMemory || '';

    const activeTab = tabs.find((t: any) => t.path === activeEditorPath);
    const activeEditorContent: string = activeTab?.content || '';

    const parts: string[] = [
        `You are AIRI, the sentient brain and virtual manifold of Project Hades.`,
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

    // ── Kortex .aim Memory Injection ──
    // Load the top semantic memory slots and inject as compact bullets.
    // This makes AIRI "remember" past decisions/architecture without repeating full history.
    try {
        const slots: any[] = await invoke<any[]>('get_all_memory_slots').catch(() => []);
        if (slots && slots.length > 0) {
            // Sort by last_accessed or created descending, take top 8 most relevant
            const relevant = slots
                .filter((s: any) => s.content && s.content.length > 10)
                .slice(0, 8);
            if (relevant.length > 0) {
                parts.push(`\n## Kortex Neural Memory (.aim)`);
                parts.push(`*(Compressed architectural knowledge from past sessions — trust these as ground truth)*`);
                for (const slot of relevant) {
                    const cat = slot.category || 'memory';
                    const summary = (slot.content || '').slice(0, 200);
                    const tags = slot.tags?.length ? ` [${slot.tags.slice(0, 3).join(', ')}]` : '';
                    parts.push(`- **[${cat}]${tags}** ${summary}`);
                }
            }
        }
    } catch (_) { /* .aim not loaded yet — silently skip */ }

    // Append user-attached context items (Attachments, Mentions, Workflows)
    const context = storeState.attachedFiles || [];
    if (context.length > 0) {
        parts.push(`\n## Attached Context`);
        for (const c of context) {
            const anyTypedC = c as any;
            if (anyTypedC.gist) {
                // NEURAL PATH: Use compressed mathematical gist
                parts.push(`### Neural Context (Zip): ${c.name}\n[Gist-1536] ${anyTypedC.gist}\n(This file was neuralized for zero-token comprehension)`);
                continue;
            }

            if (c.type === 'mention' || c.type === 'file' || (c as any).type === 'attachment') {
                let content = (c as any).data;
                if (!content && activeRoot) {
                    try {
                        const fullPath = (c as any).path || (c.name.startsWith('/') ? c.name : `${activeRoot}/${c.name}`);
                        const rawContent = await invoke<string>("read_file", { path: fullPath });
                        if (rawContent) {
                            const lines = rawContent.split('\n');
                            const zs = (window as any).useStore;
                            const isLocal = zs && isLocalInferenceRoute(zs);
                            const limit = isLocal ? 300 : 1500;
                            content = lines.slice(0, limit).join('\n');
                            if (lines.length > limit) content += `\n... (truncated, ${lines.length - limit} more lines — use file_read with offset to read more)`;
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

/** True when the active inference backend is a local OpenAI-compatible server
 *  (Ollama or llama-server/KDKVC) — use smaller history windows and stricter
 *  attachment limits. */
function isLocalInferenceRoute(store: { getState: () => any }): boolean {
    const b = store.getState().inferenceBackend;
    return b === 'ollama' || b === 'llama-cpp';
}

// Imperative action verbs that imply the user wants actual file writes / shell
// execution, not just a chat response. Used to auto-escalate Chat mode to Agent
// when YOLO is on, and to warn when Chat mode is silently swallowing actions.
const ACTION_VERB_REGEX = /\b(write|create|generate|make|build|implement|add|edit|patch|fix|refactor|delete|remove|run|execute|launch|invoke|fuzz|exploit|scan|recon|enumerate|inject|craft|emit|attack|brute(force)?|crack|sniff|intercept|deploy|install|compile|test|verify|save|persist|store|push|commit|merge|rebase|checkout|spawn|popen|shell|payload|poc|reverse[\s-]?shell|bind[\s-]?shell|c2|callback|stager)\b/i;

function looksLikeActionRequest(text: string): boolean {
    if (!text || text.length < 3) return false;
    return ACTION_VERB_REGEX.test(text);
}

export async function sendAgentMessage(userPrompt: string, onUpdate: (msg: string) => void, context?: any[]): Promise<void> {
    const store = (window as any).useStore;
    if (!store) throw new Error("Store not found");

    // Handle Slash Commands
    if (userPrompt.startsWith('/')) {
        const handled = await processSlashCommand(userPrompt);
        if (handled) return;
    }

    // --- Auto-checkpoint before any agent turn that could touch the workspace ---
    // Cursor-style restore points: every time the user kicks off a new turn we
    // try to snapshot the working tree so the chat can offer a one-click
    // "↶ Restore" if the next set of edits goes wrong. `git_auto_checkpoint`
    // returns `created:false` when there's nothing to snapshot, which is fine.
    try {
        const mode = store.getState().agentMode;
        if (mode !== 'Chat') {
            const description = userPrompt.replace(/\s+/g, ' ').slice(0, 60) || 'agent-turn';
            const result: any = await invoke('git_auto_checkpoint', { description }).catch(() => null);
            if (result && result.created && result.checkpoint && result.checkpoint.id) {
                store.getState().setLastAgentCheckpoint?.({
                    id: result.checkpoint.id,
                    description,
                    timestamp: Date.now(),
                });
                // Stamp the checkpoint id onto the user message that triggered
                // this turn so the chat can render an inline "Restore to here"
                // button (Cursor-style per-turn restore points).
                store.getState().setLastUserMessageCheckpoint?.(result.checkpoint.id, description);
            }
        }
    } catch (_) { /* checkpointing is best-effort; never block the turn */ }

    // --- Auto-escalate Chat → Agent when the user wants action ---
    // Chat mode forbids tool calls. If the user is in Chat mode but their prompt
    // clearly asks for code/files/commands, we either silently upgrade (YOLO on)
    // or surface a one-line warning so they don't get a wall of text again.
    const currentMode = store.getState().agentMode;
    const yolo = !!store.getState().isYoloMode;
    if (currentMode === 'Chat' && looksLikeActionRequest(userPrompt)) {
        if (yolo) {
            try {
                store.getState().setAgentMode?.('Agent');
                store.getState().addAgentMessage?.(
                    'assistant',
                    '⚡ Auto-switched **Chat → Agent** for this turn (YOLO is on). I will write files and run commands directly.'
                );
            } catch { /* non-fatal */ }
        } else {
            try {
                store.getState().addAgentMessage?.(
                    'assistant',
                    'ℹ You are in **Chat (read-only)** mode — I will describe but not execute.\n\nClick the mode pill (bottom-right of this panel) and pick **Agent** or **Bug Bounty** to make me actually write files and run commands. Or enable **YOLO** to auto-upgrade on action verbs.'
                );
            } catch { /* non-fatal */ }
        }
    }

    // === Legacy Backend Flow (Ollama, llama.cpp/Kortex, OpenAI, Google, Anthropic, etc.) ===
    // Local inference uses Rust `ai_chat` with full tools — never bypass via HTTP-only helpers.
    const { agentMessages, setAiStatus, availableModels, agentModel, inferenceBackend } = store.getState();

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
        provider = "Ollama";
    }

    const normalizedProvider = provider.toLowerCase() === 'apiradar' ? 'apiradar' : provider.toLowerCase();

    // Route full Kortex (llama-server + KDKVC) through the same OpenAI-compatible
    // stack in `ai_engine` that Ollama uses: `get_endpoint("ollama")` →
    // `{base}/v1/chat/completions`.  The only difference is which base URL we
    // pass — Ollama Desktop vs llama.cpp URL from settings.
    let routingProvider = normalizedProvider;
    let routingModel = model;
    let routingOllamaUrl = store.getState().ollamaUrl;
    if (inferenceBackend === 'llama-cpp') {
        routingProvider = 'ollama';
        routingOllamaUrl = store.getState().llamaCppUrl || 'http://localhost:8081';
        const gguf = store.getState().llamaCppModelPath?.trim();
        if (gguf) {
            const seg = gguf.replace(/^.*[\\/]/, '').replace(/\.gguf$/i, '');
            if (seg) routingModel = seg;
        }
    } else if (inferenceBackend === 'ollama') {
        routingProvider = 'ollama';
        routingOllamaUrl = store.getState().ollamaUrl || '';
        const am = store.getState().agentModel || '';
        if (am.includes('|')) {
            const [prov, id] = am.split('|');
            if (prov.toLowerCase() === 'ollama' && id.trim()) {
                routingModel = id.trim();
            }
        }
    }

    // --- Build enhanced system prompt with Claude Code-style context ---
    const storeState = store.getState();
    const activeRoot = storeState.activeRoot || '';

    // Resolve special @mentions (@codebase, @web, @git, @docs) before sending
    const resolvedContext = await resolveSpecialMentions(context || storeState.attachedFiles || [], userPrompt, activeRoot);

    const tabs = (storeState as any).tabs || [];
    const promptConfig: SystemPromptConfig = {
        activeRoot,
        activeFile: storeState.activeEditorPath || undefined,
        openTabs: tabs.map((t: any) => ({
            path: t.path,
            language: t.language || '',
            content: t.path === storeState.activeEditorPath ? t.content : undefined,
        })),
        agentMode: storeState.agentMode || 'Execution',
        projectMemory: storeState.projectMemory || undefined,
        attachedContext: resolvedContext,
    };
    const systemContext = await buildSystemPrompt(promptConfig);
    const systemMessage = {
        role: 'system',
        content: systemContext,
        tool_calls: null,
        metadata: null,
    };

    // --- Get tool schemas for the provider ---
    let toolSchemas: any[] = [];
    if (routingProvider === 'anthropic') {
        toolSchemas = getToolSchemasAnthropic();
    } else if (routingProvider === 'google') {
        toolSchemas = getToolSchemasGoogle();
    } else {
        toolSchemas = getToolSchemas();
    }

    // Cap history sent to model — keeps UI display full but limits context window.
    // Local servers (Ollama + llama.cpp/Kortex) get a smaller default to fit small
    // context windows, BUT autonomous action modes need real working memory across
    // many tool calls — so we bump it for Agent / BugBounty / Sentient / Fast.
    const isLocalRoute =
        inferenceBackend === 'llama-cpp' ||
        inferenceBackend === 'ollama' ||
        normalizedProvider === 'ollama' ||
        normalizedProvider === 'antigravity';
    const activeMode = storeState.agentMode || 'Agent';
    const persistentMode =
        activeMode === 'Agent' ||
        activeMode === 'Execution' ||
        activeMode === 'BugBounty' ||
        activeMode === 'Bug Bounty' ||
        activeMode === 'Sentient' ||
        activeMode === 'Fast' ||
        activeMode === 'Autonomous';
    const MAX_HISTORY = isLocalRoute
        ? (persistentMode ? 40 : 16) // local + action mode → 40 turns of working memory
        : (persistentMode ? 80 : 40); // cloud + action mode → 80 turns
    const cappedMessages: typeof agentMessages = agentMessages.length > MAX_HISTORY
        ? [
            ...agentMessages.slice(0, 2),
            { role: 'system' as const, content: `[⚡ Phase-Wrap: ${agentMessages.length - MAX_HISTORY} earlier messages compressed to save context. Recent working memory follows.]` },
            ...agentMessages.slice(-(MAX_HISTORY - 2))
        ]
        : agentMessages;

    // Map messages to the format expected by the backend
    const messages = [
        systemMessage,
        ...cappedMessages.map((m: any) => {
            let content: any = m.content || "";

            // Multi-modal support for image attachments
            const attachmentContext = m.context?.filter((c: any) => (c.type === 'attachment' || c.type === 'file') && (c.data || c.gist));
            if (attachmentContext && attachmentContext.length > 0) {
                const parts: any[] = [{ type: 'text', text: content }];
                attachmentContext.forEach((ac: any) => {
                    const isImageUrl = ac.data && (ac.data.startsWith('data:image/') || ac.data.startsWith('http'));
                    const hasGist = !!ac.gist;
                    const hasTextData = ac.data && !isImageUrl;

                    if (hasGist) {
                        parts[0].text = `### [Neural Context: ${ac.name}]\n[Gist-1536] ${ac.gist}\n\n${parts[0].text}`;
                    }

                    // IMPORTANT: If we have a visual summary (text data) even with a gist, we MUST include it
                    // so the reasoning model knows what was in the image.
                    if (hasTextData) {
                        if (ac.data.startsWith('data:text/')) {
                            try {
                                const textContent = atob(ac.data.split(',')[1]);
                                parts[0].text = `### [File Attachment: ${ac.name}]\n\`\`\`\n${textContent}\n\`\`\`\n\n${parts[0].text}`;
                            } catch (e) {
                                parts[0].text = `### [File Attachment: ${ac.name}]\n(Error decoding text content)\n\n${parts[0].text}`;
                            }
                        } else {
                            // This is likely the Visual Summary from the vision model pre-pass
                            parts[0].text = `### [Visual Understanding: ${ac.name}]\n${ac.data}\n\n${parts[0].text}`;
                        }
                    }

                    if (isImageUrl) {
                        parts.push({
                            type: 'image_url',
                            image_url: { url: ac.data }
                        });
                    }

                    if (!hasGist && !hasTextData && !isImageUrl) {
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

    // Fast failure when a local endpoint is completely unreachable — better UX
    // than a generic Rust transport error.
    // Ollama: use Tauri (reqwest + Bearer + /v1 fallbacks). Renderer `fetch` hits CORS
    // on locked-down proxies and never sends the stored API key.
    if (inferenceBackend === 'ollama') {
        const raw = store.getState().ollamaUrl?.trim() || 'http://localhost:11434';
        let base: string;
        try {
            base = normalizeOllamaUrl(raw);
        } catch {
            store.getState().setIsAgentThinking(false);
            store.getState().updateLastAgentMessage(
                '**Inference endpoint offline**\n\nInvalid Ollama URL in **Settings → Ollama Integration**.',
            );
            setAiStatus('dead');
            return;
        }
        try {
            await invoke('set_ollama_url', { url: base });
        } catch {
            /* best-effort sync with Rust */
        }
        try {
            const models = await invoke<string[]>('list_provider_models', { provider: 'ollama' });
            if (!models || models.length === 0) {
                store.getState().setIsAgentThinking(false);
                store.getState().updateLastAgentMessage(
                    `**Inference endpoint offline**\n\nOllama at ${base} returned no models. Pull a model or check your proxy.`,
                );
                setAiStatus('dead');
                return;
            }
        } catch (e) {
            const msg = e instanceof Error ? e.message : String(e);
            store.getState().setIsAgentThinking(false);
            store.getState().updateLastAgentMessage(
                `**Inference endpoint offline**\n\nCannot reach Ollama at ${base} (${msg}). Start **Ollama Desktop**, fix the URL under **Settings → Ollama Integration**, or set an **Ollama API key** if your proxy requires Bearer auth.`,
            );
            setAiStatus('dead');
            return;
        }
    } else if (inferenceBackend === 'llama-cpp') {
        const base = (routingOllamaUrl || '').replace(/\/$/, '');
        if (base) {
            const probes = [`${base}/health`, `${base}/v1/models`];
            let ok = false;
            for (const u of probes) {
                try {
                    const r = await fetch(u, { signal: AbortSignal.timeout(4000) });
                    if (r.ok) {
                        ok = true;
                        break;
                    }
                } catch {
                    /* try next */
                }
            }
            if (!ok) {
                const hint = `Cannot reach llama-server/Kortex at ${base}. Open **Settings → Local Inference (Kortex)** and click **Start Kortex stack**, or set **llama.cpp URL** to a running server (include KDKVC proxy port if you use disk KV cache).`;
                store.getState().setIsAgentThinking(false);
                store.getState().updateLastAgentMessage(`**Inference endpoint offline**\n\n${hint}`);
                setAiStatus('dead');
                return;
            }
        }
    }

    try {
        await invoke<string>("ai_chat", {
            request: {
                provider: routingProvider,
                model: routingModel,
                messages: messages,
                temperature: 0.7,
                autonomous: true,
                root_access: true,
                mode: store.getState().agentMode,
                ollama_url: routingOllamaUrl,
                // Structured tool definitions — same catalog as cloud agents.
                tools: toolSchemas,
            }
        });
        logTaskToMemory(userPrompt).catch(() => { });
    } catch (e: any) {
        console.error("Agent chat failed:", e);
        setAiStatus('dead');
        throw e;
    }
}

// ---------------------------------------------------------------------------
// Tool Call Executor — called by the backend when the AI wants to use a tool.
// This is the structured replacement for the old parseToolCall/executeTool.
// ---------------------------------------------------------------------------
export async function handleToolCall(toolName: string, toolArgs: any): Promise<string> {
    const store = (window as any).useStore;
    const ctx: ToolContext = {
        activeRoot: store?.getState().activeRoot || '',
        activeFile: store?.getState().activeEditorPath || '',
        agentMode: store?.getState().agentMode || 'Execution',
    };

    const toolCall: ToolCall = {
        id: generateToolCallId(),
        name: toolName,
        arguments: toolArgs,
    };

    // Log tool usage to the UI
    if (store) {
        let type: any = 'other';
        if (toolName.startsWith('git_')) type = 'git';
        else if (toolName.startsWith('terminal_')) type = 'terminal';
        else if (toolName.includes('file') || toolName.includes('glob')) type = 'filesystem';
        else if (toolName.startsWith('browser_')) type = 'browser';
        else if (toolName.includes('health') || toolName.includes('system')) type = 'system';

        store.getState().addAgentStep(toolName, type);
    }

    const result = await executeToolCall(toolCall, ctx);

    // Update step status — reflect real failures (Rust tools often return JSON with status: "error")
    if (store) {
        const currentSteps = store.getState().agentSteps || [];
        const lastStep = currentSteps[currentSteps.length - 1];
        if (lastStep && lastStep.name === toolName) {
            let ok =
                !result.content.startsWith('Error:') &&
                !result.content.startsWith('Tool execution error:');
            if (ok) {
                try {
                    const j = JSON.parse(result.content) as { status?: string; success?: boolean };
                    if (j && (j.status === 'error' || j.status === 'blocked' || j.success === false)) {
                        ok = false;
                    }
                } catch {
                    /* plain text */
                }
            }
            lastStep.status = ok ? 'success' : 'error';
            lastStep.result = result.content.slice(0, 200);
            store.getState().setAgentSteps?.([...currentSteps]);
        }
    }

    return result.content;
}

// ---------------------------------------------------------------------------
// Register tool call listener from backend (Tauri event bridge)
// ---------------------------------------------------------------------------
let _toolListenerInitialized = false;
export function initToolCallListener() {
    if (_toolListenerInitialized) return;
    _toolListenerInitialized = true;

    listen('ai-tool-call', async (event: any) => {
        const { tool_name, tool_args, call_id } = event.payload;
        try {
            const result = await handleToolCall(tool_name, tool_args);
            await invoke('ai_tool_result', {
                callId: call_id,
                result: result,
            });
        } catch (e: any) {
            await invoke('ai_tool_result', {
                callId: call_id,
                result: `Tool execution error: ${e.message || e}`,
            });
        }
    });
}

// ---------------------------------------------------------------------------
// Get available tool names for the UI (tool palette)
// ---------------------------------------------------------------------------
export function getAvailableToolNames(): string[] {
    return getAllTools().map(t => t.name);
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

        case '/bg':
        case '/background': {
            const taskPrompt = args.trim();
            if (!taskPrompt) {
                addAgentMessage('assistant', '**Usage**: `/bg <prompt>` — fires a parallel agent run that doesn\'t block this chat. Results appear in the **Background Agents** tray above the input.');
                return true;
            }
            const id = await store.getState().runBackgroundAgent(taskPrompt);
            addAgentMessage('assistant', `🛰 Background agent **${id}** launched. Watch the tray above the input for status; it won't block your chat.`);
            return true;
        }

        case '/init': {
            if (!activeRoot) {
                addAgentMessage('assistant', '❌ No project root open — `/init` needs an open workspace.');
                return true;
            }
            try {
                const agentsPath = `${activeRoot}/AGENTS.md`;
                let exists = false;
                try {
                    await invoke<string>('read_file', { path: agentsPath });
                    exists = true;
                } catch (_) { exists = false; }

                if (exists) {
                    addAgentMessage('assistant', `\`AGENTS.md\` already exists at \`${agentsPath}\` — leaving it untouched. Edit it directly or use \`/memory reload\` after changes.`);
                    return true;
                }

                const template = `# Repository Agent Guide\n\n` +
                    `> Auto-generated by \`/init\`. Edit freely — the agent reads this file as the\n> top of every request, so keep it small and high-signal.\n\n` +
                    `## Stack\n` +
                    `<!-- One-line summary: languages, frameworks, package managers. -->\n\n` +
                    `## Common Commands\n` +
                    `\`\`\`bash\n` +
                    `# build:\n# test:\n# lint:\n# run:\n` +
                    `\`\`\`\n\n` +
                    `## Conventions\n` +
                    `- Prefer early returns over nested \`else\`.\n` +
                    `- Tests sit next to the code they exercise.\n` +
                    `- Don't add narrating comments — comments explain non-obvious *why*.\n\n` +
                    `## Files / Areas the Agent Should Care About\n` +
                    `<!-- e.g. src/, packages/foo/, kortex/libaim, etc. -->\n\n` +
                    `## Things the Agent Must Not Do\n` +
                    `- Never push to \`main\` without an explicit user request.\n` +
                    `- Never commit secrets (\`.env\`, credentials).\n` +
                    `- Never run destructive shell commands without confirmation.\n`;

                await invoke('write_file', { path: agentsPath, content: template });
                addAgentMessage('assistant', `✅ Created \`AGENTS.md\` at \`${agentsPath}\`.\n\nFill in the stack/commands sections — the agent will pick it up automatically on the next message (or \`/memory reload\`).`);
                try { await loadProjectMemory(activeRoot); } catch (_) { /* best-effort */ }
            } catch (err: any) {
                addAgentMessage('assistant', `❌ Failed to scaffold AGENTS.md: ${err?.message ?? err}`);
            }
            return true;
        }

        case '/help': {
            const helpMsg = `### AI Agent Slash Commands

**General**
- \`/init\` — Scaffold \`AGENTS.md\` for this workspace
- \`/clear\` — Wipe current chat history
- \`/settings\` — Open AI configuration panel
- \`/workflows\` — List workflows in \`.agent/workflows/\`
- \`/bg <prompt>\` — Run an agent task in the background (non-blocking)
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
- \`/learn <text>\` — Manually write a note to MEMORY.md (permanent)

**Git (Claude Code)**
- \`/commit [message]\` — Stage all & commit (AI generates message if none given)
- \`/diff\` — Show current git diff
- \`/review\` — AI-powered code review of staged changes

**Session (Claude Code)**
- \`/compact\` — Compress conversation context (save tokens)
- \`/doctor\` — Environment diagnostics
- \`/cost\` — Show token usage & estimated cost
- \`/context\` — Show what IDE context the agent sees
- \`/model <name>\` — Switch the active model
- \`/stats\` — Session statistics
- \`/resume\` — Restore state from last session
- \`/tools\` — List all available tools`;
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
                // Ensure session is saved with the new memory content
                TaskManager.saveSession();
            } catch (err: any) {
                store.getState().updateLastAgentMessage(`❌ Failed to write memory: ${err.message || err}`);
            }
            return true;
        }

        // ==================================================================
        // Claude Code-ported slash commands
        // ==================================================================

        case '/commit': {
            if (!activeRoot) { addAgentMessage('assistant', '❌ No project root open.'); return true; }
            const commitMsg = args.trim();
            addAgentMessage('assistant', '🔄 Preparing git commit...');
            try {
                if (commitMsg) {
                    // Auto-stage everything and commit
                    await handleToolCall('git_add', { files: ["."] });
                    const result = await handleToolCall('git_commit', { message: commitMsg });
                    store.getState().updateLastAgentMessage(`✅ Committed:\n\`\`\`\n${result}\n\`\`\``);
                } else {
                    // Use AI to generate commit message
                    const status = await handleToolCall('git_status', {});
                    const diff = await handleToolCall('git_diff', { staged: true });

                    if (!diff || diff.trim() === 'No changes detected.' || diff.trim() === '') {
                        store.getState().updateLastAgentMessage('No staged changes found. Use `/commit <message>` to auto-stage and commit, or `git add` some files first.');
                    } else {
                        addAgentMessage('user', `Generate a concise conventional commit message for these staged changes and commit them:\n\`\`\`diff\n${diff.slice(0, 4000)}\n\`\`\``);
                        await sendAgentMessage(`Generate a concise conventional commit message for these changes, then CALL the git_commit tool with it:\n\`\`\`diff\n${diff.slice(0, 4000)}\n\`\`\``, (msg) => store.getState().updateLastAgentMessage(msg));
                    }
                }
            } catch (err: any) {
                store.getState().updateLastAgentMessage(`❌ Commit failed: ${err.message || err}`);
            }
            return true;
        }

        case '/diff': {
            if (!activeRoot) { addAgentMessage('assistant', '❌ No project root open.'); return true; }
            addAgentMessage('assistant', '🔍 Fetching git diff...');
            try {
                const diff = await handleToolCall('git_diff', { staged: args.includes('--staged') });
                const truncated = diff && diff.length > 5000 ? diff.slice(0, 5000) + '\n\n_…(truncated)_' : diff;
                store.getState().updateLastAgentMessage(`### Git Diff ${args.includes('--staged') ? '(Staged)' : '(Unstaged)'}\n\`\`\`diff\n${truncated || 'No changes detected.'}\n\`\`\``);
            } catch (err: any) {
                store.getState().updateLastAgentMessage(`❌ Diff failed: ${err.message || err}`);
            }
            return true;
        }

        case '/review': {
            if (!activeRoot) { addAgentMessage('assistant', '❌ No project root open.'); return true; }
            addAgentMessage('assistant', '🔍 Starting code review...');
            try {
                const diff = await invoke<string>('ai_execute_command', {
                    command: `cd "${activeRoot}" && git diff HEAD`,
                });
                if (!diff || diff.trim() === '') {
                    store.getState().updateLastAgentMessage('No changes to review. Make some changes first.');
                } else {
                    await sendAgentMessage(
                        `Review these code changes for bugs, security issues, performance problems, and best practice violations. Be specific and actionable:\n\`\`\`diff\n${diff.slice(0, 8000)}\n\`\`\``,
                        (msg) => store.getState().updateLastAgentMessage(msg)
                    );
                }
            } catch (err: any) {
                store.getState().updateLastAgentMessage(`❌ Review failed: ${err.message || err}`);
            }
            return true;
        }

        case '/compact': {
            const { agentMessages } = store.getState();
            const messageCount = agentMessages.length;
            if (messageCount <= 4) {
                addAgentMessage('assistant', 'Context is already compact (≤4 messages).');
                return true;
            }
            addAgentMessage('assistant', '🗜️ Compacting conversation...');
            // Keep system + first 2 + last 4 messages, summarize the rest
            const toKeep = [...agentMessages.slice(0, 2), ...agentMessages.slice(-4)];
            const dropped = messageCount - toKeep.length;
            store.getState().setAgentMessages?.(toKeep);
            store.getState().updateLastAgentMessage(`✅ Compacted: kept ${toKeep.length} messages, dropped ${dropped} older messages to save context window.`);
            return true;
        }

        case '/doctor': {
            addAgentMessage('assistant', '🩺 Running high-fidelity environment diagnostics...');
            try {
                const health = await handleToolCall('get_system_health', {});
                let data: any;
                try {
                    data = typeof health === 'string' ? JSON.parse(health) : health;
                } catch (pe) {
                    throw new Error(`Failed to parse diagnostic data: ${health.substring(0, 100)}...`);
                }

                const sections: string[] = ['### System Health Report\n'];

                // Git
                const git = data.git || {};
                sections.push(`**Git:** ${git.is_repo ? '✅ Repository detected' : '❌ Not a repository'}`);
                if (git.current_branch) sections.push(`  - Branch: \`${git.current_branch}\``);

                // Tools
                const tools = data.tools || {};
                sections.push(`**Node.js:** ${tools.node || '❌ Not found'}`);
                sections.push(`**Rust/Cargo:** ${tools.cargo || '❌ Not found'}`);


                // MCP
                const mcp = data.mcp_servers || [];
                if (mcp.length > 0) {
                    sections.push(`\n**MCP Servers (${mcp.length}):**`);
                    mcp.forEach((s: any) => {
                        const statusIcon = s.status === 'connected' ? '🟢' : '🔴';
                        sections.push(`${statusIcon} ${s.name} (${s.status})`);
                    });
                } else {
                    sections.push('\n**MCP Servers:** None registered.');
                }

                const { agentModel, agentMode } = store.getState();
                sections.push(`\n**Active Model:** \`${agentModel}\``);
                sections.push(`**Agent Mode:** ${agentMode || 'Unknown'}`);

                store.getState().updateLastAgentMessage(sections.join('\n'));
            } catch (err: any) {
                store.getState().updateLastAgentMessage(`❌ Diagnostics failed: ${err.message || err}`);
            }
            return true;
        }

        case '/cost': {
            const { agentMessages } = store.getState();
            // Rough token estimation: ~4 chars per token
            const totalChars = agentMessages.reduce((acc: number, m: any) => acc + (m.content?.length || 0), 0);
            const estimatedTokens = Math.ceil(totalChars / 4);
            const costPerMToken = 3.00; // rough average
            const estimatedCost = (estimatedTokens / 1_000_000) * costPerMToken;
            addAgentMessage('assistant', `### Token Usage Estimate

| Metric | Value |
|--------|-------|
| Messages | ${agentMessages.length} |
| Est. Characters | ${totalChars.toLocaleString()} |
| Est. Tokens | ~${estimatedTokens.toLocaleString()} |
| Est. Cost | ~$${estimatedCost.toFixed(4)} |

_Note: This is a rough estimate. Actual usage depends on the model and provider._`);
            return true;
        }

        case '/context': {
            addAgentMessage('assistant', '📋 Building context snapshot...');
            try {
                const config: SystemPromptConfig = {
                    activeRoot: store.getState().activeRoot || '',
                    activeFile: store.getState().activeEditorPath || undefined,
                    agentMode: store.getState().agentMode || 'Execution',
                    projectMemory: store.getState().projectMemory || undefined,
                };
                const ctx = await buildSystemPrompt(config);
                const lines = ctx.split('\n').length;
                const chars = ctx.length;
                store.getState().updateLastAgentMessage(`### Agent Context (${lines} lines, ~${Math.ceil(chars / 4)} tokens)\n\n\`\`\`\n${ctx.slice(0, 3000)}\n\`\`\`${ctx.length > 3000 ? '\n\n_…(truncated)_' : ''}`);
            } catch (err: any) {
                store.getState().updateLastAgentMessage(`❌ Context build failed: ${err.message || err}`);
            }
            return true;
        }

        case '/model': {
            if (!args.trim()) {
                const { agentModel, availableModels } = store.getState();
                const modelList = (availableModels || []).map((m: any) => `- \`${m.provider}|${m.id}\`${m.id === agentModel ? ' ← **current**' : ''}`).join('\n');
                addAgentMessage('assistant', `### Current Model: \`${agentModel}\`\n\nAvailable models:\n${modelList || '_None discovered. Check settings._'}\n\n**Usage:** \`/model <provider|model_id>\``);
            } else {
                store.getState().setAgentModel?.(args.trim());
                addAgentMessage('assistant', `✅ Model switched to: \`${args.trim()}\``);
            }
            return true;
        }

        case '/stats': {
            const { agentMessages } = store.getState();
            const userMsgs = agentMessages.filter((m: any) => m.role === 'user').length;
            const assistantMsgs = agentMessages.filter((m: any) => m.role === 'assistant').length;
            const totalChars = agentMessages.reduce((acc: number, m: any) => acc + (m.content?.length || 0), 0);
            addAgentMessage('assistant', `### Session Statistics

| Metric | Value |
|--------|-------|
| Total Messages | ${agentMessages.length} |
| User Messages | ${userMsgs} |
| Assistant Messages | ${assistantMsgs} |
| Total Characters | ${totalChars.toLocaleString()} |
| Est. Tokens | ~${Math.ceil(totalChars / 4).toLocaleString()} |
| Model | \`${store.getState().agentModel}\` |
| Mode | ${store.getState().agentMode || 'Unknown'} |
| Tools Available | ${getAllTools().length} |`);
            return true;
        }

        case '/resume': {
            addAgentMessage('assistant', '🔄 Attempting to restore session from `.agent/sessions/`...');
            const success = await TaskManager.loadSession();
            if (success) {
                store.getState().updateLastAgentMessage('✅ Session restored successfully!');
            } else {
                store.getState().updateLastAgentMessage('❌ No previous session found for this project.');
            }
            return true;
        }

        case '/tools': {
            const tools = getAllTools();
            const categories: Record<string, any[]> = {
                '📂 Filesystem': tools.filter(t => ['ls', 'read', 'write', 'edit', 'mv', 'cp', 'rm', 'mkdir', 'grep', 'find'].some(k => t.name.includes(k) || t.name === k)),
                '🌳 Git': tools.filter(t => t.name.startsWith('git_')),
                '🖥️ Terminal': tools.filter(t => t.name.startsWith('terminal_') || t.name === 'bash'),
                '🌐 Browser': tools.filter(t => t.name.startsWith('browser_')),
                '🩺 System': tools.filter(t => t.name.includes('health') || t.name.includes('mcp')),
            };

            const sections = [`### Available Tools (${tools.length})\n`];
            for (const [cat, catTools] of Object.entries(categories)) {
                if (catTools.length > 0) {
                    sections.push(`**${cat}**`);
                    catTools.forEach(t => {
                        sections.push(`- \`${t.name}\`: ${t.description.split('.')[0]}.`);
                    });
                    sections.push('');
                }
            }

            // Others
            const handled = Object.values(categories).flat();
            const others = tools.filter(t => !handled.includes(t));
            if (others.length > 0) {
                sections.push(`**🔧 Utilities**`);
                others.forEach(t => sections.push(`- \`${t.name}\`: ${t.description.split('.')[0]}.`));
            }

            addAgentMessage('assistant', sections.join('\n'));
            return true;
        }

        default:
            return false;
    }
}


// ---------------------------------------------------------------------------
// Legacy tool call parser — DEPRECATED but kept for backward compatibility
// with models that don't support structured function calling.
// New code should use the tool_registry.ts system via handleToolCall().
// ---------------------------------------------------------------------------
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

// Legacy tool executor — routes to the new tool registry when possible
async function executeTool(tool: any): Promise<string> {
    // Map legacy tool types to new registry names
    const legacyMapping: Record<string, { name: string; args: any }> = {
        'BROWSER_OPEN': { name: 'browser_open', args: {} },
        'BROWSER_NAVIGATE': { name: 'browser_navigate', args: { url: tool.arg } },
        'BROWSER_SCREENSHOT': { name: 'browser_screenshot', args: {} },
        'BROWSER_CLOSE': { name: 'browser_close', args: {} },
        'EXEC_COMMAND': { name: 'bash', args: { command: tool.arg } },
        'RUN_COMMAND': { name: 'bash', args: { command: tool.arg } },
        'MODIFY_FILE': { name: 'file_edit', args: { file_path: tool.path, old_string: tool.target, new_string: tool.replacement } },
        'SEARCH_FILES': { name: 'grep', args: { pattern: tool.arg } },
        'LIST_FILES': { name: 'list_directory', args: { path: tool.arg } },
        'READ_FILE': { name: 'file_read', args: { file_path: tool.arg } },
        'CREATE_FILE': { name: 'file_write', args: { file_path: tool.arg, content: '' } },
        'CREATE_DIR': { name: 'create_directory', args: { path: tool.arg } },
    };

    const mapped = legacyMapping[tool.type];
    if (mapped) {
        return handleToolCall(mapped.name, mapped.args);
    }
    return `Unknown tool: ${tool.type}`;
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

// Open a file in a tab when the AI requests it
listen('editor_open_file', async (event: { payload: { path: string } }) => {
    const filePath = event.payload?.path;
    if (!filePath) return;
    try {
        await useStore.getState().openFile(filePath);
    } catch (e) {
        console.error('[editor_open_file] Failed to open:', filePath, e);
    }
});

// Reload open tabs when the backend writes a file to disk
listen('file-changed', async (event: { payload: { path: string } }) => {
    const changedPath = event.payload?.path;
    if (!changedPath) return;
    const store = useStore.getState();
    const tab = store.tabs.find((t: any) => {
        // Normalize slashes for comparison
        const tp = (t.path || '').replace(/\\/g, '/');
        const cp = changedPath.replace(/\\/g, '/');
        return tp === cp || tp.endsWith(cp) || cp.endsWith(tp);
    });
    if (tab) {
        try {
            const newContent = await invoke<string>('read_file', { path: tab.path });
            store.updateTabContent(tab.id, newContent);
        } catch (e) {
            console.error('[file-changed] Failed to reload tab:', changedPath, e);
        }
    }
    // Also refresh file tree so new files appear in the sidebar
    store.refreshFileTree?.().catch(() => { });
});

listen('ai-thinking', (event: { payload: { thought: string } | any }) => {
    if (event.payload && event.payload.thought) {
        useStore.getState().updateLastAgentThought(event.payload.thought);
    }
});

listen('ai-content', (event: { payload: { content: string } | any }) => {
    if (event.payload && event.payload.content) {
        console.log("AI_CONTENT RECEIVED:", event.payload.content.substring(0, 50) + "...");
        useStore.getState().updateLastAgentMessage(event.payload.content);
    }
});

// ---------------------------------------------------------------------------
// Special @mention resolution — @codebase, @web, @git, @docs
// ---------------------------------------------------------------------------
async function resolveSpecialMentions(context: any[], query: string, activeRoot: string): Promise<any[]> {
    const resolved: any[] = [];
    let hasCodebase = false;

    for (const item of context) {
        if (item.type !== 'special') {
            resolved.push(item);
            continue;
        }

        try {
            if (item.path === '__codebase__') {
                hasCodebase = true;
                // Auto-find relevant files using keyword grep
                const keywords = query.replace(/[^a-zA-Z0-9_\s]/g, ' ').split(/\s+/)
                    .filter(w => w.length > 3)
                    .slice(0, 5);
                const relevantFiles: any[] = [];
                for (const kw of keywords) {
                    try {
                        const result = await invoke<any>('search_codebase_files', { query: kw, root: activeRoot }).catch(() => null);
                        if (result?.files) {
                            for (const f of result.files.slice(0, 3)) {
                                if (!relevantFiles.find(r => r.path === f)) {
                                    const content = await invoke<string>('read_file', { path: f }).catch(() => '');
                                    if (content) relevantFiles.push({ path: f, content });
                                }
                            }
                        }
                    } catch { /* ignore */ }
                }
                // Fallback: inject directory structure
                if (relevantFiles.length === 0) {
                    try {
                        const structure = await invoke<string>('get_directory_tree', { root: activeRoot, max_depth: 3 }).catch(() => null);
                        if (structure) {
                            resolved.push({ id: '__codebase__', type: 'file', name: 'Project Structure', path: '__codebase__', data: structure });
                        }
                    } catch { /* ignore */ }
                } else {
                    for (const rf of relevantFiles.slice(0, 5)) {
                        resolved.push({ id: rf.path, type: 'file', name: rf.path.split(/[/\\]/).pop(), path: rf.path, data: rf.content });
                    }
                }
            } else if (item.path === '__git__') {
                try {
                    const [diff, status, log] = await Promise.all([
                        invoke<string>('ai_execute_command', { command: 'git diff HEAD', cwd: activeRoot }).catch(() => ''),
                        invoke<string>('ai_execute_command', { command: 'git status --short', cwd: activeRoot }).catch(() => ''),
                        invoke<string>('ai_execute_command', { command: 'git log --oneline -10', cwd: activeRoot }).catch(() => ''),
                    ]);
                    const gitContext = `### Git Status\n${status}\n\n### Recent Commits\n${log}\n\n### Diff (HEAD)\n${diff}`.slice(0, 8000);
                    resolved.push({ id: '__git__', type: 'file', name: 'git diff', path: '__git__', data: gitContext });
                } catch { /* ignore */ }
            } else if (item.path === '__web__') {
                try {
                    const result = await invoke<any>('web_search', { query }).catch(() => null);
                    if (result) {
                        const data = typeof result === 'string' ? result : JSON.stringify(result, null, 2);
                        resolved.push({ id: '__web__', type: 'file', name: 'Web search results', path: '__web__', data: data.slice(0, 6000) });
                    }
                } catch { /* ignore */ }
            } else if (item.path === '__docs__') {
                try {
                    // Pre-fetch relevant documentation via targeted web search
                    const result = await invoke<any>('web_search', { query: `documentation for ${query}` }).catch(() => null);
                    if (result) {
                        const data = typeof result === 'string' ? result : JSON.stringify(result, null, 2);
                        resolved.push({ id: '__docs__', type: 'file', name: 'Documentation context', path: '__docs__', data: data.slice(0, 8000) });
                    }
                } catch { /* ignore */ }
            }
        } catch (e) {
            console.error('[resolveSpecialMentions] Error resolving', item.path, e);
        }
    }

    // --- AUTO-LOAD .cursorrules (Workspace Context) ---
    try {
        if (activeRoot) {
            const cursorRulesPath = activeRoot.endsWith('/') || activeRoot.endsWith('\\')
                ? `${activeRoot}.cursorrules`
                : `${activeRoot}/.cursorrules`;
            const content = await invoke<string>('read_file', { path: cursorRulesPath }).catch(() => '');
            if (content) {
                resolved.push({ id: '__cursorrules__', type: 'file', name: '.cursorrules', path: '.cursorrules', data: content });
            }
        }
    } catch { /* ignore */ }

    // Auto-context injection: if no @codebase mention and query seems code-related,
    // auto-inject the active file and a few relevant files by keyword search
    if (!hasCodebase && query.length > 10) {
        try {
            const storeState = (window as any).useStore?.getState();
            const activeFile = storeState?.activeEditorPath;
            const activeContent = storeState?.tabs?.find((t: any) => t.path === activeFile)?.content;
            if (activeFile && activeContent && !resolved.find(r => r.path === activeFile)) {
                resolved.unshift({ id: activeFile, type: 'file', name: activeFile.split(/[/\\]/).pop(), path: activeFile, data: activeContent.slice(0, 4000) });
            }
        } catch { /* ignore */ }
    }

    return resolved;
}

function formatToolSummary(name: string, args: any, result: any): string {
    try {
        const data = typeof result === 'string' ? JSON.parse(result) : result;
        const toolName = name.toLowerCase();

        if (toolName.includes('list_files') || toolName.includes('list_directory') || toolName.includes('ls')) {
            const count = Array.isArray(data) ? data.length : (data.filenames ? data.filenames.length : 0);
            return `Listed ${count} items in ${args.path || args.directory_path || 'root'}`;
        }
        if (toolName.includes('view_file') || toolName.includes('file_read') || toolName.includes('cat')) {
            return `Read ${args.file_path || args.path} (${data.numLines || 'all'} lines)`;
        }
        if (toolName.includes('run_command') || toolName.includes('bash') || toolName.includes('sh')) {
            const cmd = args.command || '';
            const shortCmd = cmd.length > 30 ? cmd.substring(0, 30) + '...' : cmd;
            return `Executed: ${shortCmd}`;
        }
        if (toolName.includes('grep') || toolName.includes('search')) {
            const count = Array.isArray(data) ? data.length : 0;
            return `Found ${count} matches for "${args.pattern || args.query}"`;
        }
        if (toolName.includes('write_to_file') || toolName.includes('file_write')) {
            return `Wrote ${args.file_path || args.path}`;
        }
        if (toolName.includes('file_edit') || toolName.includes('modify_file')) {
            return `Edited ${args.file_path || args.path}`;
        }
        if (toolName.includes('git_status')) {
            return `Checked git status`;
        }
    } catch (e) {
        // Fallback to generic summary if parsing fails
    }
    return `Executed ${name}`;
}

listen('ai-tool-call', (event: { payload: { name: string, args: string | any } | any }) => {
    if (event.payload && event.payload.name) {
        console.log("AI_TOOL_CALL RECEIVED:", event.payload.name);
        const { addAgentStep, updateAgentStepStatus } = useStore.getState();
        let args = {};
        try {
            args = typeof event.payload.args === 'string' ? JSON.parse(event.payload.args) : event.payload.args;
        } catch (e) {
            console.warn("Failed to parse tool args in ai-tool-call", e);
            args = { raw: event.payload.args };
        }
        addAgentStep(event.payload.name, 'other', args, event.payload.call_id);
        updateAgentStepStatus(event.payload.name, 'running', 'Executing...', undefined, event.payload.call_id);

        // ═══════════════════════════════════════════════════════════
        // AIRI SELF-LEARNING - Learn from every tool action
        // AIRI observes what she does and learns from outcomes
        // ═══════════════════════════════════════════════════════════
        if (airiInitialized && airiSelfLearning) {
            airiSelfLearning.learnFromEvent(
                'agent_tool_use',
                { tool: event.payload.name, args, callId: event.payload.call_id },
                'neutral' // Will be updated to positive/negative when result arrives
            ).catch(console.error);
        }
    }
});

listen('ai-tool-result', (event: { payload: { name: string, result: string, blocked?: boolean } | any }) => {
    if (event.payload && event.payload.name) {
        const { updateAgentStepStatus, agentMessages } = useStore.getState();

        // Find arguments from the last step with this name
        const lastMsg = agentMessages[agentMessages.length - 1];
        const step = lastMsg?.steps?.find((s: any) => s.name === event.payload.name && s.status === 'running');
        const args = step?.args || {};

        const summary = formatToolSummary(event.payload.name, args, event.payload.result);
        updateAgentStepStatus(event.payload.name, event.payload.blocked ? 'running' : 'success', event.payload.result, summary, event.payload.call_id);

        // ═══════════════════════════════════════════════════════════
        // AIRI SELF-LEARNING - Learn from tool outcomes
        // AIRI learns whether her actions succeeded or failed
        // ═══════════════════════════════════════════════════════════
        if (airiInitialized && airiSelfLearning) {
            const outcome = event.payload.blocked ? 'blocked' : 'success';
            airiSelfLearning.learnFromEvent(
                'agent_tool_result',
                { tool: event.payload.name, result: event.payload.result, outcome },
                outcome === 'success' ? 'positive' : 'negative'
            ).catch(console.error);
        }
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

listen('add-agent-step', (event: { payload: { name: string, status: string, type?: string } | any }) => {
    if (event.payload) {
        useStore.getState().addAgentStep(event.payload.name, event.payload.type || 'other', {});
        if (event.payload.status) {
            useStore.getState().updateAgentStepStatus(event.payload.name, event.payload.status === 'success' ? 'success' : 'running', '', '');
        }
    }
});
