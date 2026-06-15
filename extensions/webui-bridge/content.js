// HADES WebUI Bridge — content script (runs on the chat page).
//
// Eyes + hands only. All logic (parsing tool calls, executing them) lives in the IDE
// bridge. This script:
//   1. detects the provider from the hostname,
//   2. observes the newest assistant message and forwards it to the bridge,
//   3. injects bridge-sent text into the composer and submits,
//   4. shows a small status panel.

(() => {
  'use strict';
  if (window.__hadesWebuiBridgeInstalled) return;
  window.__hadesWebuiBridgeInstalled = true;

  // ── Provider + selectors (ported from auth.rs) ──────────────────────────────
  const HOST = location.hostname;
  const PROVIDER =
    HOST.includes('claude.ai') ? 'claude' :
    (HOST.includes('chatgpt.com') || HOST.includes('openai.com')) ? 'openai' :
    HOST.includes('gemini.google.com') ? 'gemini' :
    HOST.includes('chat.deepseek.com') ? 'deepseek' :
    HOST.includes('chat.qwen.ai') ? 'qwen' : null;

  if (!PROVIDER) return;

  const SELECTORS = {
    claude: {
      msg: '[data-testid*="message"], div.font-claude-message, div.prose',
      input: 'div[contenteditable="true"], textarea',
      send: 'button[aria-label="Send message"], button[aria-label="Send Message"], button[type="submit"]',
    },
    openai: {
      msg: '[data-message-author-role="assistant"], article div.markdown, .markdown',
      input: '#prompt-textarea, textarea, div[contenteditable="true"]',
      send: 'button[data-testid="send-button"], button[aria-label="Send prompt"], button[aria-label="Send message"]',
    },
    gemini: {
      msg: 'message-content, model-response, .model-response-text, .markdown',
      input: 'div.ql-editor, textarea, div[contenteditable="true"]',
      send: 'button[aria-label*="Send"], button.send-button, button[mattooltip*="Send"]',
    },
    deepseek: {
      msg: '[class*="message"], [class*="answer"], .markdown, article',
      input: 'textarea, div[contenteditable="true"]',
      send: 'button[type="submit"], button[aria-label*="Send"]',
    },
    qwen: {
      msg: '[class*="message"], [class*="answer"], .markdown, article',
      input: 'textarea, div[contenteditable="true"]',
      send: 'button[type="submit"], button[aria-label*="Send"]',
    },
  };
  const SEL = SELECTORS[PROVIDER];

  // ── State ───────────────────────────────────────────────────────────────────
  let connected = false;
  let sessionActive = false;
  let protocolText = '';
  let lastForwarded = '';
  let settleTimer = null;
  const SETTLE_MS = 1500;

  // ── Messaging with background SW ────────────────────────────────────────────
  function toBridge(payload) {
    chrome.runtime.sendMessage({ type: 'bridge_send', payload }).catch(() => {});
  }
  chrome.runtime.sendMessage({ type: 'bridge_connect', provider: PROVIDER }).catch(() => {});

  chrome.runtime.onMessage.addListener((msg) => {
    if (msg.type === 'bridge_status') {
      connected = !!msg.connected;
      renderPanel();
    } else if (msg.type === 'bridge_msg') {
      handleBridge(msg.payload);
    }
  });

  function handleBridge(msg) {
    switch (msg.type) {
      case 'hello_ack':
        protocolText = msg.protocol || '';
        setStatus('ready');
        break;
      case 'system_prompt':
        // Command/supervisor-driven start: protocol + task is already in the text.
        sessionActive = true;
        renderPanel();
        inject(msg.text);
        setStatus('task injected');
        break;
      case 'inject':
        inject(msg.text);
        setStatus('tool results injected');
        break;
      case 'ping':
        toBridge({ type: 'pong' });
        break;
    }
  }

  // ── DOM: read newest assistant message ──────────────────────────────────────
  function pickAssistant() {
    const nodes = Array.from(document.querySelectorAll(SEL.msg));
    const texts = nodes
      .map((n) => (n.innerText || n.textContent || '').trim())
      .filter((t) => t.length > 10);
    return texts[texts.length - 1] || '';
  }

  function onMutation() {
    if (!sessionActive) return;
    clearTimeout(settleTimer);
    settleTimer = setTimeout(() => {
      const text = pickAssistant();
      if (!text || text === lastForwarded) return;
      lastForwarded = text;
      toBridge({ type: 'assistant_text', text });
      setStatus('forwarded reply (' + text.length + ' chars)');
    }, SETTLE_MS);
  }

  const observer = new MutationObserver(onMutation);
  observer.observe(document.body, { childList: true, subtree: true, characterData: true });

  // ── DOM: inject text into composer + submit ─────────────────────────────────
  function setNativeValue(el, value) {
    const proto =
      el.tagName === 'TEXTAREA' ? HTMLTextAreaElement.prototype : HTMLInputElement.prototype;
    const desc = Object.getOwnPropertyDescriptor(proto, 'value');
    if (desc && desc.set) desc.set.call(el, value);
    else el.value = value;
  }

  function inject(text) {
    const input = document.querySelector(SEL.input);
    if (!input) {
      setStatus('⚠ composer not found');
      return false;
    }
    input.focus();
    const isTextarea = input.tagName === 'TEXTAREA' || 'value' in input && input.tagName !== 'DIV';
    if (isTextarea) {
      setNativeValue(input, text);
      input.dispatchEvent(new Event('input', { bubbles: true }));
      input.dispatchEvent(new Event('change', { bubbles: true }));
    } else {
      // contenteditable (Claude ProseMirror, Gemini Quill, etc.)
      input.textContent = '';
      try { document.execCommand('insertText', false, text); } catch {}
      input.dispatchEvent(new InputEvent('input', { bubbles: true, inputType: 'insertText', data: text }));
    }
    setTimeout(() => {
      const send = document.querySelector(SEL.send);
      if (send && !send.disabled) {
        send.click();
      } else {
        // Fallback: press Enter.
        input.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', code: 'Enter', keyCode: 13, bubbles: true }));
        input.dispatchEvent(new KeyboardEvent('keyup', { key: 'Enter', code: 'Enter', keyCode: 13, bubbles: true }));
      }
    }, 300);
    return true;
  }

  // ── Status panel ────────────────────────────────────────────────────────────
  let panel, dot, statusLine, startBtn;

  function buildPanel() {
    panel = document.createElement('div');
    panel.id = 'hades-bridge-panel';
    Object.assign(panel.style, {
      position: 'fixed', bottom: '16px', right: '16px', zIndex: '2147483647',
      width: '220px', padding: '10px 12px', borderRadius: '10px',
      background: '#0d1117', color: '#e6edf3', border: '1px solid rgba(255,255,255,0.12)',
      font: '12px -apple-system, "Segoe UI", sans-serif', boxShadow: '0 6px 24px rgba(0,0,0,0.4)',
    });

    const header = document.createElement('div');
    Object.assign(header.style, { display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '8px' });
    dot = document.createElement('span');
    Object.assign(dot.style, { width: '9px', height: '9px', borderRadius: '50%', background: '#8b949e', flexShrink: '0' });
    const title = document.createElement('span');
    title.textContent = 'HADES Bridge';
    Object.assign(title.style, { fontWeight: '700', flex: '1' });
    const prov = document.createElement('span');
    prov.textContent = PROVIDER;
    Object.assign(prov.style, { fontFamily: 'monospace', fontSize: '10px', color: '#79c0ff' });
    header.append(dot, title, prov);

    statusLine = document.createElement('div');
    Object.assign(statusLine.style, { fontSize: '10px', color: '#8b949e', marginBottom: '8px', minHeight: '12px', wordBreak: 'break-word' });

    const btnRow = document.createElement('div');
    Object.assign(btnRow.style, { display: 'flex', gap: '6px' });
    startBtn = document.createElement('button');
    styleBtn(startBtn, '#238636');
    startBtn.onclick = toggleSession;
    btnRow.append(startBtn);

    panel.append(header, statusLine, btnRow);
    document.body.appendChild(panel);
    renderPanel();
  }

  function styleBtn(b, bg) {
    Object.assign(b.style, {
      flex: '1', padding: '5px 8px', border: 'none', borderRadius: '6px',
      background: bg, color: '#fff', fontSize: '11px', fontWeight: '600', cursor: 'pointer',
    });
  }

  function toggleSession() {
    sessionActive = !sessionActive;
    if (sessionActive) {
      // Manual standalone start: prime the chat with the tool protocol, then the
      // user types their task. Tool calls in the reply will fire via the bridge.
      if (protocolText) inject(protocolText);
      setStatus('session ON — type your task in the chat');
    } else {
      setStatus('session paused');
    }
    renderPanel();
  }

  function setStatus(s) {
    if (statusLine) statusLine.textContent = s;
  }

  function renderPanel() {
    if (!dot) return;
    dot.style.background = connected ? (sessionActive ? '#3fb950' : '#d29922') : '#8b949e';
    startBtn.textContent = sessionActive ? '■ Stop' : '▶ Start';
    startBtn.style.background = sessionActive ? '#7d2d2d' : '#238636';
  }

  if (document.body) buildPanel();
  else window.addEventListener('DOMContentLoaded', buildPanel);
})();
