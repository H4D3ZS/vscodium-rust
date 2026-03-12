import React, { useCallback, useEffect, useRef } from 'react';
import MonacoEditor from '@monaco-editor/react';
import type { OnMount, OnChange } from '@monaco-editor/react';
import { useStore } from '../store';

const CTRL_S = 2048 | 49; // KeyMod.CtrlCmd | KeyCode.KeyS

const Editor: React.FC = () => {
    const activeTabId = useStore(state => state.activeTabId);
    const tabs = useStore(state => state.tabs);
    const updateTabContent = useStore(state => state.updateTabContent);
    const saveActiveFile = useStore(state => state.saveActiveFile);
    const theme = useStore(state => state.theme);

    const activeTab = tabs.find(t => t.id === activeTabId) ?? null;

    const editorRef = useRef<any>(null);

    const handleMount: OnMount = useCallback((editor) => {
        editorRef.current = editor;
        editor.addCommand(CTRL_S, () => saveActiveFile());
    }, [saveActiveFile]);

    const handleChange: OnChange = useCallback((value) => {
        if (activeTabId && value !== undefined) {
            updateTabContent(activeTabId, value);
        }
    }, [activeTabId, updateTabContent]);

    // When switching tabs, sync the editor value
    useEffect(() => {
        if (editorRef.current && activeTab) {
            const currentValue = editorRef.current.getValue();
            if (currentValue !== activeTab.content) {
                editorRef.current.setValue(activeTab.content);
            }
        }
    }, [activeTabId]);

    if (!activeTab) {
        return null;
    }

    return (
        <MonacoEditor
            height="100%"
            width="100%"
            theme={theme}
            language={activeTab.language}
            value={activeTab.content}
            onMount={handleMount}
            onChange={handleChange}
            options={{
                fontSize: 13,
                fontFamily: 'var(--font-mono)',
                lineNumbers: 'on',
                lineNumbersMinChars: 3,
                glyphMargin: false,
                folding: true,
                lineDecorationsWidth: 10,
                minimap: { enabled: true },
                scrollBeyondLastLine: false,
                wordWrap: 'off',
                tabSize: 4,
                insertSpaces: true,
                automaticLayout: true,
                renderWhitespace: 'selection',
                smoothScrolling: true,
                cursorBlinking: 'smooth',
                cursorSmoothCaretAnimation: 'on',
                bracketPairColorization: { enabled: true },
            }}
        />
    );
};

export default Editor;
