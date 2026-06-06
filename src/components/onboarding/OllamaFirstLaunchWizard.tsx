import React, { useCallback, useEffect, useMemo, useState } from 'react';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import {
  OLLAMA_RAM_TIERS,
  OLLAMA_WIZARD_KEY,
  LOCAL_VERIFY_KEY,
  filterPresets,
} from '../../lib/ollamaPresets';

type Step = 'intro' | 'detect' | 'pick' | 'pull' | 'done';
type RamTier = (typeof OLLAMA_RAM_TIERS)[number]['id'];

interface LibraryHit {
  name: string;
  ram_gb: number;
  tags: string[];
  desc: string;
}

function modelBase(name: string): string {
  return name.split(':')[0] ?? name;
}

function isInstalled(name: string, local: string[]): boolean {
  const base = modelBase(name);
  return local.some((m) => m === name || m.startsWith(`${base}:`) || modelBase(m) === base);
}

const OllamaFirstLaunchWizard: React.FC = () => {
  const [open, setOpen] = useState(false);
  const [step, setStep] = useState<Step>('intro');
  const [ollamaUp, setOllamaUp] = useState<boolean | null>(null);
  const [localModels, setLocalModels] = useState<string[]>([]);
  const [ramTier, setRamTier] = useState<RamTier>('mid');
  const [search, setSearch] = useState('');
  const [catalog, setCatalog] = useState<LibraryHit[]>([]);
  const [selected, setSelected] = useState<string>('qwen2.5-coder:7b');
  const [pulling, setPulling] = useState(false);
  const [pullError, setPullError] = useState('');
  const [busy, setBusy] = useState(false);

  const setOllamaServerMode = useStore((s) => s.setOllamaServerMode);
  const syncOllamaEndpoint = useStore((s) => s.syncOllamaEndpoint);
  const refreshAvailableModels = useStore((s) => s.refreshAvailableModels);
  const setAgentModel = useStore((s) => s.setAgentModel);
  const setInferenceBackend = useStore((s) => s.setInferenceBackend);
  const pullOllamaModel = useStore((s) => s.pullOllamaModel);
  const isPullingModel = useStore((s) => s.isPullingModel);

  const maxRam = OLLAMA_RAM_TIERS.find((t) => t.id === ramTier)?.maxGb ?? 16;

  useEffect(() => {
    try {
      if (localStorage.getItem(OLLAMA_WIZARD_KEY) === '1') return;
    } catch {
      return;
    }
    setOpen(true);
  }, []);

  const loadCatalog = useCallback(async (q: string) => {
    try {
      const hits = await invoke<LibraryHit[]>('search_ollama_library', {
        query: q,
        limit: 20,
      });
      setCatalog(hits);
    } catch {
      setCatalog([]);
    }
  }, []);

  const detectOllama = useCallback(async () => {
    setBusy(true);
    setStep('detect');
    try {
      await setOllamaServerMode?.('local');
      await syncOllamaEndpoint?.();
      const up = await invoke<boolean>('check_ollama_status');
      setOllamaUp(up);
      let models: string[] = [];
      if (up) {
        models = await invoke<string[]>('list_provider_models', { provider: 'ollama' });
        setLocalModels(models);
      }
      await loadCatalog('');
      const presets = filterPresets('', maxRam);
      const pick =
        presets.find((p) => p.recommended && isInstalled(p.name, models))?.name
        ?? presets.find((p) => p.recommended)?.name
        ?? presets[0]?.name
        ?? 'qwen2.5-coder:7b';
      setSelected(pick);
      setStep('pick');
    } catch {
      setOllamaUp(false);
      setStep('pick');
    } finally {
      setBusy(false);
    }
  }, [loadCatalog, maxRam, setOllamaServerMode, syncOllamaEndpoint]);

  useEffect(() => {
    if (step !== 'pick') return;
    const t = window.setTimeout(() => void loadCatalog(search), 200);
    return () => clearTimeout(t);
  }, [search, step, loadCatalog]);

  const presetCards = useMemo(() => filterPresets(search, maxRam), [search, maxRam]);

  const mergedResults = useMemo(() => {
    const seen = new Set<string>();
    const out: { name: string; desc: string; ram_gb: number; source: string }[] = [];
    for (const p of presetCards) {
      if (seen.has(p.name)) continue;
      seen.add(p.name);
      out.push({ name: p.name, desc: p.desc, ram_gb: p.ramGb, source: 'preset' });
    }
    for (const c of catalog) {
      if (seen.has(c.name)) continue;
      if (c.ram_gb > maxRam && maxRam < 999) continue;
      seen.add(c.name);
      out.push({ name: c.name, desc: c.desc, ram_gb: c.ram_gb, source: 'library' });
    }
    return out.slice(0, 16);
  }, [presetCards, catalog, maxRam]);

  const finishWizard = async (model: string) => {
    try {
      localStorage.setItem(OLLAMA_WIZARD_KEY, '1');
      localStorage.setItem(LOCAL_VERIFY_KEY, '1');
      localStorage.setItem('inferenceBackend', 'ollama');
    } catch { /* */ }
    setInferenceBackend?.('ollama');
    setOllamaServerMode?.('local');
    await syncOllamaEndpoint?.();
    setAgentModel?.(`Ollama|${model}`);
    try {
      localStorage.setItem('agentModel', `Ollama|${model}`);
    } catch { /* */ }
    await refreshAvailableModels?.('ollama');
    setStep('done');
    window.setTimeout(() => setOpen(false), 2200);
  };

  const onPullAndContinue = async () => {
    if (!selected) return;
    setPullError('');
    if (isInstalled(selected, localModels)) {
      await finishWizard(selected);
      return;
    }
    setStep('pull');
    setPulling(true);
    try {
      await pullOllamaModel?.(selected);
      const models = await invoke<string[]>('list_provider_models', { provider: 'ollama' });
      setLocalModels(models);
      if (!isInstalled(selected, models)) {
        setPullError('Pull finished but model not listed yet — try again or pick an installed model.');
        setStep('pick');
        return;
      }
      await finishWizard(selected);
    } catch (e: unknown) {
      setPullError(e instanceof Error ? e.message : String(e));
      setStep('pick');
    } finally {
      setPulling(false);
    }
  };

  const dismiss = () => {
    try {
      localStorage.setItem(OLLAMA_WIZARD_KEY, '1');
    } catch { /* */ }
    setOpen(false);
  };

  if (!open) return null;

  return (
    <div className="ollama-wizard-overlay" role="dialog" aria-modal="true" aria-labelledby="ollama-wizard-title">
      <div className="ollama-wizard-card">
        <header className="ollama-wizard-header">
          <span className="ollama-wizard-badge">Open source · Local-first</span>
          <h2 id="ollama-wizard-title">Ollama-ready in minutes</h2>
          <p className="ollama-wizard-sub">
            Cyber-Ifrit is built for <strong>local open models</strong> — full agent, tools, and checkpoints on your
            hardware. Optional cloud tiers when you need scale.
          </p>
        </header>

        {step === 'intro' && (
          <div className="ollama-wizard-body">
            <ul className="ollama-wizard-list">
              <li>Native agent loop wired to Ollama (not a bolt-on extension)</li>
              <li>Search the model library and pull with one click</li>
              <li>Diff review before you trust local-model edits</li>
            </ul>
            <div className="ollama-wizard-actions">
              <button type="button" className="ollama-wizard-primary" onClick={() => void detectOllama()}>
                Set up local Ollama
              </button>
              <button type="button" className="ollama-wizard-ghost" onClick={dismiss}>
                Skip — I&apos;ll use cloud / BYOK
              </button>
            </div>
          </div>
        )}

        {step === 'detect' && (
          <div className="ollama-wizard-body ollama-wizard-center">
            <div className="ollama-wizard-spinner" aria-hidden />
            <p>{busy ? 'Detecting Ollama on localhost:11434…' : 'Checking…'}</p>
          </div>
        )}

        {(step === 'pick' || step === 'pull') && (
          <div className="ollama-wizard-body">
            {ollamaUp === false && (
              <div className="ollama-wizard-warn">
                Ollama not detected.{' '}
                <a href="https://ollama.com/download" target="_blank" rel="noopener noreferrer">
                  Install Ollama
                </a>{' '}
                and click Re-detect, or skip to use cloud models.
              </div>
            )}

            <div className="ollama-wizard-row">
              <label className="ollama-wizard-label">Your RAM tier</label>
              <div className="ollama-wizard-chips">
                {OLLAMA_RAM_TIERS.map((t) => (
                  <button
                    key={t.id}
                    type="button"
                    className={`ollama-wizard-chip${ramTier === t.id ? ' is-active' : ''}`}
                    onClick={() => setRamTier(t.id)}
                  >
                    {t.label}
                  </button>
                ))}
              </div>
            </div>

            <div className="ollama-wizard-row">
              <label className="ollama-wizard-label" htmlFor="ollama-search">
                Search models
              </label>
              <input
                id="ollama-search"
                className="ollama-wizard-input"
                placeholder="e.g. qwen coder, deepseek, llama…"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                disabled={step === 'pull'}
              />
            </div>

            <div className="ollama-wizard-models">
              {mergedResults.map((m) => {
                const installed = isInstalled(m.name, localModels);
                return (
                  <button
                    key={m.name}
                    type="button"
                    className={`ollama-wizard-model${selected === m.name ? ' is-selected' : ''}`}
                    onClick={() => setSelected(m.name)}
                    disabled={step === 'pull'}
                  >
                    <span className="ollama-wizard-model-name">{m.name}</span>
                    <span className="ollama-wizard-model-meta">
                      ~{m.ram_gb} GB · {installed ? '✓ installed' : 'pull required'}
                    </span>
                    <span className="ollama-wizard-model-desc">{m.desc}</span>
                  </button>
                );
              })}
            </div>

            {pullError && <p className="ollama-wizard-error">{pullError}</p>}

            <div className="ollama-wizard-actions">
              <button
                type="button"
                className="ollama-wizard-primary"
                disabled={!selected || pulling || isPullingModel || step === 'pull'}
                onClick={() => void onPullAndContinue()}
              >
                {pulling || isPullingModel
                  ? 'Pulling model…'
                  : isInstalled(selected, localModels)
                    ? 'Use this model'
                    : `Pull ${selected} & start`}
              </button>
              <button type="button" className="ollama-wizard-ghost" onClick={() => void detectOllama()}>
                Re-detect Ollama
              </button>
              <button type="button" className="ollama-wizard-ghost" onClick={dismiss}>
                Skip wizard
              </button>
            </div>
          </div>
        )}

        {step === 'done' && (
          <div className="ollama-wizard-body ollama-wizard-center">
            <p className="ollama-wizard-success">✓ Local model ready — press <kbd>Ctrl+L</kbd> to open the agent.</p>
            <p className="ollama-wizard-sub">Diff review is on for local models. Review changes before shipping.</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default OllamaFirstLaunchWizard;
