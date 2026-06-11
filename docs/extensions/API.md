# HADES Extension API (v1)

Extensions run in a **sidecar process** (Node) and talk JSON-RPC to the IDE
over the existing `ext_host` IPC. The typed surface lives in
`packages/hades-extension-api/index.d.ts` — that file is the contract; this
doc is the tour.

## Capability model

Your manifest declares capabilities; anything you didn't declare **rejects at
runtime** and shows the user a permission prompt (same flow as agent tool
permissions). Granted capabilities persist per extension.

```jsonc
// package.json
{
  "name": "hello-extension",
  "version": "0.1.0",
  "main": "extension.js",
  "hades": {
    "capabilities": ["commands", "window"],
    "contributes": {
      "commands": [{ "id": "hello.sayHi", "title": "Hello: Say Hi" }]
    }
  }
}
```

## v1 namespaces

| Namespace | Capability | What you get |
|---|---|---|
| `commands` | `commands` | `register(id, fn)`, `execute(id, ...args)` |
| `window` | `window` | `showMessage`, `createStatusBarItem` |
| `workspace.fs` | `fs` | `readFile`/`writeFile`/`readDir` — **workspace-scoped**, no absolute paths |
| `workspace.onDidChangeTextDocument` | `fs` | full-content change events |
| `languages` | `languages` | `registerCompletionProvider` (Monaco language ids) |
| `settings` | `settings` | `get(key)`, `onChange` — reads the settings registry |

Themes need no API: contribute them in the manifest (`contributes.themes`)
and they install like VSIX themes do today.

## Lifecycle

```js
// extension.js
const hades = require('hades');

exports.activate = () => {
    const cmd = hades.commands.register('hello.sayHi', () => {
        hades.window.showMessage('Hi from hello-extension!');
    });
    const item = hades.window.createStatusBarItem('right', 10);
    item.text = '$(rocket) Hello';
    item.show();
    return { dispose: () => { cmd.dispose(); item.dispose(); } };
};
```

`activate()` runs when an activation event fires (declared contributions
auto-derive events: contributing a command activates on first invocation).
Return value's `dispose()` runs at unload.

## Install paths

- **Folder**: Settings → Extensions → Install from folder
- **VSIX**: existing VSIX install flow (themes/grammars work today)

## Status (Milestone E)

- [x] Typed surface (`packages/hades-extension-api`)
- [x] Sample extension (`examples/hello-extension`)
- [ ] Sidecar JSON-RPC dispatcher wired to the typed surface
- [ ] Manifest validation in Rust (contribution points)
- [ ] Capability prompts via tool_permission_senders
- [ ] Open VSX gallery search hardening
