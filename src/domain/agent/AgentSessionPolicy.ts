/**
 * Domain policy: how much agent state we keep in RAM.
 *
 * WHY cap messages?
 * Each turn stores full model output + tool steps. Uncapped arrays in Zustand
 * and duplicate history buffers were pushing the WebView past 1 GB in long sessions.
 * The store and backend both enforce these limits.
 */
export const MAX_AGENT_MESSAGES_IN_UI = 40;
export const MAX_AGENT_MESSAGE_CHARS = 12_000;
export const MAX_WEBUI_RESPONSE_CACHE_ENTRIES = 8;
