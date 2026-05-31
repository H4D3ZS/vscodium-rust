import React, { useState, useEffect, useRef } from 'react';
import { SearchAddon } from '@xterm/addon-search';

export interface ISearchOptions {
  regex?: boolean;
  wholeWord?: boolean;
  caseSensitive?: boolean;
  incremental?: boolean;
}

interface TerminalFindWidgetProps {
  searchAddon: SearchAddon | null;
  visible: boolean;
  options?: ISearchOptions;
  onOptionsChange?: (options: ISearchOptions) => void;
  onClose: () => void;
  onFindNext?: (term: string) => void;
  onFindPrevious?: (term: string) => void;
}

const TerminalFindWidget: React.FC<TerminalFindWidgetProps> = ({ 
    searchAddon, 
    visible, 
    options,
    onOptionsChange,
    onClose,
    onFindNext,
    onFindPrevious
}) => {
    const [searchTerm, setSearchTerm] = useState('');
    const [isCaseSensitive, setIsCaseSensitive] = useState(options?.caseSensitive || false);
    const [isWholeWord, setIsWholeWord] = useState(options?.wholeWord || false);
    const [isRegex, setIsRegex] = useState(options?.regex || false);
    const inputRef = useRef<HTMLInputElement>(null);

    useEffect(() => {
        if (visible && inputRef.current) {
            inputRef.current.focus();
            inputRef.current.select();
        }
    }, [visible]);

    useEffect(() => {
        if (options) {
            setIsCaseSensitive(options.caseSensitive || false);
            setIsWholeWord(options.wholeWord || false);
            setIsRegex(options.regex || false);
        }
    }, [options]);

    const handleSearch = (next = true) => {
        if (!searchTerm) return;

        const searchOptions: ISearchOptions = {
            caseSensitive: isCaseSensitive,
            wholeWord: isWholeWord,
            regex: isRegex,
            incremental: true
        };

        if (onOptionsChange) {
            onOptionsChange(searchOptions);
        }

        if (next && onFindNext) {
            onFindNext(searchTerm);
        } else if (!next && onFindPrevious) {
            onFindPrevious(searchTerm);
        } else if (searchAddon) {
            if (next) {
                searchAddon.findNext(searchTerm, searchOptions);
            } else {
                searchAddon.findPrevious(searchTerm, searchOptions);
            }
        }
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter') {
            handleSearch(!e.shiftKey);
        } else if (e.key === 'Escape') {
            onClose();
        }
    };

    if (!visible) return null;

    return (
        <div 
            className="terminal-find-widget glass-panel"
            style={{
                position: 'absolute',
                top: '0',
                right: '40px',
                zIndex: 100,
                background: 'rgba(30, 30, 31, 0.85)',
                backdropFilter: 'blur(12px)',
                border: '1px solid rgba(255, 255, 255, 0.08)',
                display: 'flex',
                alignItems: 'center',
                padding: '6px 10px',
                gap: '8px',
                boxShadow: 'var(--shadow-macos)',
                borderRadius: '0 0 8px 8px'
            }}
        >
            <div style={{ position: 'relative', display: 'flex', alignItems: 'center' }}>
                <input
                    ref={inputRef}
                    type="text"
                    placeholder="Find"
                    value={searchTerm}
                    onChange={(e) => {
                        setSearchTerm(e.target.value);
                        if (searchAddon && e.target.value) {
                            searchAddon.findNext(e.target.value, {
                                caseSensitive: isCaseSensitive,
                                wholeWord: isWholeWord,
                                regex: isRegex,
                                incremental: true
                            });
                        }
                    }}
                    onKeyDown={handleKeyDown}
                    style={{
                        background: 'rgba(255, 255, 255, 0.05)',
                        color: 'var(--vscode-editor-foreground, #fff)',
                        border: '1px solid rgba(255, 255, 255, 0.1)',
                        padding: '4px 35px 4px 10px',
                        fontSize: '12px',
                        width: '200px',
                        borderRadius: '6px',
                        outline: 'none'
                    }}
                />
                
                <div style={{ position: 'absolute', right: '6px', display: 'flex', gap: '4px' }}>
                    <i 
                        className="codicon codicon-case-sensitive" 
                        title="Match Case"
                        onClick={() => setIsCaseSensitive(!isCaseSensitive)}
                        style={{ 
                            fontSize: '14px', 
                            cursor: 'pointer',
                            opacity: isCaseSensitive ? 1 : 0.4,
                            padding: '3px',
                            color: isCaseSensitive ? '#3b82f6' : 'inherit',
                            transition: 'all 0.2s'
                        }}
                    ></i>
                    <i 
                        className="codicon codicon-whole-word" 
                        title="Match Whole Word"
                        onClick={() => setIsWholeWord(!isWholeWord)}
                        style={{ 
                            fontSize: '14px', 
                            cursor: 'pointer',
                            opacity: isWholeWord ? 1 : 0.4,
                            padding: '3px',
                            color: isWholeWord ? '#3b82f6' : 'inherit',
                            transition: 'all 0.2s'
                        }}
                    ></i>
                    <i 
                        className="codicon codicon-regex" 
                        title="Use Regular Expression"
                        onClick={() => setIsRegex(!isRegex)}
                        style={{ 
                            fontSize: '14px', 
                            cursor: 'pointer',
                            opacity: isRegex ? 1 : 0.4,
                            padding: '3px',
                            color: isRegex ? '#3b82f6' : 'inherit',
                            transition: 'all 0.2s'
                        }}
                    ></i>
                </div>
            </div>

            <div style={{ display: 'flex', gap: '2px', borderLeft: '1px solid var(--vscode-panel-border, #444)', paddingLeft: '4px', marginLeft: '4px' }}>
                <i 
                    className="codicon codicon-arrow-up" 
                    title="Previous Match (Shift+Enter)"
                    onClick={() => handleSearch(false)}
                    style={{ fontSize: '14px', cursor: 'pointer', padding: '2px', borderRadius: '3px' }}
                ></i>
                <i 
                    className="codicon codicon-arrow-down" 
                    title="Next Match (Enter)"
                    onClick={() => handleSearch(true)}
                    style={{ fontSize: '14px', cursor: 'pointer', padding: '2px', borderRadius: '3px' }}
                ></i>
                <i 
                    className="codicon codicon-close" 
                    title="Close (Escape)"
                    onClick={onClose}
                    style={{ fontSize: '14px', cursor: 'pointer', padding: '2px', borderRadius: '3px', marginLeft: '4px' }}
                ></i>
            </div>
        </div>
    );
};

export default TerminalFindWidget;
