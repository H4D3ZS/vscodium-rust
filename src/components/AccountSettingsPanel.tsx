import React, { useEffect, useState, useCallback } from 'react';
import { invoke } from '../tauri_bridge';

// Account & Terms — subscription tier, entitlements, the Bug-Bounty Terms of
// Service (acceptance is recorded on the account, backend `account.rs`), and the
// MiMo pre-model promo. Local-first today; the same commands back onto a real
// billing/auth backend later with no UI change.

const BUG_BOUNTY_TOS_ID = 'bug-bounty';
const BUG_BOUNTY_TOS_VERSION = '1.0';

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
}

const TIERS = [
    { id: 'community', label: 'Community', price: 'Free', quota: '50 / day', accent: '#8a8a8a' },
    { id: 'pro', label: 'Pro Developer', price: '$30/mo', quota: '5,000 / mo', accent: '#4daafc' },
    { id: 'security', label: 'Security Researcher', price: '$75/mo', quota: '~20K / mo', accent: '#f7768e' },
    { id: 'enterprise', label: 'Enterprise', price: '$225/mo', quota: 'Custom', accent: '#bb9af7' },
];

const AccountSettingsPanel: React.FC = () => {
    const [data, setData] = useState<AccountView | null>(null);
    const [tosAccepted, setTosAccepted] = useState(false);
    const [showTos, setShowTos] = useState(false);
    const [hasMimo, setHasMimo] = useState(false);

    const refresh = useCallback(() => {
        invoke<AccountView>('account_get').then((d) => {
            setData(d);
            setHasMimo((d.account.addons || []).some((a) => a.id === 'mimo_pro'));
        }).catch(() => {});
        invoke<boolean>('account_tos_status', { docId: BUG_BOUNTY_TOS_ID }).then(setTosAccepted).catch(() => {});
    }, []);
    useEffect(() => { refresh(); }, [refresh]);

    const acceptTos = () => {
        invoke('account_accept_tos', { docId: BUG_BOUNTY_TOS_ID, version: BUG_BOUNTY_TOS_VERSION })
            .then(() => { setShowTos(false); refresh(); }).catch(() => {});
    };
    const setTier = (id: string) => { invoke('account_set_tier', { tier: id }).then(refresh).catch(() => {}); };
    const acquireMimo = () => { invoke('account_acquire_addon', { id: 'mimo_pro', label: 'MiMo Pro model' }).then(refresh).catch(() => {}); };

    return (
        <div style={{ padding: '4px 4px 40px', color: 'var(--vscode-foreground)' }}>
            <h2 style={{ fontSize: 18, fontWeight: 600, margin: '0 0 4px' }}>Account &amp; Subscription</h2>
            <p style={{ fontSize: 12, opacity: 0.6, margin: '0 0 18px' }}>
                Your plan, entitlements, the Bug-Bounty Terms of Service, and add-ons.
            </p>

            {/* Current plan */}
            {data && (
                <div style={{ marginBottom: 22, padding: '12px 16px', borderRadius: 10, border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.1))', background: 'var(--vscode-editorWidget-background, rgba(255,255,255,0.02))' }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
                        <i className="codicon codicon-account" style={{ fontSize: 18, opacity: 0.8 }} />
                        <div>
                            <div style={{ fontSize: 14, fontWeight: 600 }}>{data.tier_label} {data.tier_price_usd > 0 ? `· $${data.tier_price_usd}/mo` : '· Free'}</div>
                            <div style={{ fontSize: 11, opacity: 0.55 }}>
                                {data.entitlements.daily_requests > 0 ? `${data.entitlements.daily_requests} requests/day` : `${data.entitlements.monthly_requests || 'Custom'} requests/mo`}
                            </div>
                        </div>
                    </div>
                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: 6, marginTop: 10 }}>
                        {data.entitlements.features.map((f) => (
                            <span key={f} style={{ fontSize: 10, padding: '2px 8px', borderRadius: 10, background: 'rgba(255,255,255,0.06)', opacity: 0.8 }}>{f}</span>
                        ))}
                    </div>
                </div>
            )}

            {/* Plan picker (dev/local — billing drives this in production) */}
            <SectionLabel>Plans</SectionLabel>
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(160px, 1fr))', gap: 10, marginBottom: 24 }}>
                {TIERS.map((t) => {
                    const active = data?.account.tier === (t.id === 'pro' ? 'pro_developer' : t.id === 'security' ? 'security_researcher' : t.id);
                    return (
                        <div key={t.id} onClick={() => setTier(t.id)} style={{
                            cursor: 'pointer', padding: '12px 14px', borderRadius: 8,
                            border: `1px solid ${active ? t.accent : 'var(--vscode-panel-border, rgba(255,255,255,0.12))'}`,
                            background: active ? `${t.accent}22` : 'var(--vscode-editorWidget-background, rgba(255,255,255,0.02))',
                        }}>
                            <div style={{ fontSize: 13, fontWeight: 700, color: t.accent }}>{t.label}</div>
                            <div style={{ fontSize: 18, fontWeight: 700, margin: '4px 0' }}>{t.price}</div>
                            <div style={{ fontSize: 11, opacity: 0.55 }}>{t.quota} AI requests</div>
                        </div>
                    );
                })}
            </div>

            {/* MiMo first-time offer */}
            <SectionLabel>Add-ons</SectionLabel>
            <div style={{ marginBottom: 24, padding: '14px 16px', borderRadius: 10, border: '1px solid rgba(224,175,104,0.4)', background: 'linear-gradient(135deg, rgba(224,175,104,0.12), rgba(224,175,104,0.03))', position: 'relative', overflow: 'hidden' }}>
                <span style={{ position: 'absolute', top: 10, right: 12, fontSize: 9, fontWeight: 800, letterSpacing: '0.06em', color: '#1a1a1a', background: '#e0af68', padding: '2px 8px', borderRadius: 10 }}>
                    TODAY ONLY · FIRST-TIME OFFER
                </span>
                <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
                    <i className="codicon codicon-sparkle" style={{ fontSize: 20, color: '#e0af68' }} />
                    <div>
                        <div style={{ fontSize: 14, fontWeight: 600 }}>MiMo Pro model — <span style={{ color: '#e0af68' }}>$10</span></div>
                        <div style={{ fontSize: 11, opacity: 0.6, marginTop: 2 }}>Xiaomi MiMo (v2.5-pro) reasoning model. One-time unlock.</div>
                    </div>
                    <button
                        onClick={acquireMimo}
                        disabled={hasMimo}
                        style={{
                            marginLeft: 'auto', padding: '7px 16px', borderRadius: 8, border: 'none', cursor: hasMimo ? 'default' : 'pointer',
                            fontSize: 12, fontWeight: 700,
                            background: hasMimo ? 'rgba(158,206,106,0.2)' : '#e0af68', color: hasMimo ? '#9ece6a' : '#1a1a1a',
                        }}
                    >
                        {hasMimo ? '✓ Unlocked' : 'Get for $10'}
                    </button>
                </div>
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
                    <button onClick={() => setShowTos((s) => !s)} style={{ padding: '6px 12px', borderRadius: 6, border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.15))', background: 'transparent', color: 'inherit', cursor: 'pointer', fontSize: 12 }}>
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

const SectionLabel: React.FC<{ children: React.ReactNode }> = ({ children }) => (
    <div style={{ fontSize: 11, textTransform: 'uppercase', letterSpacing: '0.06em', opacity: 0.5, fontWeight: 600, marginBottom: 10 }}>{children}</div>
);

export default AccountSettingsPanel;
