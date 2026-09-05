// Selector hooks (ARCHITECTURE.md): components consume these
// instead of raw useStore selectors. Multi-field selections go through
// useShallow so a re-render only happens when a selected field actually
// changes — the store has 350+ subscriptions and most returned fresh objects.

import { useShallow } from 'zustand/react/shallow';
import { useStore } from '../store';

/** Agent chat transcript + run state. */
export function useAgentMessages() {
    return useStore(
        useShallow((s) => ({
            messages: s.agentMessages,
        })),
    );
}

/** Currently focused editor tab/file. */
export function useActiveTab() {
    return useStore(
        useShallow((s) => ({
            activeTabId: s.activeTabId,
            tabs: s.tabs,
        })),
    );
}

/** Workbench layout: panels, sidebar, layout mode. */
export function useLayout() {
    return useStore(
        useShallow((s) => ({
            layoutMode: s.layoutMode,
            isSidebarOpen: s.isSidebarOpen,
            isRightSidebarOpen: s.isRightSidebarOpen,
            isBottomPanelOpen: s.isBottomPanelOpen,
        })),
    );
}

/** Theme + appearance settings. */
export function useAppearance() {
    return useStore(
        useShallow((s) => ({
            theme: s.theme,
        })),
    );
}
