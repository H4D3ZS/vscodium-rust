/**
 * Cursor-style @ mention context menu — extracted from RightSidebar.
 * Shows file suggestions, special context sources (@codebase, @web, @git, etc.),
 * and slash commands when user types @ or / in the chat input.
 */
import React, { useMemo, useCallback } from 'react';

export interface MentionItem {
    path: string;
    name: string;
    is_dir?: boolean;
    _special?: boolean;
    _icon?: string;
    _desc?: string;
}

interface MentionPopupProps {
    inputValue: string;
    allFiles: { path: string; name: string; is_dir: boolean }[];
    isOpen: boolean;
    selectedIndex: number;
    onSelect: (item: MentionItem) => void;
    onSelectIndex: (index: number) => void;
}

export const SPECIAL_MENTIONS: MentionItem[] = [
    { path: '__codebase__', name: '@codebase', is_dir: false, _special: true, _icon: 'codicon-repo', _desc: 'Auto-find relevant files' },
    { path: '__web__', name: '@web', is_dir: false, _special: true, _icon: 'codicon-globe', _desc: 'Search the web' },
    { path: '__git__', name: '@git', is_dir: false, _special: true, _icon: 'codicon-git-branch', _desc: 'Git diff & status' },
    { path: '__docs__', name: '@docs', is_dir: false, _special: true, _icon: 'codicon-book', _desc: 'Documentation context' },
    { path: '__symbol__', name: '@symbol', is_dir: false, _special: true, _icon: 'codicon-symbol-class', _desc: 'LSP workspace symbol lookup' },
    { path: '__folder__', name: '@folder', is_dir: false, _special: true, _icon: 'codicon-folder', _desc: 'Inject a directory listing' },
    { path: '__problems__', name: '@problems', is_dir: false, _special: true, _icon: 'codicon-warning', _desc: 'Current LSP diagnostics' },
    { path: '__terminal__', name: '@terminal', is_dir: false, _special: true, _icon: 'codicon-terminal', _desc: 'Last terminal output' },
];

export const SLASH_COMMANDS: MentionItem[] = [
    { path: '/generate', name: '/generate', _special: true, _icon: 'codicon-code', _desc: 'Generate code' },
    { path: '/explain', name: '/explain', _special: true, _icon: 'codicon-book', _desc: 'Explain code' },
    { path: '/refactor', name: '/refactor', _special: true, _icon: 'codicon-wrench', _desc: 'Refactor code' },
    { path: '/debug', name: '/debug', _special: true, _icon: 'codicon-bug', _desc: 'Debug code' },
    { path: '/document', name: '/document', _special: true, _icon: 'codicon-list-selection', _desc: 'Document code' },
    { path: '/test', name: '/test', _special: true, _icon: 'codicon-beaker', _desc: 'Create tests' },
    { path: '/review', name: '/review', _special: true, _icon: 'codicon-shield', _desc: 'Review for bugs & security' },
    { path: '/bugbot', name: '/bugbot', _special: true, _icon: 'codicon-bug', _desc: 'AI code review (BugBot)' },
    { path: '/design', name: '/design', _special: true, _icon: 'codicon-device-desktop', _desc: 'Image → code' },
    { path: '/commit', name: '/commit', _special: true, _icon: 'codicon-git-commit', _desc: 'Git commit' },
    { path: '/fix', name: '/fix', _special: true, _icon: 'codicon-tools', _desc: 'Fix errors' },
];

function filterSuggestions(inputValue: string, allFiles: { path: string; name: string; is_dir: boolean }[]): MentionItem[] {
    const lastWord = inputValue.split(/\s+/).pop() || '';
    if (!lastWord.startsWith('@') && !lastWord.startsWith('/')) return [];

    const query = lastWord.slice(1).toLowerCase();

    if (lastWord.startsWith('/')) {
        return SLASH_COMMANDS.filter(c => c.name.startsWith(lastWord.toLowerCase()));
    }

    const specials = (query === '' || SPECIAL_MENTIONS.some(s => s.name.slice(1).startsWith(query)))
        ? SPECIAL_MENTIONS.filter(s => s.name.slice(1).startsWith(query) || query === '')
        : [];
    const files = allFiles.filter(f => f.name.toLowerCase().includes(query)).slice(0, 8);
    return [...specials, ...files] as MentionItem[];
}

const MentionPopup: React.FC<MentionPopupProps> = ({
    inputValue, allFiles, isOpen, selectedIndex, onSelect, onSelectIndex,
}) => {
    const suggestions = useMemo(() => filterSuggestions(inputValue, allFiles), [inputValue, allFiles]);

    if (!isOpen || suggestions.length === 0) return null;

    const specialItems = suggestions.filter(s => s._special);
    const fileItems = suggestions.filter(s => !s._special);
    let globalIdx = 0;

    return (
        <div style={{
            position: 'absolute', bottom: '100%', left: '10px', right: '10px',
            background: 'var(--vscode-menu-background, #1e1e2e)',
            border: '1px solid var(--vscode-menu-border, rgba(255,255,255,0.12))',
            borderRadius: '8px', overflow: 'hidden',
            boxShadow: '0 -8px 30px rgba(0,0,0,0.45)',
            zIndex: 100, marginBottom: '6px',
            maxHeight: '320px', overflowY: 'auto',
        }}>
            {specialItems.length > 0 && (
                <>
                    <div style={{
                        padding: '6px 12px 4px', fontSize: '9px', fontWeight: 700,
                        textTransform: 'uppercase', letterSpacing: '0.08em',
                        color: 'rgba(255,255,255,0.4)',
                    }}>
                        Context
                    </div>
                    {specialItems.map((item) => {
                        const idx = globalIdx++;
                        return (
                            <MentionItemRow
                                key={item.path}
                                item={item}
                                isSelected={idx === selectedIndex}
                                onSelect={() => onSelect(item)}
                                onHover={() => onSelectIndex(idx)}
                            />
                        );
                    })}
                </>
            )}
            {fileItems.length > 0 && (
                <>
                    {specialItems.length > 0 && (
                        <div style={{ height: 1, background: 'rgba(255,255,255,0.06)', margin: '2px 0' }} />
                    )}
                    <div style={{
                        padding: '6px 12px 4px', fontSize: '9px', fontWeight: 700,
                        textTransform: 'uppercase', letterSpacing: '0.08em',
                        color: 'rgba(255,255,255,0.4)',
                    }}>
                        Files
                    </div>
                    {fileItems.map((item) => {
                        const idx = globalIdx++;
                        return (
                            <MentionItemRow
                                key={item.path}
                                item={item}
                                isSelected={idx === selectedIndex}
                                onSelect={() => onSelect(item)}
                                onHover={() => onSelectIndex(idx)}
                            />
                        );
                    })}
                </>
            )}
        </div>
    );
};

const MentionItemRow: React.FC<{
    item: MentionItem;
    isSelected: boolean;
    onSelect: () => void;
    onHover: () => void;
}> = ({ item, isSelected, onSelect, onHover }) => (
    <div
        onMouseDown={(e) => { e.preventDefault(); onSelect(); }}
        onMouseEnter={onHover}
        style={{
            padding: item._special ? '7px 12px' : '5px 12px',
            cursor: 'pointer', fontSize: '12px',
            background: isSelected ? 'var(--vscode-list-activeSelectionBackground, rgba(59,130,246,0.15))' : 'transparent',
            color: isSelected ? 'var(--vscode-list-activeSelectionForeground, #fff)' : 'rgba(255,255,255,0.75)',
            display: 'flex', alignItems: 'center', gap: '8px',
            borderLeft: isSelected ? '2px solid var(--vscode-focusBorder, #3b82f6)' : '2px solid transparent',
            transition: 'all 0.08s',
        }}
    >
        <i
            className={`codicon ${item._icon || 'codicon-file'}`}
            style={{
                fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px',
                opacity: item._special ? 0.9 : 0.55,
                color: item._special ? (isSelected ? '#3b82f6' : undefined) : undefined,
            }}
        />
        <span style={{
            flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap',
            fontWeight: item._special ? 600 : 400,
        }}>
            {item.name}
        </span>
        {item._desc && (
            <span style={{
                fontSize: '10px', opacity: 0.45, whiteSpace: 'nowrap',
                maxWidth: '120px', overflow: 'hidden', textOverflow: 'ellipsis',
            }}>
                {item._desc}
            </span>
        )}
        {!item._special && (
            <span style={{
                fontSize: '9px', opacity: 0.3, overflow: 'hidden',
                textOverflow: 'ellipsis', whiteSpace: 'nowrap', maxWidth: '80px',
                fontFamily: 'var(--font-mono)',
            }}>
                {item.path.split(/[\\/]/).slice(-3, -1).join('/')}
            </span>
        )}
    </div>
);

export default MentionPopup;
