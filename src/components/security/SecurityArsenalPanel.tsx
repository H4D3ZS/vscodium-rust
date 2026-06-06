import React, { useMemo, useState, useCallback, useRef } from 'react';
import { invoke } from '../../tauri_bridge';
import { CHEATSHEETS, searchCheatsheets } from '../../security/cheatsheets';

type ToolId = 'reverse_shell' | 'listener' | 'csp' | 'shellcode' | 'payload' | 'cheatsheet' | 'voice' | 'review';

const CARD: React.CSSProperties = {
    padding: '14px 16px',
    borderRadius: 10,
    border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.12))',
    background: 'var(--vscode-editorWidget-background, rgba(255,255,255,0.03))',
    cursor: 'pointer',
    textAlign: 'left',
    color: 'inherit',
};

const TOOLS: { id: ToolId; title: string; desc: string; icon: string; active: boolean }[] = [
    { id: 'reverse_shell', title: 'Reverse Shell', desc: '15+ languages · bash, python, ps1, php…', icon: 'codicon-terminal', active: true },
    { id: 'listener', title: 'Listener', desc: 'nc · ncat · socat · msf handler', icon: 'codicon-radio-tower', active: true },
    { id: 'csp', title: 'CSP Analyzer', desc: 'Parse policy · bypass research angles', icon: 'codicon-shield', active: true },
    { id: 'shellcode', title: 'Shellcode Recipe', desc: 'msfvenom commands · no raw bytes shipped', icon: 'codicon-circuit-board', active: true },
    { id: 'payload', title: 'Payload Encoder', desc: 'base64 · URL · hex · double-URL', icon: 'codicon-symbol-string', active: true },
    { id: 'cheatsheet', title: 'Cheatsheets', desc: 'AD · GTFOBins · LOLBins · WADComs (9 refs)', icon: 'codicon-book', active: true },
    { id: 'voice', title: 'Voice Query', desc: 'Hands-free security research · speech in', icon: 'codicon-mic', active: true },
    { id: 'review', title: 'Codebase Review', desc: 'ai_vuln_hunt + static audit (sidebar)', icon: 'codicon-search', active: true },
];

const LANGUAGES = ['bash', 'python', 'powershell', 'php', 'ruby', 'nc', 'ncat', 'node', 'go', 'rust', 'java', 'csharp', 'perl', 'awk', 'lua', 'openssl'];

export const SecurityArsenalPanel: React.FC<{ onOpenReview?: () => void }> = ({ onOpenReview }) => {
    const [tool, setTool] = useState<ToolId | null>(null);
    const [host, setHost] = useState('10.10.14.1');
    const [port, setPort] = useState('4444');
    const [lang, setLang] = useState('bash');
    const [listenerKind, setListenerKind] = useState('nc');
    const [cspHeader, setCspHeader] = useState('');
    const [plat, setPlat] = useState('windows');
    const [arch, setArch] = useState('x64');
    const [rawPayload, setRawPayload] = useState('whoami');
    const [encoding, setEncoding] = useState('base64');
    const [sheetQuery, setSheetQuery] = useState('');
    const [output, setOutput] = useState('');
    const [err, setErr] = useState('');
    const [busy, setBusy] = useState(false);
    const [voiceText, setVoiceText] = useState('');
    const [listening, setListening] = useState(false);
    const recognitionRef = useRef<any>(null);

    const sheets = useMemo(() => searchCheatsheets(sheetQuery), [sheetQuery]);

    const startVoice = useCallback(() => {
        const SR = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
        if (!SR) { setErr('Speech recognition not supported in this browser/webview.'); return; }
        if (listening && recognitionRef.current) { recognitionRef.current.stop(); setListening(false); return; }
        const rec = new SR();
        rec.continuous = false;
        rec.interimResults = true;
        rec.lang = 'en-US';
        rec.onresult = (ev: any) => {
            const t = Array.from(ev.results).map((r: any) => r[0].transcript).join('');
            setVoiceText(t);
        };
        rec.onend = () => setListening(false);
        rec.onerror = () => setListening(false);
        recognitionRef.current = rec;
        setListening(true);
        setErr('');
        rec.start();
    }, [listening]);

    const run = async (fn: () => Promise<void>) => {
        setErr(''); setBusy(true);
        try { await fn(); } catch (e) { setErr(String(e)); } finally { setBusy(false); }
    };

    const genReverse = () => run(async () => {
        const r = await invoke<any>('security_reverse_shell', { language: lang, host, port: parseInt(port, 10) || 4444, shell: null });
        setOutput(r.payload || JSON.stringify(r, null, 2));
    });

    const genListener = () => run(async () => {
        const r = await invoke<any>('security_listener', { kind: listenerKind, host: '0.0.0.0', port: parseInt(port, 10) || 4444 });
        setOutput(r.command || JSON.stringify(r, null, 2));
    });

    const genCsp = () => run(async () => {
        const r = await invoke<any>('security_csp_analyze', { header: cspHeader });
        setOutput(JSON.stringify(r, null, 2));
    });

    const genShellcode = () => run(async () => {
        const r = await invoke<any>('security_shellcode_recipe', { platform: plat, arch, payload: 'shell_reverse_tcp' });
        setOutput(JSON.stringify(r, null, 2));
    });

    const genEncode = () => run(async () => {
        const r = await invoke<any>('security_encode_payload', { payload: rawPayload, encoding });
        setOutput(r.output || JSON.stringify(r, null, 2));
    });

    const openTool = (id: ToolId) => {
        if (id === 'review') { onOpenReview?.(); return; }
        setTool(id); setOutput(''); setErr('');
    };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', overflow: 'hidden', fontSize: 12 }}>
            <div style={{ padding: '10px 12px', borderBottom: '1px solid var(--vscode-panel-border)', flexShrink: 0 }}>
                <div style={{ fontWeight: 700, fontSize: 14 }}>Security Arsenal</div>
                <div style={{ opacity: 0.55, marginTop: 4, lineHeight: 1.45 }}>
                    Obsidian Labs–style toolkit built into Cyber-Ifrit IDE. Requires Bug Bounty ToS + Security Researcher tier.
                </div>
            </div>

            {!tool ? (
                <div style={{ padding: 12, overflow: 'auto', flex: 1 }}>
                    <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(200px, 1fr))', gap: 10 }}>
                        {TOOLS.map((t) => (
                            <button key={t.id} type="button" style={CARD} onClick={() => openTool(t.id)}>
                                <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 6 }}>
                                    <i className={`codicon ${t.icon}`} style={{ fontSize: 18, color: '#9ece6a' }} />
                                    <span style={{ fontWeight: 700 }}>{t.title}</span>
                                    <span style={{ marginLeft: 'auto', fontSize: 9, padding: '2px 6px', borderRadius: 8, background: 'rgba(158,206,106,0.2)', color: '#9ece6a' }}>ACTIVE</span>
                                </div>
                                <div style={{ opacity: 0.6, fontSize: 11 }}>{t.desc}</div>
                            </button>
                        ))}
                    </div>
                    <div style={{ marginTop: 16, padding: 12, borderRadius: 8, border: '1px solid rgba(77,170,252,0.35)', background: 'rgba(77,170,252,0.06)' }}>
                        <div style={{ fontWeight: 600, marginBottom: 6 }}>Pricing vs Obsidian Labs</div>
                        <div style={{ opacity: 0.75, lineHeight: 1.5 }}>
                            Obsidian: Starter $10 · Pro $25 · Enterprise $60 + API ([obsidianlabs.cloud](https://obsidianlabs.cloud/pricing)).
                            Cyber-Ifrit: Community (local) free · Pro $30 · Security $75 (this arsenal + vuln-hunt) · Enterprise $225 + REST API.
                        </div>
                    </div>
                    <div style={{ marginTop: 10, opacity: 0.45, fontSize: 10 }}>
                        Enterprise: REST API at cyberifrit.xyz/docs/api · Voice uses browser speech (no cloud upload).
                    </div>
                </div>
            ) : (
                <div style={{ flex: 1, overflow: 'auto', padding: 12 }}>
                    <button type="button" onClick={() => setTool(null)} style={{ ...CARD, cursor: 'pointer', marginBottom: 12, padding: '6px 10px' }}>
                        ← All tools
                    </button>

                    {tool === 'reverse_shell' && (
                        <>
                            <label>Language</label>
                            <select value={lang} onChange={(e) => setLang(e.target.value)} style={{ width: '100%', marginBottom: 8 }}>
                                {LANGUAGES.map((l) => <option key={l} value={l}>{l}</option>)}
                            </select>
                            <div style={{ display: 'flex', gap: 8, marginBottom: 8 }}>
                                <input placeholder="LHOST" value={host} onChange={(e) => setHost(e.target.value)} style={{ flex: 2 }} />
                                <input placeholder="LPORT" value={port} onChange={(e) => setPort(e.target.value)} style={{ flex: 1 }} />
                            </div>
                            <button type="button" disabled={busy} onClick={genReverse} style={{ marginBottom: 12 }}>Generate</button>
                        </>
                    )}
                    {tool === 'listener' && (
                        <>
                            <select value={listenerKind} onChange={(e) => setListenerKind(e.target.value)} style={{ width: '100%', marginBottom: 8 }}>
                                {['nc', 'ncat', 'socat_tcp', 'socat_udp', 'msf', 'pwncat'].map((k) => <option key={k} value={k}>{k}</option>)}
                            </select>
                            <input placeholder="LPORT" value={port} onChange={(e) => setPort(e.target.value)} style={{ width: '100%', marginBottom: 8 }} />
                            <button type="button" disabled={busy} onClick={genListener} style={{ marginBottom: 12 }}>Generate</button>
                        </>
                    )}
                    {tool === 'csp' && (
                        <>
                            <textarea placeholder="Content-Security-Policy header value…" value={cspHeader} onChange={(e) => setCspHeader(e.target.value)} rows={4} style={{ width: '100%', marginBottom: 8 }} />
                            <button type="button" disabled={busy} onClick={genCsp} style={{ marginBottom: 12 }}>Analyze</button>
                        </>
                    )}
                    {tool === 'shellcode' && (
                        <>
                            <div style={{ display: 'flex', gap: 8, marginBottom: 8 }}>
                                <select value={plat} onChange={(e) => setPlat(e.target.value)} style={{ flex: 1 }}>
                                    {['windows', 'linux', 'osx'].map((p) => <option key={p} value={p}>{p}</option>)}
                                </select>
                                <select value={arch} onChange={(e) => setArch(e.target.value)} style={{ flex: 1 }}>
                                    {['x64', 'x86', 'elf', 'macho'].map((a) => <option key={a} value={a}>{a}</option>)}
                                </select>
                            </div>
                            <button type="button" disabled={busy} onClick={genShellcode} style={{ marginBottom: 12 }}>Get recipe</button>
                        </>
                    )}
                    {tool === 'payload' && (
                        <>
                            <textarea value={rawPayload} onChange={(e) => setRawPayload(e.target.value)} rows={3} style={{ width: '100%', marginBottom: 8 }} />
                            <select value={encoding} onChange={(e) => setEncoding(e.target.value)} style={{ width: '100%', marginBottom: 8 }}>
                                {['base64', 'url', 'hex', 'double_url'].map((e) => <option key={e} value={e}>{e}</option>)}
                            </select>
                            <button type="button" disabled={busy} onClick={genEncode} style={{ marginBottom: 12 }}>Encode</button>
                        </>
                    )}
                    {tool === 'cheatsheet' && (
                        <>
                            <input placeholder="Search AD, GTFOBins, LOLBins…" value={sheetQuery} onChange={(e) => setSheetQuery(e.target.value)} style={{ width: '100%', marginBottom: 8 }} />
                            {sheets.map((s) => (
                                <details key={s.id} style={{ marginBottom: 8 }}>
                                    <summary style={{ cursor: 'pointer', fontWeight: 600 }}>{s.title}</summary>
                                    <pre style={{ whiteSpace: 'pre-wrap', fontSize: 11, padding: 8, background: 'rgba(0,0,0,0.25)', borderRadius: 6 }}>{s.body}</pre>
                                </details>
                            ))}
                        </>
                    )}
                    {tool === 'voice' && (
                        <>
                            <p style={{ opacity: 0.65, marginBottom: 8, lineHeight: 1.45 }}>
                                Speak a security query — transcript appears below. Copy into the agent chat for hands-free research.
                            </p>
                            <button type="button" disabled={busy} onClick={startVoice} style={{ marginBottom: 8 }}>
                                {listening ? 'Stop listening' : 'Start microphone'}
                            </button>
                            <textarea value={voiceText} onChange={(e) => setVoiceText(e.target.value)} rows={4} placeholder="Transcript…" style={{ width: '100%', marginBottom: 8 }} />
                        </>
                    )}

                    {err && <div style={{ color: '#f7768e', marginBottom: 8 }}>{err}</div>}
                    {output && (
                        <pre style={{ whiteSpace: 'pre-wrap', wordBreak: 'break-all', padding: 12, borderRadius: 8, background: 'var(--vscode-textCodeBlock-background, rgba(0,0,0,0.3))', fontSize: 11 }}>
                            {output}
                        </pre>
                    )}
                </div>
            )}
        </div>
    );
};

export default SecurityArsenalPanel;
