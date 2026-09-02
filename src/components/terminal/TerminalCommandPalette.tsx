// ════════════════════════════════════════════════════════════════════════════
// Terminal command palette — cmder/Warp-style command recall (NO AI).
//
// Ctrl+Shift+R opens a fuzzy-search overlay of the active terminal's command
// history + saved workflows. Enter inserts the command at the prompt (you review
// and press Enter); Ctrl+Enter runs it immediately. Pure terminal feature.
// ════════════════════════════════════════════════════════════════════════════

import React, { useState, useEffect, useRef, useMemo } from 'react';
import { terminalManager } from '../../terminal';
import {
    listTerminalWorkflows,
    deleteTerminalWorkflow,
    insertWorkflowCommand,
    runWorkflowCommand,
} from '../../application/terminal/runWorkflow';

interface PaletteItem {
  kind: 'history' | 'workflow';
  label: string;
  command: string;
  id?: string;
}

const TerminalCommandPalette: React.FC = () => {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState('');
  const [sel, setSel] = useState(0);
  const [version, setVersion] = useState(0); // bump to re-read after delete
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const onOpen = () => {
      setQuery('');
      setSel(0);
      setVersion((v) => v + 1);
      setOpen(true);
    };
    window.addEventListener('vscr:open-terminal-palette', onOpen);
    return () => window.removeEventListener('vscr:open-terminal-palette', onOpen);
  }, []);

  useEffect(() => {
    if (open) setTimeout(() => inputRef.current?.focus(), 0);
  }, [open]);

  const items: PaletteItem[] = useMemo(() => {
    if (!open) return [];
    const wf = listTerminalWorkflows().map<PaletteItem>((w) => ({
      kind: 'workflow',
      label: w.name,
      command: w.command,
      id: w.id,
    }));
    const wfCmds = new Set(wf.map((w) => w.command));
    const hist = terminalManager
      .getActiveCommandHistory()
      .filter((c) => !wfCmds.has(c))
      .map<PaletteItem>((c) => ({ kind: 'history', label: c, command: c }));
    return [...wf, ...hist];
  }, [open, version]);

  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return items;
    return items.filter(
      (i) => i.label.toLowerCase().includes(q) || i.command.toLowerCase().includes(q),
    );
  }, [items, query]);

  useEffect(() => {
    if (sel >= filtered.length) setSel(Math.max(0, filtered.length - 1));
  }, [filtered, sel]);

  if (!open) return null;

  const close = () => setOpen(false);

  const choose = (run: boolean) => {
    const item = filtered[sel];
    if (item) {
      if (run) void runWorkflowCommand(item.command);
      else void insertWorkflowCommand(item.command);
    }
    close();
  };

  const onKey = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      e.preventDefault();
      close();
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      setSel((s) => Math.min(s + 1, filtered.length - 1));
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setSel((s) => Math.max(s - 1, 0));
    } else if (e.key === 'Enter') {
      e.preventDefault();
      choose(e.ctrlKey || e.metaKey);
    }
  };

  return (
    <div
      onMouseDown={close}
      style={{
        position: 'fixed',
        inset: 0,
        zIndex: 9000,
        display: 'flex',
        alignItems: 'flex-start',
        justifyContent: 'center',
        paddingTop: '12vh',
        background: 'rgba(0,0,0,0.45)',
        backdropFilter: 'blur(2px)',
      }}
    >
      <div
        onMouseDown={(e) => e.stopPropagation()}
        style={{
          width: 'min(640px, 92vw)',
          maxHeight: '60vh',
          display: 'flex',
          flexDirection: 'column',
          background: 'rgba(22,24,30,0.98)',
          border: '1px solid rgba(122,162,247,0.25)',
          borderRadius: '12px',
          boxShadow: '0 18px 60px rgba(0,0,0,0.55)',
          overflow: 'hidden',
          font: '13px/1.4 "Cascadia Mono", Consolas, monospace',
          color: '#d5d8e0',
        }}
      >
        <div style={{ display: 'flex', alignItems: 'center', gap: 8, padding: '10px 14px', borderBottom: '1px solid rgba(255,255,255,0.07)' }}>
          <i className="codicon codicon-terminal" style={{ opacity: 0.6, fontSize: 15 }} />
          <input
            ref={inputRef}
            value={query}
            onChange={(e) => { setQuery(e.target.value); setSel(0); }}
            onKeyDown={onKey}
            placeholder="Run a command — history & workflows  (Enter: insert · Ctrl+Enter: run)"
            style={{
              flex: 1,
              background: 'transparent',
              border: 'none',
              outline: 'none',
              color: '#e6e9f0',
              fontFamily: 'inherit',
              fontSize: '13px',
            }}
          />
        </div>

        <div style={{ overflowY: 'auto' }}>
          {filtered.length === 0 && (
            <div style={{ padding: '16px', opacity: 0.5 }}>No matching commands.</div>
          )}
          {filtered.map((item, i) => (
            <div
              key={`${item.kind}-${item.id ?? item.command}-${i}`}
              onMouseEnter={() => setSel(i)}
              onMouseDown={(e) => { e.preventDefault(); choose(e.ctrlKey || e.metaKey); }}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 10,
                padding: '7px 14px',
                cursor: 'pointer',
                background: i === sel ? 'rgba(122,162,247,0.16)' : 'transparent',
                borderLeft: i === sel ? '2px solid #7aa2f7' : '2px solid transparent',
              }}
            >
              <i
                className={`codicon ${item.kind === 'workflow' ? 'codicon-bookmark' : 'codicon-history'}`}
                style={{ fontSize: 13, opacity: 0.6, flexShrink: 0 }}
              />
              <span style={{ flex: 1, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                {item.label}
              </span>
              {item.kind === 'workflow' && item.command !== item.label && (
                <span style={{ opacity: 0.45, fontSize: 11, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis', maxWidth: '40%' }}>
                  {item.command}
                </span>
              )}
              {item.kind === 'workflow' && item.id && (
                <i
                  className="codicon codicon-close"
                  title="Delete workflow"
                  onMouseDown={(e) => { e.preventDefault(); e.stopPropagation(); deleteTerminalWorkflow(item.id!); setVersion((v) => v + 1); }}
                  style={{ fontSize: 12, opacity: 0.4, flexShrink: 0 }}
                />
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default TerminalCommandPalette;
