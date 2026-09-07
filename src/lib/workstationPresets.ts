/**
 * One-click workstation presets — Composer-style "pick stack and go" for local the local backend.
 */

import { useStore } from '../store';

export interface WorkstationPreset {
  id: string;
  label: string;
  desc: string;
  /** the local backend server mode */
  inferenceMode: 'local' | 'remote';
  /** When remote — host or full URL (port 13305 appended if missing) */
  remoteHost?: string;
  planner: string;
  executor: string;
  enableHybrid: boolean;
}

export const WORKSTATION_PRESETS: WorkstationPreset[] = [
  {
    id: 'amd-192',
    label: 'AMD 192 GB — Kimi + MiniMax',
    desc: 'Remote local backend on your GPU server. Planner: Kimi K2.6 · Executor: MiniMax M2.7',
    inferenceMode: 'remote',
    remoteHost: '192.168.1.50',
    planner: 'lemonade|batiai/kimi-k2.6:iq3',
    executor: 'lemonade|batiai/minimax-m2.7:iq3',
    enableHybrid: true,
  },
  {
    id: 'laptop-16',
    label: 'Laptop 16 GB — Qwen Coder',
    desc: 'Local server. Single-model agent on qwen2.5-coder:14b',
    inferenceMode: 'local',
    planner: '',
    executor: 'lemonade|qwen2.5-coder:14b',
    enableHybrid: false,
  },
  {
    id: 'mac-128',
    label: 'Mac 128 GB — MiniMax agent',
    desc: 'Local or remote. MiniMax M2.7 as primary agent',
    inferenceMode: 'local',
    planner: '',
    executor: 'lemonade|batiai/minimax-m2.7:iq3',
    enableHybrid: false,
  },
];

function normalizeRemoteInferenceUrl(hostOrUrl: string): string {
  const raw = hostOrUrl.trim();
  if (!raw) return 'http://127.0.0.1:13305';
  if (raw.startsWith('http://') || raw.startsWith('https://')) return raw.replace(/\/$/, '');
  return `http://${raw.replace(/\/$/, '')}:13305`;
}

export async function applyWorkstationPreset(
  preset: WorkstationPreset,
  remoteHostOverride?: string,
): Promise<void> {
  const st = useStore.getState();

  if (preset.inferenceMode === 'remote') {
    const host = remoteHostOverride?.trim() || preset.remoteHost || '';
    if (!host) throw new Error('Enter your server IP or URL first');
    st.setCustomInferenceUrl?.(normalizeRemoteInferenceUrl(host));
    await st.setInferenceServerMode?.('remote');
  } else {
    await st.setInferenceServerMode?.('local');
  }

  await st.syncInferenceEndpoint?.();
  st.setInferenceBackend?.('lemonade');

  st.setHybridAuto?.(preset.enableHybrid);
  st.setPlannerEnabled?.(preset.enableHybrid);
  if (preset.planner) st.setPlannerModel?.(preset.planner);
  else st.setPlannerModel?.('');

  st.setAgentModel?.(preset.executor);

  try {
    localStorage.setItem('workstation.preset', preset.id);
  } catch { /* ignore */ }

  await st.refreshAvailableModels?.('lemonade');
}
