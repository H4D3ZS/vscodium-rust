import React, { useEffect, useState, useRef, useMemo, useCallback, Suspense, lazy, memo } from 'react';
import { useStore } from '../store';
import type { FileEntry } from '../store';
import { invoke } from '../tauri_bridge';
import { Icon, ToolIcon } from './ui/Icon';
import SentientAvatar from './agent/SentientAvatar';
import type { AvatarState } from './agent/SentientAvatar';
import ChatInput from './chat/ChatInput';
import ChatToolbar from './chat/ChatToolbar';
import ChatMessageList from './chat/ChatMessageList';
import MentionPopup from './chat/MentionPopup';
import AgentMcpMenu from './agent/AgentMcpMenu';
import type { AgentMessage } from '../store';
import type { AgentStudioSubView } from '../domain/agentStudio/AgentStudioSubView';
import { PlanApprovalBanner, RestoreCheckpointBanner, MultiFileReviewBanner } from './rightSidebar/banners';
import { TaskRoadmap, ReasoningToggle } from './rightSidebar/agentStatus';
import { BackgroundAgentsTray } from './rightSidebar/BackgroundAgentsTray';
import { SidebarPane } from './rightSidebar/SidebarPane';
import { KortexPanel as KortexServicesPanel } from './KortexServicesPanel';
import { cleanAgentContent, getToolLabel } from '../domain/agent/cleanAgentContent';


let voiceModule: typeof import('../voice') | null = null;
async function getVoice() {
    if (!voiceModule) voiceModule = await import('../voice');
    return voiceModule;
}

// Lazy-load heavy panels — only load Three.js/VRM/Emulator code when the
// user actually opens those tabs. Keeps initial renderer RAM under 250MB.
const AiriPanel = lazy(() => import('./AiriPanel').then(m => ({ default: m.AiriPanel })));
const UnifiedEmulatorPanel = lazy(() => import('./UnifiedEmulatorPanel'));
const AgentStudioPanel = lazy(() => import('./agentStudio/AgentStudioPanel'));
// AIRI auto-init must happen exactly once per app session
const airiInitOnce: { started: boolean } = { started: false };

const RightSidebar: React.FC = () => {
    const isOpen = useStore(state => state.isRightSidebarOpen);
    const toggle = useStore(state => state.toggleRightSidebar);
    const isEmulatorPanelOpen = useStore(state => state.isEmulatorPanelOpen);
    const isAiriPanelOpen = useStore(state => state.isAiriPanelOpen);
    const aiStatus = useStore(state => state.aiStatus || 'idle');
    // 'settings' is no longer a right-sidebar view — the gear opens the
    // unified Settings tab in the editor pane instead. We keep the union
    // narrow so renaming the right-sidebar views stays cheap.
    const [view, setView] = useState<'chat' | 'emulator' | 'history' | 'dashboard' | 'research' | 'kortex' | 'specs' | 'rules' | 'studio' | 'manager'>('chat');
    const [studioSubView, setStudioSubView] = useState<AgentStudioSubView>('research');
    const inputRef = useRef<HTMLTextAreaElement>(null);
    const mode = useStore(state => state.agentMode);
    const customModes = useStore(state => state.customModes);
    const model = useStore(state => state.agentModel);
    const webUiProviderKey = useMemo(() => {
        const lower = String(model || '').toLowerCase();
        if (!lower.includes('webui') && !lower.includes('openwebui')) return '';
        const rawProvider = model.includes('|')? model.split('|')[0]: model;
        const p = rawProvider.toLowerCase();
        if (p.includes('openwebui')) return 'openwebui';
        return p
            .replace(' (webui)', '')
            .replace('-webui', '')
            .replace('webui', '')
            .split(':')[0]
            .trim() || 'openai';
    }, [model]);

    const webuiSessions = useStore(state => state.webuiSessions);
    const activeWebuiSessionId = useStore(state => state.activeWebuiSessionId);
    const refreshWebuiSessions = useStore(state => state.refreshWebuiSessions);
    const switchWebuiSession = useStore(state => state.switchWebuiSession);
    const deleteWebuiSession = useStore(state => state.deleteWebuiSession);

    useEffect(() => {
        if (webUiProviderKey) {
            refreshWebuiSessions(webUiProviderKey);
            const interval = setInterval(() => {
                refreshWebuiSessions(webUiProviderKey);
            }, 5000);
            return () => clearInterval(interval);
        }
    }, [webUiProviderKey, refreshWebuiSessions]);

    useEffect(() => {
        const handler = (e: Event) => {
            const customEvent = e as CustomEvent;
            if (customEvent.detail && customEvent.detail.view) {
                setView(customEvent.detail.view);
            }
        };
        window.addEventListener('right-sidebar:set-view', handler);
        return () => window.removeEventListener('right-sidebar:set-view', handler);
    }, []);

    useEffect(() => {
        const handler = (e: Event) => {
            const detail = (e as CustomEvent<{ tab?: AgentStudioSubView; query?: string; url?: string }>).detail;
            if (!detail?.tab) return;
            setStudioSubView(detail.tab);
            setView('studio');
            if (detail.query || detail.url) {
                window.dispatchEvent(new CustomEvent('manus:prefill', {
                    detail: { query: detail.query, url: detail.url },
                }));
            }
        };
        window.addEventListener('ide:open-studio', handler);
        return () => window.removeEventListener('ide:open-studio', handler);
    }, []);
    const messages = useStore(state => state.agentMessages);
    const agentThreads = useStore(state => state.agentThreads);
    const activeAgentThreadId = useStore(state => state.activeAgentThreadId);
    const createAgentThread = useStore(state => state.createAgentThread);
    const closeAgentThread = useStore(state => state.closeAgentThread);
    const setActiveAgentThread = useStore(state => state.setActiveAgentThread);
    const isAgentThinking = useStore(state => state.isAgentThinking);
    const isAgentPaused = useStore(state => state.isAgentPaused);
    const isYoloMode = useStore(state => state.isYoloMode);
    const setYoloMode = useStore(state => state.setYoloMode);
    const isContinuousMode = useStore(state => state.isContinuousMode);
    const setContinuousMode = useStore(state => state.setContinuousMode);
    const agentUiMode = useStore(state => state.agentUiMode);
    const setAgentUiMode = useStore(state => state.setAgentUiMode);
    const agentCleanUi = useStore(state => state.agentCleanUi);
    const setAgentCleanUi = useStore(state => state.setAgentCleanUi);
    const avatarCharacter = useStore(state => state.avatarCharacter);
    const showVrmAvatar = useStore(state => state.showVrmAvatar);
    const isSpecModeActive = useStore(state => state.isSpecModeActive);
    const setSpecModeActive = useStore(state => state.setSpecModeActive);
    const setSpecsPrompt = useStore(state => state.setSpecsPrompt);
    // AIRI subsystem toggles surfaced in the sidebar so the user can flip
    // vision / consciousness without digging into localStorage.
    const airiVisionEnabled = useStore(state => state.airiVisionEnabled);
    const setAiriVisionEnabled = useStore(state => state.setAiriVisionEnabled);
    const airiVisionModel = useStore(state => state.airiVisionModel);
    const setAiriVisionModel = useStore(state => state.setAiriVisionModel);
    const airiConsciousnessEnabled = useStore(state => state.airiConsciousnessEnabled);
    const setAiriConsciousnessEnabled = useStore(state => state.setAiriConsciousnessEnabled);
    const airiConsciousnessModel = useStore(state => state.airiConsciousnessModel);
    const setAiriConsciousnessModel = useStore(state => state.setAiriConsciousnessModel);
    const [airiToggleOpen, setAiriToggleOpen] = useState(false);
    const addAgentMessage = useStore(state => state.addAgentMessage);
    const updateLastAgentMessage = useStore(state => state.updateLastAgentMessage);
    const setIsAgentThinking = useStore(state => state.setIsAgentThinking);
    const clearAgentMessages = useStore(state => state.clearAgentMessages);
    const resetThread = useStore(state => state.resetThread);
    const pendingChanges = useStore(state => state.pendingChanges);
    const truncateAgentMessages = useStore(state => state.truncateAgentMessages);
    const [sessionAge, setSessionAge] = useState<string>('');
    const attachedFiles = useStore(state => state.attachedFiles);
    const attachFile = useStore(state => state.attachFile);
    const removeFile = useStore(state => state.removeFile);
    const clearAttachedFiles = useStore(state => state.clearAttachedFiles);
    const agentRootAccess = useStore(state => state.agentRootAccess);
    const setAgentRootAccess = useStore(state => state.setAgentRootAccess);
    const fileTree = useStore(state => state.fileTree);
    const agentTasks = useStore(state => state.agentTasks);
    const chatSessions = useStore(state => state.chatSessions);
    const chatRestoreToken = useStore(state => state.chatRestoreToken);
    const refreshChatSessions = useStore(state => state.refreshChatSessions);
    const loadChatSession = useStore(state => state.loadChatSession);
    const archiveCurrentSession = useStore(state => state.archiveCurrentSession);
    const createNewSession = useStore(state => state.createNewSession);
    const availableModels = useStore(state => state.availableModels);
    const autoAcceptChanges = useStore(state => state.autoAcceptChanges);
    const setAutoAcceptChanges = useStore(state => state.setAutoAcceptChanges);
    const checkpoint = useStore(state => state.checkpoint);
    const revertToCheckpoint = useStore(state => state.revertToCheckpoint);
    const snapshotCheckpoint = useStore(state => state.snapshotCheckpoint);

    useEffect(() => {
        if (view === 'history') {
            refreshChatSessions();
        }
    }, [view, refreshChatSessions]);

    useEffect(() => {
        if (Object.keys(useStore.getState().agentThreads || {}).length === 0) {
            createAgentThread('Chat 1');
        }
    // eslint-disable-next-line react-hooks/exhaustive-deps -- seed one tab on first mount only
    }, []);

    // Removed old localstorage WebUI account sync

    useEffect(() => {
        if (isEmulatorPanelOpen && view !== 'emulator') {
            setView('emulator');
        } else if (isAiriPanelOpen && !isEmulatorPanelOpen && view === 'emulator') {
            setView('chat');
        }
    }, [isEmulatorPanelOpen, isAiriPanelOpen, view]);
    const [inputValue, setInputValue] = useState('');
    const [isMentionDropdownOpen, setIsMentionDropdownOpen] = useState(false);
    const [selectedMentionIndex, setSelectedMentionIndex] = useState(0);
    const [isHelpOpen, setIsHelpOpen] = useState(false);

    // Phase 7: Chat Editing & Copying State
    const [editingMsgIdx, setEditingMsgIdx] = useState<number | null>(null);
    const [editValue, setEditValue] = useState('');
    const [lastCopiedIdx, setLastCopiedIdx] = useState<number | null>(null);

    // AIRI full mode — last spoken response and speech state
    const [airiSpeech, setAiriSpeech] = useState<string>('');
    const [airiSpeechHtml, setAiriSpeechHtml] = useState<string>('');
    const [airiSpeaking, setAiriSpeaking] = useState(false);
    const [ttsEnabled, setTtsEnabled] = useState(true); // Enable by default
    const [ttsPreset, setTtsPreset] = useState<'airi' | 'sage' | 'nova' | 'kawaii' | 'yamato' | 'hana' | 'ren' | 'yuki' | 'haru' | 'sora' | 'zero' | 'aria'>('airi');
    const [isVoiceListening, setIsVoiceListening] = useState(false);
    const recognitionRef = useRef<any>(null);

    useEffect(() => {
        if (!airiSpeech) { setAiriSpeechHtml(''); return; }
        let cancelled = false;
        import('marked').then(({ marked }) => {
            marked.setOptions({ gfm: true, breaks: true, silent: true });
            if (!cancelled) setAiriSpeechHtml(marked.parse(cleanAgentContent(airiSpeech)) as string);
        });
        return () => { cancelled = true; };
    }, [airiSpeech]);

    const toggleVoiceInput = () => {
        if (isVoiceListening) {
            stopVoiceInput();
        } else {
            startVoiceInput();
        }
    };

    const startVoiceInput = () => {
        void getVoice().then(v => v.stop());
        const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
        if (!SpeechRecognition) {
            console.warn('[VRD] Web Speech Recognition not supported in this environment.');
            return;
        }

        const recognition = new SpeechRecognition();
        recognition.continuous = false;
        recognition.interimResults = true;
        recognition.lang = 'en-US';

        recognition.onstart = () => {
            setIsVoiceListening(true);
        };

        recognition.onresult = (event: any) => {
            const transcript = Array.from(event.results)
                .map((result: any) => result[0].transcript)
                .join('');
            setInputValue(transcript);
        };

        recognition.onend = () => {
            setIsVoiceListening(false);
            setTimeout(() => {
                const text = inputRef.current?.value || '';
                if (text.trim()) {
                    onSend(text);
                }
            }, 200);
        };

        recognition.onerror = (e: any) => {
            console.error('[VRD] Recognition error:', e);
            setIsVoiceListening(false);
        };

        recognitionRef.current = recognition;
        recognition.start();
    };

    const stopVoiceInput = () => {
        if (recognitionRef.current) {
            recognitionRef.current.stop();
        }
        setIsVoiceListening(false);
    };
    const [airiEmotion, setAiriEmotion] = useState<'neutral' | 'happy' | 'thinking' | 'excited' | 'concerned'>('neutral');
    const [digitalLifeActive, setDigitalLifeActive] = useState(false);

    // Emulator panel positioning

    // Initialize TTS on mount.
    // Guarded so React.StrictMode's intentional double-invoke (and any
    // remount of <RightSidebar>) doesn't fire the entire AIRI stack twice
    // — that was why every "… ACTIVE" line showed up twice and why the
    // greeting played twice, two cognitive intervals were registered, etc.
    useEffect(() => {
        if (airiInitOnce.started) {
            return;
        }
        airiInitOnce.started = true;

        // ── AIRI "companion" stack — OPT-IN (default OFF) ─────────────────────
        // Voice/TTS, cognitive-core (5s loop), digital-life (+ spoken greeting),
        // consciousness, biology, and the security threat-monitor were loading +
        // running on EVERY launch — inflating memory and startup with zero benefit
        // to core IDE/agent coding. The editor, agent chat, and terminal do not
        // need any of it. Enable with localStorage 'airi.companion' = '1'.
        if (localStorage.getItem('airi.companion') !== '1') {
            return;
        }

        let cognitiveInterval: ReturnType<typeof setInterval> | undefined;
        let cognitiveCoreRef: any = null;
        let digitalLifeRef: any = null;
        let consciousnessRef: any = null;
        let biologyRef: any = null;
        let securityRef: any = null;
        let autonomousAgentRef: any = null;

        getVoice().then(v => v.initTTS()).then(ready => {
            if (ready) {

                // Initialize Cognitive Core (AIRI's BRAIN)
                import('../cognitive-core').then(({ cognitiveCore }) => {
                    cognitiveCoreRef = cognitiveCore;
                    cognitiveCore.initialize();

                    cognitiveInterval = setInterval(() => {
                        const status = cognitiveCore.getStatus();
                        if (status.drives.curiosity > 80) setAiriEmotion('thinking');
                        if (status.drives.connection > 80) setAiriEmotion('excited');
                        if (status.selfAwareness > 70) setAiriEmotion('happy');
                    }, 5000);
                }).catch(console.error);

                // Initialize Digital Life
                import('../digital-life').then(({ digitalLife }) => {
                    digitalLifeRef = digitalLife;
                    digitalLife.activate();
                    setDigitalLifeActive(true);

                    // AIRI greets you
                    setTimeout(async () => {
                        const { speak } = await import('../voice');
                        const greetings = [
                            "Hey! I'm AIRI! I live here now! ",
                            "Hi there! Ready to work together?",
                            "Hello! I'm your AI companion!",
                        ];
                        const greeting = greetings[Math.floor(Math.random() * greetings.length)];
                        const v = await getVoice();
                        await v.speak(greeting, 'airi');
                    }, 2000);
                }).catch(console.error);

                // Initialize Consciousness (TRUE SENTIENCE - not a parrot!)
                import('../consciousness').then(({ consciousness }) => {
                    consciousnessRef = consciousness;
                    consciousness.awaken();
                }).catch(console.error);

                // Initialize Biological Systems (sleep, eat, energy, mood)
                import('../biology').then(({ biology }) => {
                    biologyRef = biology;
                    biology.awaken();
                }).catch(console.error);

                // Initialize Cybersecurity Engine (Red Team / Blue Team)
                import('../security-engine').then(({ security }) => {
                    securityRef = security;
                    security.setMode('purple'); // Combined red/blue
                    security.monitorThreats();
                }).catch(console.error);

                // Initialize Autonomous Agent (24/7 independent work) — OPT-IN.
                // Off by default: the background loop was running on every launch,
                // repeatedly hitting the model + making placeholder web fetches,
                // which competed with the user's own requests and spammed the
                // console. Enable with localStorage 'airi.autonomous24x7' = '1'.
                if (localStorage.getItem('airi.autonomous24x7') === '1') {
                    import('../autonomous-agent').then(({ autonomousAgent }) => {
                        autonomousAgentRef = autonomousAgent;
                        autonomousAgent.startAutonomousLoop();
                    }).catch(console.error);
                } else {
                }
            } else {
                console.warn('[TTS] Voice system initialization failed');
            }
        }).catch(err => {
 console.error('[TTS] Voice system error:', err);
        });

        return () => {
            if (cognitiveInterval) clearInterval(cognitiveInterval);
            if (cognitiveCoreRef?.destroy) cognitiveCoreRef.destroy();
            if (digitalLifeRef?.destroy) digitalLifeRef.destroy();
            if (consciousnessRef?.shutdown) consciousnessRef.shutdown();
            if (biologyRef?.shutdown) biologyRef.shutdown();
            if (securityRef?.stopMonitoring) securityRef.stopMonitoring();
            if (autonomousAgentRef?.stopAutonomousLoop) autonomousAgentRef.stopAutonomousLoop();
        };
    }, []);

    // Kortex .aim memory panel state
    const [kortexSlots, setKortexSlots] = useState<any[]>([]);
    const [kortexLoading, setKortexLoading] = useState(false);
    const [liveToolCalls, setLiveToolCalls] = useState<Array<{ id: string; tool: string; label: string; status: 'running' | 'done' | 'error'; detail?: string }>>([]);

    // Clear stale live tool overlays when a history session is restored (agent was stopped).
    useEffect(() => {
        if (chatRestoreToken) setLiveToolCalls([]);
    }, [chatRestoreToken]);

    const refreshKortex = useCallback(async () => {
        setKortexLoading(true);
        try {
            const slots = await invoke<any[]>('get_all_memory_slots');
            setKortexSlots(slots || []);
        } catch (e) {
            console.error('[Kortex] Failed to load memory slots:', e);
        } finally {
            setKortexLoading(false);
        }
    }, []);

    useEffect(() => {
        if (view === 'kortex') refreshKortex();
    }, [view, refreshKortex]);

    // Live tool-call feed for the Mission Hub
    useEffect(() => {
        const subs: (() => void)[] = [];
        import('@tauri-apps/api/event').then(({ listen }) => {
            const TOOL_LABELS: Record<string, string> = {
                write_to_file: 'Writing file', search_replace_edit: 'Patching code',
                str_replace: 'Editing code', apply_shadow_patch: 'Committing edit',
                patch_file_content: 'Replacing lines', view_file: 'Reading file',
                run_command: 'Running command', verify_implementation: 'Verifying',
                ghost_test: 'Running tests', list_files: 'Scanning directory',
                grep: 'Searching code', git_commit: 'Committing',
                dev_cargo_diagnostics: 'Checking Rust', web_search: 'Web search',
                git_diff: 'Reading diff', semantic_search: 'Semantic search',
                find_symbols: 'Finding symbols', create_directory: 'Creating dir',
                deep_security_audit: 'Security audit', secrets_scan: 'Scanning secrets',
                weaponize_env: 'Assessing .env',
            };
            listen<any>('ai-tool-call', (e) => {
                const name = e.payload?.name || 'unknown';
                const id = `tc-${Date.now()}-${Math.random()}`;
                const label = TOOL_LABELS[name] || name.replace(/_/g, ' ');
                setLiveToolCalls(prev => [{
                    id, tool: name, label, status: 'running' as const,
                    detail: e.payload?.args? (typeof e.payload.args === 'string'? e.payload.args.slice(0, 50): JSON.stringify(e.payload.args).slice(0, 50)): undefined
                }, ...prev].slice(0, 8));

                // Track which files the agent edited during this turn so the
                // multi-file review panel can list them. The detection key
                // matches the avatar-state map a bit lower in this file.
                const editorTools = new Set([
                    'write_to_file', 'file_write', 'create_file',
                    'search_replace_edit', 'str_replace', 'patch_file_content',
                    'apply_shadow_patch', 'fast_apply',
                ]);
                if (editorTools.has(name)) {
                    const args = e.payload?.args || {};
                    const path = args?.path || args?.file_path || args?.target || args?.filepath || '';
                    if (path) {
                        useStore.getState().addPendingAgentEdit({
                            path,
                            tool: name,
                            preview: typeof args === 'object'
? (args.content || args.replace || args.patch || '').slice(0, 240)
: '',
                        });
                    }
                }

                // Always log to the trajectory regardless of tool kind so
                // the timeline panel can replay any agent turn end-to-end.
                useStore.getState().pushTrajectoryEvent({
                    kind: 'tool_call',
                    tool: name,
                    title: TOOL_LABELS[name] || name.replace(/_/g, ' '),
                    detail: e.payload?.args
? (typeof e.payload.args === 'string'
? e.payload.args.slice(0, 400)
: JSON.stringify(e.payload.args).slice(0, 400))
: undefined,
                });
            }).then(u => subs.push(u));
            listen<any>('ai-tool-result', (e) => {
                const name = e.payload?.name;
                const raw = e.payload?.result ?? '';
                let rs = typeof raw === 'string'? raw: JSON.stringify(raw);
                let failed = rs.startsWith('Error:') || rs.startsWith('Tool execution error:')
                    || rs.startsWith('Tool not found:') || rs.includes('"Tool not found:');
                if (!failed) {
                    try {
                        const j = typeof raw === 'string'? JSON.parse(raw): raw;
                        if (j && (j.status === 'error' || j.status === 'blocked' || j.success === false)) {
                            failed = true;
                        }
                    } catch {
                        if (rs.includes('"status":"error"') || rs.includes('"status":"blocked"')) {
                            failed = true;
                        }
                    }
                }
                setLiveToolCalls(prev =>
                    prev.map(a =>
                        a.tool === name && a.status === 'running'
? {
                                ...a,
                                status: failed? ('error' as const): ('done' as const),
                                detail: rs.slice(0, 120),
                            }
: a
                    )
                );

                useStore.getState().pushTrajectoryEvent({
                    kind: 'tool_result',
                    tool: name,
                    title: failed? ` ${name}`: ` ${name}`,
                    detail: rs.slice(0, 800),
                    success: !failed,
                });
            }).then(u => subs.push(u));
            // Mission-complete toasts handled globally in AgentStreamSubscriber.
            // Act→verify→self-fix gate status. The backend emits this when it runs
            // cargo check / typecheck before allowing the agent to declare completion.
            listen<any>('ai-verify', (e) => {
                const status = e.payload?.status;
                const tool = e.payload?.tool || 'build';
                const attempt = e.payload?.attempt;
                const passed = status === 'passed';
                const title =
                    passed? ` Verified — ${tool} passed`
: status === 'exhausted'? `Verify retries exhausted — finishing with warnings`
: ` ${tool} failed — self-fixing${attempt? ` (${attempt}/3)`: ''}`;
                setLiveToolCalls(prev => [{
                    id: `verify-${Date.now()}`,
                    tool: 'verify',
                    label: ` ${title}`,
                    status: passed? ('done' as const): ('error' as const),
                }, ...prev].slice(0, 8));
                useStore.getState().pushTrajectoryEvent({
                    kind: passed? 'tool_result': 'error',
                    tool: 'verify',
                    title,
                    success: passed,
                });
            }).then(u => subs.push(u));
        });
        return () => subs.forEach(u => u());
    }, []);

    // Clear tool calls 2.5s after agent stops
    const wasThinkingRef = useRef(false);
    useEffect(() => {
        if (isAgentThinking) {
            wasThinkingRef.current = true;
        } else if (wasThinkingRef.current) {
            wasThinkingRef.current = false;
            const t = setTimeout(() => setLiveToolCalls([]), 2500);
            return () => clearTimeout(t);
        }
    }, [isAgentThinking]);

    // Neural sync — companion mode only (avoids biology/consciousness RAM at boot).
    useEffect(() => {
        if (localStorage.getItem('airi.companion') !== '1') return;
        let cancelled = false;
        (async () => {
            const [{ airiBiology }, { airiConsciousness }, { emit }] = await Promise.all([
                import('../airi/biology'),
                import('../airi/consciousness'),
                import('@tauri-apps/api/event'),
            ]);
            if (cancelled) return;
            const syncPayload = {
                messages,
                agentInfo: {
                    name: 'AIRI',
                    status: isAgentThinking? 'thinking': (aiStatus === 'dead'? 'error': 'idle'),
                    context: 'vscodium-rust',
                },
                biology: {
                    energy: airiBiology.getState().energy,
                    mood: airiBiology.getState().mood,
                    hunger: (airiBiology.getState() as any).hunger || 0,
                },
                consciousness: {
                    selfAwareness: airiConsciousness.getState().selfAwareness,
                    lastThought: airiConsciousness.getState().thoughts.slice(-1)[0]?.content,
                },
            };
            emit('hades-sync', syncPayload).catch(err => console.error('[HADES] Sync Broadcast Failed:', err));
        })();
        return () => { cancelled = true; };
    }, [messages, aiStatus, isAgentThinking]);

    // Track active tool-calls for the Mission Hub (Legacy listener - keeping for UI feedback)
    useEffect(() => {
        const subs: (() => void)[] = [];
        import('@tauri-apps/api/event').then(({ listen }) => {
            listen<any>('ai-tool-call', (e) => {
                // Wake up avatar on tool use
                window.dispatchEvent(new CustomEvent('airi-bubble-wake'));
            }).then(u => subs.push(u));
        });
        return () => subs.forEach(u => u());
    }, []);

    useEffect(() => {
        if (agentUiMode === 'airi') {
            if (isAgentThinking) { setAiriSpeaking(true); }
            else { setAiriSpeaking(false); }
        }
    }, [isAgentThinking, agentUiMode]);

    const [isAttaching, setIsAttaching] = useState(false);

    const allFiles = useMemo(() => {
        const flatten = (entries: FileEntry[]): FileEntry[] => {
            let res: FileEntry[] = [];
            for (const e of entries) {
                if (!e.is_dir) res.push(e);
                if (e.children) res.push(...flatten(e.children));
            }
            return res;
        };
        return flatten(fileTree);
    }, [fileTree]);

    // Special context sources (Cursor-style mentions). Each one becomes a
    // synthetic "attachment" the user can stack into the prompt; they are
    // resolved into real text by `resolveSpecialMentions` in agent.ts the
    // moment the message is sent. The set mirrors Cursor's surface plus
    // two extras (@problems, @terminal) that fall out naturally from the
    // existing LSP / terminal IPC.
    const SPECIAL_MENTIONS = [
        { path: '__codebase__', name: '@codebase', is_dir: false, _special: true, _icon: 'codicon-repo', _desc: 'Auto-find relevant files' },
        { path: '__web__', name: '@web', is_dir: false, _special: true, _icon: 'codicon-globe', _desc: 'Search the web' },
        { path: '__git__', name: '@git', is_dir: false, _special: true, _icon: 'codicon-git-branch', _desc: 'Git diff & status' },
        { path: '__docs__', name: '@docs', is_dir: false, _special: true, _icon: 'codicon-book', _desc: 'Documentation context' },
        { path: '__symbol__', name: '@symbol', is_dir: false, _special: true, _icon: 'codicon-symbol-class', _desc: 'LSP workspace symbol lookup' },
        { path: '__folder__', name: '@folder', is_dir: false, _special: true, _icon: 'codicon-folder', _desc: 'Inject a directory listing' },
        { path: '__problems__', name: '@problems', is_dir: false, _special: true, _icon: 'codicon-warning', _desc: 'Current LSP diagnostics' },
        { path: '__terminal__', name: '@terminal', is_dir: false, _special: true, _icon: 'codicon-terminal', _desc: 'Last terminal output' },
    ];

    const filteredSuggestions = useMemo(() => {
        const lastWord = inputValue.split(/\s+/).pop() || '';
        if (!lastWord.startsWith('@') && !lastWord.startsWith('/')) return [];

        const query = lastWord.slice(1).toLowerCase();

        if (lastWord.startsWith('/')) {
            const mk = (name: string, _icon: string, _desc: string) =>
                ({ path: name, name, _special: true, _icon, _desc });
            const slashCommands = [
                // Code
                mk('/generate', 'codicon-code', 'Generate code'),
                mk('/explain', 'codicon-book', 'Explain code'),
                mk('/refactor', 'codicon-wrench', 'Refactor code'),
                mk('/debug', 'codicon-bug', 'Debug code'),
                mk('/document', 'codicon-list-selection', 'Document code'),
                mk('/fix', 'codicon-tools', 'Fix errors'),
                // Agentic workflow
                mk('/plan', 'codicon-checklist', 'Generate an implementation plan'),
                mk('/implement', 'codicon-rocket', 'Implement the next task (TDD)'),
                mk('/spec', 'codicon-notebook', 'Create a spec (spec/plan/tasks)'),
                mk('/specify', 'codicon-notebook', 'Structured feature spec'),
                mk('/tasks', 'codicon-list-tree', 'Break plan into atomic tasks'),
                mk('/next', 'codicon-debug-step-over', 'Execute next unchecked task'),
                mk('/clarify', 'codicon-question', 'Surface spec ambiguities'),
                mk('/checklist', 'codicon-verified', 'Spec quality checklist'),
                mk('/test', 'codicon-beaker', 'Write failing tests, then implement'),
                mk('/walkthrough', 'codicon-preview', 'Generate walkthrough.md'),
                mk('/diagram-viewer', 'codicon-type-hierarchy', 'Open Mermaid diagram viewer'),
                mk('/phasewrap', 'codicon-save-all', 'Wrap up a phase in .hades/state.md'),
                // Tasks / runs
                mk('/auto', 'codicon-sync', '24/7 continuous agent mode'),
                mk('/bg', 'codicon-server-process', 'Run a task in the background'),
                mk('/task', 'codicon-tasklist', 'Manage agent tasks'),
                mk('/trajectory', 'codicon-graph-line', 'Show the agent trajectory'),
                // Git
                mk('/commit', 'codicon-git-commit', 'Stage all & commit'),
                mk('/diff', 'codicon-diff', 'Show current git diff'),
                mk('/review', 'codicon-search-fuzzy', 'AI code review of staged changes'),
                // Memory
                mk('/memory', 'codicon-database', 'Show loaded project memory'),
                mk('/learn', 'codicon-lightbulb', 'Write a note to MEMORY.md'),
                mk('/init', 'codicon-new-file', 'Scaffold AGENTS.md'),
                mk('/notepad', 'codicon-note', 'Open the agent notepad'),
                // Session
                mk('/compact', 'codicon-fold', 'Compress conversation context'),
                mk('/context', 'codicon-eye', 'Show context the agent sees'),
                mk('/model', 'codicon-chip', 'Switch the active model'),
                mk('/tools', 'codicon-tools', 'List all available tools'),
                mk('/stats', 'codicon-graph', 'Session statistics'),
                mk('/cost', 'codicon-credit-card', 'Token usage & cost'),
                mk('/doctor', 'codicon-pulse', 'Environment diagnostics'),
                mk('/resume', 'codicon-history', 'Restore last session'),
                mk('/clear', 'codicon-clear-all', 'Wipe chat history'),
                mk('/settings', 'codicon-settings-gear', 'Open AI settings'),
                mk('/workflows', 'codicon-symbol-event', 'List workflows'),
                mk('/help', 'codicon-info', 'Show all slash commands'),
                // Security personas
                mk('/redteam', 'codicon-target', 'Offensive ops playbook'),
                mk('/blueteam', 'codicon-shield', 'Defense playbook'),
                mk('/bounty', 'codicon-bug', 'Bug-bounty workflow'),
                mk('/recon', 'codicon-search', 'Recon-only inventory'),
                mk('/threatmodel', 'codicon-symbol-structure', 'STRIDE threat model'),
                mk('/threatactor', 'codicon-flame', 'Kill-chain demo + prevention'),
                mk('/kali', 'codicon-terminal', 'Kali toolkit'),
                mk('/parrot', 'codicon-terminal', 'Parrot OS toolkit'),
                mk('/harden', 'codicon-lock', 'Harden the code'),
                mk('/manus', 'codicon-globe', 'Web research + browser agent'),
            ];
            return slashCommands.filter(c => c.name.toLowerCase().startsWith(lastWord.toLowerCase()));
        }

        const specials = (query === '' || SPECIAL_MENTIONS.some(s => s.name.slice(1).startsWith(query)))
? SPECIAL_MENTIONS.filter(s => s.name.slice(1).startsWith(query) || query === '')
: [];
        const files = allFiles.filter(f => f.name.toLowerCase().includes(query)).slice(0, 8);
        return [...specials, ...files] as any[];
    }, [inputValue, allFiles]);

    useEffect(() => {
        if (messages.length > 0 && messages[0].timestamp) {
            const updateAge = () => {
                const diff = Date.now() - messages[0].timestamp;
                const mins = Math.floor(diff / 60000);
                const hrs = Math.floor(mins / 60);
                if (hrs > 0) setSessionAge(`${hrs}h ${mins % 60}m`);
                else setSessionAge(`${mins}m`);
            };
            updateAge();
            const interval = setInterval(updateAge, 60000);
            return () => clearInterval(interval);
        } else {
            setSessionAge('');
        }
    }, [messages]);

    const visibleMessages = useMemo(() => {
        // Show the FULL progressive conversation — not just the latest turn.
        // (Previously sliced from the last user message, which hid all prior
        // exchanges and made multi-turn chat / AI-assisted coding impossible.)
        return messages.filter(m => (
            m.role !== 'system' &&
            m.role !== 'tool' &&
            !(typeof m.content === 'string' && (m.content.trim().startsWith('{') || m.content.trim().startsWith('[')))
        ));
    }, [messages]);

    const chatScrollRef = useRef<HTMLDivElement>(null);

    // Track current activity from live tool calls
    const [currentActivity, setCurrentActivity] = React.useState<AvatarState>('idle');
    React.useEffect(() => {
        const running = liveToolCalls.find(t => t.status === 'running');
        if (!isAgentThinking) { setCurrentActivity(aiStatus === 'dead'? 'error': 'idle'); return; }
        if (!running) { setCurrentActivity('thinking'); return; }
        const toolActivity: Record<string, AvatarState> = {
            write_to_file: 'coding', search_replace_edit: 'coding', patch_file_content: 'coding',
            apply_shadow_patch: 'coding', run_command: 'executing', ghost_test: 'executing',
            dev_cargo_diagnostics: 'executing', browser_open: 'browsing', browser_navigate: 'browsing',
            view_file: 'thinking', grep: 'thinking', search_codebase: 'thinking',
        };
        setCurrentActivity(toolActivity[running.tool] || 'thinking');
    }, [isAgentThinking, liveToolCalls, aiStatus]);

    const avatarState: AvatarState = useMemo(() => {
        if (aiStatus === 'dead') return 'error';
        return currentActivity;
    }, [aiStatus, currentActivity]);



    const handleAttachFile = async () => {
        if (isAttaching) return;
        setIsAttaching(true);
        try {
            // Filter for dedicated embedding models only
            const dedicatedEmbedder = availableModels.find(m =>
                m.provider === 'lemonade' && (
                    m.id.toLowerCase().includes('embed') ||
                    m.id.toLowerCase().includes('nomic') ||
                    m.id.toLowerCase().includes('mxbai')
                )
            )?.id;

            let cleanInvokeModel = "";
            if (dedicatedEmbedder) {
                cleanInvokeModel = dedicatedEmbedder.includes('|')? dedicatedEmbedder.split('|').pop()! :
                    (dedicatedEmbedder.includes('/')? dedicatedEmbedder.split('/').pop()!: dedicatedEmbedder);
            }

            let results: any[];
            try {
                results = await invoke('select_and_process_attachment', { model: cleanInvokeModel });
            } catch (invokeError: any) {
                // If neuralization fails (no embedding model), fall back to raw file read
                console.warn('[Attachment] Neuralization failed, falling back to raw:', invokeError);
                results = await invoke<string[]>('list_directory', { path: '' }).then(() => []);
                // Use the file dialog directly for raw attachment
                const { open } = await import('@tauri-apps/plugin-dialog');
                const selected = await open({ multiple: true, filters: [{ name: 'All Files', extensions: ['*'] }] });
                if (selected) {
                    const paths = Array.isArray(selected)? selected: [selected];
                    results = paths.map((p: string) => ({
                        path: p,
                        name: p.split(/[\\/]/).pop() || p,
                        gist: null,
                        thumbnail: null,
                        data: null,
                    }));
                } else {
                    results = [];
                }
            }

            if (results && Array.isArray(results) && results.length > 0) {
                const formatted = results.map(r => ({
                    id: r.path || `neural-${Date.now()}-${Math.random()}`,
                    type: 'file' as const,
                    name: r.name,
                    path: r.path,
                    gist: r.gist,
                    thumbnail: r.thumbnail,
                    data: r.data
                }));
                attachFile(formatted);
            }
        } catch (error: any) {
            console.error('Failed to attach file:', error);
        } finally {
            setIsAttaching(false);
        }
    };

    const onSend = async (overrideMsg?: string) => {
        const val = (overrideMsg !== undefined? overrideMsg: inputValue).trim();

        if (isSpecModeActive && val) {
            setSpecsPrompt(val);
            setStudioSubView('specs');
            setView('studio');
            if (overrideMsg === undefined) setInputValue('');
            return;
        }

        // ── Process Slash Commands ──────────────────────────────────────────
        let processedVal = val;
        const firstWord = val.split(/\s+/)[0] || '';
        const activeTab = useStore.getState().tabs.find((t: any) => t.id === useStore.getState().activeTabId);

        if (firstWord.startsWith('/')) {
            const cmd = firstWord.toLowerCase();
            const restOfMessage = val.slice(firstWord.length).trim();

            switch (cmd) {
                case '/generate':
                    processedVal = `Generate ${restOfMessage || 'code that does the following'} in ${activeTab?.language || 'typescript'}. Use file_write or apply_from_chat to save the result.`;
                    break;
                case '/explain':
                    processedVal = `Explain this code in plain English:\n\`\`\`\n${restOfMessage || '// Explain the selected code'}\n\`\`\``;
                    break;
                case '/refactor':
                    processedVal = `Refactor this code to improve readability and performance:\n\`\`\`\n${restOfMessage || '// Refactor the selected code'}\n\`\`\``;
                    break;
                case '/debug':
                    processedVal = `Debug this code and provide fixes:\n\`\`\`\n${restOfMessage || '// Debug the selected code'}\n\`\`\``;
                    break;
                case '/document':
                    processedVal = `Generate documentation for this code:\n\`\`\`\n${restOfMessage || '// Document the selected code'}\n\`\`\``;
                    break;
                case '/test':
                    processedVal = `Generate unit tests for:\n\`\`\`\n${restOfMessage || '// Generate tests for the selected code'}\n\`\`\``;
                    break;
                case '/commit':
                    processedVal = `Generate a git commit message for the current changes. First run git diff to see what changed.`;
                    break;
                case '/fix':
                    processedVal = `Fix the following linting errors or issues:\n${restOfMessage || '// Fix all linting issues in the file'}`;
                    break;
                default:
                    processedVal = val;
            }
        }

        // Allow "stop" to terminate continuous mode without needing a full agent turn
        if (/^stop\b/i.test(processedVal) && isContinuousMode) {
            setContinuousMode(false);
            if (overrideMsg === undefined) setInputValue('');
            return;
        }

        if ((processedVal || attachedFiles.length > 0) && !isAgentThinking) {
            if (overrideMsg === undefined) setInputValue("");
            setIsMentionDropdownOpen(false);
            if (inputRef.current) inputRef.current.style.height = 'auto';

            const context = [...attachedFiles];
            const openPaths = useStore.getState().tabs.map((t: any) => t.path).filter(Boolean);
            snapshotCheckpoint(openPaths);
            // Reset per-turn edit tracking so the multi-file review panel
            // only shows files touched by this user turn, not the
            // accumulation of the whole session.
            useStore.getState().clearPendingAgentEdits();
            useStore.getState().beginNewTurn();
            useStore.getState().pushTrajectoryEvent({
                kind: 'phase',
                title: 'User turn',
                detail: val.slice(0, 600),
            });
            setIsAgentThinking(true);
            addAgentMessage('user', processedVal, context);
            clearAttachedFiles();
            addAgentMessage('assistant', "");
            import('../application/agent/syncAgentMessages').then(m => m.scheduleChatHistorySync()).catch(() => {});

            try {
                const { ensureAgentRuntime } = await import('../application/performance/ensureAgentRuntime');
                await ensureAgentRuntime();
                const { sendAgentTurn } = await import('../application/agent/sendAgentTurn');
                await sendAgentTurn({ prompt: processedVal, context });
            } catch (err: any) {
                console.error('Agent chat failed:', err);
                const errorMsg = err.message || JSON.stringify(err);

                // Detect connection errors and route through resilience module
                const isConnectionError = /econnrefused|fetch failed|not responding|connection failed|network/i.test(errorMsg);
                if (isConnectionError) {
                    const { agentResilience } = await import('../lib/agentResilience');
                    agentResilience.markOffline();
                    // Queue the message for replay when backend recovers
                    agentResilience.queueMessage(processedVal);
                }

                updateLastAgentMessage(`Error: ${errorMsg}`);
            } finally {
                setIsAgentThinking(false);

                // ── Speak AI response with TTS ───────────────────────────────────
                if (ttsEnabled) {
                    const messages = useStore.getState().agentMessages;
                    const lastMsg = messages[messages.length - 1];
                    if (lastMsg?.role === 'assistant' && lastMsg.content) {
                        const textToSpeak = lastMsg.content.slice(0, 500); // Limit length
                        void getVoice().then(v => v.speak(textToSpeak, ttsPreset, () => setAiriSpeaking(false), () => setAiriSpeaking(true)));
                        setAiriSpeaking(true);
                    }
                }
            }
        }
    };

    // ── Voice interaction handler ──
    useEffect(() => {
        const handleVoiceMission = (e: any) => {
            const text = e.detail?.text;
            if (text) {
                onSend(text);
            }
        };
        window.addEventListener('airi-voice-mission', handleVoiceMission);
        return () => window.removeEventListener('airi-voice-mission', handleVoiceMission);
    }, [onSend]);

    const handleCopy = (content: string, idx: number) => {
        navigator.clipboard.writeText(content).then(() => {
            setLastCopiedIdx(idx);
            setTimeout(() => setLastCopiedIdx(null), 2000);
        });
    };

    const handleEditSave = (idx: number) => {
        const newVal = editValue.trim();
        if (newVal) {
            truncateAgentMessages(idx);
            setEditingMsgIdx(null);
            onSend(newVal);
        }
    };

    const handleRestoreCheckpoint = useCallback(async (msg: AgentMessage) => {
        const summary = (msg.checkpointDescription || msg.content || '').slice(0, 80);
        const ok = window.confirm(
            `Restore workspace to before this message?\n\n"${summary}"\n\nAll changes since then will be discarded and the chat will be truncated to this turn.`
        );
        if (!ok) return;
        const res = await useStore.getState().restoreToMessageCheckpoint(msg.timestamp);
        if (!res.ok) window.alert('Restore failed: ' + res.message);
    }, []);

    const onModeClick = (e: React.MouseEvent) => {
        const target = e.currentTarget as HTMLElement;
        import('../agent').then(m => m.openModeDropdown(target, () => { }));
    };

    // Visual style for the mode pill. Read-only modes get an orange tint so
    // the user sees at a glance they're in a "describe-only" mode and won't
    // get file writes / command executions.
    const modeStyle = useMemo(() => {
        const m = (mode || '').toLowerCase();
        const customId = (mode || '').match(/^custom:(.+)$/i)?.[1];
        const custom = customId? customModes?.find((c: { id: string }) => c.id === customId): null;
        const displayMode = custom?.label || mode || 'Harness';
        const readOnly = custom?.readOnly || m === 'chat' || m === 'planning' || m.includes('source control');
        const bug = m === 'bugbounty' || m === 'bug bounty';
        const harness = m === 'harness' && !custom;
        const danger = m === 'sentient' || bug;
        return {
            label: bug? 'Bug Bounty': displayMode,
            color: readOnly? '#f59e0b': (danger? '#ef4444': (harness? '#38bdf8': '#10b981')),
            background: readOnly? 'rgba(245,158,11,0.10)': (danger? 'rgba(239,68,68,0.10)': (harness? 'rgba(56,189,248,0.10)': 'rgba(16,185,129,0.10)')),
            border: readOnly? '1px solid rgba(245,158,11,0.35)': (danger? '1px solid rgba(239,68,68,0.35)': (harness? '1px solid rgba(56,189,248,0.35)': '1px solid rgba(16,185,129,0.30)')),
            title: readOnly
? `${displayMode} — READ-ONLY (no tool calls). Click to switch mode.`
: `${displayMode} — agent will write files and run commands. Click to change.`,
        };
    }, [mode, customModes]);

    const onModelClick = (e: React.MouseEvent) => {
        const target = e.currentTarget as HTMLElement;
        import('../agent').then(m => m.openModelDropdown(target, () => { }));
    };

    const modelLabel = useMemo(() => {
        const raw = (model || '').trim();
        if (!raw) return 'Select model';
        const id = raw.includes('|')? raw.split('|').slice(1).join('|'): raw;
        return id || 'Select model';
    }, [model]);

    // Removed old localstorage WebUI account setter helper

    const handleDragOver = (e: React.DragEvent) => {
        e.preventDefault();
        e.stopPropagation();
    };

    const readFileAsDataUrl = (file: File): Promise<string> =>
        new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = () => resolve(reader.result as string);
            reader.onerror = reject;
            reader.readAsDataURL(file);
        });

    const handleDrop = (e: React.DragEvent) => {
        e.preventDefault();
        e.stopPropagation();
        const files = e.dataTransfer.files;
        if (files.length > 0) {
            for (let i = 0; i < files.length; i++) {
                const file = files[i];
                const isImage = file.type.startsWith('image/');
                if (isImage) {
                    // Read image as base64 data URL so vision models can receive it
                    readFileAsDataUrl(file).then(dataUrl => {
                        attachFile({
                            id: `dropped-${Date.now()}-${i}`,
                            type: 'attachment',
                            name: file.name,
                            path: (file as any).path || file.name,
                            data: dataUrl,
                            thumbnail: dataUrl,
                        } as any);
                    }).catch(() => {
                        attachFile({
                            id: `dropped-${Date.now()}-${i}`,
                            type: 'attachment',
                            name: file.name,
                            path: (file as any).path || file.name,
                        });
                    });
                } else {
                    attachFile({
                        id: `dropped-${Date.now()}-${i}`,
                        type: 'file',
                        name: file.name,
                        path: (file as any).path || file.name,
                    });
                }
            }
        }
    };

    const handlePaste = (e: React.ClipboardEvent) => {
        const items = Array.from(e.clipboardData.items);
        const imageItem = items.find(item => item.type.startsWith('image/'));
        if (imageItem) {
            const file = imageItem.getAsFile();
            if (file) {
                e.preventDefault();
                readFileAsDataUrl(file).then(dataUrl => {
                    attachFile({
                        id: `pasted-${Date.now()}`,
                        type: 'attachment',
                        name: `pasted-image-${Date.now()}.png`,
                        path: `pasted-${Date.now()}.png`,
                        data: dataUrl,
                        thumbnail: dataUrl,
                    } as any);
                });
            }
        }
    };

    const handleMentionSelect = (file: any) => {
        const words = inputValue.split(/\s+/);
        // For special mentions, insert the full name; for files, insert @filename
        words[words.length - 1] = (file as any)._special? file.name: `@${file.name}`;
        const newValue = words.join(' ') + ' ';
        setInputValue(newValue);
        setIsMentionDropdownOpen(false);
        if ((file as any)._special) {
            // Special context source — resolved at send time in agent.ts
            attachFile({
                id: file.path,
                type: 'special' as any,
                name: file.name,
                path: file.path,
            });
        } else {
            attachFile({
                id: file.path,
                type: 'file',
                name: file.name,
                path: file.path,
            });
        }
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        // CRITICAL: stop ALL keyboard events from bubbling to the global handler
        // to prevent sidebar toggles, Ctrl+W closing tabs, etc. while typing
        e.stopPropagation();

        if (isMentionDropdownOpen && filteredSuggestions.length > 0) {
            if (e.key === 'ArrowDown') {
                e.preventDefault();
                setSelectedMentionIndex(prev => (prev + 1) % filteredSuggestions.length);
            } else if (e.key === 'ArrowUp') {
                e.preventDefault();
                setSelectedMentionIndex(prev => (prev - 1 + filteredSuggestions.length) % filteredSuggestions.length);
            } else if (e.key === 'Enter') {
                e.preventDefault();
                handleMentionSelect(filteredSuggestions[selectedMentionIndex]);
            } else if (e.key === 'Escape') {
                e.preventDefault();
                setIsMentionDropdownOpen(false);
            }
        } else if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            onSend();
        }
    };

    const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        const val = e.target.value;
        setInputValue(val);

        if (inputRef.current) {
            inputRef.current.style.height = 'auto';
            inputRef.current.style.height = `${Math.min(e.target.scrollHeight, 200)}px`;
        }

        const lastWord = val.split(/\s+/).pop() || '';
        setIsMentionDropdownOpen(lastWord.startsWith('@') || lastWord.startsWith('/'));
        setSelectedMentionIndex(0);
    };

    if (!isOpen) return null;

    return (
        <aside
            onDragOver={handleDragOver}
            onDrop={handleDrop}
            className="right-sidebar" id="right-sidebar"
            style={{
                background: 'var(--vscode-sideBar-background)',
                borderLeft: '1px solid var(--vscode-sideBar-border, rgba(0,0,0,0.2))',
                display: 'flex',
                flexDirection: 'column',
                height: '100%',
                overflow: 'hidden',
                position: 'relative'
            }}>
            <style>{`
                .agent-message-container:hover .message-actions {
                    opacity: 1 !important;
                }
                .hoverable:hover {
                    background: rgba(255,255,255,0.1) !important;
                    color: #fff;
                }
                .right-sidebar-body {
                    flex: 1 1 auto;
                    height: 0;
                    min-height: 0;
                    overflow: hidden;
                    display: flex;
                    flex-direction: column;
                    justify-content: flex-start !important;
                    align-items: stretch !important;
                    scroll-padding-top: 0;
                }
                .right-sidebar-active-surface {
                    flex: 1 1 auto;
                    min-height: 0;
                    height: 100%;
                    overflow: hidden;
                    display: flex;
                    flex-direction: column;
                    justify-content: flex-start !important;
                    align-items: stretch !important;
                }
                .right-sidebar-scroll {
                    flex: 1 1 auto;
                    min-height: 0;
                    overflow-y: auto;
                    display: flex;
                    flex-direction: column;
                    justify-content: flex-start !important;
                    align-items: stretch !important;
                }
                .right-sidebar-messages > div {
                    margin-top: 0 !important;
                }
                .right-sidebar-body > * {
                    margin-top: 0 !important;
                    align-self: stretch;
                }
                .right-sidebar-empty-chat {
                    flex: 0 0 auto !important;
                    min-height: 0 !important;
                    justify-content: flex-start !important;
                }
                .markdown-content p { margin: 0 0 1em 0; }
                .markdown-content p:last-child { margin-bottom: 0; }
                .markdown-content pre { 
                    background: rgba(0,0,0,0.3); 
                    padding: 12px; 
                    border-radius: 8px; 
                    overflow-x: auto;
                    border: 1px solid rgba(255,255,255,0.05);
                    margin: 12px 0;
                }
                .markdown-content code {
                    font-family: var(--font-mono);
                    background: rgba(255,255,255,0.1);
                    padding: 2px 4px;
                    border-radius: 4px;
                    font-size: 0.9em;
                }
                .markdown-content h1, .markdown-content h2, .markdown-content h3 {
                    margin: 1.5em 0 0.5em 0;
                    font-weight: 600;
                    color: #fff;
                }
                .help-modal-overlay {
                    position: absolute;
                    top: 0; left: 0; right: 0; bottom: 0;
                    background: rgba(0,0,0,0.8);
                    backdrop-filter: blur(4px);
                    z-index: 1000;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    padding: 20px;
                }
                .help-modal-content {
                    background: var(--vscode-sideBar-background);
                    border: 1px solid var(--vscode-sideBar-border);
                    border-radius: 12px;
                    width: 100%;
                    max-height: 80%;
                    overflow-y: auto;
                    padding: 24px;
                    box-shadow: 0 10px 40px rgba(0,0,0,0.5);
                }
                @keyframes hubPulse {
                    0%, 100% { opacity: 1; transform: scale(1); }
                    50% { opacity: 0.4; transform: scale(1.3); }
                }
            `}</style>

            <div className="right-sidebar-tabs" style={{ display: 'flex', flexDirection: 'column', flex: '0 0 auto', overflow: 'visible' }}>
                {/* ── Chat session tabs (Cursor Ctrl+T style) ── */}
                <div style={{
                    display: 'flex',
                    alignItems: 'center',
                    padding: '4px 8px 0',
                    background: 'var(--vscode-sideBar-background)',
                    borderBottom: '1px solid rgba(255,255,255,0.04)',
                    overflowX: 'auto',
                    gap: 2,
                    flexShrink: 0,
                }}>
                    {Object.values(agentThreads || {}).map((thread: any) => (
                        <div
                            key={thread.id}
                            onClick={() => setActiveAgentThread(thread.id)}
                            style={{
                                display: 'flex', alignItems: 'center', gap: 4,
                                padding: '3px 6px 3px 8px', borderRadius: '4px 4px 0 0',
                                background: activeAgentThreadId === thread.id
? 'rgba(255,255,255,0.07)': 'transparent',
                                border: '1px solid rgba(255,255,255,0.1)',
                                borderBottom: activeAgentThreadId === thread.id
? '1px solid var(--vscode-sideBar-background)': '1px solid transparent',
                                fontSize: 11, cursor: 'pointer',
                                whiteSpace: 'nowrap', maxWidth: 160,
                            }}
                        >
                            <i className="codicon codicon-comment-discussion" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 11, opacity: 0.7, flexShrink: 0 }} />
                            <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', maxWidth: 90, opacity: 0.9 }}>
                                {thread.firstUserSnippet || thread.name || 'Chat'}
                            </span>
                            <button
                                type="button"
                                title="Close chat"
                                onClick={(e) => {
                                    e.stopPropagation();
                                    closeAgentThread(thread.id);
                                }}
                                style={{
                                    display: 'flex', alignItems: 'center', justifyContent: 'center',
                                    width: 16, height: 16, padding: 0, marginLeft: 2,
                                    border: 'none', borderRadius: 3, cursor: 'pointer',
                                    background: 'transparent', color: 'rgba(255,255,255,0.35)',
                                    flexShrink: 0,
                                }}
                                onMouseEnter={e => { (e.currentTarget as HTMLButtonElement).style.background = 'rgba(255,255,255,0.12)'; (e.currentTarget as HTMLButtonElement).style.color = '#f87171'; }}
                                onMouseLeave={e => { (e.currentTarget as HTMLButtonElement).style.background = 'transparent'; (e.currentTarget as HTMLButtonElement).style.color = 'rgba(255,255,255,0.35)'; }}
                            >
                                <i className="codicon codicon-close" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 10 }} />
                            </button>
                        </div>
                    ))}
                    {Object.keys(agentThreads || {}).length === 0 && (
                        <div style={{ fontSize: 11, opacity: 0.5, padding: '3px 8px' }}>New Chat</div>
                    )}
                    <button
                        onClick={() => { void createNewSession?.(); }}
                        title="New Chat (Ctrl+T) — archives current conversation to History"
                        style={{
                            background: 'transparent', border: '1px dashed rgba(255,255,255,0.15)',
                            borderRadius: 4, padding: '2px 6px', fontSize: 11,
                            color: 'rgba(255,255,255,0.4)', cursor: 'pointer',
                            transition: 'all 0.12s',
                        }}
                        onMouseOver={e => { (e.target as HTMLElement).style.background = 'rgba(255,255,255,0.06)'; (e.target as HTMLElement).style.color = 'rgba(255,255,255,0.8)'; }}
                        onMouseOut={e => { (e.target as HTMLElement).style.background = 'transparent'; (e.target as HTMLElement).style.color = 'rgba(255,255,255,0.4)'; }}
                    >
                        <i className="codicon codicon-add" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 11 }} />
                    </button>
                    <div style={{ flex: 1 }} />
                    {/* History shortcut */}
                    <button
                        onClick={() => setView('history')}
                        title="Chat History"
                        style={{
                            background: view === 'history'? 'rgba(255,255,255,0.08)': 'transparent',
                            border: 'none', padding: '3px 5px', borderRadius: 4,
                            color: 'rgba(255,255,255,0.4)', cursor: 'pointer', fontSize: 11,
                        }}
                    >
                        <i className="codicon codicon-history" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 13 }} />
                    </button>
                </div>

                <div style={{
                    display: 'flex',
                    flexDirection: 'row',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    padding: '4px 12px 0',
                    gap: '8px',
                    borderBottom: '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.06))',
                    background: 'var(--vscode-sideBar-background)',
                }}>
                    <div className="vscr-agent-tabs" style={{ display: 'flex', gap: '2px', alignItems: 'center', flex: 1, minWidth: 0, overflowX: 'auto', flexWrap: 'nowrap', scrollbarWidth: 'none' as any }}>
                        {(['chat', 'studio', 'emulator', 'kortex', 'history'] as const).map(v => {
                            const tabLabels: Record<string, string> = {
                                chat: 'Chat', studio: 'Studio', emulator: 'Devices', kortex: 'Kortex', history: 'History',
                            };
                            return (
                            <button
                                key={v}
                                onClick={() => {
                                    setView(v as any);
                                    if (v === 'emulator') {
                                        useStore.getState().openEmulatorPanel();
                                    } else {
                                        useStore.getState().openAiriPanel();
                                    }
                                }}
                                style={{
                                    border: 'none',
                                    borderBottom: view === v
? '1.5px solid var(--vscode-panelTitle-activeBorder, var(--vscode-focusBorder, #007acc))'
: '1.5px solid transparent',
                                    background: 'transparent',
                                    color: view === v
? 'var(--vscode-panelTitle-activeForeground, #e7e7e7)'
: 'var(--vscode-panelTitle-inactiveForeground, rgba(231,231,231,0.55))',
                                    padding: '6px 7px 5px',
                                    borderRadius: 0,
                                    fontSize: '11px',
                                    fontWeight: 500,
                                    letterSpacing: '0.3px',
                                    cursor: 'pointer',
                                    textTransform: 'uppercase',
                                    whiteSpace: 'nowrap',
                                    flexShrink: 0,
                                    transition: 'color 0.12s ease, border-color 0.12s ease'
                                }}
                                className="hoverable"
                            >
                                {tabLabels[v] ?? v}
                            </button>
                            );
                        })}
                    </div>
                    <div style={{ display: 'flex', gap: '8px', alignItems: 'center', marginLeft: 'auto' }}>
                        {/* UI Mode toggle: Chat ↔ AIRI 3D — hidden when VRM is disabled */}
                        {showVrmAvatar && (
                            <div
                                onClick={() => setAgentUiMode(agentUiMode === 'chat'? 'airi': 'chat')}
                                style={{
                                    cursor: 'pointer',
                                    display: 'flex', alignItems: 'center', gap: '3px',
                                    fontSize: '9px', fontWeight: 700,
                                    padding: '2px 6px', borderRadius: '5px',
                                    background: agentUiMode === 'airi'? 'var(--vscode-button-secondaryBackground, rgba(255,255,255,0.08))': 'transparent',
                                    border: agentUiMode === 'airi'? '1px solid var(--vscode-focusBorder, #007acc)': '1px solid var(--vscode-panel-border, rgba(255,255,255,0.1))',
                                    color: agentUiMode === 'airi'? 'var(--vscode-foreground)': 'rgba(255,255,255,0.5)',
                                    transition: 'all 0.2s'
                                }}
                                title={agentUiMode === 'airi'? 'Switch to Chat mode': 'Switch to AIRI 3D mode'}
                            >
                                <Icon name={agentUiMode === 'airi'? 'agent': 'code'} size={14} />
                                <span>{agentUiMode === 'airi'? 'AIRI': 'CHAT'}</span>
                            </div>
                        )}
                        <div
                            onClick={() => setAiriToggleOpen(v => !v)}
                            style={{
                                cursor: 'pointer',
                                opacity: airiToggleOpen? 1: 0.6,
                                color: airiToggleOpen? 'var(--vscode-textLink-foreground, #3794ff)': 'inherit',
                                display: 'flex',
                                alignItems: 'center'
                            }}
                            title="AIRI subsystems (vision / consciousness)"
                        >
                            <i className="codicon codicon-eye" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px' }}></i>
                        </div>
                        <AgentMcpMenu />
                        <div
                            onClick={() => useStore.getState().openSettings('agent')}
                            style={{ cursor: 'pointer', opacity: 0.7, display: 'flex', alignItems: 'center' }}
                            title="Open unified Settings (AI Agent tab)"
                        >
                            <i className="codicon codicon-settings-gear" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px' }}></i>
                        </div>
                        <div
                            onClick={() => setIsHelpOpen(true)}
                            style={{ cursor: 'pointer', opacity: 0.8, color: '#3b82f6', display: 'flex', alignItems: 'center' }}
                            title="Command Help"
                        >
                            <i className="codicon codicon-question" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px' }}></i>
                        </div>
                        <div
                            onClick={toggle}
                            style={{ cursor: 'pointer', opacity: 0.5, display: 'flex', alignItems: 'center' }}
                            title="Close"
                        >
                            <i className="codicon codicon-close" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px' }}></i>
                        </div>
                    </div>
                </div>
            </div>

            {airiToggleOpen && (
                <div
                    style={{
                        padding: '10px 14px',
                        borderBottom: '1px solid rgba(255,255,255,0.06)',
                        background: 'var(--vscode-list-hoverBackground, rgba(255,255,255,0.04))',
                        display: 'flex',
                        flexDirection: 'column',
                        gap: '10px',
                        fontSize: '11px',
                    }}
                >
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                        <span style={{ textTransform: 'uppercase', letterSpacing: '0.08em', opacity: 0.55, fontSize: '9px', fontWeight: 700 }}>
                            AIRI subsystems
                        </span>
                        <span style={{ fontSize: '9px', opacity: 0.4 }}>
                            local models only
                        </span>
                    </div>

                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <button
                            type="button"
                            onClick={() => setAiriVisionEnabled(!airiVisionEnabled)}
                            style={{
                                cursor: 'pointer',
                                padding: '3px 8px',
                                borderRadius: '5px',
                                fontSize: '9px',
                                fontWeight: 700,
                                letterSpacing: '0.08em',
                                textTransform: 'uppercase',
                                color: airiVisionEnabled? '#34d399': 'rgba(255,255,255,0.55)',
                                background: airiVisionEnabled? 'rgba(52,211,153,0.12)': 'rgba(255,255,255,0.05)',
                                border: airiVisionEnabled? '1px solid rgba(52,211,153,0.45)': '1px solid rgba(255,255,255,0.1)',
                            }}
                            title={airiVisionEnabled? 'Disable screen vision (heavy — VL model + capture)': 'Enable screen vision (off by default — requires vision model)'}
                        >
                            Vision {airiVisionEnabled? 'ON': 'OFF'}
                        </button>
                        <input
                            type="text"
                            value={airiVisionModel}
                            onChange={e => setAiriVisionModel(e.target.value)}
                            placeholder="vision model tag"
                            style={{
                                flex: 1,
                                background: 'rgba(255,255,255,0.04)',
                                border: '1px solid rgba(255,255,255,0.1)',
                                borderRadius: '5px',
                                padding: '4px 8px',
                                color: 'inherit',
                                fontSize: '11px',
                                outline: 'none',
                            }}
                        />
                    </div>

                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <button
                            type="button"
                            onClick={() => setAiriConsciousnessEnabled(!airiConsciousnessEnabled)}
                            style={{
                                cursor: 'pointer',
                                padding: '3px 8px',
                                borderRadius: '5px',
                                fontSize: '9px',
                                fontWeight: 700,
                                letterSpacing: '0.08em',
                                textTransform: 'uppercase',
                                color: airiConsciousnessEnabled? 'var(--vscode-focusBorder, #007acc)': 'rgba(255,255,255,0.55)',
                                background: airiConsciousnessEnabled? 'rgba(255,255,255,0.08)': 'rgba(255,255,255,0.05)',
                                border: airiConsciousnessEnabled? '1px solid var(--vscode-focusBorder, rgba(0,122,204,0.45))': '1px solid rgba(255,255,255,0.1)',
                            }}
                            title={airiConsciousnessEnabled? 'Pause AIRI background thoughts': 'Resume AIRI background thoughts'}
                        >
                            Thoughts {airiConsciousnessEnabled? 'ON': 'OFF'}
                        </button>
                        <input
                            type="text"
                            value={airiConsciousnessModel}
                            onChange={e => setAiriConsciousnessModel(e.target.value)}
                            placeholder="consciousness model tag"
                            style={{
                                flex: 1,
                                background: 'rgba(255,255,255,0.04)',
                                border: '1px solid rgba(255,255,255,0.1)',
                                borderRadius: '5px',
                                padding: '4px 8px',
                                color: 'inherit',
                                fontSize: '11px',
                                outline: 'none',
                            }}
                        />
                    </div>

                    <div style={{ fontSize: '9px', opacity: 0.45, lineHeight: 1.4 }}>
                        Vision sends screen captures to a local Ollama VL model. Thoughts runs a lightweight LLM
                        for AIRI's background monologue. Both default to your local install — turn off to save GPU.
                    </div>
                </div>
            )}

            {isHelpOpen && (
                <div className="help-modal-overlay" onClick={() => setIsHelpOpen(false)}>
                    <div className="help-modal-content" onClick={e => e.stopPropagation()}>
                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px' }}>
                            <h3 style={{ margin: 0, fontSize: '14px', textTransform: 'uppercase', letterSpacing: '0.1em' }}>Command Reference</h3>
                            <i className="codicon codicon-close" style={{ cursor: 'pointer', fontFamily: 'codicon', fontStyle: 'normal' }} onClick={() => setIsHelpOpen(false)}></i>
                        </div>
                        <div className="markdown-content" style={{ fontSize: '12px', opacity: 0.9 }}>
                            <p><strong>CORE COMMANDS:</strong></p>
                            <ul>
                                <li><code>/doctor</code> - Run system environment diagnostics.</li>
                                <li><code>/help</code> - Show this reference in chat.</li>
                                <li><code>/tools</code> - List all available tools & schemas.</li>
                                <li><code>/clear</code> - Reset conversation context.</li>
                                <li><code>/resume</code> - Restore last persistent session.</li>
                            </ul>
                            <p><strong>ENGINEERING COMMANDS:</strong></p>
                            <ul>
                                <li><code>/diff</code> - View workspace changes.</li>
                                <li><code>/commit</code> - Automated staging and committing.</li>
                                <li><code>/compact</code> - Compress chat history.</li>
                            </ul>
                            <p><strong>REASONING TIERS:</strong></p>
                            <ul>
                                <li><code>/advisor &lt;model&gt;</code> - Delegate planning to high-tier model.</li>
                                <li><code>/ultraplan</code> - Trigger deep architectural reasoning loop.</li>
                                <li><code>/insights</code> - Generate project architectural report.</li>
                            </ul>
                            <p style={{ marginTop: '20px', fontSize: '10px', opacity: 0.5 }}><em>Integrates all features from the original Claude Code architecture.</em></p>
                        </div>
                    </div>
                </div>
            )}

            {/* Top mode/model picker REMOVED — redundant with the picker in the input bar
                at the bottom of the chat. Keeps the header clean (VSCode-like). */}

            <div style={{ flex: '1 1 auto', height: 0, display: 'flex', flexDirection: 'column', overflow: 'hidden', minHeight: 0 }}>

                {/* ── AIRI 3D FULL MODE — only available when VRM is enabled ── */}
                {showVrmAvatar && agentUiMode === 'airi'? (
                    <div style={{ display: 'flex', flexDirection: 'column', flex: 1, overflow: 'hidden', position: 'relative' }}>

                        {/* Full-height 3D avatar */}
                        <div style={{ flex: 1, minHeight: 0, position: 'relative', overflow: 'hidden' }}>
                            <Suspense fallback={<div style={{ padding: 16, opacity: 0.4, fontSize: 11 }}>Loading 3D avatar…</div>}>
                                <AiriPanel style={{ width: '100%', height: '100%' }} transparent={true} character={avatarCharacter} />
                            </Suspense>

                            {/* Active tool pill — bottom of avatar */}
                            {liveToolCalls[0] && liveToolCalls[0].status === 'running' && (
                                <div style={{
                                    position: 'absolute', bottom: '12px', left: '50%', transform: 'translateX(-50%)',
                                    background: 'rgba(0,0,0,0.6)', backdropFilter: 'blur(8px)',
                                    border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.15))',
                                    borderRadius: '20px', padding: '4px 14px',
                                    display: 'flex', alignItems: 'center', gap: '8px',
                                    fontSize: '10px', color: 'var(--vscode-focusBorder, #007acc)', fontWeight: 700,
                                    pointerEvents: 'none', whiteSpace: 'nowrap',
                                    maxWidth: '90%', overflow: 'hidden', textOverflow: 'ellipsis',
                                }}>
                                    <span style={{ width: '6px', height: '6px', borderRadius: '50%', background: 'var(--vscode-focusBorder, #007acc)', display: 'inline-block', animation: 'hubPulse 1s infinite', flexShrink: 0 }} />
                                    <ToolIcon tool={liveToolCalls[0].tool} size={12} style={{ flexShrink: 0 }} />
                                    {liveToolCalls[0].label}
                                </div>
                            )}
                        </div>

                        {/* Speech / response overlay at the bottom */}
                        <div style={{
                            flexShrink: 0,
                            background: 'linear-gradient(0deg, rgba(10,8,20,0.97) 60%, transparent)',
                            padding: '12px 14px 6px',
                            maxHeight: '32%',
                            overflowY: 'auto',
                            transition: 'max-height 0.3s ease',
                        }}>
                            {(() => {
                                const cleaned = cleanAgentContent(airiSpeech);
                                if (cleaned) return (
                                    <div>
                                        <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginBottom: '6px' }}>
                                            <span style={{ fontSize: '10px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.08em', color: 'var(--vscode-focusBorder, #007acc)' }}>
                                                {airiSpeaking? '◉ Speaking': ' AIRI'}
                                            </span>
                                            {airiSpeaking && <span style={{ width: '5px', height: '5px', borderRadius: '50%', background: 'var(--vscode-focusBorder, #007acc)', display: 'inline-block', animation: 'hubPulse 0.8s infinite' }} />}
                                        </div>
                                        <div
                                            className="markdown-content"
                                            style={{ fontSize: '12.5px', lineHeight: '1.55', color: 'rgba(255,255,255,0.88)' }}
                                            dangerouslySetInnerHTML={{ __html: airiSpeechHtml }}
                                        />
                                    </div>
                                );
                                return <div style={{ fontSize: '10px', opacity: 0.2, textAlign: 'center', padding: '6px 0' }}>Awaiting mission</div>;
                            })()}
                        </div>

                        {/* Mission input at the very bottom */}
                        <div style={{ flexShrink: 0, padding: '8px 12px', borderTop: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.12))' }}>
                            <div className="ac-input-shell" style={{
                                background: 'var(--ac-surface)', border: '1px solid var(--ac-border)',
                                borderRadius: 'var(--ac-radius-lg)', padding: '6px 10px', display: 'flex', gap: '8px', alignItems: 'flex-end'
                            }}>
                                <textarea
                                    ref={inputRef} value={inputValue} onChange={handleInputChange} onKeyDown={handleKeyDown} onPaste={handlePaste}
                                    className="agent-mission-input"
                                    placeholder="Speak to AIRI..."
                                    disabled={isAgentThinking}
                                    rows={1}
                                    style={{
                                        background: 'transparent', border: 'none', outline: 'none', color: 'var(--vscode-editor-foreground, #fff)',
                                        resize: 'none', fontSize: '13px', lineHeight: '1.5', flex: 1, minHeight: '22px',
                                        opacity: isAgentThinking? 0.4: 1
                                    }}
                                />
                                <div onClick={() => onSend()} style={{
                                    width: '28px', height: '28px', borderRadius: '50%', flexShrink: 0,
                                    background: (inputValue.trim() && !isAgentThinking)? 'var(--vscode-button-background, #0e639c)': 'rgba(255,255,255,0.08)',
                                    display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer',
                                    transition: 'background 0.2s'
                                }}>
                                    <i className="codicon codicon-arrow-right" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', color: '#000' }}></i>
                                </div>
                                {/* ── TTS Voice Controls ────────────────────────────────────────── */}
                                <div
                                    onClick={() => {
                                        if (!ttsEnabled) {
                                            getVoice().then(v => v.initTTS()).then(ready => {
                                                if (ready) setTtsEnabled(true);
                                            });
                                        } else {
                                            setTtsEnabled(!ttsEnabled);
                                        }
                                    }}
                                    style={{
                                        width: '28px', height: '28px', borderRadius: '50%', flexShrink: 0,
                                        background: ttsEnabled? '#10b981': 'rgba(255,255,255,0.08)',
                                        display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer',
                                        transition: 'background 0.2s'
                                    }}
                                    title={ttsEnabled? 'AIRI Voice ON': 'AIRI Voice OFF'}
                                >
                                    <i className="codicon codicon-unmute" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', color: 'var(--vscode-editor-foreground, #fff)' }}></i>
                                </div>
                                {ttsEnabled && (
                                    <select
                                        value={ttsPreset}
                                        onChange={(e) => setTtsPreset(e.target.value as any)}
                                        style={{
                                            background: 'rgba(255,255,255,0.08)', border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.15))',
                                            borderRadius: '4px', color: 'var(--vscode-editor-foreground, #fff)', fontSize: '10px', padding: '2px 4px',
                                            cursor: 'pointer', outline: 'none'
                                        }}
                                    >
                                        <option value="airi">AIRI (Female)</option>
                                        <option value="sage">Sage (Female)</option>
                                        <option value="nova">Nova (Female)</option>
                                        <option value="kawaii">Kawaii (Female)</option>
                                        <option value="hana">Hana (Female)</option>
                                        <option value="yuki">Yuki (Female)</option>
                                        <option value="sora">Sora (Female)</option>
                                        <option value="aria">Aria (Female)</option>
                                        <option value="yamato">Yamato (Male)</option>
                                        <option value="ren">Ren (Male)</option>
                                        <option value="haru">Haru (Male)</option>
                                        <option value="zero">Zero (Male)</option>
                                    </select>
                                )}
                            </div>
                            <div style={{ display: 'flex', gap: '8px', justifyContent: 'center', marginTop: '6px' }}>
                                <span onClick={onModeClick} style={{ fontSize: '9px', opacity: 0.35, cursor: 'pointer' }}>{mode}</span>
                                <span style={{ fontSize: '9px', opacity: 0.2 }}>·</span>
                                <span onClick={onModelClick} style={{ fontSize: '9px', opacity: model? 0.35: 0.85, cursor: 'pointer', color: model? undefined: '#f59e0b' }}>{modelLabel}</span>
                                <span style={{ fontSize: '9px', opacity: 0.2 }}>·</span>
                                <span
                                    onClick={() => import('../agent').then(m => m.setYoloMode(!isYoloMode).then(() => setYoloMode(!isYoloMode)))}
                                    style={{ fontSize: '9px', cursor: 'pointer', color: isYoloMode? '#f97316': 'rgba(255,255,255,0.3)', fontWeight: isYoloMode? 700: 400 }}
                                >YOLO</span>
                                <span style={{ fontSize: '9px', opacity: 0.2 }}>·</span>
                                <span
                                    onClick={() => {
                                        const next = !isContinuousMode;
                                        setContinuousMode(next);
                                        if (next) {
                                            import('../agent').then(m => m.runContinuousLoop('Continue working on pending tasks. Pick the next unchecked task and implement it fully.'));
                                        }
                                    }}
                                    title="Continuous Mode: agent keeps working until all tasks done"
                                    style={{ fontSize: '9px', cursor: 'pointer', color: isContinuousMode? '#22d3ee': 'rgba(255,255,255,0.3)', fontWeight: isContinuousMode? 700: 400 }}
                                >∞ AUTO</span>
                            </div>
                        </div>
                    </div>
                ): (

                    /* ── CHAT / MISSION HUB MODE ── */
                    <div className="right-sidebar-body">
                        {view === 'chat'? (
                            <div ref={chatScrollRef} className={`right-sidebar-messages right-sidebar-scroll ${messages.length === 0? 'right-sidebar-empty-chat': ''}`} style={{ justifyContent: 'flex-start', alignItems: 'stretch', paddingTop: 0 }}>

                                {/* Offline/reconnecting banner — shown when inference backend is unreachable */}
                                {(() => {
                                    const OfflineBanner = React.lazy(() => import('./chat/OfflineBanner'));
                                    return <React.Suspense fallback={null}><OfflineBanner /></React.Suspense>;
                                })()}

                                {/* Crash recovery banner — shown when previous session was interrupted */}
                                {(() => {
                                    const CrashRecoveryBanner = React.lazy(() => import('./CrashRecoveryBanner'));
                                    return <React.Suspense fallback={null}><CrashRecoveryBanner /></React.Suspense>;
                                })()}

                                {/* AIRI Sentient Header — only rendered when VRM is enabled */}
                                {showVrmAvatar? (
                                    <div style={{
                                        position: 'sticky', top: 0, zIndex: 10,
                                        background: 'linear-gradient(180deg, var(--vscode-sideBar-background) 60%, transparent)',
                                        display: 'flex', flexDirection: 'column', alignItems: 'center',
                                        transition: 'all 0.3s ease-in-out',
                                        height: messages.length === 0? '72px': '0px',
                                        minHeight: messages.length === 0? '72px': '0px',
                                        paddingTop: messages.length === 0? '6px': '0px',
                                        overflow: 'hidden', pointerEvents: 'none'
                                    }}>
                                        <div style={{
                                            width: messages.length === 0? '52px': '0px',
                                            height: messages.length === 0? '52px': '0px',
                                            borderRadius: messages.length === 0? '0%': '50%',
                                            overflow: 'hidden',
                                            background: messages.length === 0? 'transparent': 'rgba(255,255,255,0.05)',
                                            border: messages.length === 0? 'none': '1px solid rgba(255,255,255,0.1)',
                                            transition: 'all 0.3s ease-in-out'
                                        }}>
                                            <Suspense fallback={<div style={{ padding: 16, opacity: 0.4, fontSize: 11 }}>Loading 3D avatar…</div>}>
                                                <AiriPanel style={{ width: '100%', height: '100%' }} scale={messages.length === 0? 0.5: 0.6} yOffset={messages.length === 0? "-44%": "-44%"} transparent={true} character={avatarCharacter} />
                                            </Suspense>
                                        </div>
                                        {messages.length === 0 && (
                                            <div style={{ marginTop: '2px', textAlign: 'center', pointerEvents: 'auto' }}>
                                                <div style={{ fontSize: '13px', fontWeight: 700, marginBottom: '4px', letterSpacing: '0.05em' }}>AIRI SENTIENT CORE</div>
                                                <div style={{ fontSize: '11px', opacity: 0.4 }}>
                                                    {isYoloMode? 'YOLO — Full autonomy': 'Ready for your mission'}
                                                </div>
                                            </div>
                                        )}
                                    </div>
                                ): (
                                    /* Clean minimal header when VRM is disabled — Cursor-style */
                                    messages.length === 0? (
                                        <div style={{
                                            padding: '10px 16px 8px',
                                            textAlign: 'center',
                                        }}>
                                            <div style={{ fontSize: '13px', fontWeight: 700, marginBottom: '4px', letterSpacing: '0.05em', opacity: 0.85 }}>Agent</div>
                                            <div style={{ fontSize: '11px', opacity: 0.4 }}>
                                                {isYoloMode? 'YOLO — Full autonomy': 'What can I help you with?'}
                                            </div>
                                        </div>
                                    ): null
                                )}

                                {/* Quick Mission Workflows — shown only on fresh session */}
                                {messages.length === 0 && (
                                    <div style={{ padding: '0 12px 12px', flexShrink: 0 }}>
                                        <div style={{ fontSize: '9px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.1em', opacity: 0.35, marginBottom: '8px' }}>Quick Missions</div>
                                        <div className="quick-missions-grid">
                                            {[
                                                { icon: 'search', label: 'Audit Codebase', mission: 'Audit the entire codebase for bugs, dead code, and architectural issues. List findings.' },
                                                { icon: 'globe', label: 'Web Research', mission: '/manus Research this project and any URLs I mention — full scrape, browser agent, and security audit.' },
                                                { icon: 'tools', label: 'Fix All Errors', mission: 'Find all compiler errors and runtime issues in this project. Fix them one by one.' },
                                                { icon: 'git-commit', label: 'Git Commit', mission: 'Stage all modified files and create a meaningful commit message based on the changes.' },
                                                { icon: 'rocket', label: 'Build & Verify', mission: 'Run cargo build, fix any errors, then verify the implementation is correct.' },
                                                { icon: 'beaker', label: 'Write Tests', mission: 'Write comprehensive unit tests for the most critical functions in this project.' },
                                                { icon: 'symbol-method', label: 'Refactor', mission: 'Identify and refactor the most complex functions for clarity and performance.' },
                                            ].map(({ icon, label, mission }) => (
                                                <button
                                                    key={label}
                                                    className="ac-mission"
                                                    onClick={() => !isAgentThinking && onSend(mission)}
                                                    disabled={isAgentThinking}
                                                >
                                                    <i className={`codicon codicon-${icon}`} style={{ fontSize: '14px', opacity: 0.8 }} />
                                                    <span style={{ fontWeight: 600 }}>{label}</span>
                                                </button>
                                            ))}
                                        </div>
                                    </div>
                                )}

                                {/* Live tool activity — Cursor-style feed rendered inside ChatMessageList. */}

                                {!agentCleanUi && (isAgentThinking || liveToolCalls.length > 0) && (() => {
                                    // Deduplicate consecutive same-tool calls → show label + count badge
                                    const deduped: { id: string; label: string; tool: string; status: 'running' | 'done' | 'error'; count: number }[] = [];
                                    for (const tc of liveToolCalls.slice(0, 12)) {
                                        const last = deduped[deduped.length - 1];
                                        if (last && last.tool === tc.tool) {
                                            last.count++;
                                            if (tc.status === 'running') last.status = 'running';
                                            else if (tc.status === 'error') last.status = 'error';
                                        } else {
                                            deduped.push({ ...tc, count: 1 });
                                        }
                                    }
                                    return (
                                        <div style={{
                                            margin: '8px 10px 6px',
                                            background: 'rgba(15,15,25,0.8)',
                                            border: `1px solid ${isAgentThinking? 'rgba(249,115,22,0.25)': 'rgba(16,185,129,0.2)'}`,
                                            borderRadius: '8px', padding: '7px 10px',
                                            transition: 'border-color 0.5s'
                                        }}>
                                            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '6px' }}>
                                                <div style={{ fontSize: '9px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.1em', color: isAgentThinking? 'rgba(249,115,22,0.7)': 'rgba(16,185,129,0.6)' }}>
                                                    {isAgentThinking? (isYoloMode? 'YOLO Executing': '● Live Actions'): ' Completed'}
                                                </div>
                                                <div style={{ display: 'flex', gap: 6, alignItems: 'center' }}>
                                                    {/* Open the trajectory timeline. We render it here
                                                        instead of in a global toolbar so it appears
                                                        right next to the live action feed it's tied to. */}
                                                    <div
                                                        onClick={() => useStore.getState().openTrajectory()}
                                                        style={{ cursor: 'pointer', fontSize: '10px', opacity: 0.55, padding: '0 4px', lineHeight: 1 }}
                                                        title="Open agent trajectory timeline"
                                                    >⏱</div>
                                                    <div
                                                        onClick={() => setLiveToolCalls([])}
                                                        style={{ cursor: 'pointer', fontSize: '11px', opacity: 0.35, padding: '0 2px', lineHeight: 1 }}
                                                        title="Clear feed"
                                                    ></div>
                                                </div>
                                            </div>
                                            {deduped.length === 0 && isAgentThinking && (
                                                <div style={{ fontSize: '11px', opacity: 0.4, fontStyle: 'italic' }}>Thinking...</div>
                                            )}
                                            {deduped.slice(0, 6).map(tc => (
                                                <div key={tc.id} style={{
                                                    display: 'flex', alignItems: 'center', gap: '6px',
                                                    fontSize: '11px', padding: '2px 0',
                                                    color: tc.status === 'done'? 'rgba(255,255,255,0.25)': tc.status === 'error'? '#ef4444': 'rgba(255,255,255,0.8)',
                                                }}>
                                                    {tc.status === 'running' && <span style={{ width: '5px', height: '5px', borderRadius: '50%', background: '#f97316', display: 'inline-block', animation: 'hubPulse 1s infinite', flexShrink: 0 }} />}
                                                    {tc.status === 'done' && <Icon name="check" size={12} style={{ color: '#10b981', flexShrink: 0 }} />}
                                                    {tc.status === 'error' && <Icon name="x" size={12} style={{ color: '#ef4444', flexShrink: 0 }} />}
                                                    <ToolIcon tool={tc.tool} size={12} style={{ flexShrink: 0, opacity: 0.8 }} />
                                                    <span style={{ flex: 1, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>{tc.label}</span>
                                                    {tc.count > 1 && (
                                                        <span style={{
                                                            fontSize: '9px', fontWeight: 700, padding: '0 5px', borderRadius: '10px',
                                                            background: 'rgba(255,255,255,0.08)', color: 'rgba(255,255,255,0.4)',
                                                            flexShrink: 0
                                                        }}>×{tc.count}</span>
                                                    )}
                                                </div>
                                            ))}
                                        </div>
                                    );
                                })()}

                                {/* Agent Task Roadmap */}
                                <TaskRoadmap />

                                {/* Agent task progress bars */}
                                {agentTasks.filter((t: any) => t.status === 'running' && t.id.includes('-')).map((task: any) => (
                                    <div key={task.id} style={{
                                        margin: '6px 10px',
                                        background: 'rgba(59,130,246,0.05)',
                                        border: '1px solid rgba(59,130,246,0.2)',
                                        padding: '8px 10px', borderRadius: '8px',
                                    }}>
                                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '6px' }}>
                                            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                                <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px', color: '#3b82f6' }}></i>
                                                <span style={{ fontSize: '11px', fontWeight: 600 }}>{task.title}</span>
                                            </div>
                                            <span style={{ fontSize: '10px', opacity: 0.5 }}>{task.progress}%</span>
                                        </div>
                                        <div style={{ background: 'rgba(0,0,0,0.3)', height: '3px', borderRadius: '2px', overflow: 'hidden' }}>
                                            <div style={{ background: '#3b82f6', width: `${task.progress}%`, height: '100%', transition: 'width 0.3s ease' }} />
                                        </div>
                                        {task.message && <div style={{ fontSize: '10px', marginTop: '4px', opacity: 0.5 }}>{task.message}</div>}
                                    </div>
                                ))}

                                <ChatMessageList
                                    scrollContainerRef={chatScrollRef}
                                    messages={visibleMessages}
                                    isAgentThinking={isAgentThinking}
                                    lastCopiedIdx={lastCopiedIdx}
                                    editingMsgIdx={editingMsgIdx}
                                    editValue={editValue}
                                    onCopy={handleCopy}
                                    onEditStart={(idx, content) => { setEditingMsgIdx(idx); setEditValue(content); }}
                                    onEditChange={setEditValue}
                                    onEditSave={handleEditSave}
                                    onEditCancel={() => setEditingMsgIdx(null)}
                                    onRestoreCheckpoint={handleRestoreCheckpoint}
                                />
                            </div>
                        ): view === 'emulator'? (
                            <div className="right-sidebar-active-surface" style={{ justifyContent: 'flex-start', alignItems: 'stretch' }}>
                                <Suspense fallback={<div style={{ padding: 20, opacity: 0.5, fontSize: 11 }}>Loading emulator panel…</div>}>
                                    <UnifiedEmulatorPanel />
                                </Suspense>
                            </div>
                        ): view === 'kortex'? (
                            /* Kortex .aim Brain Panel */
                            <div className="right-sidebar-active-surface">
                                <div style={{ padding: '16px 16px 8px', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                        <Icon name="brain" size={16} />
                                        <div>
                                            <div style={{ fontSize: '11px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.08em' }}>Kortex Brain</div>
                                            <div style={{ fontSize: '10px', opacity: 0.4 }}>{kortexSlots.length} knowledge slots</div>
                                        </div>
                                    </div>
                                    <button
                                        onClick={refreshKortex}
                                        disabled={kortexLoading}
                                        style={{ background: 'rgba(255,255,255,0.06)', border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.15))', color: 'var(--vscode-textLink-foreground, #3794ff)', padding: '4px 10px', borderRadius: '6px', fontSize: '10px', cursor: kortexLoading? 'not-allowed': 'pointer', fontWeight: 600 }}
                                    >
                                        {kortexLoading? '...': 'Refresh'}
                                    </button>
                                </div>

                                {/* Category breakdown */}
                                {kortexSlots.length > 0 && (() => {
                                    const cats: Record<string, number> = {};
                                    kortexSlots.forEach(s => { cats[s.category] = (cats[s.category] || 0) + 1; });
                                    return (
                                        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '4px', padding: '0 16px 12px' }}>
                                            {Object.entries(cats).map(([cat, count]) => (
                                                <span key={cat} style={{
                                                    fontSize: '9px', padding: '2px 7px', borderRadius: '10px',
                                                    background: 'rgba(255,255,255,0.06)', border: '1px solid rgba(255,255,255,0.12)',
                                                    color: 'var(--vscode-focusBorder, #007acc)', fontWeight: 600
                                                }}>{cat} ({count})</span>
                                            ))}
                                        </div>
                                    );
                                })()}

                                <div style={{ flex: 1, overflowY: 'auto', padding: '0 12px 12px', display: 'flex', flexDirection: 'column', gap: '6px' }}>
                                    {kortexLoading? (
                                        <div style={{ padding: '40px 20px', textAlign: 'center', opacity: 0.4, fontSize: '12px' }}>
                                            <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '20px', display: 'block', marginBottom: '8px' }}></i>
                                            Loading neural weights...
                                        </div>
                                    ): kortexSlots.length === 0? (
                                        <div style={{ padding: '40px 20px', textAlign: 'center', opacity: 0.4, fontSize: '12px' }}>
                                            <Icon name="brain" size={32} style={{ display: 'block', marginBottom: '8px' }} />
                                            No knowledge stored yet.<br />
                                            <span style={{ fontSize: '10px', opacity: 0.6 }}>Run a mission to populate the brain.</span>
                                        </div>
                                    ): (
                                        kortexSlots.map((slot, i) => (
                                            <div key={slot.id || i} style={{
                                                background: 'var(--vscode-list-hoverBackground, rgba(255,255,255,0.04))',
                                                border: '1px solid rgba(255,255,255,0.08)',
                                                borderRadius: '8px', padding: '8px 12px',
                                            }}>
                                                <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginBottom: '4px' }}>
                                                    <span style={{
                                                        fontSize: '8px', padding: '1px 5px', borderRadius: '8px',
                                                        background: 'var(--vscode-panel-border, rgba(255,255,255,0.12))', color: 'var(--vscode-focusBorder, #007acc)', fontWeight: 700, textTransform: 'uppercase'
                                                    }}>{slot.category}</span>
                                                    <span style={{ fontSize: '9px', opacity: 0.3, marginLeft: 'auto' }}>
                                                        {new Date(slot.timestamp * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                                                    </span>
                                                </div>
                                                <div style={{ fontSize: '11px', opacity: 0.8, lineHeight: 1.4, fontFamily: 'var(--font-mono)' }}>
                                                    {slot.content.slice(0, 140)}{slot.content.length > 140? '…': ''}
                                                </div>
                                                {slot.tags && slot.tags.length > 0 && (
                                                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: '3px', marginTop: '6px' }}>
                                                        {slot.tags.map((tag: string, ti: number) => (
                                                            <span key={ti} style={{ fontSize: '9px', opacity: 0.4, padding: '1px 4px', background: 'rgba(255,255,255,0.05)', borderRadius: '4px' }}>#{tag}</span>
                                                        ))}
                                                    </div>
                                                )}
                                            </div>
                                        ))
                                    )}
                                </div>

                                {/* Kortex Services */}
                                <KortexServicesPanel />
                            </div>
                        ): view === 'history'? (
                            <div className="right-sidebar-scroll" style={{ padding: '8px 16px 16px', gap: '12px', justifyContent: 'flex-start', alignItems: 'stretch' }}>
                                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '4px', gap: 8 }}>
                                    <div>
                                        <div style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', opacity: 0.5 }}>Conversation history</div>
                                        <div style={{ fontSize: 10, opacity: 0.4, marginTop: 2 }}>Click any session to restore the full chat in the panel.</div>
                                    </div>
                                    <button
                                        type="button"
                                        onClick={() => { void archiveCurrentSession?.(); refreshChatSessions(); }}
                                        title="Save current chat as a named archive"
                                        style={{
                                            padding: '4px 8px', fontSize: 10, borderRadius: 4, cursor: 'pointer',
                                            border: '1px solid rgba(255,255,255,0.12)', background: 'rgba(255,255,255,0.04)', color: 'inherit',
                                        }}
                                    >
                                        Archive current
                                    </button>
                                </div>
                                {chatSessions.length === 0? (
                                    <div style={{ padding: '20px', textAlign: 'center', opacity: 0.5, fontSize: '12px', lineHeight: 1.5 }}>
                                        No conversations yet.<br />
                                        Send a message in Chat — it appears here automatically.
                                    </div>
                                ): (
                                    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                                        {chatSessions.map((session: any) => {
                                            const title = session.title || String(session.name || '').replace('session_', 'Chat ') || 'Conversation';
                                            const preview = session.preview || '';
                                            const ts = session.updated_at? new Date(session.updated_at * 1000).toLocaleString(): '';
                                            const isCurrent = !!session.is_current;
                                            const restore = () => {
                                                void loadChatSession(session.path).then(() => setView('chat'));
                                            };
                                            return (
                                                <div
                                                    key={session.path}
                                                    role="button"
                                                    tabIndex={0}
                                                    onClick={restore}
                                                    onKeyDown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); restore(); } }}
                                                    style={{
                                                        padding: '10px 12px', borderRadius: '8px', cursor: 'pointer',
                                                        background: isCurrent? 'rgba(0,122,204,0.12)': 'rgba(255,255,255,0.03)',
                                                        border: isCurrent? '1px solid rgba(0,122,204,0.35)': '1px solid rgba(255,255,255,0.05)',
                                                        transition: 'background 0.2s',
                                                    }}
                                                    onMouseEnter={(e) => e.currentTarget.style.background = isCurrent? 'rgba(0,122,204,0.18)': 'rgba(255,255,255,0.06)'}
                                                    onMouseLeave={(e) => e.currentTarget.style.background = isCurrent? 'rgba(0,122,204,0.12)': 'rgba(255,255,255,0.03)'}
                                                >
                                                    <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '4px', gap: 8, alignItems: 'center' }}>
                                                        <span style={{ fontSize: '12px', fontWeight: 600, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{title}</span>
                                                        {isCurrent && (
                                                            <span style={{ fontSize: 9, padding: '1px 6px', borderRadius: 4, background: 'rgba(0,122,204,0.25)', color: '#7ec8ff', flexShrink: 0 }}>Live</span>
                                                        )}
                                                        <span style={{ fontSize: '10px', opacity: 0.4, flexShrink: 0 }}>{session.messages} msgs</span>
                                                    </div>
                                                    {preview && (
                                                        <div style={{ fontSize: 11, opacity: 0.55, marginBottom: 6, lineHeight: 1.4, overflow: 'hidden', display: '-webkit-box', WebkitLineClamp: 2, WebkitBoxOrient: 'vertical' as any }}>
                                                            {preview}
                                                        </div>
                                                    )}
                                                    <div style={{ fontSize: '10px', opacity: 0.45, display: 'flex', alignItems: 'center', gap: 4 }}>
                                                        <i className="codicon codicon-comment-discussion" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 10 }} />{ts}
                                                    </div>
                                                </div>
                                            );
                                        })}
                                    </div>
                                )}
                                <div style={{ marginTop: 16, paddingTop: 12, borderTop: '1px solid rgba(255,255,255,0.06)', fontSize: 10, opacity: 0.4, lineHeight: 1.5 }}>
                                    Code restore points (git checkpoints) live in Source Control — not here.
                                </div>
                            </div>
                        ): view === 'studio' || view === 'dashboard' || view === 'research' || view === 'specs' || view === 'rules'? (
                            <Suspense fallback={<div style={{ padding: 20, opacity: 0.5, fontSize: 11 }}>Loading Agent Studio…</div>}>
                                <AgentStudioPanel
                                    activeSubView={
                                        view === 'dashboard'? 'dashboard'
: view === 'research'? 'research'
: view === 'specs'? 'specs'
: view === 'rules'? 'rules'
: studioSubView
                                    }
                                    onSubViewChange={(sub) => {
                                        setStudioSubView(sub);
                                        if (view !== 'studio') setView('studio');
                                    }}
                                />
                            </Suspense>
                        ): null}
                    </div>
                )}
            </div>
            {
                view === 'chat' && agentUiMode === 'chat' && (
                    <div style={{ padding: '8px 10px 10px', borderTop: '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.1))', position: 'relative' }}>
                        {/* @mention dropdown — Cursor-style extracted component */}
                        <MentionPopup
                            inputValue={inputValue}
                            allFiles={allFiles}
                            isOpen={isMentionDropdownOpen}
                            selectedIndex={selectedMentionIndex}
                            onSelect={handleMentionSelect}
                            onSelectIndex={setSelectedMentionIndex}
                        />
                        <PlanApprovalBanner />
                        <RestoreCheckpointBanner />
                        {isContinuousMode && (
                            <div style={{
                                display: 'flex', alignItems: 'center', gap: 8, padding: '6px 10px', marginBottom: 6,
                                background: 'rgba(34,211,238,0.08)', border: '1px solid rgba(34,211,238,0.3)',
                                borderRadius: 8, fontSize: 11,
                            }}>
                                <i className="codicon codicon-loading~spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', color: '#22d3ee', fontSize: 12 }} />
                                <span style={{ flex: 1, color: 'rgba(255,255,255,0.85)' }}>
                                    <strong>Continuous Mode</strong> — agent is working autonomously until all tasks complete
                                </span>
                                <button
                                    onClick={() => setContinuousMode(false)}
                                    style={{ background: 'rgba(248,81,73,0.15)', border: '1px solid rgba(248,81,73,0.4)', color: '#f85149', padding: '2px 8px', fontSize: 10, fontWeight: 600, borderRadius: 4, cursor: 'pointer' }}
                                >
                                    Stop
                                </button>
                            </div>
                        )}
                        <MultiFileReviewBanner />
                        <BackgroundAgentsTray />
                        <ChatInput
                            inputRef={inputRef}
                            inputValue={inputValue}
                            isAgentThinking={isAgentThinking}
                            isSpecModeActive={isSpecModeActive}
                            attachedFiles={attachedFiles}
                            isAttaching={isAttaching}
                            onChange={handleInputChange}
                            onKeyDown={handleKeyDown}
                            onPaste={handlePaste}
                            onRemoveFile={removeFile}
                            toolbar={
                                <ChatToolbar
                                    mode={mode}
                                    model={model}
                                    modeStyle={modeStyle}
                                    modelLabel={modelLabel}
                                    ttsEnabled={ttsEnabled}
                                    ttsPreset={ttsPreset}
                                    isAgentThinking={isAgentThinking}
                                    onModeClick={onModeClick}
                                    onModelClick={onModelClick}
                                    onAttach={handleAttachFile}
                                    onToggleTts={() => setTtsEnabled(!ttsEnabled)}
                                    onTtsPresetChange={(p) => setTtsPreset(p as any)}
                                    onToggleVoice={toggleVoiceInput}
                                    isVoiceListening={isVoiceListening}
                                    reasoningToggle={<ReasoningToggle />}
                                    webUiControls={webUiProviderKey? (
                                        <div style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                                            <select
                                                value={activeWebuiSessionId || ''}
                                                onChange={async (e) => {
                                                    const val = e.target.value;
                                                    if (val === 'add_account') {
                                                        try {
                                                            const label = window.prompt('Account slot name', `free-${webuiSessions.filter(s => s.provider === webUiProviderKey).length + 1}`)?.trim();
                                                            const account = (label || 'default').replace(/[^a-zA-Z0-9_.-]/g, '_') || 'default';
                                                            await invoke('start_webui_login', { request: { provider: `${webUiProviderKey}:${account}` } });
                                                            await refreshWebuiSessions(webUiProviderKey);
                                                        } catch (err) {
                                                            console.error('Failed to trigger login:', err);
                                                        }
                                                    } else if (val) {
                                                        await switchWebuiSession(val);
                                                    }
                                                }}
                                                title="WebUI account switcher"
                                                style={{
                                                    height: '20px',
                                                    maxWidth: '120px',
                                                    background: 'rgba(255,255,255,0.04)',
                                                    border: '1px solid rgba(255,255,255,0.10)',
                                                    borderRadius: '4px',
                                                    color: 'rgba(255,255,255,0.72)',
                                                    fontSize: '10px',
                                                    outline: 'none',
                                                }}
                                            >
                                                {webuiSessions
                                                    .filter(s => s.provider === webUiProviderKey)
                                                    .map(s => (
                                                        <option key={s.session_id} value={s.session_id}>
                                                            {s.display_name} {s.is_active? '': ''}
                                                        </option>
                                                    ))
                                                }
                                                <option value="add_account">+ Add Account...</option>
                                            </select>
                                            {activeWebuiSessionId && (
                                                <div
                                                    onClick={async () => {
                                                        try {
                                                            await invoke('toggle_webui_window_visibility', { sessionId: activeWebuiSessionId });
                                                        } catch (err) {
                                                            console.error('Failed to toggle window:', err);
                                                        }
                                                    }}
                                                    style={{ cursor: 'pointer', opacity: 0.8, display: 'flex', alignItems: 'center', padding: '2px 4px', background: 'rgba(255,255,255,0.06)', borderRadius: '4px' }}
                                                    title="Show/Hide Session Window"
                                                >
                                                    <i className="codicon codicon-eye" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }}></i>
                                                </div>
                                            )}
                                            {activeWebuiSessionId && (
                                                <i
                                                    className="codicon codicon-trash"
                                                    onClick={async () => {
                                                        if (confirm('Delete this account session?')) {
                                                            await deleteWebuiSession(activeWebuiSessionId);
                                                        }
                                                    }}
                                                    style={{
                                                        fontFamily: 'codicon',
                                                        fontStyle: 'normal',
                                                        cursor: 'pointer',
                                                        opacity: 0.5,
                                                        fontSize: '11px',
                                                        padding: '2px',
                                                    }}
                                                    title="Delete session"
                                                />
                                            )}
                                        </div>
                                    ): undefined}
                                    onSend={() => onSend()}
                                    inputEmpty={!inputValue.trim() && attachedFiles.length === 0}
                                />
                            }
                        />
                        <div style={{ display: 'flex', gap: '10px', alignItems: 'center', marginTop: '6px', padding: '0 2px', flexWrap: 'wrap' }}>
                            <div
                                onClick={() => isAgentPaused? import('../agent').then(m => m.resumeAgent()): import('../agent').then(m => m.pauseAgent())}
                                style={{ cursor: 'pointer', color: isAgentPaused? '#10b981': '#f59e0b', display: 'flex', alignItems: 'center' }}
                                title={isAgentPaused? 'Resume Agent': 'Pause Agent'}
                            >
                                <i className={`codicon codicon-${isAgentPaused? 'play': 'debug-pause'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px' }} />
                            </div>
                            <div
                                onClick={() => import('../agent').then(m => m.stopAgent())}
                                style={{ cursor: 'pointer', color: '#ef4444', display: 'flex', alignItems: 'center' }}
                                title="Stop Agent"
                            >
                                <i className="codicon codicon-primitive-square" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px' }} />
                            </div>
                            <div
                                onClick={() => setAutoAcceptChanges(!autoAcceptChanges)}
                                style={{
                                    cursor: 'pointer', color: autoAcceptChanges? '#10b981': 'rgba(255,255,255,0.35)',
                                    display: 'flex', alignItems: 'center', gap: '3px', fontSize: '10px', fontWeight: 600,
                                }}
                                title={autoAcceptChanges? 'Auto-accept ON': 'Auto-accept OFF — review diffs first'}
                            >
                                <i className="codicon codicon-check-all" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px' }} />
                                <span>AUTO</span>
                            </div>
                            {checkpoint && (
                                <div onClick={() => revertToCheckpoint()} style={{ cursor: 'pointer', color: '#f59e0b', fontSize: '10px', fontWeight: 600 }} title="Restore checkpoint">
                                    <i className="codicon codicon-history" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px' }} /> UNDO
                                </div>
                            )}
                            <div
                                onClick={() => setAgentCleanUi(!agentCleanUi)}
                                style={{ cursor: 'pointer', color: agentCleanUi? '#60a5fa': 'rgba(255,255,255,0.35)', fontSize: '10px', fontWeight: 600 }}
                                title={agentCleanUi? 'Clean UI — Cursor-style tool log in chat': 'Verbose UI — show live tool feed in chat'}
                            >
                                CLEAN
                            </div>
                            <div
                                onClick={() => import('../agent').then(m => m.setYoloMode(!isYoloMode).then(() => setYoloMode(!isYoloMode)))}
                                style={{ cursor: 'pointer', color: isYoloMode? '#f97316': 'rgba(255,255,255,0.35)', fontSize: '10px', fontWeight: 600 }}
                                title={isYoloMode? 'YOLO ON': 'YOLO OFF'}
                            >
                                YOLO
                            </div>
                            <span style={{ fontSize: '9px', opacity: 0.35, fontVariantNumeric: 'tabular-nums', marginLeft: 'auto' }} title="Estimated context tokens">
                                ~{Math.round(messages.reduce((n, m) => n + (typeof m.content === 'string'? m.content.length: 0), 0) / 4).toLocaleString()}t
                            </span>
                        </div>
                    </div>
                )
            }

            
        </aside >
    );
};

export default RightSidebar;
