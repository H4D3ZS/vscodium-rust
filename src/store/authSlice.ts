import type { StateCreator } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { AppState } from './index';

// ─────────────────────────────────────────────────────────────────────────────
//  Auth / subscription slice.
//
//  Local inference (Lemonade / the local backend) is ALWAYS free and needs no login. Signing
//  in unlocks the powerful cloud models (GLM-5.2, Qwen3.6-35B-MoE, …) served via
//  the Cyber-Ifrit cloud gateway — for users whose GPU can't run them locally.
//
//  The returned token is a JWT that doubles as the `cyberifrit` provider key, so
//  cloud requests authenticate through the existing get_key_for_provider path.
// ─────────────────────────────────────────────────────────────────────────────

export interface SubscriptionAccount {
    email: string;
    tier: string;          // e.g. "free" | "pro" | "team"
    token: string;         // JWT — also stored as the cyberifrit provider key
    expiresAt: number | null; // epoch ms, null = no expiry
}

/** Cloud providers that require an active subscription session. */
export const CLOUD_SUBSCRIPTION_PROVIDERS = new Set(['cyberifrit', 'cyber-ifrit', 'cyberifrit-cloud']);

const DEFAULT_AUTH_URL = 'https://api.cyberifrit.xyz/auth';

export interface AuthSlice {
    account: SubscriptionAccount | null;
    authStatus: 'idle' | 'authenticating' | 'error';
    authError: string | null;
    authUrl: string;
    isLoginModalOpen: boolean;

    setAuthUrl: (url: string) => void;
    openLoginModal: () => void;
    closeLoginModal: () => void;
    login: (email: string, password: string) => Promise<boolean>;
    logout: () => Promise<void>;
    restoreSession: () => void;
    /** True when a non-expired subscription session exists. */
    isCloudUnlocked: () => boolean;
}

function readStoredAccount(): SubscriptionAccount | null {
    try {
        const raw = localStorage.getItem('subscription.account');
        if (!raw) return null;
        const acc = JSON.parse(raw) as SubscriptionAccount;
        if (acc?.expiresAt && acc.expiresAt < Date.now()) return null; // expired
        return acc && acc.token ? acc : null;
    } catch { return null; }
}

function persistAccount(acc: SubscriptionAccount | null): void {
    try {
        if (acc) localStorage.setItem('subscription.account', JSON.stringify(acc));
        else localStorage.removeItem('subscription.account');
    } catch { /* ignore */ }
}

export const createAuthSlice: StateCreator<AppState, [], [], AuthSlice> = (set, get) => ({
    account: readStoredAccount(),
    authStatus: 'idle',
    authError: null,
    authUrl: (() => { try { return localStorage.getItem('subscription.authUrl') || DEFAULT_AUTH_URL; } catch { return DEFAULT_AUTH_URL; } })(),
    isLoginModalOpen: false,

    setAuthUrl: (url) => {
        const u = url.trim() || DEFAULT_AUTH_URL;
        try { localStorage.setItem('subscription.authUrl', u); } catch { /* ignore */ }
        set({ authUrl: u });
    },
    openLoginModal: () => set({ isLoginModalOpen: true }),
    closeLoginModal: () => set({ isLoginModalOpen: false }),

    login: async (email, password) => {
        set({ authStatus: 'authenticating', authError: null });
        try {
            const mock = (() => { try { return localStorage.getItem('subscription.mockAuth') === 'true'; } catch { return false; } })();
            // Backend performs the POST (avoids webview CORS) AND persists the JWT as
            // the cyberifrit provider key at the path the engine actually reads.
            const resp = await invoke<{ token: string; tier?: string; email?: string; expires?: number }>('subscription_login', {
                authUrl: get().authUrl,
                email,
                password,
                mock,
            });
            if (!resp?.token) throw new Error('No token returned by auth server');

            const account: SubscriptionAccount = {
                email: resp.email || email,
                tier: resp.tier || 'pro',
                token: resp.token,
                expiresAt: resp.expires ?? null,
            };
            persistAccount(account);
            set({ account, authStatus: 'idle', authError: null, isLoginModalOpen: false });
            try { await get().refreshAvailableModels?.('cyberifrit'); } catch { /* non-fatal */ }
            return true;
        } catch (e: any) {
            const msg = typeof e === 'string' ? e : (e?.message ?? 'Sign-in failed');
            set({ authStatus: 'error', authError: msg });
            return false;
        }
    },

    logout: async () => {
        persistAccount(null);
        set({ account: null, authStatus: 'idle', authError: null });
        try { await invoke('subscription_logout'); } catch { /* non-fatal */ }
        try { await get().refreshAvailableModels?.('cyberifrit'); } catch { /* non-fatal */ }
    },

    restoreSession: () => {
        const acc = readStoredAccount();
        set({ account: acc });
        // The token persists server-side in api_keys.json; nothing else to restore.
    },

    isCloudUnlocked: () => {
        const acc = get().account;
        if (!acc?.token) return false;
        if (acc.expiresAt && acc.expiresAt < Date.now()) return false;
        return true;
    },
});
