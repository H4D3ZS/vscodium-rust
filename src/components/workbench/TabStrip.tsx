import React from 'react';
import { useStore } from '../../store';

function detectLanguageIcon(filename: string): { type: 'icon' | 'img'; value: string } {
    const ext = filename.split('.').pop()?.toLowerCase() ?? '';
    const fileSvg = `data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiNhZGRmZmYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIj48cGF0aCBkPSJNMTMgM0g2YTIgMiAwIDAgMC0yIDJ2MTRhMiAyIDAgMCAwIDIgMmgxMmEyIDIgMCAwIDAgMi0yVjlsLTYtNnoiPjwvcGF0aD48cG9seWxpbmUgcG9pbnRzPSIxMyAzIDEzIDkgMTkgOSI+PC9wb2x5bGluZT48L3N2Zz4=`;
    const codeSvg = `data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiM3OWI4ZmYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIj48cG9seWxpbmUgcG9pbnRzPSIxNiAxOCAyMiAxMiAxNiA2Ii8+PHBvbHlsaW5lIHBvaW50cz0iOCA2IDIgMTIgOCAxOCIvPjwvc3ZnPg==`;
    const codeExts = ['rs', 'ts', 'tsx', 'js', 'jsx', 'c', 'cpp', 'py', 'go', 'java'];
    return codeExts.includes(ext)
        ? { type: 'img', value: codeSvg }
        : { type: 'img', value: fileSvg };
}

const TabStrip: React.FC = () => {
    const tabs = useStore(state => state.tabs);
    const activeTabId = useStore(state => state.activeTabId);
    const closeTab = useStore(state => state.closeTab);
    const setActiveTab = useStore(state => state.setActiveTab);

    return (
        <div className="tabs-row">
            {tabs.map(tab => {
                const isActive = tab.id === activeTabId;
                const icon = detectLanguageIcon(tab.filename);
                return (
                    <div
                        key={tab.id}
                        className={`tab${isActive ? ' active' : ''}`}
                        onClick={() => setActiveTab(tab.id)}
                        title={tab.path}
                    >
                        {icon.type === 'img' ? (
                            <img src={icon.value} style={{ width: '14px', height: '14px', marginRight: '6px', opacity: isActive ? 1 : 0.6 }} />
                        ) : (
                            <i className={`${icon.value} tab-icon`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px', marginRight: '6px', color: isActive ? 'inherit' : 'var(--vscode-tab-activeForeground)', opacity: isActive ? 1 : 0.6 }} />
                        )}
                        <span className="tab-label">{tab.filename}</span>
                        <div className="tab-actions">
                            {tab.isModified && <span className="dirty-indicator" style={{ marginRight: '2px' }} />}
                            <i
                                className="codicon codicon-close"
                                style={{ fontFamily: 'codicon', fontStyle: 'normal' }}
                                title={tab.isModified ? 'Close (unsaved changes)' : 'Close'}
                                onClick={(e) => {
                                    e.stopPropagation();
                                    if (tab.isModified) {
                                        const choice = window.confirm(
                                            `'${tab.filename}' has unsaved changes.\n\nOK = Save and close\nCancel = Don't save`
                                        );
                                        if (choice) {
                                            useStore.getState().saveActiveFile().then(() => closeTab(tab.id));
                                        } else {
                                            const discard = window.confirm(`Discard unsaved changes in '${tab.filename}'?`);
                                            if (discard) closeTab(tab.id);
                                        }
                                    } else {
                                        closeTab(tab.id);
                                    }
                                }}
                            />
                        </div>
                    </div>
                );
            })}
        </div>
    );
};

export default TabStrip;
