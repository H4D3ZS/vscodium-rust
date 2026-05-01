import React, { useEffect, useRef, useState } from 'react';
import { terminalManager, getVSCodeTheme, registerTerminalShortcuts } from '../../terminal';
import { SearchAddon } from '@xterm/addon-search';
import TerminalFindWidget, { ISearchOptions } from './TerminalFindWidget';
import { useStore } from '../../store';

interface TerminalInstanceProps {
    id: string;
    groupId: string;
    active: boolean;
}

const TerminalInstance: React.FC<TerminalInstanceProps> = ({ id, groupId, active }) => {
    const containerRef = useRef<HTMLDivElement>(null);
    const setActiveInstance = useStore(state => state.setActiveTerminalInstance);
    const [findAddon, setFindAddon] = useState<SearchAddon | null>(null);
    const [findVisible, setFindVisible] = useState(false);
    const [searchOptions, setSearchOptions] = useState<ISearchOptions>({
        regex: false,
        wholeWord: false,
        caseSensitive: false,
        incremental: false
    });

    // Initial attachment
    useEffect(() => {
        if (containerRef.current) {
            terminalManager.attach(id, containerRef.current);
            const t = terminalManager.getTerminal(id);
            if (t) {
                setFindAddon(t.searchAddon);
            }
        }
        
        // Register shortcuts once
        registerTerminalShortcuts(terminalManager);
    }, [id]);

    // Handle visibility and focus
    useEffect(() => {
        if (active && containerRef.current) {
            terminalManager.resize(id);
            const t = terminalManager.getTerminal(id);
            if (t) {
                t.term.focus();
                setActiveInstance(groupId, id);
            }
        }
    }, [active, id, groupId, setActiveInstance]);

    // Resize handling
    useEffect(() => {
        if (!containerRef.current) return;

        const observer = new ResizeObserver(() => {
            if (active) {
                terminalManager.resize(id);
            }
        });

        observer.observe(containerRef.current);
        return () => observer.disconnect();
    }, [id, active]);

    // Theme updates
    const currentTheme = useStore(state => state.theme);
    useEffect(() => {
        terminalManager.updateAllThemes();
    }, [currentTheme]);

    // Keyboard shortcuts for find widget
    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (!active) return;

            // Ctrl+F or Cmd+F - Find
            if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
                e.preventDefault();
                setFindVisible(prev => !prev);
            }
            
            // Escape - Close find widget
            if (e.key === 'Escape' && findVisible) {
                setFindVisible(false);
                const t = terminalManager.getTerminal(id);
                if (t) t.term.focus();
            }
        };

        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    }, [active, findVisible, id]);

    return (
        <div
            className={`terminal-instance-wrapper ${active ? 'active' : ''}`}
            onClick={() => setActiveInstance(groupId, id)}
            style={{
                flex: 1,
                width: '100%',
                height: '100%',
                background: 'var(--vscode-terminal-background, #1e1e1e)',
                borderLeft: active ? '2px solid var(--vscode-terminal-tab-activeBorder, #007acc)' : '1px solid transparent',
                position: 'relative',
                overflow: 'hidden',
                display: active ? 'flex' : 'none',
                flexDirection: 'column'
            }}
        >
            {/* Find Widget */}
            <TerminalFindWidget
                searchAddon={findAddon}
                visible={findVisible}
                options={searchOptions}
                onOptionsChange={setSearchOptions}
                onClose={() => {
                    setFindVisible(false);
                    const t = terminalManager.getTerminal(id);
                    if (t) t.term.focus();
                }}
                onFindNext={(term: string) => {
                    terminalManager.findNext(id, term, searchOptions);
                }}
                onFindPrevious={(term: string) => {
                    terminalManager.findPrevious(id, term, searchOptions);
                }}
            />
            
            {/* Terminal Container */}
            <div
                ref={containerRef}
                className="terminal-container"
                style={{
                    flex: 1,
                    width: '100%',
                    height: '100%',
                    overflow: 'hidden'
                }}
                onContextMenu={(e) => {
                    e.preventDefault();
                    // Context menu is handled by xterm's onContextMenu
                }}
            />
        </div>
    );
};

export default TerminalInstance;
