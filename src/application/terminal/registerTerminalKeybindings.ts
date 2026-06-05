import type { TerminalManager } from '../../terminal';
import { useStore } from '../../store';
import { scrollToPreviousCommand, scrollToNextCommand, rerunLastCommand } from './navigateCommandBlocks';
import { splitTerminalInGroup } from './splitTerminal';
import { spawnTerminalGroup } from './spawnTerminal';

/**
 * VSCode/Warp/cmder-inspired keybindings for the integrated terminal.
 * Pure shell UX — no AI shortcuts here.
 */
export function registerTerminalKeybindings(manager: TerminalManager): void {
    window.addEventListener('keydown', (e: KeyboardEvent) => {
        const inTerminal = document.activeElement?.closest(
            '.terminal-view-host, .terminal-instance-wrapper, .terminal-container, .xterm',
        );
        const group = manager.getActiveGroup();
        if (!group?.activeInstanceId) return;

        const instance = manager.getTerminal(group.activeInstanceId);
        if (!instance) return;

        // Ctrl+Shift+` — new terminal
        if (e.key === '`' && e.ctrlKey && e.shiftKey) {
            e.preventDefault();
            void spawnTerminalGroup();
            return;
        }

        // Ctrl+Shift+5 — split horizontal
        if (e.key === '5' && e.ctrlKey && e.shiftKey && !e.altKey) {
            e.preventDefault();
            const gid = useStore.getState().activeTerminalGroupId;
            if (gid) void splitTerminalInGroup(gid, instance.id, 'horizontal');
            return;
        }

        // Ctrl+Alt+Shift+5 — split vertical (cmder/ConEmu pattern)
        if (e.key === '5' && e.ctrlKey && e.shiftKey && e.altKey) {
            e.preventDefault();
            const gid = useStore.getState().activeTerminalGroupId;
            if (gid) void splitTerminalInGroup(gid, instance.id, 'vertical');
            return;
        }

        // Ctrl+Shift+R — command palette (history + workflows)
        if ((e.key === 'r' || e.key === 'R') && e.ctrlKey && e.shiftKey) {
            e.preventDefault();
            window.dispatchEvent(new CustomEvent('vscr:open-terminal-palette'));
            return;
        }

        // Ctrl+Shift+T — new terminal
        if (e.key === 't' && e.ctrlKey && e.shiftKey) {
            e.preventDefault();
            void spawnTerminalGroup();
            return;
        }

        // Ctrl+Shift+W — close terminal
        if (e.key === 'w' && e.ctrlKey && e.shiftKey) {
            e.preventDefault();
            void manager.closeTerminal(instance.id);
            return;
        }

        if (!inTerminal) return;

        // Alt+Up/Down — Warp-style block navigation
        if (e.altKey && e.key === 'ArrowUp') {
            e.preventDefault();
            void scrollToPreviousCommand(instance.id);
            return;
        }
        if (e.altKey && e.key === 'ArrowDown') {
            e.preventDefault();
            void scrollToNextCommand(instance.id);
            return;
        }

        // Ctrl+Enter — re-run last command block
        if (e.key === 'Enter' && e.ctrlKey && !e.shiftKey) {
            e.preventDefault();
            void rerunLastCommand(instance.id);
            return;
        }

        // Ctrl+C — copy when selection exists (activity feed + PTY terminals)
        if (e.key === 'c' && e.ctrlKey && instance.term.hasSelection()) {
            e.preventDefault();
            manager.copySelection(instance);
            return;
        }

        // Ctrl+Shift+C — copy entire scrollback (handy for AIRI activity export)
        if (e.key === 'c' && e.ctrlKey && e.shiftKey) {
            e.preventDefault();
            manager.copyAll(instance);
            return;
        }

        // Ctrl+V — paste
        if (e.key === 'v' && e.ctrlKey) {
            e.preventDefault();
            void manager.paste(instance);
            return;
        }

        // Ctrl+A — select all
        if (e.key === 'a' && e.ctrlKey) {
            e.preventDefault();
            manager.selectAll(instance);
            return;
        }

        // Shift+PageUp/Down — scrollback
        if (e.key === 'PageUp' && e.shiftKey) {
            e.preventDefault();
            instance.term.scrollPages(-1);
            return;
        }
        if (e.key === 'PageDown' && e.shiftKey) {
            e.preventDefault();
            instance.term.scrollPages(1);
            return;
        }
    });
}
