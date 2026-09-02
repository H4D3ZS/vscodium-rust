# HADES WebUI Bridge (browser extension)

Lets a **free web-chat model** (Claude / ChatGPT / Gemini / DeepSeek / Qwen) drive the
HADES IDE's real tools — read/write files, run commands, `cargo check`, etc. — with **no
API key and no subscription**. Inspired by ZeroScript, but the bridge runs *inside the IDE*
(WebSocket server on `127.0.0.1:1538`), so there's no separate Python process to install.

```
 chat tab  ──content.js──▶  background.js ──WebSocket──▶  HADES IDE bridge ──▶ AiTools
   ▲  (observes reply,                                      (parses tool calls,
   └───injects results)  ◀──────────────────────────────────executes, returns)
```

## One-time setup

1. **Start the IDE.** The bridge auto-starts on `ws://127.0.0.1:1538` ~20s after launch
   (skipped in "lite/potato" mode on <9 GB RAM machines).
2. **Load the extension** (Chrome or Edge):
   - Open `chrome://extensions` (or `edge://extensions`).
   - Toggle **Developer mode** on.
   - **Load unpacked** → select this folder (`extensions/webui-bridge`).
3. **Log in** to whichever chat you want to use (claude.ai, chatgpt.com, gemini.google.com,
   chat.deepseek.com, chat.qwen.ai) in a normal tab — your existing free session is fine.

## Using it

- A small **HADES Bridge** panel appears bottom-right on a supported chat page.
  - **Grey dot** = bridge not reachable (is the IDE running?).
  - **Amber dot** = connected, session idle.
  - **Green dot** = connected, session active (forwarding replies).
- **Manual mode:** click **▶ Start**. The tool protocol is injected into the chat; then just
  type your task ("refactor src/foo.rs to…"). When the model emits a tool call, the bridge
  executes it and pastes the result back, and the model continues until it writes
  `TASK_COMPLETE`.
- **Driven mode:** the IDE's Mission Control / `webui_bridge_start_task` command injects the
  task for you and watches for completion — no need to click Start.

## How tool calls work

The model is told to emit fenced JSON like:

````
```json
{"tool": "view_file", "args": {"path": "src/main.rs"}}
```
````

`content.js` forwards the assistant message to the bridge; the bridge parses the call with
the same `parse_webui_tool_calls` used everywhere else, runs it through `AiTools` (the exact
registry behind the `:1537` MCP server), and injects the result back into the chat.

## Troubleshooting

- **Grey dot won't go amber:** the IDE isn't running, you're in lite mode, or `:1538` is taken.
  Check the IDE console for `[webui-bridge] ✅ listening`.
- **"⚠ composer not found":** the site changed its DOM. Update the selector map at the top of
  `content.js` (`SELECTORS[provider]`).
- **Reply not forwarded:** make sure the dot is **green** (session active). The script waits
  ~1.5 s after the reply stops changing before forwarding, to avoid catching it mid-stream.

## Security

This automates **your own** logged-in chat sessions against **your own** workspace, for your
own dev tasks. The bridge only listens on loopback (`127.0.0.1`). Tool execution honors the
IDE's existing tool entitlement gating. Don't point it at workspaces or sites you aren't
authorized to modify.
