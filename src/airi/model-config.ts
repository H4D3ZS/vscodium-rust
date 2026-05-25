/**
 * AIRI Model Configuration
 *
 * Central place to set which Ollama model to use for each capability.
 *
 * IMPORTANT: On local hardware (single GPU), ALL roles should use the SAME
 * model to prevent GPU model thrashing. Ollama can only hold one model in
 * VRAM at a time on consumer GPUs. Loading a different model for each role
 * causes constant swap-in/swap-out that makes inference take 60+ seconds
 * instead of <2 seconds.
 *
 * The system will auto-detect the agent's active model and use it for all
 * roles unless you explicitly override a role in Settings.
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

// All roles default to the same lightweight model to prevent GPU thrashing.
// On a cloud server with 192GB VRAM, you can set each role to a specialized
// model via Settings → AIRI Model Configuration.
const UNIFIED_DEFAULT = 'airi-fast:latest';

export const MODEL_ROLES: ModelConfig[] = [
  { role: 'consciousness', label: 'Consciousness (thought stream)', default: UNIFIED_DEFAULT },
  { role: 'vision', label: 'Vision (screen analysis)', default: UNIFIED_DEFAULT },
  { role: 'code_fix', label: 'Code Fix (surgical editor)', default: UNIFIED_DEFAULT },
  { role: 'security', label: 'Security (vuln scanning)', default: UNIFIED_DEFAULT },
  { role: 'self_learning', label: 'Self-Learning (reflection)', default: UNIFIED_DEFAULT },
  { role: 'code_gen', label: 'Code Generation', default: UNIFIED_DEFAULT },
  { role: 'social', label: 'Social & Internet', default: UNIFIED_DEFAULT },
];

const STORAGE_KEY = 'airi_model_config';

/** Try to get the active agent model from the store, so all roles use the same model. */
function getAgentModel(): string | null {
  try {
    const stored = typeof localStorage !== 'undefined' ? localStorage.getItem('agentModel') : null;
    if (stored?.includes('|')) {
      const tag = stored.split('|').slice(1).join('|').trim();
      if (tag) return tag;
    }
  } catch { /* tracking prevention */ }
  return null;
}

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

/**
 * Get the model name for a given role.
 * Prefers: user-set override → agent's active model → unified default.
 * This ensures all roles use the SAME model on single-GPU systems.
 */
export function getModel(role: ModelRole): string {
  const explicit = currentModels[role];
  // If the user explicitly set a model for this role AND it's not a stale
  // cloud-era default, use it.
  if (explicit && explicit !== 'llama3.2:3b') return explicit;
  // Otherwise use whatever the agent is set to — prevents GPU thrashing.
  return getAgentModel() || UNIFIED_DEFAULT;
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
