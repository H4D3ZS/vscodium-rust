import React, { useState, useEffect } from 'react';
import { useStore } from '../../store';

/**
 * Lemonade server settings — styled to match the the local backend Integration panel
 * (gradient cards, codicon icons, status pill, monospace endpoints).
 *
 * All server I/O goes through the Rust `list_provider_models('lemonade')` command
 * so it attaches the signed-in cloud JWT, normalizes the base (no `/v1/v1`), and
 * bypasses browser CORS for a JWT-gated cloud Lemonade.
 */
const ACCENT = '#f59e0b'; // lemonade amber

const card: React.CSSProperties = {
    padding: '14px',
    borderRadius: '10px',
    background: 'linear-gradient(180deg, rgba(255,255,255,0.04) 0%, rgba(0,0,0,0.15) 100%)',
    border: '1px solid rgba(255,255,255,0.08)',
};

const inputStyle: React.CSSProperties = {
    flex: 1,
    fontSize: 12,
    padding: '7px 10px',
    background: 'rgba(0,0,0,0.25)',
    color: 'var(--vscode-input-foreground)',
    border: '1px solid rgba(255,255,255,0.12)',
    borderRadius: 8,
    outline: 'none',
};

const Icon: React.FC<{ name: string; size?: number; color?: string; style?: React.CSSProperties }> = ({ name, size = 14, color, style }) => (
    <i className={`codicon codicon-${name}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: size, color, ...style }} />
);

const LemonadeSettingsPanel: React.FC = () => {
    const lemonadeUrl = useStore((s) => s.lemonadeUrl);
    const setLemonadeUrl = useStore((s) => s.setLemonadeUrl);
    const lemonadeStatus = useStore((s) => s.lemonadeStatus);
    const checkLemonade = useStore((s) => s.checkLemonadeStatus);
    const [models, setModels] = useState<string[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [pullModel, setPullModel] = useState('');
    const [pulling, setPulling] = useState(false);

    useEffect(() => {
        void checkLemonade();
    }, [lemonadeUrl]);

    const fetchModels = async () => {
        setLoading(true);
        setError(null);
        try {
            // Route through Rust: attaches the signed-in cloud JWT, normalizes the
            // base (no /v1/v1), bypasses browser CORS. A direct fetch 401s on the
            // JWT-gated cloud proxy and can't read the token.
            const { invoke } = await import('../../tauri_bridge');
            await invoke('set_lemonade_url', { url: lemonadeUrl }).catch(() => { });
            const modelList = await invoke<string[]>('list_provider_models', { provider: 'lemonade' });
            setModels(Array.isArray(modelList) ? modelList : []);
            if (!modelList?.length) setError('No models returned (check sign-in / server).');
        } catch (e: any) {
            setError(e?.message || String(e));
            setModels([]);
        } finally {
            setLoading(false);
        }
    };

    const handlePullModel = async () => {
        if (!pullModel.trim()) return;
        setPulling(true);
        setError(null);
        try {
            // Route through Rust (JWT + CORS), same as fetchModels above.
            const { invoke } = await import('../../tauri_bridge');
            await invoke('set_lemonade_url', { url: lemonadeUrl }).catch(() => { });
            await invoke('pull_lemonade_model', { name: pullModel.trim() });
            setPullModel('');
            await fetchModels();
        } catch (e: any) {
            setError(`Pull failed: ${e?.message || String(e)} — note: cloud servers manage their own models.`);
        } finally {
            setPulling(false);
        }
    };

    const isCloud = /^https:\/\//i.test(lemonadeUrl) && !/127\.0\.0\.1|localhost/i.test(lemonadeUrl);
    const running = lemonadeStatus === 'running';
    const statusMeta = running
        ? { color: '#9ece6a', icon: 'pass-filled', label: 'Connected' }
        : lemonadeStatus === 'error'
            ? { color: '#f7768e', icon: 'error', label: 'Unreachable' }
            : lemonadeStatus === 'checking'
                ? { color: ACCENT, icon: 'sync~spin', label: 'Checking…' }
                : { color: '#888', icon: 'circle-outline', label: 'Idle' };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 16, maxWidth: 680 }}>
            {/* Header */}
            <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
                <div style={{
                    width: 30, height: 30, borderRadius: 8, display: 'flex', alignItems: 'center', justifyContent: 'center',
                    background: `linear-gradient(135deg, ${ACCENT}, #fbbf24)`,
                }}>
                    <Icon name="zap" size={16} color="#1a1205" />
                </div>
                <div>
                    <div style={{ fontSize: 13, fontWeight: 700 }}>Lemonade Server</div>
                    <div style={{ fontSize: 10.5, opacity: 0.6 }}>llama.cpp · NVIDIA/ROCm GPU · OpenAI-compatible</div>
                </div>
                <div style={{ flex: 1 }} />
                <div style={{
                    display: 'flex', alignItems: 'center', gap: 6, padding: '4px 10px', borderRadius: 999,
                    background: `${statusMeta.color}1a`, border: `1px solid ${statusMeta.color}55`, fontSize: 11, fontWeight: 600, color: statusMeta.color,
                }}>
                    <Icon name={statusMeta.icon} size={12} />
                    {statusMeta.label}
                </div>
            </div>

            {/* Server URL card */}
            <div style={card}>
                <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginBottom: 8 }}>
                    <Icon name={isCloud ? 'cloud' : 'home'} size={13} color={ACCENT} />
                    <span style={{ fontSize: 11, fontWeight: 700, textTransform: 'uppercase', letterSpacing: 0.4 }}>
                        {isCloud ? 'Cloud Endpoint' : 'Local Server'}
                    </span>
                </div>
                <div style={{ display: 'flex', gap: 8 }}>
                    <input
                        value={lemonadeUrl}
                        onChange={(e) => setLemonadeUrl(e.target.value)}
                        placeholder="http://127.0.0.1:13305"
                        style={inputStyle}
                    />
                    <button type="button" className="settings-button" onClick={() => void checkLemonade()}
                        style={{ display: 'flex', alignItems: 'center', gap: 6, whiteSpace: 'nowrap' }}>
                        <Icon name="plug" size={13} /> Check
                    </button>
                </div>
                {isCloud && (
                    <p style={{ margin: '8px 0 0', fontSize: 10.5, opacity: 0.6, lineHeight: 1.45 }}>
                        Behind the Cyber-Ifrit JWT gate — sign in (Settings → Account) so your token is sent automatically.
                    </p>
                )}
            </div>

            {/* Installed models card */}
            <div style={card}>
                <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 10 }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                        <Icon name="layers" size={13} color={ACCENT} />
                        <span style={{ fontSize: 11, fontWeight: 700, textTransform: 'uppercase', letterSpacing: 0.4 }}>
                            Installed Models
                        </span>
                        {models.length > 0 && (
                            <span style={{ fontSize: 10, opacity: 0.5 }}>({models.length})</span>
                        )}
                    </div>
                    <button type="button" className="settings-button" onClick={() => void fetchModels()} disabled={loading}
                        style={{ display: 'flex', alignItems: 'center', gap: 6, fontSize: 11, padding: '3px 10px' }}>
                        <Icon name={loading ? 'sync~spin' : 'refresh'} size={12} /> {loading ? 'Loading' : 'Refresh'}
                    </button>
                </div>

                {error && (
                    <div style={{
                        display: 'flex', gap: 8, padding: '8px 10px', marginBottom: 8, borderRadius: 8,
                        background: '#f7768e18', border: '1px solid #f7768e44', color: '#fca5a5', fontSize: 11, lineHeight: 1.4,
                    }}>
                        <Icon name="warning" size={13} style={{ marginTop: 1, flexShrink: 0 }} />
                        <span>{error}</span>
                    </div>
                )}

                {models.length > 0 ? (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: 4, maxHeight: 220, overflowY: 'auto' }}>
                        {models.map((m) => (
                            <div key={m} style={{
                                display: 'flex', alignItems: 'center', gap: 8, padding: '7px 10px', borderRadius: 7,
                                background: 'rgba(255,255,255,0.03)', border: '1px solid rgba(255,255,255,0.06)', fontSize: 12,
                            }}>
                                <Icon name="circle-filled" size={8} color="#9ece6a" />
                                <span style={{ fontFamily: 'monospace' }}>{m}</span>
                            </div>
                        ))}
                    </div>
                ) : (
                    <div style={{
                        display: 'flex', alignItems: 'center', justifyContent: 'center', gap: 8, padding: '22px 8px',
                        fontSize: 11, opacity: 0.45, textAlign: 'center',
                    }}>
                        <Icon name="inbox" size={14} />
                        {loading ? 'Fetching models…' : 'No models. Click Refresh, or pull one below.'}
                    </div>
                )}
            </div>

            {/* Pull model card */}
            <div style={card}>
                <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginBottom: 8 }}>
                    <Icon name="cloud-download" size={13} color={ACCENT} />
                    <span style={{ fontSize: 11, fontWeight: 700, textTransform: 'uppercase', letterSpacing: 0.4 }}>
                        Pull New Model
                    </span>
                </div>
                <div style={{ display: 'flex', gap: 8 }}>
                    <input
                        value={pullModel}
                        onChange={(e) => setPullModel(e.target.value)}
                        placeholder="e.g. Qwen2.5-Coder-7B-Instruct-GGUF"
                        onKeyDown={(e) => { if (e.key === 'Enter') void handlePullModel(); }}
                        style={inputStyle}
                    />
                    <button type="button" className="settings-button" onClick={() => void handlePullModel()}
                        disabled={pulling || !pullModel.trim()}
                        style={{ display: 'flex', alignItems: 'center', gap: 6, whiteSpace: 'nowrap' }}>
                        <Icon name={pulling ? 'sync~spin' : 'cloud-download'} size={13} /> {pulling ? 'Pulling' : 'Pull'}
                    </button>
                </div>
                <p style={{ margin: '8px 0 0', fontSize: 10.5, opacity: 0.55, lineHeight: 1.45 }}>
                    Auto-optimized for your GPU (NVIDIA/ROCm), downloaded from HuggingFace. Cloud servers manage their own catalog.
                </p>
            </div>

            {/* Features card */}
            <div style={{ ...card, opacity: 0.85 }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginBottom: 8 }}>
                    <Icon name="sparkle" size={13} color={ACCENT} />
                    <span style={{ fontSize: 11, fontWeight: 700, textTransform: 'uppercase', letterSpacing: 0.4 }}>Features</span>
                </div>
                <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '6px 14px' }}>
                    {[
                        ['plug', 'OpenAI-compatible API'],
                        ['chip', 'CUDA & ROCm acceleration'],
                        ['search', 'Auto-detects models'],
                        ['symbol-string', 'Proper tokenization'],
                        ['device-camera', 'Multi-modal: vision/TTS'],
                        ['code', 'Agentic coding ready'],
                    ].map(([icon, label]) => (
                        <div key={label} style={{ display: 'flex', alignItems: 'center', gap: 7, fontSize: 11, opacity: 0.8 }}>
                            <Icon name={icon} size={12} color={ACCENT} /> {label}
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
};

export default LemonadeSettingsPanel;
