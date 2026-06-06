/**
 * Local-only Ollama model registry — unlimited agentic use, no Ollama Cloud.
 * Pull tags from community registries (mostly batiai/*) or HF → Modelfile when noted.
 */

export type LocalModelRole = 'agent' | 'planner' | 'executor' | 'coding' | 'vision';

export interface LocalOllamaEntry {
  /** `ollama pull` tag — runs 100% on your machine once downloaded. */
  pullTag: string;
  ramGb: number;
  tags: string[];
  role: LocalModelRole;
  desc: string;
  /** HuggingFace repo when manual GGUF import is required. */
  hfRepo?: string;
  /** Extra setup (bati.cpp, Ollama 0.30+, weak tool format, etc.) */
  setupNote?: string;
  recommended?: boolean;
  rank?: number;
}

/** Leaderboard-aligned models that have a realistic LOCAL path (not Ollama Cloud). */
export const LOCAL_OLLAMA_REGISTRY: LocalOllamaEntry[] = [
  {
    pullTag: 'batiai/minimax-m2.7:iq3',
    ramGb: 128,
    tags: ['agent', 'coding', 'self-improve', 'rank-10'],
    role: 'agent',
    desc: 'MiniMax M2.7 — best local coding agent on 128 GB Mac (229B dense, ~82 GB).',
    hfRepo: 'batiai/MiniMax-M2.7-GGUF',
    recommended: true,
    rank: 10,
  },
  {
    pullTag: 'batiai/qwen3.6-35b:iq4',
    ramGb: 48,
    tags: ['agent', 'multimodal', 'coding', 'rank-7'],
    role: 'agent',
    desc: 'Qwen 3.6 35B — strong dense agent on 48–96 GB machines.',
    hfRepo: 'batiai/Qwen3.6-35B-GGUF',
    recommended: true,
    rank: 7,
  },
  {
    pullTag: 'batiai/qwen3.5-35b:iq4',
    ramGb: 48,
    tags: ['multimodal', 'agent'],
    role: 'agent',
    desc: 'Qwen 3.5 35B — multimodal-capable local agent (48 GB+).',
    recommended: true,
  },
  {
    pullTag: 'batiai/kimi-k2.6:iq3',
    ramGb: 384,
    tags: ['agent', 'vision', 'long-horizon', 'rank-1'],
    role: 'planner',
    desc: 'Kimi K2.6 IQ3 — #1 open agent; needs 384 GB+ unified RAM (MoE 1T/32B active).',
    hfRepo: 'batiai/Kimi-K2.6-GGUF',
    setupNote: 'Workstation / M3 Ultra 512 GB class. Not for 16–64 GB laptops.',
    rank: 1,
  },
  {
    pullTag: 'batiai/kimi-k2.6:iq4',
    ramGb: 512,
    tags: ['agent', 'vision', 'rank-1'],
    role: 'planner',
    desc: 'Kimi K2.6 IQ4 — higher quality; 512 GB+ RAM.',
    hfRepo: 'batiai/Kimi-K2.6-GGUF',
    rank: 1,
  },
  {
    pullTag: 'frob/glm-5.1',
    ramGb: 466,
    tags: ['long-horizon', 'agent', 'rank-4'],
    role: 'planner',
    desc: 'GLM-5.1 via community import (~466 GB). Ollama 0.30+; tool format may need parser tuning.',
    hfRepo: 'unsloth/GLM-5.1-GGUF',
    setupNote: 'Agent tool-calling weaker than Kimi/MiniMax until Ollama parser template lands.',
    rank: 4,
  },
  {
    pullTag: 'qwen2.5-coder:14b',
    ramGb: 16,
    tags: ['coding', 'agent'],
    role: 'coding',
    desc: 'Qwen 2.5 Coder 14B — proven local agent on 16 GB.',
    recommended: true,
  },
  {
    pullTag: 'qwen2.5-coder:7b',
    ramGb: 8,
    tags: ['coding', 'agent', 'popular'],
    role: 'coding',
    desc: 'Default laptop coder — fast, unlimited, 8 GB+.',
    recommended: true,
  },
  {
    pullTag: 'batiai/gemma4-e4b:q4',
    ramGb: 16,
    tags: ['small', 'rank-9'],
    role: 'agent',
    desc: 'Gemma 4 12B-class — best SLM tier on 16 GB (rank #9 dense).',
    hfRepo: 'batiai/Gemma4-E4B-GGUF',
    rank: 9,
  },
  {
    pullTag: 'batiai/gemma4-26b:iq4',
    ramGb: 24,
    tags: ['chat', 'coding'],
    role: 'agent',
    desc: 'Gemma 4 26B — step up from 16 GB tier.',
    hfRepo: 'batiai/Gemma4-26B-GGUF',
  },
  {
    pullTag: 'deepseek-r1:8b',
    ramGb: 8,
    tags: ['reasoning', 'planner'],
    role: 'planner',
    desc: 'DeepSeek R1 8B — local reasoning/planner on modest hardware.',
  },
];

/** Strip `:cloud` suffix — never use Ollama Cloud IDs for local agent runs. */
export function sanitizeLocalOllamaTag(id: string): string {
  let s = id.trim();
  if (s.endsWith(':cloud')) s = s.slice(0, -':cloud'.length);
  if (s.endsWith('-cloud')) s = s.slice(0, -'-cloud'.length);
  return s;
}

export function localModelsForRam(maxGb: number): LocalOllamaEntry[] {
  return LOCAL_OLLAMA_REGISTRY.filter((m) => m.ramGb <= maxGb);
}

export function recommendedLocalForRam(maxGb: number): LocalOllamaEntry | undefined {
  const fits = localModelsForRam(maxGb);
  return (
    fits.find((m) => m.recommended && m.role === 'agent')
    ?? fits.find((m) => m.recommended)
    ?? fits.sort((a, b) => (b.rank ?? 0) - (a.rank ?? 0))[0]
  );
}

export function filterLocalRegistry(query: string, maxGb?: number): LocalOllamaEntry[] {
  const q = query.trim().toLowerCase();
  let list = maxGb != null ? localModelsForRam(maxGb) : LOCAL_OLLAMA_REGISTRY;
  if (!q) return list;
  return list.filter(
    (m) =>
      m.pullTag.toLowerCase().includes(q)
      || m.desc.toLowerCase().includes(q)
      || m.tags.some((t) => t.includes(q)),
  );
}

/** IDs to show in picker after sanitizing — does not inject cloud-only names. */
export function mergeLocalRegistryHints(existing: string[], maxRamGb?: number): string[] {
  const seen = new Set(existing.map(sanitizeLocalOllamaTag));
  const out = [...existing.map(sanitizeLocalOllamaTag)];
  const hints = maxRamGb != null ? localModelsForRam(maxRamGb) : LOCAL_OLLAMA_REGISTRY;
  for (const m of hints) {
    const tag = sanitizeLocalOllamaTag(m.pullTag);
    if (!seen.has(tag)) {
      seen.add(tag);
      out.push(tag);
    }
  }
  return out;
}
