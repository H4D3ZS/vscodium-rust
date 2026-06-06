import React, { useEffect, useState, useCallback } from 'react';
import { invoke } from '../tauri_bridge';

// Community edition — local entitlements + Bug-Bounty ToS. Hosted billing lives on
// cyberifrit.xyz (Cyber-Ifrit Pro), not in this open-source build.

const BUG_BOUNTY_TOS_ID = 'bug-bounty';
const BUG_BOUNTY_TOS_VERSION = '1.0';
const PRO_PRICING_URL = 'https://cyberifrit.xyz/pricing';

const BUG_BOUNTY_TOS = `BUG BOUNTY / OFFENSIVE-SECURITY TERMS OF SERVICE (v1.0)

By enabling Bug Bounty / offensive-security features you agree that:

1. AUTHORIZATION. You will only test systems, applications, networks, or code that
   you OWN or for which you have EXPLICIT, WRITTEN authorization (e.g. an in-scope
   bug-bounty program, a signed engagement, or your own assets). You are solely
   responsible for confirming scope and authorization before any action.

2. LEGAL COMPLIANCE. You will comply with all applicable laws and regulations
   (including the CFAA, Computer Misuse Act, GDPR, and local equivalents) and with
   the rules/scope of any program you participate in.

3. NO UNAUTHORIZED USE. You will not use these tools to access, disrupt, exfiltrate
   from, or damage any system without authorization. Denial-of-service, mass
   targeting, and supply-chain compromise are prohibited.

4. RESPONSIBILITY & INDEMNITY. All actions you take are your responsibility. You
   agree to indemnify and hold harmless the IDE and its operators from any claim,
   loss, or liability arising from your use of these features.

5. THE TOOLS ARE PROVIDED "AS IS", without warranty. Findings may include false
   positives; you are responsible for verifying and for responsible disclosure.

Acceptance is recorded on your account with a timestamp for audit purposes.`;

interface AccountView {
    account: { id: string; email?: string | null; display_name?: string | null; tier: string; created_at: number; addons: { id: string; label: string }[] };
    tier_label: string;
    tier_price_usd: number;
    entitlements: { daily_requests: number; monthly_requests: number; features: string[] };
    status?: string;
    community_edition?: boolean;
}

const AccountSettingsPanel: React.FC = () => {
    const [data, setData] = useState<AccountView | null>(null);
    const [usage, setUsage] = useState<{ used_tokens: number; limit_tokens: number; used_month: number; limit_month: number } | null>(null);
    const [tosAccepted, setTosAccepted] = useState(false);
    const [showTos, setShowTos] = useState(false);

    const refresh = useCallback(() => {
        invoke<AccountView>('account_get').then((d) => {
            setData(d);
            window.dispatchEvent(new Event('account:changed'));
        }).catch(() => {});
        invoke<{ used_tokens: number; limit_tokens: number; used_month: number; limit_month: number }>('account_usage').then(setUsage).catch(() => {});
        invoke<boolean>('account_tos_status', { docId: BUG_BOUNTY_TOS_ID }).then(setTosAccepted).catch(() => {});
    }, []);
    useEffect(() => { refresh(); }, [refresh]);

    const acceptTos = () => {
        invoke('account_accept_tos', { docId: BUG_BOUNTY_TOS_ID, version: BUG_BOUNTY_TOS_VERSION })
            .then(() => { setShowTos(false); refresh(); }).catch(() => {});
    };
    const openPro = () => { invoke('account_open_billing').catch(() => window.open(PRO_PRICING_URL, '_blank')); };

    return (
        <div style={{ padding: '4px 4px 40px', color: 'var(--vscode-foreground)' }}>
            <h2 style={{ fontSize: 18, fontWeight: 600, margin: '0 0 4px' }}>Account</h2>
            <p style={{ fontSize: 12, opacity: 0.6, margin: '0 0 18px' }}>
                Community edition — local Ollama, agentic tools, and security features. Cloud AI requires Cyber-Ifrit Pro.
            </p>

            {/* Community plan */}
            {data && (
                <div style={{ marginBottom: 22, padding: '12px 16px', borderRadius: 10, border: '1px solid rgba(158,206,106,0.35)', background: 'linear-gradient(135deg, rgba(158,206,106,0.12), rgba(158,206,106,0.02))' }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
                        <i className="codicon codicon-package" style={{ fontSize: 18, color: '#9ece6a' }} />
                        <div style={{ flex: 1 }}>
                            <div style={{ fontSize: 14, fontWeight: 600 }}>{data.tier_label} · Free &amp; open source</div>
                            <div style={{ fontSize: 11, opacity: 0.55, marginTop: 2 }}>
                                Unlimited local AI via Ollama · full agentic + security tooling after ToS
                            </div>
                        </div>
                        <span style={{ fontSize: 10, padding: '3px 9px', borderRadius: 10, background: 'rgba(158,206,106,0.2)', color: '#9ece6a', fontWeight: 700, textTransform: 'uppercase' }}>MIT</span>
                    </div>
                    {usage && (
                        <div style={{ marginTop: 12, display: 'flex', flexDirection: 'column', gap: 10 }}>
                            <UsageBar label="Tokens this month (local meter)" used={usage.used_tokens} limit={usage.limit_tokens} fmt={fmtK} />
                            <UsageBar label="Requests this month (local meter)" used={usage.used_month} limit={usage.limit_month} fmt={(n) => n.toLocaleString()} />
                        </div>
                    )}
                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: 6, marginTop: 12 }}>
                        {data.entitlements.features.map((f) => (
                            <span key={f} style={{ fontSize: 10, padding: '2px 8px', borderRadius: 10, background: 'rgba(255,255,255,0.06)', opacity: 0.8 }}>{f}</span>
                        ))}
                    </div>
                </div>
            )}

            {/* Upgrade CTA */}
            <SectionLabel>Cyber-Ifrit Pro (hosted cloud)</SectionLabel>
            <div style={{ marginBottom: 22, padding: '16px', borderRadius: 10, border: '1px solid rgba(77,170,252,0.45)', background: 'linear-gradient(135deg, rgba(77,170,252,0.12), rgba(187,154,247,0.05))' }}>
                <div style={{ fontSize: 14, fontWeight: 700, marginBottom: 6 }}>Need cloud AI on AMD MI300X?</div>
                <div style={{ fontSize: 12, opacity: 0.7, marginBottom: 12, lineHeight: 1.5 }}>
                    Neural VFS compression, managed cloud models, trials, and QR Ph billing are hosted services — not part of this repo.
                    Subscribe at cyberifrit.xyz to unlock <b>ai.cyberifrit.xyz</b> from the IDE.
                </div>
                <button onClick={openPro} style={{ ...btnPrimary, padding: '9px 18px', fontSize: 13 }}>
                    View Cyber-Ifrit Pro plans →
                </button>
            </div>

            {/* Bug Bounty ToS */}
            <SectionLabel>Bug Bounty — Terms of Service</SectionLabel>
            <div style={{ padding: '14px 16px', borderRadius: 10, border: `1px solid ${tosAccepted ? 'rgba(158,206,106,0.35)' : 'rgba(247,118,142,0.4)'}`, background: 'var(--vscode-editorWidget-background, rgba(255,255,255,0.02))' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
                    <i className={`codicon ${tosAccepted ? 'codicon-verified-filled' : 'codicon-warning'}`} style={{ fontSize: 18, color: tosAccepted ? '#9ece6a' : '#f7768e' }} />
                    <div style={{ flex: 1 }}>
                        <div style={{ fontSize: 13, fontWeight: 600 }}>
                            {tosAccepted ? 'Accepted — Bug Bounty features unlocked' : 'Not accepted — required before using Bug Bounty'}
                        </div>
                        <div style={{ fontSize: 11, opacity: 0.55, marginTop: 2 }}>
                            Offensive-security tooling requires accepting authorized-use terms, recorded on your account.
                        </div>
                    </div>
                    <button onClick={() => setShowTos((s) => !s)} style={btnGhost}>
                        {showTos ? 'Hide' : 'Review'}
                    </button>
                </div>
                {showTos && (
                    <>
                        <pre style={{ marginTop: 12, padding: 14, borderRadius: 8, maxHeight: '40vh', overflow: 'auto', background: 'var(--vscode-textCodeBlock-background, rgba(0,0,0,0.25))', fontFamily: 'var(--font-mono)', fontSize: 11.5, lineHeight: 1.55, whiteSpace: 'pre-wrap' }}>{BUG_BOUNTY_TOS}</pre>
                        {!tosAccepted && (
                            <button onClick={acceptTos} style={{ marginTop: 10, padding: '8px 18px', borderRadius: 8, border: 'none', cursor: 'pointer', fontSize: 12, fontWeight: 700, background: '#f7768e', color: '#1a1a1a' }}>
                                I have read and accept these terms
                            </button>
                        )}
                    </>
                )}
            </div>
        </div>
    );
};

const btnPrimary: React.CSSProperties = {
    padding: '8px 14px', borderRadius: 8, border: 'none', cursor: 'pointer', fontSize: 12, fontWeight: 700,
    background: 'var(--vscode-button-background, #4daafc)', color: 'var(--vscode-button-foreground, #fff)',
};
const btnGhost: React.CSSProperties = {
    padding: '6px 12px', borderRadius: 6, border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.15))',
    background: 'transparent', color: 'inherit', cursor: 'pointer', fontSize: 12,
};

const fmtK = (n: number) => (n >= 1_000_000 ? `${(n / 1_000_000).toFixed(2)}M` : n >= 1000 ? `${(n / 1000).toFixed(1)}k` : `${n}`);

const UsageBar: React.FC<{ label: string; used: number; limit: number; fmt: (n: number) => string }> = ({ label, used, limit, fmt }) => {
    const unlimited = !limit || limit <= 0;
    const pct = unlimited ? 100 : Math.min(100, Math.round((used / limit) * 100));
    const color = unlimited ? '#9ece6a' : pct > 90 ? '#f7768e' : pct > 70 ? '#e0af68' : '#4daafc';
    return (
        <div>
            <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: 11, opacity: 0.75, marginBottom: 4 }}>
                <span>{label}</span>
                <span>{fmt(used)}{unlimited ? ' · ∞' : ` / ${fmt(limit)}`}</span>
            </div>
            <div style={{ height: 6, borderRadius: 4, background: 'rgba(255,255,255,0.08)', overflow: 'hidden' }}>
                <div style={{ width: `${pct}%`, height: '100%', background: color, transition: 'width 0.3s' }} />
            </div>
        </div>
    );
};

const SectionLabel: React.FC<{ children: React.ReactNode }> = ({ children }) => (
    <div style={{ fontSize: 11, textTransform: 'uppercase', letterSpacing: '0.06em', opacity: 0.5, fontWeight: 600, marginBottom: 10 }}>{children}</div>
);

export default AccountSettingsPanel;
