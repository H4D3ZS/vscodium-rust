import { describe, it, expect } from 'vitest';
import { classifyModels, modelKey } from '../../model_capabilities';

// Hybrid planner/executor auto-detection. The strongest available model should
// plan (deep reasoning), the fastest capable model should execute (act/edit).
describe('classifyModels — hybrid planner/executor auto-detect', () => {
    it('returns nulls when no models are available', () => {
        expect(classifyModels([])).toEqual({ planner: null, executor: null });
    });

    it('collapses both roles to the only model when one is installed', () => {
        const only = { id: 'qwen2.5-coder:7b', provider: 'Ollama' };
        const pick = classifyModels([only]);
        expect(pick.planner).toEqual(only);
        expect(pick.executor).toEqual(only);
    });

    it('picks a frontier cloud planner + a fast coder executor', () => {
        const pick = classifyModels([
            { id: 'qwen2.5-coder:7b', provider: 'Ollama' },
            { id: 'claude-opus-4-0', provider: 'Anthropic' },
        ]);
        expect(pick.planner?.id).toBe('claude-opus-4-0');
        expect(pick.executor?.id).toBe('qwen2.5-coder:7b');
    });

    it('prefers Gemini Pro to plan and a small local model to execute', () => {
        const pick = classifyModels([
            { id: 'gemini-2.5-pro', provider: 'Google' },
            { id: 'llama3.2:3b', provider: 'Ollama' },
        ]);
        expect(pick.planner?.id).toBe('gemini-2.5-pro');
        expect(pick.executor?.id).toBe('llama3.2:3b');
    });

    it('never uses a 30B+ local model as planner on all-Ollama rigs', () => {
        const pick = classifyModels([
            { id: 'aware/qwen3.6-40b-deck-opus-neo-code:latest', provider: 'Ollama' },
            { id: 'gemma4:12b', provider: 'Ollama' },
            { id: 'qwen2.5-coder:7b', provider: 'Ollama' },
        ]);
        expect(pick.planner?.id).toBe('qwen2.5-coder:7b');
        expect(pick.executor?.id).toBe('qwen2.5-coder:7b');
    });

    it('prefers a small local model over a large one when all models are local', () => {
        const pick = classifyModels([
            { id: 'qwen2.5:32b', provider: 'Ollama' },
            { id: 'qwen2.5-coder:7b', provider: 'Ollama' },
        ]);
        expect(pick.planner?.id).toBe('qwen2.5-coder:7b');
        expect(pick.executor?.id).toBe('qwen2.5-coder:7b');
    });

    it('never returns the same model for both roles when two distinct models exist', () => {
        const pick = classifyModels([
            { id: 'claude-sonnet-4-0', provider: 'Anthropic' },
            { id: 'gpt-4o', provider: 'OpenAI' },
        ]);
        expect(modelKey(pick.planner!)).not.toBe(modelKey(pick.executor!));
    });
});

describe('modelKey', () => {
    it('formats as lowercased provider | id (matches set_advisor_model)', () => {
        expect(modelKey({ id: 'claude-opus-4-0', provider: 'Anthropic' })).toBe('anthropic|claude-opus-4-0');
    });
});
