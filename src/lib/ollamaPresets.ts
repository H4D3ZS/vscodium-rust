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
    tags: ['coding', 'agent', 'popular'],
    desc: 'Default for agentic coding on most laptops.',
    recommended: true,
  },
  {
    name: 'deepseek-coder:6.7b',
    ramGb: 8,
    tags: ['coding'],
    desc: 'Lightweight alternative when Qwen is unavailable.',
  },
  {
    name: 'qwen2.5-coder:14b',
    ramGb: 16,
    tags: ['coding', 'agent'],
    desc: 'Stronger edits and tool use on 16 GB machines.',
    recommended: true,
  },
  {
    name: 'deepseek-coder-v2:16b',
    ramGb: 16,
    tags: ['coding', 'popular'],
    desc: 'Community favorite for local codegen.',
  },
  {
    name: 'llama3.2:3b',
    ramGb: 4,
    tags: ['chat', 'small'],
    desc: 'Quick smoke test — not ideal for big refactors.',
  },
  {
    name: 'qwen2.5-coder:32b',
    ramGb: 32,
    tags: ['coding', 'agent'],
    desc: 'Workstation-class local agent model.',
    recommended: true,
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
