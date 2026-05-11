/**
 * Bridge Ollama Desktop model names to on-disk GGUF paths for the Kortex
 * llama.cpp / llama-server stack. The CLI is the only stable cross-platform
 * way to read a model’s Modelfile `FROM` line.
 */

import { invoke } from '../tauri_bridge';

/**
 * Returns the absolute path to the GGUF file backing an Ollama model tag, or
 * `null` if the modelfile does not reference a `.gguf` (e.g. vision adapters).
 * Throws if `ollama` is not on PATH or the model is not pulled.
 */
export async function resolveOllamaModelToGguf(modelTag: string): Promise<string | null> {
    const v = await invoke<string | null>('kortex_resolve_ollama_gguf', { model: modelTag.trim() });
    return v ?? null;
}
