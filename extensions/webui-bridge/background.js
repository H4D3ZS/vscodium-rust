// HADES WebUI Bridge — background service worker.
//
// Holds one WebSocket per chat tab to the in-IDE bridge (ws://127.0.0.1:1538).
// The WebSocket MUST live here (not in the content script) because chat sites' CSP
// blocks connections to localhost from page context; the extension background page
// is exempt and reaches localhost via host_permissions.
//
// Relays:  content script  <—chrome.runtime—>  background  <—WebSocket—>  IDE bridge

const BRIDGE_URL = 'ws://127.0.0.1:1538';

/** tabId -> { ws, provider, ready } */
const conns = new Map();

function log(...a) { console.log('[webui-bridge:bg]', ...a); }

function connect(tabId, provider) {
  const existing = conns.get(tabId);
  if (existing && existing.ws && existing.ws.readyState <= WebSocket.OPEN) return;

  let ws;
  try {
    ws = new WebSocket(BRIDGE_URL);
  } catch (e) {
    log('WebSocket ctor failed', e);
    scheduleReconnect(tabId, provider);
    return;
  }

  const conn = { ws, provider, ready: false };
  conns.set(tabId, conn);

  ws.onopen = () => {
    conn.ready = true;
    ws.send(JSON.stringify({ type: 'hello', provider, tab_id: `tab_${tabId}` }));
    sendToTab(tabId, { type: 'bridge_status', connected: true });
    log('connected', tabId, provider);
  };

  ws.onmessage = (ev) => {
    let msg;
    try { msg = JSON.parse(ev.data); } catch { return; }
    sendToTab(tabId, { type: 'bridge_msg', payload: msg });
  };

  ws.onclose = () => {
    conn.ready = false;
    sendToTab(tabId, { type: 'bridge_status', connected: false });
    log('closed', tabId);
    scheduleReconnect(tabId, provider);
  };

  ws.onerror = () => {
    try { ws.close(); } catch {}
  };
}

function scheduleReconnect(tabId, provider) {
  setTimeout(() => {
    // Only reconnect if the tab still exists.
    chrome.tabs.get(tabId).then(() => connect(tabId, provider)).catch(() => {
      conns.delete(tabId);
    });
  }, 3000);
}

function sendToTab(tabId, message) {
  chrome.tabs.sendMessage(tabId, message).catch(() => {});
}

chrome.runtime.onMessage.addListener((msg, sender, sendResponse) => {
  const tabId = sender.tab && sender.tab.id;
  if (tabId == null) return;

  switch (msg.type) {
    case 'bridge_connect':
      connect(tabId, msg.provider);
      sendResponse && sendResponse({ ok: true });
      return true;
    case 'bridge_send': {
      const c = conns.get(tabId);
      if (c && c.ready) {
        try { c.ws.send(JSON.stringify(msg.payload)); } catch {}
      }
      return;
    }
    case 'bridge_disconnect': {
      const c = conns.get(tabId);
      if (c && c.ws) { try { c.ws.close(); } catch {} }
      conns.delete(tabId);
      return;
    }
  }
});

chrome.tabs.onRemoved.addListener((tabId) => {
  const c = conns.get(tabId);
  if (c && c.ws) { try { c.ws.close(); } catch {} }
  conns.delete(tabId);
});

// Keepalive: an open WebSocket extends the SW lifetime, and this alarm both pings
// live connections and revives any that died while the SW was suspended.
chrome.alarms.create('keepalive', { periodInMinutes: 0.5 });
chrome.alarms.onAlarm.addListener((alarm) => {
  if (alarm.name !== 'keepalive') return;
  for (const [tabId, c] of conns) {
    if (c.ws && c.ready) {
      try { c.ws.send(JSON.stringify({ type: 'pong' })); } catch {}
    } else {
      connect(tabId, c.provider);
    }
  }
});
