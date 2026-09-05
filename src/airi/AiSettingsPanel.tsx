import React, { useState, useEffect, useCallback } from 'react';
import { invoke } from '../tauri_bridge';

type ConnectionStatus = 'idle' | 'testing' | 'connected' | 'error';
type EmulatorStatus = 'stopped' | 'launching' | 'running' | 'error';

const styles: Record<string, React.CSSProperties> = {
  container: {
    padding: '16px',
    background: 'var(--vscode-sideBar-background, #1e1e1e)',
    color: 'var(--vscode-editor-foreground, #cccccc)',
    fontFamily: 'var(--vscode-font-family, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif)',
    fontSize: '13px',
    height: '100%',
    overflowY: 'auto',
    display: 'flex',
    flexDirection: 'column',
    gap: '16px',
  },
  section: {
    background: 'var(--vscode-editor-background, #252526)',
    border: '1px solid var(--vscode-panel-border, #3c3c3c)',
    borderRadius: '6px',
    padding: '12px',
  },
  sectionTitle: {
    fontSize: '11px',
    fontWeight: 600,
    textTransform: 'uppercase',
    letterSpacing: '0.08em',
    color: 'var(--vscode-editor-foreground, #cccccc)',
    marginBottom: '10px',
    opacity: 0.8,
  },
  label: {
    display: 'block',
    fontSize: '12px',
    marginBottom: '4px',
    color: 'var(--vscode-input-foreground, #cccccc)',
    opacity: 0.7,
  },
  input: {
    width: '100%',
    padding: '6px 8px',
    background: 'var(--vscode-input-background, #3c3c3c)',
    color: 'var(--vscode-input-foreground, #cccccc)',
    border: '1px solid var(--vscode-input-border, #555555)',
    borderRadius: '4px',
    fontSize: '13px',
    outline: 'none',
    boxSizing: 'border-box',
  },
  button: {
    padding: '6px 12px',
    background: 'var(--vscode-button-background, #0e639c)',
    color: 'var(--vscode-button-foreground, #ffffff)',
    border: 'none',
    borderRadius: '4px',
    fontSize: '12px',
    fontWeight: 500,
    cursor: 'pointer',
    whiteSpace: 'nowrap',
  },
  buttonDanger: {
    padding: '6px 12px',
    background: 'var(--vscode-errorForeground, #f48771)',
    color: '#ffffff',
    border: 'none',
    borderRadius: '4px',
    fontSize: '12px',
    fontWeight: 500,
    cursor: 'pointer',
    whiteSpace: 'nowrap',
  },
  buttonSecondary: {
    padding: '6px 12px',
    background: 'var(--vscode-button-secondaryBackground, #3a3d41)',
    color: 'var(--vscode-button-secondaryForeground, #cccccc)',
    border: '1px solid var(--vscode-button-secondaryBorder, #555555)',
    borderRadius: '4px',
    fontSize: '12px',
    fontWeight: 500,
    cursor: 'pointer',
    whiteSpace: 'nowrap',
  },
  buttonRow: {
    display: 'flex',
    gap: '8px',
    alignItems: 'center',
    marginTop: '8px',
  },
  statusBadge: {
    display: 'inline-flex',
    alignItems: 'center',
    gap: '4px',
    padding: '2px 8px',
    borderRadius: '10px',
    fontSize: '11px',
    fontWeight: 500,
  },
  select: {
    width: '100%',
    padding: '6px 8px',
    background: 'var(--vscode-dropdown-background, #3c3c3c)',
    color: 'var(--vscode-dropdown-foreground, #cccccc)',
    border: '1px solid var(--vscode-dropdown-border, #555555)',
    borderRadius: '4px',
    fontSize: '13px',
    outline: 'none',
    boxSizing: 'border-box',
  },
  statusRow: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    padding: '6px 0',
    borderBottom: '1px solid var(--vscode-panel-border, #3c3c3c)',
  },
  statusLabel: {
    fontSize: '12px',
    opacity: 0.8,
  },
  spinner: {
    width: '12px',
    height: '12px',
    border: '2px solid var(--vscode-button-background, #0e639c)',
    borderTopColor: 'transparent',
    borderRadius: '50%',
    animation: 'aiSettingsSpin 0.6s linear infinite',
  },
};

const statusColorMap: Record<string, React.CSSProperties> = {
  connected: { background: '#1b3a2b', color: '#4ade80', border: '1px solid #4ade8033' },
  error: { background: '#3a1b1b', color: '#f87171', border: '1px solid #f8717133' },
  idle: { background: '#2a2a2a', color: '#9ca3af', border: '1px solid #9ca3af33' },
  running: { background: '#1b3a2b', color: '#4ade80', border: '1px solid #4ade8033' },
  stopped: { background: '#2a2a2a', color: '#9ca3af', border: '1px solid #9ca3af33' },
  available: { background: '#1b3a2b', color: '#4ade80', border: '1px solid #4ade8033' },
  unavailable: { background: '#3a1b1b', color: '#f87171', border: '1px solid #f8717133' },
};

const StatusBadge: React.FC<{ status: string; label: string }> = ({ status, label }) => (
  <span style={{ ...styles.statusBadge, ...(statusColorMap[status] || statusColorMap.idle) }}>
    <span style={{
      width: '6px', height: '6px', borderRadius: '50%',
      background: statusColorMap[status]?.color || '#9ca3af',
    }} />
    {label}
  </span>
);

export const AiSettingsPanel: React.FC = () => {
  const [ollamaUrl, setOllamaUrl] = useState('http://localhost:13305');
  const [connectionStatus, setConnectionStatus] = useState<ConnectionStatus>('idle');
  const [connectionMessage, setConnectionMessage] = useState('');
  const [models, setModels] = useState<string[]>([]);
  const [selectedModel, setSelectedModel] = useState('');
  const [modelsLoading, setModelsLoading] = useState(false);
  const [broadcastAvailable, setBroadcastAvailable] = useState<boolean | null>(null);
  const [callToolAvailable, setCallToolAvailable] = useState<boolean | null>(null);
  const [emulatorStatus, setEmulatorStatus] = useState<EmulatorStatus>('stopped');
  const [surgicalEditorRunning, setSurgicalEditorRunning] = useState(false);
  const [visionSystemRunning, setVisionSystemRunning] = useState(false);

  useEffect(() => {
    const checkSystemStatus = async () => {
      try {
        await invoke('airi_broadcast', { event: 'status_ping', payload: {} });
        setBroadcastAvailable(true);
      } catch {
        setBroadcastAvailable(false);
      }
      try {
        await invoke('call_tool', { name: 'view_file', arguments: { path: '' } });
        setCallToolAvailable(true);
      } catch {
        setCallToolAvailable(false);
      }
      try {
        await invoke('check_lemonade_status');
        setConnectionStatus('connected');
        setConnectionMessage('Ollama is running');
      } catch {
        setConnectionStatus('error');
        setConnectionMessage('Ollama not reachable');
      }
    };
    checkSystemStatus();
  }, []);

  const testConnection = useCallback(async () => {
    setConnectionStatus('testing');
    setConnectionMessage('');
    try {
      await invoke('check_lemonade_status');
      setConnectionStatus('connected');
      setConnectionMessage('Connection successful');
    } catch (err: any) {
      setConnectionStatus('error');
      setConnectionMessage(err?.message || 'Connection failed');
    }
  }, []);

  const listModels = useCallback(async () => {
    setModelsLoading(true);
    try {
      const result = await invoke<string[]>('list_provider_models', { provider: 'lemonade' });
      setModels(result);
      if (result.length > 0 && !selectedModel) {
        setSelectedModel(result[0]);
      }
    } catch {
      setModels([]);
    } finally {
      setModelsLoading(false);
    }
  }, [selectedModel]);

  const saveOllamaUrl = useCallback(async () => {
    try {
      await invoke('set_lemonade_url', { url: ollamaUrl });
    } catch (err: any) {
      console.error("Failed to set Ollama URL:", err);
    }
  }, [ollamaUrl]);

  const launchEmulator = useCallback(async () => {
    setEmulatorStatus('launching');
    try {
      await invoke('launch_iphone_emulator');
      setEmulatorStatus('running');
    } catch {
      setEmulatorStatus('error');
    }
  }, []);

  const stopEmulator = useCallback(async () => {
    try {
      await invoke('stop_iphone_emulator');
      setEmulatorStatus('stopped');
    } catch {
      setEmulatorStatus('error');
    }
  }, []);

  useEffect(() => {
    if (models.length === 0) return;
    setSurgicalEditorRunning(true);
    setVisionSystemRunning(true);
  }, [models]);

  return (
    <div style={styles.container}>
      <style>{`
        @keyframes aiSettingsSpin {
          to { transform: rotate(360deg); }
        }
      `}</style>

      <div style={styles.section}>
        <div style={styles.sectionTitle}>Ollama Connection</div>
        <label style={styles.label}>Ollama URL</label>
        <div style={{ display: 'flex', gap: '8px' }}>
          <input
            style={{ ...styles.input, flex: 1 }}
            value={ollamaUrl}
            onChange={e => setOllamaUrl(e.target.value)}
            placeholder="http://localhost:13305"
          />
          <button style={styles.button} onClick={testConnection} disabled={connectionStatus === 'testing'}>
            {connectionStatus === 'testing' ? 'Testing...' : 'Test'}
          </button>
        </div>
        {connectionStatus !== 'idle' && (
          <div style={{ marginTop: '8px', display: 'flex', alignItems: 'center', gap: '8px' }}>
            <StatusBadge
              status={connectionStatus}
              label={connectionStatus === 'connected' ? 'Connected' : connectionStatus === 'error' ? 'Failed' : 'Testing'}
            />
            {connectionMessage && (
              <span style={{ fontSize: '11px', opacity: 0.6 }}>{connectionMessage}</span>
            )}
          </div>
        )}
      </div>

      <div style={styles.section}>
        <div style={styles.sectionTitle}>Model Selector</div>
        <div style={styles.buttonRow}>
          <button style={styles.buttonSecondary} onClick={listModels} disabled={modelsLoading}>
            {modelsLoading ? 'Loading...' : 'List Models'}
          </button>
        </div>
        {models.length > 0 && (
          <div style={{ marginTop: '8px' }}>
            <label style={styles.label}>Select Model</label>
            <select
              style={styles.select}
              value={selectedModel}
              onChange={e => setSelectedModel(e.target.value)}
            >
              {models.map(m => (
                <option key={m} value={m}>{m}</option>
              ))}
            </select>
          </div>
        )}
        {models.length === 0 && !modelsLoading && (
          <div style={{ fontSize: '11px', opacity: 0.5, marginTop: '6px' }}>
            Click "List Models" to fetch available models
          </div>
        )}
      </div>

      <div style={styles.section}>
        <div style={styles.sectionTitle}>System Status</div>
        <div style={styles.statusRow}>
          <span style={styles.statusLabel}>airi_broadcast</span>
          <StatusBadge
            status={broadcastAvailable === null ? 'idle' : broadcastAvailable ? 'available' : 'unavailable'}
            label={broadcastAvailable === null ? 'Checking' : broadcastAvailable ? 'Available' : 'Unavailable'}
          />
        </div>
        <div style={styles.statusRow}>
          <span style={styles.statusLabel}>call_tool</span>
          <StatusBadge
            status={callToolAvailable === null ? 'idle' : callToolAvailable ? 'available' : 'unavailable'}
            label={callToolAvailable === null ? 'Checking' : callToolAvailable ? 'Available' : 'Unavailable'}
          />
        </div>
      </div>

      <div style={styles.section}>
        <div style={styles.sectionTitle}>iPhone Emulator</div>
        <div style={styles.buttonRow}>
          <button
            style={emulatorStatus === 'running' ? styles.buttonSecondary : styles.button}
            onClick={launchEmulator}
            disabled={emulatorStatus === 'launching' || emulatorStatus === 'running'}
          >
            {emulatorStatus === 'launching' ? 'Launching...' : 'Launch'}
          </button>
          <button
            style={styles.buttonDanger}
            onClick={stopEmulator}
            disabled={emulatorStatus !== 'running'}
          >
            Stop
          </button>
          <StatusBadge
            status={emulatorStatus}
            label={emulatorStatus === 'running' ? 'Running' : emulatorStatus === 'launching' ? 'Launching' : emulatorStatus === 'error' ? 'Error' : 'Stopped'}
          />
        </div>
      </div>

      <div style={styles.section}>
        <div style={styles.sectionTitle}>AI Agent Status</div>
        <div style={styles.statusRow}>
          <span style={styles.statusLabel}>Surgical Editor</span>
          <StatusBadge
            status={surgicalEditorRunning ? 'running' : 'stopped'}
            label={surgicalEditorRunning ? 'Running' : 'Stopped'}
          />
        </div>
        <div style={styles.statusRow}>
          <span style={styles.statusLabel}>Vision System</span>
          <StatusBadge
            status={visionSystemRunning ? 'running' : 'stopped'}
            label={visionSystemRunning ? 'Running' : 'Stopped'}
          />
        </div>
      </div>
    </div>
  );
};

export default AiSettingsPanel;
