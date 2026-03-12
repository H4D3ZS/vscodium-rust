import React, { useEffect } from 'react';
import { invoke } from './tauri_bridge';
import TitleBar from './components/TitleBar';
import Workbench from './components/Workbench';
import StatusBar from './components/StatusBar';
import './styles.css';
import { initSearch } from './search';
import { initStatusBar } from './status_bar';
import { initExtensions } from './extensions';
import { initSpecs } from './specs';
import { initMobile } from './mobile';
import { useStore } from './store.ts';
import { initCommands } from './commands.ts';
import { initScm } from './scm';
import { initDebugUI } from './debug_ui';
import { initTerminal } from './terminal';

const App: React.FC = () => {
    useEffect(() => {
        (window as any).useStore = useStore;
        // Initialize non-React behaviors once the shell is mounted.
        initCommands();
        initSearch();
        initStatusBar();
        initExtensions();
        initSpecs();
        initMobile();
        initScm();
        initDebugUI();
        initTerminal();

        // Restore last folder
        const { activeRoot, refreshFileTree } = useStore.getState();
        if (activeRoot) {
            (window as any).activeRoot = activeRoot;
            invoke('set_active_root', { path: activeRoot }).then(() => {
                refreshFileTree();
            });
        }
    }, []);

    return (
        <>
            <div id="command-palette" className="command-palette hidden">
                <div className="command-input-container">
                    <input type="text" id="command-input" placeholder="Type a command or search..." />
                </div>
                <div id="command-list" className="command-list"></div>
            </div>

            <TitleBar />
            <Workbench />
            <StatusBar />

            {/* Debug Toolbar */}
            <div id="debug-toolbar" className="debug-toolbar hidden">
                <div className="debug-tool-item" id="debug-continue" title="Continue (F5)">▶️</div>
                <div className="debug-tool-item" id="debug-step-over" title="Step Over (F10)">↷</div>
                <div className="debug-tool-item" id="debug-step-into" title="Step Into (F11)">↴</div>
                <div className="debug-tool-item" id="debug-step-out" title="Step Out (Shift+F11)">⤴</div>
                <div className="debug-tool-item" id="debug-restart" title="Restart (Ctrl+Shift+F5)">↻</div>
                <div className="debug-tool-item stop" id="debug-stop" title="Stop (Shift+F5)">⏹️</div>
            </div>

            {/* Context Menu – used primarily for Explorer items */}
            <div id="context-menu" className="context-menu hidden">
                <div className="menu-item" id="cm-open">Open</div>
                <div className="menu-item" id="cm-reveal">Reveal in Finder</div>
                <div className="menu-separator"></div>
                <div className="menu-item" id="cm-new-file">New File...</div>
                <div className="menu-item" id="cm-new-folder">New Folder...</div>
                <div className="menu-separator"></div>
                <div className="menu-item" id="cm-rename">Rename...</div>
                <div className="menu-item" id="cm-delete">Delete</div>
                <div className="menu-separator"></div>
                <div className="menu-item" id="cm-palette">Command Palette...</div>
            </div>
        </>
    );
};

export default App;
