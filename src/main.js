import { NotebookRenderer } from './notebook_renderer.js';
const { invoke } = window.__TAURI__.core;

async function openFolder() {
  try {
    const { open } = window.__TAURI__.dialog;
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (selected) {
      loadDirectory(selected);
    }
  } catch (e) {
    console.error("Open folder failed:", e);
  }
}

async function openSettings() {
  try {
    const settings = await invoke("get_settings");
    alert(`Current Settings:\nTheme: ${settings.theme}\nFont Size: ${settings.font_size}`);
  } catch (e) {
    console.error("Failed to load settings:", e);
  }
}

let editor;
let lineNumbers;
let sidebarContent;
let explorerView;
let searchView;
let searchInput;
let searchResults;
let sidebarTitle;
let extensionsView;
let extensionsSearchInput;
let installedExtensionsList;
let marketplaceExtensionsList;
let debugView;
let debugVariables;
let debugCallStack;
let debugBreakpoints;
let debugToolbar;
let terminal;
let minimapCanvas;
let minimapCtx;

let breakpoints = []; // { path: string, line: number }

let openTabs = []; // { path: string, name: string, active: boolean }

function addTab(path, name) {
  openTabs.forEach(t => t.active = false);
  const existing = openTabs.find(t => t.path === path);
  if (existing) {
    existing.active = true;
  } else {
    openTabs.push({ path, name, active: true });
  }
  renderTabs();
}

function renderTabs() {
  const tabsRow = document.querySelector(".tabs-row");
  tabsRow.innerHTML = "";
  openTabs.forEach(tab => {
    const div = document.createElement("div");
    div.className = "tab" + (tab.active ? " active" : "");
    div.innerText = tab.name;
    div.onclick = () => switchTab(tab.path);
    tabsRow.appendChild(div);
  });
}

let notebookRenderer = new NotebookRenderer('notebook-view');

async function switchTab(path) {
  try {
    const content = await invoke("switch_to_buffer", { path });

    if (path.endsWith('.ipynb')) {
      document.getElementById('editor').classList.add('hidden');
      document.getElementById('line-numbers').classList.add('hidden');

      notebookRenderer.render(content);
      invoke("set_context_key", { key: "resourceLangId", value: "notebook" });
    } else {
      document.getElementById('editor').classList.remove('hidden');
      document.getElementById('line-numbers').classList.remove('hidden');
      document.getElementById('notebook-view').classList.add('hidden');

      editor.innerText = content;
      invoke("set_context_key", { key: "resourceLangId", value: "plaintext" }); // simplified
    }

    window.activeFilePath = path;
    if (path.endsWith('.rs')) {
      startLsp("rust-analyzer");
    }

    openTabs.forEach(t => t.active = (t.path === path));
    renderTabs();
    if (!path.endsWith('.ipynb')) updateLineNumbers();

    // Update window title and breadcrumbs
    const activeTab = openTabs.find(t => t.active);
    if (activeTab) {
      document.getElementById("window-title").innerText = `${activeTab.name} - vscode-rust - VSCodium Rust`;
      document.getElementById("breadcrumbs").innerText = `vscode-rust > ${activeTab.path.replace(/\//g, " > ")}`;
    }
  } catch (e) {
    console.error("Failed to switch tab:", e);
  }
}

function switchSidebarView(viewId) {
  explorerView.classList.add("hidden");
  searchView.classList.add("hidden");
  extensionsView.classList.add("hidden"); // New

  // Sync activity bar
  const activityItems = document.querySelectorAll(".activity-item");
  activityItems.forEach(item => {
    const title = item.getAttribute("title");
    if (title.toLowerCase() === viewId) item.classList.add("active");
    else item.classList.remove("active");
  });

  if (viewId === "explorer") {
    explorerView.classList.remove("hidden");
    sidebarTitle.innerText = "Explorer";
  } else if (viewId === "search") {
    searchView.classList.remove("hidden");
    sidebarTitle.innerText = "Search";
    searchInput.focus();
  } else if (viewId === "extensions") {
    extensionsView.classList.remove("hidden");
    sidebarTitle.innerText = "Extensions";
    loadExtensions();
  } else if (viewId === "run and debug") {
    debugView.classList.remove("hidden");
    sidebarTitle.innerText = "Run and Debug";
    renderBreakpoints();
  } else if (viewId === "source control") {
    sourceControlView.classList.remove("hidden");
    sidebarTitle.innerText = "Source Control";
    scmInput.focus();
    updateGitUI();
  }
}

async function runSearch(query) {
  if (!query) {
    searchResults.innerHTML = "";
    return;
  }
  try {
    const results = await invoke("search_project", { query });
    renderSearchResults(results);
  } catch (e) {
    console.error("Search failed:", e);
  }
}

async function updateGitStatus() {
  try {
    const branch = await invoke("get_git_branch");
    const status = await invoke("get_git_status");

    const branchElement = document.getElementById("git-indicator");
    if (branchElement) {
      branchElement.innerText = `🌿 ${branch}${status ? '*' : ''}`;
    }
  } catch (e) {
    console.error("Failed to update Git status:", e);
  }
}

function renderSearchResults(results) {
  searchResults.innerHTML = "";
  if (results.length === 0) {
    searchResults.innerHTML = `<div style="padding: 10px; color: var(--text-secondary);">No results found.</div>`;
    return;
  }

  results.forEach(res => {
    const div = document.createElement("div");
    div.className = "tree-item";
    div.style.flexDirection = "column";
    div.style.alignItems = "flex-start";
    div.style.padding = "5px 10px";
    div.innerHTML = `
      <div style="font-weight: bold; color: var(--text-primary); font-size: 12px;">${res.path.split("/").pop()}</div>
      <div style="color: var(--text-secondary); font-size: 11px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; width: 100%;">
        <span style="color: var(--accent-color);">${res.line}:</span> ${res.content}
      </div>
    `;
    div.onclick = () => openFile(res.path, res.path.split("/").pop());
    searchResults.appendChild(div);
  });
}

async function loadExtensions() {
  const extensions = await invoke("get_running_extensions");
  installedExtensionsList.innerHTML = "";
  extensions.forEach(ext => {
    const div = createExtensionItem(ext, true);
    installedExtensionsList.appendChild(div);
  });
}

async function searchMarketplace(query) {
  try {
    const results = await invoke("search_marketplace", { query });
    marketplaceExtensionsList.innerHTML = "";
    results.extensions.forEach(ext => {
      const div = createExtensionItem(ext, false);
      marketplaceExtensionsList.appendChild(div);
    });
  } catch (e) {
    console.error("Marketplace search failed:", e);
  }
}

function createExtensionItem(ext, isInstalled) {
  const div = document.createElement("div");
  div.className = "extension-item";

  const name = ext.displayName || ext.name;
  const author = (ext.publisher && ext.publisher.displayName) || ext.publisher || ext.namespace || "Unknown";
  const desc = ext.description || "No description provided.";
  const downloadUrl = ext.files && ext.files.download;
  const fullName = ext.publisher ? `${ext.publisher.name}.${ext.name}` : ext.name;

  div.innerHTML = `
    <div class="extension-icon">🧩</div>
    <div class="extension-info">
      <div class="extension-name">${name} ${isInstalled ? '<span class="extension-badge">Running</span>' : ''}</div>
      <div class="extension-author">${author}</div>
      <div class="extension-description">${desc}</div>
      ${!isInstalled ? `<button class="extension-install-btn" data-url="${downloadUrl}" data-name="${fullName}">Install</button>` : ''}
    </div>
  `;

  if (!isInstalled) {
    const btn = div.querySelector(".extension-install-btn");
    if (btn) btn.onclick = (e) => installExtension(e.target);
  }

  return div;
}

async function installExtension(btn) {
  const url = btn.getAttribute("data-url");
  const name = btn.getAttribute("data-name");
  if (!url) return alert("Install URL not found");

  btn.innerText = "Installing...";
  btn.disabled = true;
  try {
    await invoke("install_extension", { downloadUrl: url, name: name });
    btn.innerText = "Installed";
  } catch (e) {
    btn.innerText = "Failed";
    btn.disabled = false;
    alert("Install failed: " + e);
  }
}

let commandPalette;
let commandInput;
let commandList;

const commands = [
  { id: "workbench.action.toggleSidebar", name: "View: Toggle Side Bar Visibility", action: toggleSidebar },
  { id: "editor.action.save", name: "File: Save", action: saveFileContent },
  { id: "workbench.action.openFolder", name: "File: Open Folder...", action: openFolder },
  { id: "git.refresh", name: "Git: Refresh", action: updateGitStatus },
  { id: "workbench.action.openSettings", name: "Preferences: Open Settings (JSON)", action: saveFileContent }, // Temp action
];

function executeCommand(id) {
  const cmd = commands.find(c => c.id === id);
  if (cmd) {
    cmd.action();
  } else {
    // Check if it's an extension host command
    invoke("ext_host_send", { msg: JSON.stringify({ type: "executeCommand", id }) });
  }
}

function toggleSidebar() {
  const sidebar = document.querySelector(".sidebar");
  sidebar.style.display = sidebar.style.display === "none" ? "flex" : "none";
}

async function saveFileContent() {
  const content = editor.innerText;
  try {
    await invoke("save_file", { content });
    alert("File saved!");
  } catch (e) {
    console.error("Failed to save file:", e);
  }
}

function showCommandPalette() {
  commandPalette.classList.remove("hidden");
  commandInput.value = "";
  commandInput.focus();
  renderCommandList("");
}

function hideCommandPalette() {
  commandPalette.classList.add("hidden");
  editor.focus();
}

function renderCommandList(filter) {
  commandList.innerHTML = "";
  const filtered = commands.filter(cmd =>
    cmd.name.toLowerCase().includes(filter.toLowerCase())
  );

  filtered.forEach((cmd, index) => {
    const div = document.createElement("div");
    div.className = "command-item" + (index === 0 ? " selected" : "");
    div.innerHTML = `
      <span>${cmd.name}</span>
    `;
    div.onclick = () => {
      executeCommand(cmd.id);
      hideCommandPalette();
    };
    commandList.appendChild(div);
  });
}

function updateLineNumbers() {
  const lines = editor.innerText.split("\n");
  const count = lines.length;
  lineNumbers.innerHTML = Array.from({ length: count }, (_, i) => i + 1).join("<br>");
}

async function loadDirectory(path, container = sidebarContent) {
  try {
    if (container === sidebarContent) {
      window.activeRoot = path;
    }
    const entries = await invoke("list_directory", { path });
    renderExplorer(entries, container);
  } catch (e) {
    console.error("Failed to load directory:", e);
  }
}

function renderExplorer(entries, container) {
  const ul = document.createElement("ul");
  ul.className = "tree-list";
  ul.style.listStyle = "none";
  ul.style.paddingLeft = container === sidebarContent ? "0" : "15px";

  entries.forEach(entry => {
    const li = document.createElement("li");
    li.className = "tree-item";
    li.style.padding = "2px 5px";
    li.style.cursor = "pointer";
    li.innerHTML = `<span>${entry.is_dir ? "› 📂" : "📄"} ${entry.name}</span>`;

    if (entry.is_dir) {
      let expanded = false;
      let childContainer = null;
      li.onclick = async (e) => {
        e.stopPropagation();
        expanded = !expanded;
        li.querySelector("span").innerText = `${expanded ? "⌄" : "›"} 📂 ${entry.name}`;
        if (expanded) {
          childContainer = document.createElement("div");
          li.appendChild(childContainer);
          await loadDirectory(entry.path, childContainer);
        } else if (childContainer) {
          li.removeChild(childContainer);
          childContainer = null;
        }
      };
    } else {
      li.onclick = (e) => {
        e.stopPropagation();
        openFile(entry.path, entry.name);
      };
    }

    ul.appendChild(li);
  });

  container.appendChild(ul);
}

async function openFile(path, name) {
  try {
    const content = await invoke("open_file", { path });
    editor.innerText = content;
    addTab(path, name);
    // Notify Extension Host
    invoke("ext_host_send", {
      msg: JSON.stringify({
        type: "documentOpened",
        uri: path,
        content: content,
        languageId: path.endsWith(".rs") ? "rust" : "plaintext"
      })
    });
    updateLineNumbers();
    invoke("set_context_key", { key: "activeBuffer", value: path });
    // Trigger lazy activation
    const langId = path.endsWith(".rs") ? "rust" : "plaintext";
    invoke("check_activation_event", { event: `onLanguage:${langId}` });
  } catch (e) {
    console.error("Failed to open file:", e);
  }
}

async function syncToBackend() {
  const content = editor.innerText;
  try {
    await invoke("sync_content", { content });
  } catch (e) {
    console.error("Failed to sync content:", e);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  editor = document.getElementById("editor");
  lineNumbers = document.getElementById("line-numbers");
  sidebarContent = document.getElementById("explorer-content");
  explorerView = document.getElementById("explorer-view");
  searchView = document.getElementById("search-view");
  extensionsView = document.getElementById("extensions-view"); // New
  extensionsSearchInput = document.getElementById("extensions-search-input"); // New
  installedExtensionsList = document.getElementById("installed-extensions-list"); // New
  marketplaceExtensionsList = document.getElementById("marketplace-extensions-list"); // New
  searchInput = document.getElementById("search-input");
  searchResults = document.getElementById("search-results");
  sidebarTitle = document.getElementById("sidebar-title");
  commandPalette = document.getElementById("command-palette");
  commandInput = document.getElementById("command-input");
  commandList = document.getElementById("command-list");
  debugView = document.getElementById("debug-view");
  debugVariables = document.getElementById("debug-variables");
  debugCallStack = document.getElementById("debug-callstack");
  debugBreakpoints = document.getElementById("debug-breakpoints");
  debugToolbar = document.getElementById("debug-toolbar");

  sourceControlView = document.getElementById("source-control-view");
  scmInput = document.getElementById("scm-input");
  scmChanges = document.getElementById("scm-changes");

  // Resizing logic
  setupResizers();

  // Bottom Panel tabs
  setupPanelTabs();

  // Initial load
  loadDirectory(".");
  updateGitStatus();
  initTerminal();
  setupWindowControls();

  let sourceControlView;
  let scmInput;
  let scmChanges;

  async function openSettings() {
    try {
      const configDir = await invoke("open_file", { path: "settings.json" }); // This will fail if path logic isn't perfect
      // Actually, let's look at how open_file works. It takes a full path.
      // We need a helper to get config path.
      // Or simpler: We know the structure.
      // But for now, let's assume we can treat "settings.json" as a special case or get the path first.

      // Simpler: Just invoke get_settings to get the JSON, but we want TO EDIT IT.
      // Let's implement a 'open_settings_file' command in backend?
      // Or constructing the path here.

      // Let's assume standard behavior: The backend controls the path. 
      // We'll update lib.rs later to expose get_config_path if needed.
      // For now, let's ALERT user where it is, or try to load relative?
      // Wait, open_file takes an absolute path.

      // Let's rely on the previous get_settings for display, but honestly the user wants to EDIT.
      // I will change this to just show the JSON in a new tab for now using a virtual path.
      const settings = await invoke("get_settings");
      const content = JSON.stringify(settings, null, 2);

      // SWITCH TO BUFFER with special name?
      // Let's hack: Just set editor content and set path to "settings.json"

      editor.innerText = content;
      addTab("settings.json", "Settings");
      switchTab("settings.json");

    } catch (e) {
      console.error("Failed to load settings:", e);
    }
  }

  async function updateGitUI() {
    if (!window.activeRoot) return; // Need a root for git
    try {
      const changes = await invoke("git_status", { path: window.activeRoot });
      scmChanges.innerHTML = "";

      const staged = changes.filter(c => c.status[0] !== ' ' && c.status[0] !== '?' && c.status[0] !== '!');
      const unstaged = changes.filter(c => c.status[1] !== ' ' || c.status[0] === '?' || c.status[0] === '!');

      if (staged.length > 0) {
        const header = document.createElement("div");
        header.className = "scm-header";
        header.innerText = "STAGED CHANGES";
        scmChanges.appendChild(header);
        staged.forEach(c => renderScmItem(c, true));
      }

      if (unstaged.length > 0) {
        const header = document.createElement("div");
        header.className = "scm-header";
        header.innerText = "CHANGES";
        scmChanges.appendChild(header);
        unstaged.forEach(c => renderScmItem(c, false));
      }

      if (changes.length === 0) {
        scmChanges.innerHTML = `<div style="padding: 10px; color: var(--text-secondary);">No changes detected.</div>`;
      }
    } catch (e) {
      console.warn("Git status failed:", e);
      scmChanges.innerHTML = `<div style="padding: 10px; color: var(--text-secondary);">Git not initialized.</div>`;
    }
  }

  function renderScmItem(change, isStaged) {
    const div = document.createElement("div");
    div.className = "scm-item";
    div.innerHTML = `
        <span class="scm-icon">📄</span>
        <span class="scm-filename">${change.path}</span>
        <div class="scm-actions" style="margin-left: auto; display: flex; gap: 4px;">
            <button class="scm-action-btn" title="${isStaged ? 'Unstage' : 'Stage'}" style="background: none; border: none; color: var(--text-secondary); cursor: pointer;">
                ${isStaged ? '⊖' : '⊕'}
            </button>
        </div>
        <span class="scm-badge ${change.status.trim()}">${change.status}</span>
    `;
    div.querySelector(".scm-action-btn").onclick = async (e) => {
      e.stopPropagation();
      try {
        if (isStaged) {
          await invoke("git_unstage", { path: window.activeRoot, filePath: change.path });
        } else {
          await invoke("git_stage", { path: window.activeRoot, filePath: change.path });
        }
        updateGitUI();
      } catch (err) {
        console.error("Action failed:", err);
      }
    };
    div.onclick = () => openFile(window.activeRoot + "/" + change.path, change.path);
    scmChanges.appendChild(div);
  }

  // Quick Open Implementation
  class QuickOpen {
    constructor() {
      this.visible = false;
      this.element = document.getElementById("command-palette"); // reuse palette UI for now
      this.input = document.getElementById("command-input");
      this.list = document.getElementById("command-list");
      this.files = [];

      window.addEventListener("keydown", (e) => {
        if (e.key === "p" && (e.ctrlKey || e.metaKey)) {
          e.preventDefault();
          this.show();
        }
      });
    }

    async show() {
      this.visible = true;
      this.element.classList.remove("hidden");
      this.input.placeholder = "Search files by name...";
      this.input.value = "";
      this.input.focus();
      this.render([]);

      // Fetch files initially or on input
      this.input.oninput = async () => {
        const results = await invoke("search_project", { query: this.input.value });
        this.render(results);
      };
    }

    render(results) {
      this.list.innerHTML = "";
      results.slice(0, 10).forEach((res, i) => {
        const div = document.createElement("div");
        div.className = "command-item" + (i === 0 ? " selected" : "");
        div.innerHTML = `<span>${res.path.split('/').pop()}</span> <span style="opacity:0.5; font-size:10px">${res.path}</span>`;
        div.onclick = () => {
          openFile(window.activeRoot + "/" + res.path, res.path);
          this.hide();
        };
        this.list.appendChild(div);
      });
    }

    hide() {
      this.visible = false;
      this.element.classList.add("hidden");
    }
  }

  const quickOpen = new QuickOpen();

  // ... existing code ...

  // Activity Bar logic
  const activityItems = document.querySelectorAll(".activity-item");
  activityItems.forEach(item => {
    item.addEventListener("click", () => {
      const title = item.getAttribute("title");
      if (title === "Explorer") switchSidebarView("explorer");
      if (title === "Extensions") switchSidebarView("extensions");
      if (title === "Run and Debug") switchSidebarView("run and debug");
      if (title === "Source Control") {
        switchSidebarView("source control");
        updateGitUI();
      }
    });
  });

  lineNumbers.addEventListener("click", (e) => {
    const line = parseInt(e.target.innerText);
    if (!isNaN(line)) {
      toggleBreakpoint(window.activeFilePath, line);
    }
  });

  document.getElementById("start-debug").onclick = () => startDebug();
  document.getElementById("debug-stop").onclick = () => stopDebug();
  document.getElementById("debug-continue").onclick = () => sendDapCommand("continue");
  document.getElementById("debug-step-over").onclick = () => sendDapCommand("next");
  document.getElementById("debug-step-into").onclick = () => sendDapCommand("stepIn");
  document.getElementById("debug-step-out").onclick = () => sendDapCommand("stepOut");

  extensionsSearchInput.addEventListener("input", (e) => {
    const query = e.target.value;
    clearTimeout(window.extSearchTimeout);
    window.extSearchTimeout = setTimeout(() => {
      if (query.length > 2) {
        searchMarketplace(query);
      } else {
        marketplaceExtensionsList.innerHTML = "";
      }
    }, 500);
  });

  // Context Menu logic
  setupContextMenu();

  // Initialize Minimap
  minimapCanvas = document.getElementById("minimap");
  if (minimapCanvas) {
    minimapCtx = minimapCanvas.getContext("2d", { alpha: false });

    const resizeObserver = new ResizeObserver(() => {
      minimapCanvas.width = minimapCanvas.clientWidth;
      minimapCanvas.height = minimapCanvas.clientHeight;
      renderMinimap();
    });
    resizeObserver.observe(minimapCanvas);

    editor.addEventListener("input", renderMinimap);
    editor.addEventListener("scroll", renderMinimap);
  }

  searchInput.addEventListener("input", (e) => {
    // Basic debouncing
    clearTimeout(window.searchTimeout);
    window.searchTimeout = setTimeout(() => {
      runSearch(e.target.value);
    }, 300);
  });

  commandInput.addEventListener("input", (e) => {
    renderCommandList(e.target.value);
  });

  commandInput.addEventListener("keydown", (e) => {
    if (e.key === "Escape") {
      hideCommandPalette();
    } else if (e.key === "Enter") {
      const selected = commandList.querySelector(".command-item.selected") ||
        commandList.querySelector(".command-item");
      if (selected) {
        selected.click();
      }
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      const items = Array.from(commandList.querySelectorAll(".command-item"));
      const current = commandList.querySelector(".command-item.selected");
      const nextIndex = current ? (items.indexOf(current) + 1) % items.length : 0;
      items.forEach(item => item.classList.remove("selected"));
      items[nextIndex].classList.add("selected");
      items[nextIndex].scrollIntoView({ block: "nearest" });
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      const items = Array.from(commandList.querySelectorAll(".command-item"));
      const current = commandList.querySelector(".command-item.selected");
      const nextIndex = current ? (items.indexOf(current) - 1 + items.length) % items.length : items.length - 1;
      items.forEach(item => item.classList.remove("selected"));
      items[nextIndex].classList.add("selected");
      items[nextIndex].scrollIntoView({ block: "nearest" });
    }
  });

  window.addEventListener("keydown", (e) => {
    // Ctrl+Shift+P or Cmd+Shift+P
    if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === "P") {
      e.preventDefault();
      showCommandPalette();
    }
    // Ctrl+B for Sidebar
    if ((e.ctrlKey || e.metaKey) && e.key === "b") {
      e.preventDefault();
      toggleSidebar();
    }
    // Ctrl+S to save
    if ((e.ctrlKey || e.metaKey) && e.key === "s") {
      e.preventDefault();
      saveFileContent();
    }
  });

  editor.addEventListener("input", () => {
    updateLineNumbers();
    const content = editor.innerText;
    // Debounced sync to extension host
    clearTimeout(window.extSyncTimeout);
    window.extSyncTimeout = setTimeout(() => {
      invoke("ext_host_send", {
        msg: JSON.stringify({
          type: "documentChanged",
          uri: window.activeFilePath,
          content: content
        })
      });
    }, 300);
    syncToBackend();
    updateCursorPos();
  });

  editor.addEventListener("focus", () => {
    invoke("set_context_key", { key: "editorFocus", value: true });
  });

  editor.addEventListener("blur", () => {
    invoke("set_context_key", { key: "editorFocus", value: false });
  });

  editor.addEventListener("keyup", updateCursorPos);
  editor.addEventListener("mouseup", updateCursorPos);
  editor.addEventListener("click", updateCursorPos);

  window.addEventListener("keydown", async (e) => {
    // Construct key string (e.g., "ctrl+s", "cmd+b")
    let combo = [];
    if (e.ctrlKey) combo.push("ctrl");
    if (e.metaKey) combo.push("cmd");
    if (e.altKey) combo.push("alt");
    if (e.shiftKey) combo.push("shift");
    combo.push(e.key.toLowerCase());

    const keyStr = combo.join("+");

    // Resolve via backend
    const commandId = await invoke("resolve_keybinding", { key: keyStr });

    if (commandId) {
      e.preventDefault();
      executeCommand(commandId);
    }
  });

  editor.addEventListener("keydown", (e) => {
    if (e.key === "Tab") {
      e.preventDefault();
      const selection = window.getSelection();
      const range = selection.getRangeAt(0);
      const tabNode = document.createTextNode("    ");
      range.insertNode(tabNode);
      range.setStartAfter(tabNode);
      range.setEndAfter(tabNode);
      selection.removeAllRanges();
      selection.addRange(range);
      updateLineNumbers();
      syncToBackend();
    }
  });

  // Initial call
  updateLineNumbers();
});

function updateCursorPos() {
  const selection = window.getSelection();
  if (selection.rangeCount > 0) {
    const range = selection.getRangeAt(0);
    const contentBefore = editor.innerText.substring(0, range.startOffset);
    const lines = contentBefore.split("\n");
    const ln = lines.length;
    const col = lines[ln - 1].length + 1;
    document.getElementById("cursor-pos").innerText = `Ln ${ln}, Col ${col}`;
  }
}

function setupResizers() {
  const sidebarResizer = document.getElementById("sidebar-resizer");
  const panelResizer = document.getElementById("panel-resizer");
  const sidebar = document.getElementById("sidebar");
  const bottomPanel = document.getElementById("bottom-panel");

  sidebarResizer.onmousedown = (e) => {
    document.onmousemove = (e) => {
      const width = e.clientX - 50; // Minus activity bar width
      if (width > 100 && width < 600) {
        document.documentElement.style.setProperty("--sidebar-width", `${width}px`);
      }
    };
    document.onmouseup = () => {
      document.onmousemove = null;
      document.onmouseup = null;
    };
  };

  panelResizer.onmousedown = (e) => {
    document.onmousemove = (e) => {
      const height = window.innerHeight - e.clientY - 22; // Minus status bar height
      if (height > 50 && height < window.innerHeight - 200) {
        document.documentElement.style.setProperty("--panel-height", `${height}px`);
        if (window.fitAddon) window.fitAddon.fit();
      }
    };
    document.onmouseup = () => {
      document.onmousemove = null;
      document.onmouseup = null;
    };
  };
}

function setupPanelTabs() {
  const tabs = document.querySelectorAll(".panel-tab");
  tabs.forEach(tab => {
    tab.onclick = () => {
      tabs.forEach(t => t.classList.remove("active"));
      tab.classList.add("active");
      document.getElementById("panel-content").innerText = `Showing ${tab.innerText} content...`;
    };
  });
}

function setupContextMenu() {
  const menu = document.getElementById("context-menu");
  menu.innerHTML = `<div class="context-menu-item" id="ctx-definition">Go to Definition</div>`;

  editor.oncontextmenu = (e) => {
    e.preventDefault();
    menu.style.top = `${e.clientY}px`;
    menu.style.left = `${e.clientX}px`;
    menu.classList.remove("hidden");
    document.getElementById("ctx-definition").onclick = () => goToDefinition();
  };

  window.onclick = () => {
    menu.classList.add("hidden");
  };

  document.getElementById("cm-cut").onclick = () => document.execCommand("cut");
  document.getElementById("cm-copy").onclick = () => document.execCommand("copy");
  document.getElementById("cm-paste").onclick = () => document.execCommand("paste");
  document.getElementById("cm-palette").onclick = () => showCommandPalette();
}

const { listen } = window.__TAURI__.event;

class TerminalManager {
  constructor() {
    this.terminals = new Map();
    this.activeId = null;
    this.container = document.getElementById("panel-content");
    this.tabsContainer = document.getElementById("terminal-tabs");
    this.idCounter = 1;

    document.getElementById("new-terminal").onclick = () => this.createTerminal();
  }

  async createTerminal() {
    const id = `term-${this.idCounter++}`;
    const wrapper = document.createElement("div");
    wrapper.className = "terminal-wrapper" + (this.activeId ? " hidden" : "");
    wrapper.id = `wrapper-${id}`;
    this.container.appendChild(wrapper);

    const term = new Terminal({
      theme: { background: "#1e1e1e", foreground: "#cccccc" },
      fontSize: 12,
      fontFamily: 'var(--font-code)',
      cursorBlink: true,
    });

    const fitAddon = new FitAddon.FitAddon();
    term.loadAddon(fitAddon);
    term.open(wrapper);
    fitAddon.fit();

    term.onData(data => invoke("write_to_terminal", { term_id: id, data }));
    term.onResize(({ cols, rows }) => invoke("resize_terminal", { term_id: id, cols, rows }));

    this.terminals.set(id, { term, fitAddon, wrapper });
    this.createTab(id);

    await invoke("spawn_terminal", { term_id: id });
    this.switchTo(id);
  }

  createTab(id) {
    const btn = document.createElement("button");
    btn.className = "terminal-tab-btn";
    btn.innerText = `zsh (${id.split('-')[1]})`;
    btn.onclick = () => this.switchTo(id);
    this.tabsContainer.insertBefore(btn, document.getElementById("new-terminal"));
  }

  switchTo(id) {
    if (this.activeId) {
      this.terminals.get(this.activeId).wrapper.classList.add("hidden");
    }
    this.activeId = id;
    this.terminals.get(id).wrapper.classList.remove("hidden");
    this.terminals.get(id).term.focus();
    this.terminals.get(id).fitAddon.fit();

    // Update tab styles
    document.querySelectorAll(".terminal-tab-btn").forEach(btn => {
      btn.classList.toggle("active", btn.innerText.includes(`(${id.split('-')[1]})` || id));
    });
  }

  handleData(termId, data) {
    const t = this.terminals.get(termId);
    if (t) t.term.write(data);
  }
}

let terminalManager;

async function initTerminal() {
  const terminalElement = document.getElementById("panel-content");
  terminalElement.innerHTML = "";

  terminalManager = new TerminalManager();
  await terminalManager.createTerminal();

  listen("terminal-data", (event) => {
    const { term_id, data } = event.payload;
    terminalManager.handleData(term_id, data);
  });

  // Re-init other services
  try {
    await invoke("lsp_init"); // Legacy, should be per-language
    await invoke("ext_host_init");
    await setupDapListener();
  } catch (e) { }
}

function handleExtHostMessage(msg) {
  console.log("Extension Host Message:", msg);
  if (msg.type === 'notification') {
    showStatusBarMessage(msg.message);
  } else if (msg.type === 'ready') {
    showStatusBarMessage("Extension Host Ready");
  }
}

function showStatusBarMessage(text) {
  const leftStatus = document.querySelector(".status-left");
  const msgDiv = document.createElement("div");
  msgDiv.className = "status-item";
  msgDiv.innerText = `💡 ${text}`;
  leftStatus.appendChild(msgDiv);
  setTimeout(() => msgDiv.remove(), 5000);
}

async function setupWindowControls() {
  const { getCurrentWindow } = window.__TAURI__.window;
  const appWindow = getCurrentWindow();

  document.getElementById("win-min").onclick = () => appWindow.minimize();
  document.getElementById("win-max").onclick = async () => {
    const isMaximized = await appWindow.isMaximized();
    if (isMaximized) appWindow.unmaximize();
    else appWindow.maximize();
  };
  document.getElementById("win-close").onclick = () => appWindow.close();
}

async function startDebug() {
  try {
    const adapter = "lldb-vscode";
    await invoke("debug_start", { adapterPath: adapter });
    debugToolbar.classList.remove("hidden");
    showStatusBarMessage("Debug session started");
  } catch (e) {
    console.error("Failed to start debug:", e);
    showStatusBarMessage(`Error: ${e}`);
  }
}

async function stopDebug() {
  await invoke("debug_stop");
  debugToolbar.classList.add("hidden");
  showStatusBarMessage("Debug session stopped");
}

function sendDapCommand(command) {
  const msg = JSON.stringify({
    type: "request",
    command: command,
    arguments: { threadId: 1 }
  });
  invoke("debug_send", { msg });
}

function toggleBreakpoint(path, line) {
  const index = breakpoints.findIndex(b => b.path === path && b.line === line);
  if (index > -1) {
    breakpoints.splice(index, 1);
  } else {
    breakpoints.push({ path, line });
  }
  updateLineNumbers(); // Refresh gutter
  renderBreakpoints(); // Refresh sidebar
}

function renderBreakpoints() {
  debugBreakpoints.innerHTML = "";
  breakpoints.forEach(b => {
    const div = document.createElement("div");
    div.className = "debug-item";
    div.innerText = `${b.path.split('/').pop()}: ${b.line}`;
    debugBreakpoints.appendChild(div);
  });
}

async function setupDapListener() {
  await listen("dap-message", (event) => {
    const msg = JSON.parse(event.payload);
    handleDapMessage(msg);
  });
}

function handleDapMessage(msg) {
  if (msg.type === "event" && msg.event === "stopped") {
    showStatusBarMessage(`Paused: ${msg.body.reason}`);
  }
}

function renderMinimap() {
  if (!minimapCtx || !editor) return;

  const width = minimapCanvas.width;
  const height = minimapCanvas.height;

  // Background
  minimapCtx.fillStyle = '#1e1e1e';
  minimapCtx.fillRect(0, 0, width, height);

  const lines = editor.innerText.split('\n');
  const lineHeight = 3;
  const charWidth = 2;

  const scrollPercent = editor.scrollTop / editor.scrollHeight;

  minimapCtx.save();
  minimapCtx.translate(0, -editor.scrollTop / 5);

  minimapCtx.fillStyle = '#cccccc';

  lines.forEach((line, i) => {
    const y = i * lineHeight + 10;
    let x = 2;
    for (let j = 0; j < line.length; j++) {
      if (line[j] !== ' ' && line[j] !== '\t') {
        minimapCtx.fillRect(x + (j * charWidth), y, charWidth, 2);
      }
    }
  });

  minimapCtx.restore();

  // Overlay for visible area
  const visiblePercent = editor.clientHeight / editor.scrollHeight;
  const thumbHeight = height * visiblePercent;
  // accurate thumb position
  // If the content is large, thumb moves from 0 to height - thumbHeight
  // proportional to scrollTop / (scrollHeight - clientHeight)

  if (editor.scrollHeight > editor.clientHeight) {
    const maxScroll = editor.scrollHeight - editor.clientHeight;
    const ratio = editor.scrollTop / maxScroll;
    const thumbTravel = height - thumbHeight;
    const thumbY = ratio * thumbTravel;

    minimapCtx.fillStyle = 'rgba(255, 255, 255, 0.1)';
    minimapCtx.fillRect(0, thumbY, width, thumbHeight);
  }
}

async function pollPerformanceStats() {
  try {
    const stats = await invoke("get_process_stats");
    const el = document.getElementById("perf-stats");
    if (el) {
      el.innerText = `RAM: ${stats.memory_mb}MB | CPU: ${stats.cpu_usage.toFixed(1)}%`;
    }
  } catch (e) {
    // Silent fail
  }
}

// LSP Module
let requestId = 1;
const pendingRequests = new Map();
let lspActive = false;

async function startLsp(command) {
  if (lspActive) return;
  try {
    await invoke("lsp_start", { command });
    lspActive = true;
    sendLspRequest("initialize", {
      processId: null,
      rootUri: "file://" + window.activeRoot,
      capabilities: {
        textDocument: {
          hover: { contentFormat: ["markdown", "plaintext"] },
          completion: { completionItem: { snippetSupport: true } }
        }
      }
    });
  } catch (e) {
    console.error("LSP start failed:", e);
  }
}

function sendLspRequest(method, params) {
  const id = requestId++;
  invoke("lsp_send_request", { id, method, params });
  return new Promise((resolve) => {
    pendingRequests.set(id, resolve);
  });
}

listen("lsp-msg", (event) => {
  const msg = event.payload;
  if (msg.id && pendingRequests.has(msg.id)) {
    const resolve = pendingRequests.get(msg.id);
    resolve(msg.result);
    pendingRequests.delete(msg.id);
  } else if (msg.method === "textDocument/publishDiagnostics") {
    renderDiagnostics(msg.params.diagnostics);
  }
});

function getCursorPosition() {
  const selection = window.getSelection();
  if (!selection.rangeCount) return { line: 0, character: 0 };

  // Simplified: Find line index and character offset
  const range = selection.getRangeAt(0);
  const container = range.startContainer;

  // This is complex for a plain contenteditable, but we can approximate:
  // Split by newlines in innerText up to selection
  const textBefore = editor.innerText.substring(0, getSelectionOffset(editor));
  const lines = textBefore.split('\n');
  return {
    line: lines.length - 1,
    character: lines[lines.length - 1].length
  };
}

function getSelectionOffset(element) {
  let offset = 0;
  const selection = window.getSelection();
  if (selection.rangeCount > 0) {
    const range = selection.getRangeAt(0);
    const preCaretRange = range.cloneRange();
    preCaretRange.selectNodeContents(element);
    preCaretRange.setEnd(range.endContainer, range.endOffset);
    offset = preCaretRange.toString().length;
  }
  return offset;
}

function getCursorPixelPosition() {
  const selection = window.getSelection();
  if (!selection.rangeCount) return { top: 0, left: 0 };
  const range = selection.getRangeAt(0);
  const rect = range.getBoundingClientRect();
  const editorRect = editor.getBoundingClientRect();
  return {
    top: rect.top - editorRect.top + editor.scrollTop,
    left: rect.left - editorRect.left + editor.scrollLeft
  };
}

// Hover Implementation
editor.addEventListener("mousemove", async (e) => {
  if (!lspActive || !window.activeFilePath) return;

  // Throttled hover check...
  clearTimeout(window.hoverTimeout);
  window.hoverTimeout = setTimeout(async () => {
    const pos = getCursorPosition(); // Approximation based on mouse position would be better
    const result = await sendLspRequest("textDocument/hover", {
      textDocument: { uri: "file://" + window.activeFilePath },
      position: pos
    });

    if (result && result.contents) {
      showHover(e.clientX, e.clientY, result.contents);
    } else {
      hideHover();
    }
  }, 500);
});

function showHover(x, y, contents) {
  let tooltip = document.getElementById("lsp-hover");
  if (!tooltip) {
    tooltip = document.createElement("div");
    tooltip.id = "lsp-hover";
    tooltip.className = "hover-tooltip";
    document.body.appendChild(tooltip);
  }
  tooltip.innerText = typeof contents === 'string' ? contents : JSON.stringify(contents);
  tooltip.style.display = "block";
  tooltip.style.left = x + 10 + "px";
  tooltip.style.top = y + 10 + "px";
}

function hideHover() {
  const tooltip = document.getElementById("lsp-hover");
  if (tooltip) tooltip.style.display = "none";
}

// Diagnostics
function renderDiagnostics(diagnostics) {
  // Clear old
  document.querySelectorAll(".diagnostic-error, .diagnostic-warning").forEach(el => el.remove());

  diagnostics.forEach(diag => {
    const start = diag.range.start;
    const end = diag.range.end;
    // Logic to highlight range in contenteditable is tricky...
    // For now, we'll just log them or show in status bar
    console.log("LSP Diagnostic:", diag.message);
  });
}

async function goToDefinition() {
    const pos = getCursorPosition();
    const result = await sendLspRequest("textDocument/definition", {
        textDocument: { uri: "file://" + window.activeFilePath },
        position: { line: pos.line, character: pos.character }
    });

    if (result) {
        let loc = Array.isArray(result) ? result[0] : result;
        if (loc.uri) {
            const targetPath = loc.uri.replace("file://", "");
            openFile(targetPath);
        }
    }
}
