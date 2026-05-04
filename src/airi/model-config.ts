/**
 * AIRI Model Configuration
 *
 * Central place to set which Ollama model to use for each capability.
 * Change these values to match the models you have pulled:
 *   ollama pull llama3.2:3b        # general/consciousness
 *   ollama pull qwen2.5-coder:7b   # code fixes/editing
 *   ollama pull qwen2.5vl          # vision (defaults to 7B if available)
 *   ollama pull hf.co/BugTraceAI/BugTraceAI-Apex-G4-26B-Q4:latest # Red Team
 */

export type ModelRole =
  | 'consciousness'   // AIRI's internal thought stream
  | 'vision'          // Screen analysis & error detection
  | 'code_fix'        // Surgical editor fix generation
  | 'security'        // Security scanning & red team
  | 'self_learning'   // Knowledge acquisition & reflection
  | 'code_gen'        // Code generation & refactoring
  | 'social';         // Social/Internet/Senses

export interface ModelConfig {
  role: ModelRole;
  label: string;
  default: string;
}

export const MODEL_ROLES: ModelConfig[] = [
  { role: 'consciousness', label: 'Consciousness (thought stream)', default: 'huihui_ai/qwen3.5-abliterated:35b' },
  { role: 'vision', label: 'Vision (screen analysis)', default: 'qwen2.5vl:72b' },
  { role: 'code_fix', label: 'Code Fix (surgical editor)', default: 'qwen2.5-coder:7b' },
  { role: 'security', label: 'Security (vuln scanning)', default: 'hf.co/BugTraceAI/BugTraceAI-Apex-G4-26B-Q4:latest' },
  { role: 'self_learning', label: 'Self-Learning (reflection)', default: 'qwen2.5:14b' },
  { role: 'code_gen', label: 'Code Generation', default: 'qwen2.5:32b' },
  { role: 'social', label: 'Social & Internet', default: 'qwen2.5:14b' },
];

const STORAGE_KEY = 'airi_model_config';

function loadModels(): Record<ModelRole, string> {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) {
      const saved = JSON.parse(raw);
      // Merge with defaults so new roles get a value
      const defaults: Record<string, string> = {};
      for (const m of MODEL_ROLES) defaults[m.role] = m.default;
      return { ...defaults as any, ...saved };
    }
  } catch { }
  const defaults: Record<string, string> = {};
  for (const m of MODEL_ROLES) defaults[m.role] = m.default;
  return defaults as Record<ModelRole, string>;
}

function saveModels(models: Record<ModelRole, string>): void {
  try { localStorage.setItem(STORAGE_KEY, JSON.stringify(models)); } catch { }
}

let currentModels: Record<ModelRole, string> = loadModels();

/** Get the model name for a given role. */
export function getModel(role: ModelRole): string {
  return currentModels[role] || MODEL_ROLES.find(m => m.role === role)?.default || 'llama3.2:3b';
}

/** Update the model for a role and persist. */
export function setModel(role: ModelRole, model: string): void {
  currentModels[role] = model;
  saveModels(currentModels);
}

/** Get all current model assignments. */
export function getAllModels(): Record<ModelRole, string> {
  return { ...currentModels };
}

/** Reset all models to defaults. */
export function resetModels(): void {
  const defaults: Record<string, string> = {};
  for (const m of MODEL_ROLES) defaults[m.role] = m.default;
  currentModels = defaults as Record<ModelRole, string>;
  saveModels(currentModels);
}
