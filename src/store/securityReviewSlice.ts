import type { StateCreator } from 'zustand';
import type { AppState } from './index';
import type { SecurityAuditReport } from '../domain/security/SecurityAuditReport';
import type { ReviewPhase } from '../application/security/runCodebaseSecurityReview';

export interface SecurityReviewSlice {
    securityReviewReport: SecurityAuditReport | null;
    securityReviewRunning: boolean;
    securityReviewPhase: ReviewPhase;
    securityReviewError: string | null;
    securityReviewPanelOpen: boolean;
    securityReviewDepth: 'standard' | 'deep';

    setSecurityReviewReport: (report: SecurityAuditReport | null) => void;
    setSecurityReviewRunning: (running: boolean) => void;
    setSecurityReviewPhase: (phase: ReviewPhase) => void;
    setSecurityReviewError: (error: string | null) => void;
    setSecurityReviewPanelOpen: (open: boolean) => void;
    setSecurityReviewDepth: (depth: 'standard' | 'deep') => void;
    clearSecurityReview: () => void;
}

export const createSecurityReviewSlice: StateCreator<AppState, [], [], SecurityReviewSlice> = (set) => ({
    securityReviewReport: null,
    securityReviewRunning: false,
    securityReviewPhase: 'idle',
    securityReviewError: null,
    securityReviewPanelOpen: false,
    securityReviewDepth: 'deep',

    setSecurityReviewReport: (securityReviewReport) => set({ securityReviewReport }),
    setSecurityReviewRunning: (securityReviewRunning) => set({ securityReviewRunning }),
    setSecurityReviewPhase: (securityReviewPhase) => set({ securityReviewPhase }),
    setSecurityReviewError: (securityReviewError) => set({ securityReviewError }),
    setSecurityReviewPanelOpen: (securityReviewPanelOpen) => set({ securityReviewPanelOpen }),
    setSecurityReviewDepth: (securityReviewDepth) => set({ securityReviewDepth }),
    clearSecurityReview: () => set({
        securityReviewReport: null,
        securityReviewPhase: 'idle',
        securityReviewError: null,
    }),
});
