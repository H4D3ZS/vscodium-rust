/** Curated Ollama presets for first-run wizard and settings. */

export interface OllamaPreset {
  name: string;
  ramGb: number;
  tags: string[];
  desc: string;
  recommended?: boolean;
}

export const OLLAMA_RAM_TIERS = [
  { id: 'lite', label: '≤8 GB RAM', maxGb: 8 },
  { id: 'mid', label: '16 GB RAM', maxGb: 16 },
  { id: 'pro', label: '32 GB+ RAM', maxGb: 999 },
] as const;

export const OLLAMA_PRESETS: OllamaPreset[] = [
  {
    name: 'qwen2.5-coder:7b',
    ramGb: 8,
    tags: ['coding', 'agent', 'popular', 'local'],
    desc: 'Default for agentic coding on most laptops — unlimited local.',
    recommended: true,
  },
  {
    name: 'batiai/qwen3.6-35b:iq4',
    ramGb: 48,
    tags: ['agent', 'multimodal', 'local', 'frontier'],
    desc: 'Qwen 3.6 35B — best dense local agent for 48 GB+ (pull once, no cloud).',
    recommended: true,
  },
  {
    name: 'batiai/minimax-m2.7:iq3',
    ramGb: 128,
    tags: ['agent', 'coding', 'local', 'frontier'],
    desc: 'MiniMax M2.7 — top local coding agent on 128 GB Mac.',
    recommended: true,
  },
  {
    name: 'batiai/gemma4-e4b:q4',
    ramGb: 16,
    tags: ['small', 'local', 'frontier'],
    desc: 'Gemma 4 SLM — 16 GB tier, fully offline.',
  },
  {
    name: 'deepseek-coder:6.7b',
    ramGb: 8,
    tags: ['coding', 'local'],
    desc: 'Lightweight alternative when Qwen is unavailable.',
  },
  {
    name: 'qwen2.5-coder:14b',
    ramGb: 16,
    tags: ['coding', 'agent', 'local'],
    desc: 'Stronger edits and tool use on 16 GB machines.',
    recommended: true,
  },
  {
    name: 'deepseek-r1:8b',
    ramGb: 8,
    tags: ['reasoning', 'planner', 'local'],
    desc: 'Local planner/reasoning model — pair with a fast coder executor.',
  },
  {
    name: 'deepseek-coder-v2:16b',
    ramGb: 16,
    tags: ['coding', 'popular', 'local'],
    desc: 'Community favorite for local codegen.',
  },
  {
    name: 'llama3.2:3b',
    ramGb: 4,
    tags: ['chat', 'small', 'local'],
    desc: 'Quick smoke test — not ideal for big refactors.',
  },
  {
    name: 'qwen2.5-coder:32b',
    ramGb: 32,
    tags: ['coding', 'agent', 'local'],
    desc: 'Workstation-class local agent model.',
    recommended: true,
  },
  {
    name: 'batiai/kimi-k2.6:iq3',
    ramGb: 384,
    tags: ['agent', 'vision', 'local', 'frontier', 'workstation'],
    desc: 'Kimi K2.6 — #1 open agent; needs 384 GB+ RAM (not Ollama Cloud).',
  },
];

export function presetsForRam(maxGb: number): OllamaPreset[] {
  return OLLAMA_PRESETS.filter((p) => p.ramGb <= maxGb);
}

export function filterPresets(query: string, maxGb?: number): OllamaPreset[] {
  const q = query.trim().toLowerCase();
  let list = maxGb != null ? presetsForRam(maxGb) : OLLAMA_PRESETS;
  if (!q) return list;
  return list.filter(
    (p) =>
      p.name.toLowerCase().includes(q) ||
      p.desc.toLowerCase().includes(q) ||
      p.tags.some((t) => t.includes(q)),
  );
}

export const OLLAMA_WIZARD_KEY = 'ollamaWizard.completed';
export const LOCAL_VERIFY_KEY = 'localVerifyMode';
