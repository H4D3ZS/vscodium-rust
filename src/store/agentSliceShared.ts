// Shared imports, types, and helpers for the three agent slices.
// Split from the 1,267-LOC agentSlice.ts (docs/overhaul MASTER_PLAN A2.3).
import type { StateCreator } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { MAX_AGENT_MESSAGES_IN_UI, MAX_AGENT_MESSAGE_CHARS } from '../domain/agent/AgentSessionPolicy';
import type { AppState } from './index';
import type {
    AgentMessage, AgentStep, Artifact, AttachedContext, AgentTask, TaskArtifact, SemanticSlot,
} from './types';
import type { AgentToolBlock } from '../domain/agent/agentToolBlocks';
import { createToolBlock, enrichCanvasBlockFromResult, enrichEditBlockFromResult } from '../domain/agent/agentToolBlocks';
import { toolsMatchForFinish } from '../domain/agent/toolAliases';
import { cleanAgentContent, shouldReplaceAgentContent } from '../domain/agent/cleanAgentContent';
import { onAgentModeChanged } from '../lib/agentAutonomy';

/** A user-defined agent mode (Kilo-style): name + persona prompt + optional model. */
export interface CustomMode {
    id: string;
    label: string;
    systemPrompt: string;
    model?: string;       // optional "Provider|model" override
    readOnly?: boolean;   // if true, no file writes / commands (chat-like)
}

export function loadCustomModes(): CustomMode[] {
    try {
        const raw = localStorage.getItem('airi.customModes');
        const arr = raw ? JSON.parse(raw) : [];
        return Array.isArray(arr) ? arr : [];
    } catch { return []; }
}

export function parseThought(thought: any): { logic: string; action: string; confidence?: number } | null {
    if (!thought) return null;
    if (typeof thought === 'object') {
        return {
            logic: thought.logic || '',
            action: thought.action || 'Reasoning',
            confidence: thought.confidence !== undefined ? thought.confidence : 0.95
        };
    }
    if (typeof thought === 'string') {
        const trimmed = thought.trim();
        if (trimmed.startsWith('{') && trimmed.endsWith('}')) {
            try {
                const parsed = JSON.parse(trimmed);
                return {
                    logic: parsed.logic || parsed.reasoning || trimmed,
                    action: parsed.action || 'Thinking',
                    confidence: parsed.confidence !== undefined ? parsed.confidence : 0.95
                };
            } catch {
                // fall through
            }
        }
        return {
            logic: trimmed,
            action: 'Reasoning',
            confidence: 0.95
        };
    }
    return null;
}

/** Normalize Tauri `get_agent_messages` / `load_chat_session` payloads. */
export function normalizeBackendMessages(raw: unknown): any[] {
    if (Array.isArray(raw)) return raw;
    if (raw && typeof raw === 'object') {
        const o = raw as Record<string, unknown>;
        if (Array.isArray(o.messages)) return o.messages;
        if (Array.isArray(o.data)) return o.data;
    }
    return [];
}

/** Normalize Rust ChatMessage JSON into UI AgentMessage rows. */
export function mapBackendChatMessages(raw: unknown): AgentMessage[] {
    const extract = (c: any): string => {
        if (c == null) return '';
        if (typeof c === 'string') return c;
        if (Array.isArray(c)) return c.map((p) => extract(p?.text ?? p?.Text ?? p)).join('');
        if (typeof c === 'object') {
            if (typeof c.text === 'string') return c.text;
            if (typeof c.Text === 'string') return c.Text;
            if (typeof c.content === 'string') return c.content;
            if (Array.isArray(c.parts)) return extract(c.parts);
            // serde untagged enum sometimes round-trips as {"Text":"..."}
            const keys = Object.keys(c);
            if (keys.length === 1 && typeof (c as any)[keys[0]] === 'string') {
                return String((c as any)[keys[0]]);
            }
        }
        return String(c);
    };
    return normalizeBackendMessages(raw)
        .filter((m: any) => m && (m.role === 'user' || m.role === 'assistant'))
        .map((m: any, i: number) => ({
            id: m.id || `restored-${i}-${m.role}`,
            role: m.role as 'user' | 'assistant',
            content: extract(m.content),
            timestamp: m.timestamp
                ?? m.metadata?.timestamp
                ?? (typeof m.metadata === 'object' ? m.metadata?.['timestamp'] : undefined)
                ?? Date.now() + i,
            steps: [],
        }))
        .filter((m) => m.content.trim().length > 0);
}

