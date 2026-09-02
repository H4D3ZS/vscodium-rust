// Barrel for the agent store slices — same external shape as the old
// 1,267-LOC agentSlice.ts (no persistence change, index.ts untouched).

import type { StateCreator } from 'zustand';
import type { AppState } from './index';
import { type AgentMessagesSlice, createAgentMessagesSlice } from './agentMessagesSlice';
import { type AgentToolsSlice, createAgentToolsSlice } from './agentToolsSlice';
import { type AgentModesSlice, createAgentModesSlice } from './agentModesSlice';

export type { CustomMode } from './agentSliceShared';
export { mapBackendChatMessages } from './agentSliceShared';
export type { AgentMessagesSlice, AgentToolsSlice, AgentModesSlice };

export type AgentSlice = AgentMessagesSlice & AgentToolsSlice & AgentModesSlice;

export const createAgentSlice: StateCreator<AppState, [], [], AgentSlice> = (set, get, api) => ({
    ...createAgentMessagesSlice(set, get, api),
    ...createAgentToolsSlice(set, get, api),
    ...createAgentModesSlice(set, get, api),
});
