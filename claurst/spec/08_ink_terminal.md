# Claude Code — Ink Terminal Rendering System

## Overview

The ink directory contains a complete, custom terminal UI framework built on top of React. It is a heavily modified and extended fork of the open-source Ink library, tuned for Claude Code's requirements: fullscreen alternate-screen rendering, hardware-accelerated scroll, text selection, search highlighting, mouse tracking, bidirectional text, and fine-grained performance instrumentation.

The system can be summarized as a pipeline:

```
React tree
    → React Reconciler (reconciler.ts)
    → Virtual DOM (dom.ts)
    → Yoga layout engine (layout/)
    → Output buffer (output.ts)
    → Screen cell buffer (screen.ts)
    → Diff engine (log-update.ts)
    → Patch optimizer (optimizer.ts)
    → Terminal write (terminal.ts / termio/)
```

The top-level entry point for consumers is `/x/Bigger-Projects/Claude-Code/src/ink.ts`, which wraps the internal `root.ts` render and `createRoot` APIs with a mandatory `ThemeProvider`.

---

## File-by-File Reference

### `/x/Bigger-Projects/Claude-Code/src/ink.ts` — Public API Module

**Purpose:** The package-level façade that re-exports all public APIs from the ink subsystem. Wraps every `render()` and `createRoot()` call in a `ThemeProvider` so that `ThemedBox`/`ThemedText` components work at every call site without requiring consumers to mount the provider manually.

**Exports:**
- `render(node, options?)` — async, mounts a React tree wrapped in `ThemeProvider`; returns `Instance`
- `createRoot(options?)` — async; returns a `Root` whose `.render()` method also wraps in `ThemeProvider`
- `RenderOptions`, `Instance`, `Root` — type re-exports from `root.ts`
- `color` — from the design-system color module
- `Box`, `BoxProps` — themed box (design-system ThemedBox)
- `Text`, `TextProps` — themed text (design-system ThemedText)
- `ThemeProvider`, `usePreviewTheme`, `useTheme`, `useThemeSetting`
- `Ansi` — ANSI-string rendering component
- `BaseBox`, `BaseBoxProps` — raw ink Box without theme
- `BaseText`, `BaseTextProps` — raw ink Text without theme
- `Button`, `ButtonProps`, `ButtonState`
- `Link`, `LinkProps`
- `Newline`, `NewlineProps`
- `NoSelect`
- `RawAnsi`
- `Spacer`
- `DOMElement` — the virtual DOM element type
- `ClickEvent`, `EventEmitter`, `Event`, `Key`, `InputEvent`
- `TerminalFocusEvent`, `TerminalFocusEventType`
- `FocusManager`
- `FlickerReason`
- `useAnimationFrame`, `useApp`, `useInput`, `useAnimationTimer`, `useInterval`
- `useSelection`, `useStdin`, `useTabStatus`, `useTerminalFocus`
- `useTerminalTitle`, `useTerminalViewport`
- `measureElement`
- `supportsTabStatus`
- `wrapText`

---

### `/x/Bigger-Projects/Claude-Code/src/ink/constants.ts`

**Purpose:** Shared timing constant.

**Exports:**
- `FRAME_INTERVAL_MS = 16` — target frame interval (~60 fps), used by the throttled `scheduleRender`.

---

### `/x/Bigger-Projects/Claude-Code/src/ink/ink.tsx` — Core Ink Class

**Purpose:** The central orchestrator. `class Ink` owns the React fiber root, the yoga layout tree, the double-buffered screen, the focus manager, stdin/stdout event handlers, selection state, and the main render loop.

**Class `Ink`**

Constructor receives `Options`:
```typescript
type Options = {
  stdout: NodeJS.WriteStream
  stdin: NodeJS.ReadStream
  stderr: NodeJS.WriteStream
  exitOnCtrlC: boolean
  patchConsole: boolean
  waitUntilExit?: () => Promise<void>
  onFrame?: (event: FrameEvent) => void
}
```

Key private state:
| Field | Type | Description |
|---|---|---|
| `log` | `LogUpdate` | Diff engine that writes ANSI to stdout |
| `terminal` | `Terminal` | `{ stdout, stderr }` write streams |
| `scheduleRender` | throttled fn | Throttled at 16 ms, leading+trailing, deferred via `queueMicrotask` |
| `container` | `FiberRoot` | React-reconciler fiber root (ConcurrentRoot mode) |
| `rootNode` | `DOMElement` | The `ink-root` DOM node |
| `focusManager` | `FocusManager` | DOM-style focus state machine |
| `renderer` | `Renderer` | `createRenderer()` closure |
| `stylePool` | `StylePool` | Session-lived ANSI style interning pool |
| `charPool` | `CharPool` | Session-lived character interning pool |
| `hyperlinkPool` | `HyperlinkPool` | Session-lived hyperlink URL interning pool |
| `frontFrame` / `backFrame` | `Frame` | Double-buffered screen frames |
| `selection` | `SelectionState` | Text selection for alt-screen mode |
| `searchHighlightQuery` | `string` | Current /search term |
| `searchPositions` | object or null | Pre-scanned match positions for current-match highlight |
| `altScreenActive` | `boolean` | Set by `<AlternateScreen>` |
| `prevFrameContaminated` | `boolean` | Forces full repaint next frame |

**Render loop (`onRender`):**
1. Run `createRenderer()` — walks DOM, runs yoga, fills back-buffer screen
2. Apply selection overlay (invert styled cells in `selection`)
3. Apply search highlight (invert cells matching `searchHighlightQuery`)
4. Apply positioned highlight (yellow/bold current-match via `searchPositions`)
5. Diff back-frame vs. front-frame via `log.render()` → `Patch[]`
6. Run `optimize(patches)` to merge/deduplicate
7. Call `writeDiffToTerminal()` to serialize patches and write ANSI to stdout
8. Emit `onFrame` event if wired
9. Swap front/back frames

**Alt-screen handling:**
- `setAltScreenActive(active, mouseTracking)` — called by `<AlternateScreen>` during insertion effects; enables BSU/ESU synchronized output, DECSTBM hardware scroll hints, and selection-aware repaints
- `resetFramesForAltScreen()` — replaces both frames with blank screens, sets `prevFrameContaminated = true`
- `reenterAltScreen()` — re-asserts alt-screen state on SIGCONT

**Resize handling (`handleResize`):**
- Synchronous (no debounce) to keep `terminalColumns`/`terminalRows` and yoga in sync
- For alt-screen: resets frame buffers and sets `needsEraseBeforePaint = true` so the erase happens atomically inside the next BSU/ESU block

**Console patching:**
- `patchConsole()` — intercepts `console.log/warn/error` so they write to a separate file descriptor (not stdout), preventing output mixing
- `patchStderr()` — same for stderr

**Key public methods:**
- `render(node)` — calls `reconciler.updateContainer()`
- `unmount()` — graceful teardown: restore console, disable mouse tracking, exit alt screen, write final frame, free yoga nodes
- `waitUntilExit()` — returns a promise resolved on `unmount()`
- `clearTextSelection()` — clear selection state and force repaint
- `setSearchHighlight(query)` — set live search term
- `setSearchPositions(positions, rowOffset, currentIdx)` — set positioned highlights for search navigation

---

### `/x/Bigger-Projects/Claude-Code/src/ink/root.ts` — Public Entry Points

**Purpose:** Wraps `Ink` in the public `render()` and `createRoot()` APIs; manages the `instances` map so repeated calls to `render()` reuse the same `Ink` instance for the same stdout stream.

**Exports:**
```typescript
type RenderOptions = {
  stdout?: NodeJS.WriteStream
  stdin?: NodeJS.ReadStream
  stderr?: NodeJS.WriteStream
  exitOnCtrlC?: boolean
  patchConsole?: boolean
  onFrame?: (event: FrameEvent) => void
}

type Instance = {
  rerender: Ink['render']
  unmount: Ink['unmount']
  waitUntilExit: Ink['waitUntilExit']
  cleanup: () => void
}

type Root = {
  render: (node: ReactNode) => void
  unmount: () => void
  waitUntilExit: () => Promise<void>
}

export const renderSync(node, options?): Instance   // synchronous mount
export default async function render(node, options?): Promise<Instance>
export async function createRoot(options?): Promise<Root>
```

`renderSync` is used internally; the public-facing `render` and `createRoot` are the async versions exported through `ink.ts`.

---

### `/x/Bigger-Projects/Claude-Code/src/ink/instances.ts`

**Purpose:** Module-level singleton map keyed by `NodeJS.WriteStream`. Ensures one `Ink` instance per stdout stream.

```typescript
const instances = new Map<NodeJS.WriteStream, Ink>()
export default instances
```

---

## DOM Layer

### `/x/Bigger-Projects/Claude-Code/src/ink/dom.ts` — Virtual DOM

**Purpose:** Defines the virtual DOM node types and all mutation operations. Mirrors a minimal browser DOM API adapted for a terminal. Every mutation marks the affected node and all ancestors `dirty` so the render pass can skip clean subtrees.

**Element name types:**
```typescript
type ElementNames =
  | 'ink-root'      // Document root; owns FocusManager
  | 'ink-box'       // Flex container (yoga node)
  | 'ink-text'      // Leaf text container (yoga measure func)
  | 'ink-virtual-text' // Inline text, no yoga node
  | 'ink-link'      // OSC 8 hyperlink wrapper, no yoga node
  | 'ink-progress'  // Terminal progress indicator, no yoga node
  | 'ink-raw-ansi'  // Pre-rendered ANSI, yoga node with rawWidth/rawHeight

type TextName = '#text'
type NodeNames = ElementNames | TextName
```

**`DOMElement` structure:**
```typescript
type DOMElement = {
  nodeName: ElementNames
  attributes: Record<string, DOMNodeAttribute>
  childNodes: DOMNode[]
  textStyles?: TextStyles
  onComputeLayout?: () => void   // Called by reconciler.resetAfterCommit
  onRender?: () => void           // Points to throttled scheduleRender
  onImmediateRender?: () => void  // Synchronous, for tests
  hasRenderedContent?: boolean    // React 19 test-mode guard
  dirty: boolean
  isHidden?: boolean
  _eventHandlers?: Record<string, unknown>  // Event handlers (separated from attrs)
  // Scroll state (overflow: scroll boxes):
  scrollTop?: number
  pendingScrollDelta?: number
  scrollClampMin?: number
  scrollClampMax?: number
  scrollHeight?: number
  scrollViewportHeight?: number
  scrollViewportTop?: number
  stickyScroll?: boolean
  scrollAnchor?: { el: DOMElement; offset: number }
  focusManager?: FocusManager    // Only on ink-root
  debugOwnerChain?: string[]     // CLAUDE_CODE_DEBUG_REPAINTS mode
  yogaNode?: LayoutNode
  style: Styles
  parentNode: DOMElement | undefined
}
```

**Exported functions:**
- `createNode(nodeName)` — allocates a `DOMElement`; attaches a yoga measure function for `ink-text` and `ink-raw-ansi`
- `createTextNode(text)` — allocates a `TextNode`
- `appendChildNode(node, childNode)` — appends child, syncs yoga tree, marks dirty
- `insertBeforeNode(node, newChild, beforeChild)` — inserts before; yoga index computed separately from DOM index because some nodes lack yoga nodes
- `removeChildNode(node, removeNode)` — removes child, collects `pendingClears`, marks dirty
- `setAttribute(node, key, value)` — sets attribute only if changed; skips `children`
- `setStyle(node, style)` — shallow-compares style objects; skips if unchanged
- `setTextStyles(node, textStyles)` — shallow-compares; skips if unchanged
- `setTextNodeValue(node, text)` — updates text, marks dirty
- `markDirty(node?)` — walks the ancestor chain setting `dirty = true`; marks yoga dirty on `ink-text`/`ink-raw-ansi` leaf nodes
- `scheduleRenderFrom(node?)` — walks to root and calls `onRender()`
- `clearYogaNodeReferences(node)` — recursively clears `yogaNode` pointers (call before `freeRecursive()`)
- `findOwnerChainAtRow(root, y)` — DFS to find the React component stack covering screen row `y` (debug repaints mode)

**Dirty-checking optimization:** `stylesEqual` and `shallowEqual` prevent marking dirty when React creates a new style object with identical values on every render.

**Yoga index vs DOM index:** Nodes like `ink-virtual-text`, `ink-link`, and `ink-progress` have no yoga node. `insertBeforeNode` counts only yoga-equipped children to compute the correct yoga insertion index.

---

## Reconciler

### `/x/Bigger-Projects/Claude-Code/src/ink/reconciler.ts`

**Purpose:** Configures `react-reconciler` to use the ink virtual DOM as the host environment. This is the bridge between React's fiber tree and the ink DOM tree.

**Reconciler type parameters:**
```typescript
createReconciler<
  ElementNames,   // Type
  Props,          // Props
  DOMElement,     // Container
  DOMElement,     // Instance
  TextNode,       // TextInstance
  DOMElement,     // SuspenseInstance
  unknown,        // HydratableInstance
  unknown,        // PublicInstance
  DOMElement,     // HostContext (root)
  HostContext,    // ChildSet
  null,           // UpdatePayload (unused in React 19)
  NodeJS.Timeout, // TimeoutHandle
  -1,             // NoTimeout
  null            // TransitionStatus
>
```

**Key reconciler methods:**

| Method | Behavior |
|---|---|
| `createInstance(type, props, rootContainer, context, fiber)` | Calls `createNode(type)`; applies all props via `applyProp`; optionally captures `debugOwnerChain` from fiber |
| `createTextInstance(text)` | Calls `createTextNode(text)` |
| `appendInitialChild` / `appendChild` | Calls `appendChildNode` |
| `insertBefore` | Calls `insertBeforeNode` |
| `removeChild` | Calls `removeChildNode`; notifies `focusManager.handleNodeRemoved` |
| `commitUpdate(instance, updatePayload, type, oldProps, newProps)` | Diffs old/new props and applies changes; uses `diff()` to find changed keys |
| `commitTextUpdate(textInstance, oldText, newText)` | Calls `setTextNodeValue` |
| `hideInstance(instance)` / `unhideInstance(instance)` | Sets `isHidden` and `LayoutDisplay.None` / restores display |
| `prepareForCommit` | Records timing start |
| `resetAfterCommit(rootNode)` | Records commit duration; calls `onComputeLayout()` (yoga layout); triggers `onImmediateRender` in test mode; calls `onRender()` in production |
| `commitMount(instance, type, props)` | Calls `focusManager.focus()` if `autoFocus` prop set |

**`applyProp(node, key, value)`:** Routes to `setStyle` (key=`style`), `setTextStyles` (key=`textStyles`), `setEventHandler` (key in `EVENT_HANDLER_PROPS`), or `setAttribute`.

**Event handler separation:** Event handler props are stored in `node._eventHandlers` rather than `node.attributes`. This prevents handler identity changes from marking the node dirty and defeating the blit optimization.

**`getOwnerChain(fiber)`:** Walks the React fiber's `_debugOwner` / `return` chain to collect component names. Used for `CLAUDE_CODE_DEBUG_REPAINTS` mode to attribute flicker to source components.

**Profiling exports:**
- `recordYogaMs(ms)` / `getLastYogaMs()` — yoga layout timing
- `markCommitStart()` / `getLastCommitMs()` — React commit timing
- `resetProfileCounters()`
- `dispatcher` — the `Dispatcher` instance (event dispatch)
- `isDebugRepaintsEnabled()` — reads `CLAUDE_CODE_DEBUG_REPAINTS` env var once

---

## Layout Engine

### `/x/Bigger-Projects/Claude-Code/src/ink/layout/node.ts` — Layout Node Interface

**Purpose:** Abstract interface for a layout node. Decouples the ink DOM from the concrete Yoga WASM implementation.

**Constants:**
```typescript
LayoutEdge: { All, Horizontal, Vertical, Left, Right, Top, Bottom, Start, End }
LayoutGutter: { All, Column, Row }
LayoutDisplay: { Flex, None }
LayoutFlexDirection: { Row, RowReverse, Column, ColumnReverse }
LayoutAlign: { Auto, Stretch, FlexStart, Center, FlexEnd }
LayoutJustify: { FlexStart, Center, FlexEnd, SpaceBetween, SpaceAround, SpaceEvenly }
LayoutWrap: { NoWrap, Wrap, WrapReverse }
LayoutPositionType: { Relative, Absolute }
LayoutOverflow: { Visible, Hidden, Scroll }
LayoutMeasureMode: { Undefined, Exactly, AtMost }
```

**`LayoutNode` interface** (abbreviated):
```typescript
interface LayoutNode {
  // Tree operations
  insertChild(child, index): void
  removeChild(child): void
  getChildCount(): number
  getParent(): LayoutNode | null

  // Layout computation
  calculateLayout(width?, height?): void
  setMeasureFunc(fn: LayoutMeasureFunc): void
  unsetMeasureFunc(): void
  markDirty(): void

  // Layout reading (post-layout)
  getComputedLeft(): number
  getComputedTop(): number
  getComputedWidth(): number
  getComputedHeight(): number
  getComputedBorder(edge): number
  getComputedPadding(edge): number

  // Style setters (width, height, min/max, flex*, align*, justify, display,
  //                position, overflow, margin, padding, border, gap)

  // Lifecycle
  free(): void
  freeRecursive(): void
}
```

### `/x/Bigger-Projects/Claude-Code/src/ink/layout/yoga.ts` — Yoga Adapter

**Purpose:** Implements `LayoutNode` by wrapping the native Yoga WASM node (`src/native-ts/yoga-layout`). Maps `LayoutEdge`/`LayoutGutter`/etc. string enums to Yoga enum values.

**Class `YogaLayoutNode`:**
- Holds a `YogaNode` (`this.yoga`)
- `setMeasureFunc`: wraps the `LayoutMeasureFunc` to translate `MeasureMode` enum values
- `calculateLayout(width)`: calls `this.yoga.calculateLayout(width, undefined, Direction.LTR)` — height is always undefined (intrinsic)

Edge/gutter enum maps are static `EDGE_MAP` and `GUTTER_MAP` objects.

### `/x/Bigger-Projects/Claude-Code/src/ink/layout/engine.ts`

**Purpose:** Factory for layout nodes. A one-line indirection: `createLayoutNode()` calls `createYogaLayoutNode()`. Allows swapping the layout backend without changing the DOM layer.

### `/x/Bigger-Projects/Claude-Code/src/ink/layout/geometry.ts`

**Purpose:** 2D geometry primitives used throughout the rendering pipeline.

**Exports:**
```typescript
type Point = { x: number; y: number }
type Size = { width: number; height: number }
type Rectangle = Point & Size
type Edges = { top: number; right: number; bottom: number; left: number }

edges(all): Edges
edges(v, h): Edges
edges(t, r, b, l): Edges
addEdges(a, b): Edges
resolveEdges(partial?): Edges
ZERO_EDGES: Edges
unionRect(a, b): Rectangle   // bounding union
clampRect(rect, size): Rectangle
withinBounds(size, point): boolean
clamp(value, min?, max?): number
```

---

## Styles

### `/x/Bigger-Projects/Claude-Code/src/ink/styles.ts`

**Purpose:** TypeScript types for box/text styles plus `applyStyles()` which translates style props onto a `LayoutNode`.

**Color types:**
```typescript
type RGBColor = `rgb(${number},${number},${number})`
type HexColor = `#${string}`
type Ansi256Color = `ansi256(${number})`
type AnsiColor = 'ansi:black' | 'ansi:red' | ...  // 16 named colors
type Color = RGBColor | HexColor | Ansi256Color | AnsiColor
```

**`TextStyles`:** `{ color?, backgroundColor?, dim?, bold?, italic?, underline?, strikethrough?, inverse? }` — applied during rendering via chalk/colorize; not yoga properties.

**`Styles`:** The complete set of layout and text style props including:
- `textWrap`: `'wrap' | 'wrap-trim' | 'end' | 'middle' | 'truncate-end' | 'truncate' | 'truncate-middle' | 'truncate-start'`
- `position`: `'absolute' | 'relative'`
- `top | bottom | left | right`: `number | '${number}%'`
- `columnGap | rowGap | gap`: number
- `margin | marginX | marginY | marginTop | marginBottom | marginLeft | marginRight`: number
- `padding | paddingX | paddingY | paddingTop | paddingBottom | paddingLeft | paddingRight`: number
- `flexGrow | flexShrink | flexBasis`: number
- `flexDirection`: `'row' | 'row-reverse' | 'column' | 'column-reverse'`
- `flexWrap`: `'wrap' | 'nowrap' | 'wrap-reverse'`
- `alignItems | alignSelf | justifyContent`
- `width | height | minWidth | minHeight | maxWidth | maxHeight`: number or `'${number}%'` or `'100%'`
- `display`: `'flex' | 'none'`
- `overflow | overflowX | overflowY`: `'visible' | 'hidden' | 'scroll'`
- `borderStyle`: `BorderStyle`
- `borderColor | borderTopColor | borderRightColor | borderBottomColor | borderLeftColor`: Color
- `borderDimColor | ...`: boolean
- `borderTop | borderRight | borderBottom | borderLeft`: boolean
- `color | backgroundColor | dimColor | bold | italic | underline | strikethrough | inverse`

**`applyStyles(yogaNode, styles)`:** Translates each Styles property to `yogaNode.setXxx()` calls. Percentage values use `setWidthPercent`, etc. Position/overflow/display use enum mapping.

---

## Screen Buffer

### `/x/Bigger-Projects/Claude-Code/src/ink/screen.ts`

**Purpose:** The core cell-based screen buffer. Stores the rendered content as a 2D grid of cells. Each cell is packed into two 32-bit integers for memory efficiency.

**Cell encoding (packed as two `Int32Array` elements per cell):**
- Word 0 (low 32 bits): `charId` (high 22 bits) | `styleId` (low 10 bits)
- Word 1 (high 32 bits): `hyperlinkId` (high 16 bits) | `width` (2 bits) | flags

`CellWidth` enum: `Single = 0`, `Wide = 1`, `SpacerTail = 2`, `SpacerHead = 3`

**Pools (shared across all screens for zero-allocation diffing):**

`CharPool`:
- `intern(char)`: returns a stable integer ID; ASCII chars use a fast `Int32Array` lookup; others use a `Map`
- `get(id)`: retrieves the string
- Pool index 0 = space, index 1 = empty (spacer cell)

`HyperlinkPool`:
- `intern(hyperlink?)`: returns 0 for no hyperlink
- `get(id)`: returns the URL string or `undefined`

`StylePool`:
- `intern(styles: AnsiCode[])`: returns a tagged integer ID; bit 0 = `VISIBLE_ON_SPACE` flag (background/inverse/underline affect spaces)
- `get(id)`: strips bit-0 flag and returns `AnsiCode[]`
- `transition(fromId, toId)`: returns the cached ANSI transition string (pre-serialized diff)
- `withInverse(baseId)`: returns ID of style with SGR 7 (inverse) added
- `withCurrentMatch(baseId)`: returns ID of style with inverse + bold + yellow-fg + underline (current search match)
- `withSelectionBg(baseId)`: returns ID of style with selection background color applied
- `setSelectionBg(bg)`: sets the selection background `AnsiCode` (clears cache)

**`Screen` type:**
```typescript
type Screen = {
  cells: Int32Array   // packed cell data, width * height * 2 words per cell
  width: number
  height: number
  charPool: CharPool
  stylePool: StylePool
  hyperlinkPool: HyperlinkPool
  softWrap: Uint8Array  // 1 bit per row; 1 = soft-wrapped continuation
  noSelect: Uint8Array  // 1 bit per cell; set on gutter/non-selectable regions
}
```

**Key functions:**
- `createScreen(width, height, stylePool, charPool, hyperlinkPool)` — allocates a new screen
- `resetScreen(screen)` — zeroes all cells
- `setCellAt(screen, x, y, char, styleId, width, hyperlink?)` — writes a cell
- `cellAt(screen, x, y)` — reads a cell
- `cellAtIndex(screen, idx)` — reads cell at raw index
- `isEmptyCellAt(screen, x, y)` — both packed words = 0
- `blitRegion(dst, src, srcRect, dstX, dstY)` — bulk-copy a rectangle from one screen to another (pure `Int32Array` copy, no decoding)
- `shiftRows(screen, top, bottom, delta)` — hardware scroll simulation: move rows up/down within bounds
- `markNoSelectRegion(screen, x, y, w, h)` — set the noSelect bit on a rectangular region
- `diffEach(prev, next, callback)` — iterate only changed cells between two screens
- `migrateScreenPools(screen, charPool, hyperlinkPool)` — re-intern all cells when pools are replaced (generational reset)

---

## Rendering Pipeline

### `/x/Bigger-Projects/Claude-Code/src/ink/renderer.ts`

**Purpose:** Creates a `Renderer` function (a closure over the root DOM node and `StylePool`) that converts the virtual DOM into a `Frame` object on each call.

**`Renderer` type:** `(options: RenderOptions) => Frame`

**`RenderOptions`:**
```typescript
{
  frontFrame: Frame
  backFrame: Frame
  isTTY: boolean
  terminalWidth: number
  terminalRows: number
  altScreen: boolean
  prevFrameContaminated: boolean
}
```

**Algorithm:**
1. Check yoga computed dimensions; return empty frame if invalid
2. For alt-screen: clamp `height` to `terminalRows` (enforces the invariant)
3. Reuse or create `Output` with the back-buffer screen
4. Reset `layoutShifted`, `scrollHint`, `scrollDrainNode`
5. If `prevFrameContaminated` or an absolute node was removed: disable blit (pass `prevScreen = undefined`)
6. Call `renderNodeToOutput(node, output, { prevScreen })`
7. After render: if a scroll-drain node remains, call `markDirty(drainNode)` to schedule the next drain frame
8. Return `Frame` with the rendered screen, viewport, cursor position, and scroll hint

**Cursor position:**
- Alt-screen: `y = min(screen.height, terminalRows) - 1` — keeps cursor inside viewport to prevent LF-induced scroll
- Main-screen: `y = screen.height`
- `visible = !isTTY || screen.height === 0` — cursor is hidden during active rendering

The `Output` instance is reused across frames so its `charCache` (per-line grapheme cluster cache) persists between renders, making steady-state spinner/clock renders near zero-allocation.

### `/x/Bigger-Projects/Claude-Code/src/ink/render-node-to-output.ts`

**Purpose:** Recursively walks the DOM tree and emits write/blit/clear/clip/shift operations onto the `Output` buffer. This is the layout-to-pixels bridge.

**Key exported state:**
- `didLayoutShift()` / `resetLayoutShifted()` — set when any node moves; gates the full-damage path in `ink.tsx`
- `getScrollHint()` / `resetScrollHint()` — DECSTBM hardware scroll hint for `LogUpdate`
- `getScrollDrainNode()` / `resetScrollDrainNode()` — identifies a ScrollBox with remaining `pendingScrollDelta`
- `consumeFollowScroll()` — consumes the at-bottom follow-scroll event for selection adjustment

**Scroll drain parameters:**
```
SCROLL_MIN_PER_FRAME = 4        // minimum rows applied per frame
SCROLL_INSTANT_THRESHOLD = 5   // ≤ this: drain all at once (xterm.js wheel click)
SCROLL_HIGH_PENDING = 12        // threshold for high-speed drain
SCROLL_STEP_MED = 2             // medium pending drain step
SCROLL_STEP_HIGH = 3            // high pending drain step
```

**ScrollBox rendering (three-pass algorithm):**
1. **First pass:** compute scrollHeight from yoga, apply `pendingScrollDelta` (proportional drain), handle `stickyScroll`, detect `scrollAnchor`
2. **Second pass (blit path):** when content is unchanged and layout hasn't shifted, blit the scrollbox from `prevScreen` and emit a hardware `shiftRows` hint
3. **Third pass (full render):** render children with `clip` and `y-offset = -scrollTop`; record `absoluteRectsCur` for position:absolute descendants

**Blit optimization:** When a node's bounding box matches `nodeCache` and the node is not dirty, the renderer blits from `prevScreen` instead of re-rendering. The condition is: `!dirty && prevScreen && !hasRemovedChild && !layoutShifted`. This makes steady-state frames O(changed cells).

**`nodeCache` updates:** After rendering each node, `nodeCache.set(node, { x, y, width, height })` records the screen-space bounding box for hit-testing and blit reuse.

### `/x/Bigger-Projects/Claude-Code/src/ink/output.ts`

**Purpose:** Collects rendering operations (write, blit, clip, clear, no-select, shift) and applies them to a `Screen` buffer in `get()`.

**`Operation` union type:**
```typescript
type Operation =
  | WriteOperation    // { type: 'write', x, y, text, softWrap? }
  | ClipOperation     // { type: 'clip', clip: Clip }
  | UnclipOperation   // { type: 'unclip' }
  | BlitOperation     // { type: 'blit', srcScreen, srcRect, dstX, dstY }
  | ClearOperation    // { type: 'clear', x, y, width, height }
  | NoSelectOperation // { type: 'noSelect', x, y, width, height }
  | ShiftOperation    // { type: 'shift', top, bottom, delta }
```

**`Clip` type:** `{ x1?, x2?, y1?, y2? }` — undefined on an axis means unbounded. Clips are intersected when nested.

**Write operation processing:**
The `WriteOperation` handler is the hot path. For each character:
1. Tokenize the text (ANSI codes) via `@alcalzone/ansi-tokenize`
2. Build a `ClusteredChar[]` array (grapheme + width + styleId + hyperlink), cached per unique line via `charCache: Map<string, ClusteredChar[]>`
3. Apply bidirectional reordering (`reorderBidi`) on Windows/xterm.js
4. Call `setCellAt` for each grapheme

**`charCache`:** Keyed by the raw ANSI string line. Cache miss: tokenize + cluster + intern styles. Cache hit: reuse the `ClusteredChar[]` array directly. The cache persists across frames (it lives on the `Output` instance). This is the dominant performance optimization for text-heavy content.

**Tab expansion:** Tabs in text are expanded to spaces at write time using screen x-position (not at measurement time), so the rendered width matches the measured width.

### `/x/Bigger-Projects/Claude-Code/src/ink/render-to-screen.ts`

**Purpose:** Off-screen renderer used for search scanning. Renders a React element to an isolated `Screen` buffer at a given width without writing to the terminal. Used to pre-scan message content for search match positions.

**`renderToScreen(el, width)`:** Returns `{ screen: Screen; height: number }`. Uses a shared persistent root/container/pools (LegacyRoot mode for synchronous rendering) so repeated calls reuse Yoga nodes. Unmounts between calls to free resources.

**`scanPositions(screen, query)`:** Scans a rendered screen for all occurrences of `query`, returning `MatchPosition[]` with `{ row, col, len }` in message-relative coordinates.

**`applyPositionedHighlight(screen, positions, rowOffset, currentIdx, stylePool)`:** Applies the current-match yellow/bold/underline style to the cell range at `positions[currentIdx]` and inverse style to all other positions.

### `/x/Bigger-Projects/Claude-Code/src/ink/frame.ts`

**Purpose:** Defines the `Frame` type and the diff/patch type hierarchy.

**`Frame` type:**
```typescript
type Frame = {
  readonly screen: Screen
  readonly viewport: Size
  readonly cursor: Cursor
  readonly scrollHint?: ScrollHint | null
  readonly scrollDrainPending?: boolean
}
```

**`Patch` union type:**
```typescript
type Patch =
  | { type: 'stdout'; content: string }
  | { type: 'clear'; count: number }
  | { type: 'clearTerminal'; reason: FlickerReason; debug?: {...} }
  | { type: 'cursorHide' }
  | { type: 'cursorShow' }
  | { type: 'cursorMove'; x: number; y: number }
  | { type: 'cursorTo'; col: number }
  | { type: 'carriageReturn' }
  | { type: 'hyperlink'; uri: string }
  | { type: 'styleStr'; str: string }

type Diff = Patch[]
```

**`shouldClearScreen(prevFrame, frame)`:** Returns `'resize' | 'offscreen' | undefined`:
- `'resize'` — viewport dimensions changed
- `'offscreen'` — current or previous screen height exceeds viewport rows

**`FlickerReason`:** `'resize' | 'offscreen' | 'clear'`

**`FrameEvent`:** Timing breakdown emitted to `onFrame`:
```typescript
type FrameEvent = {
  durationMs: number
  phases?: {
    renderer: number; diff: number; optimize: number; write: number
    patches: number; yoga: number; commit: number
    yogaVisited: number; yogaMeasured: number; yogaCacheHits: number; yogaLive: number
  }
  flickers: Array<{ desiredHeight, availableHeight, reason }>
}
```

### `/x/Bigger-Projects/Claude-Code/src/ink/log-update.ts` — Diff Engine

**Purpose:** Computes a `Diff` (list of `Patch` objects) by comparing the new `Frame` to the previous frame, or handles full-screen clears when needed.

**`LogUpdate` class:**

Constructor takes `{ isTTY, stylePool }`. Maintains `previousOutput: string` (deprecated legacy string tracking).

Key methods:
- `render(prevFrame, frame)` — main diff entry point. If `shouldClearScreen()` triggers, prepends a `clearTerminal` patch. Otherwise runs the incremental diff algorithm
- `renderPreviousOutput_DEPRECATED(prevFrame)` — used for final output on exit (writes the last frame to terminal)
- `reset()` — clears `previousOutput` (called after SIGCONT)

**Incremental diff algorithm (`renderDiff`):**
1. Walk rows top-down, comparing `prevScreen` and `screen` cell-by-cell via `diffEach`
2. For unchanged rows: emit cursor moves to skip them
3. For changed rows: emit `styleStr` transitions + `stdout` content patches + `hyperlink` patches
4. Handle wide chars: skip `SpacerTail` cells; emit a space for `SpacerHead` (end-of-line wrap guard)
5. Track hyperlink state across rows (emit `LINK_END` when hyperlink changes)
6. After last row: position cursor per `frame.cursor`

**DECSTBM hardware scroll (`scrollHint`):**
When the back frame has a `scrollHint` and no layout shift occurred and the viewport can accommodate the shift:
- Emit `setScrollRegion(top, bottom)` (DECSTBM)
- Emit `scrollUp(n)` (CSI S) or `scrollDown(n)` (CSI T)
- Emit `RESET_SCROLL_REGION`
- Only repaint the rows that changed due to the scroll (a narrow repair band)

This replaces O(viewport) cell writes with O(scrolled region) writes for smooth scroll in fullscreen mode.

### `/x/Bigger-Projects/Claude-Code/src/ink/optimizer.ts`

**Purpose:** Single-pass patch list optimizer.

**`optimize(diff)`:** Rules applied:
- Remove empty `stdout` patches
- Remove no-op `cursorMove(0,0)` patches
- Remove `clear` patches with count 0
- **Merge** consecutive `cursorMove` patches (add x/y)
- **Collapse** consecutive `cursorTo` patches (keep last)
- **Concat** adjacent `styleStr` patches
- **Deduplicate** consecutive `hyperlink` patches with same URI
- **Cancel** adjacent `cursorHide`/`cursorShow` pairs

### `/x/Bigger-Projects/Claude-Code/src/ink/node-cache.ts`

**Purpose:** Stores the rendered bounding box for each `DOMElement`, used for blit optimization and hit-testing.

**Exports:**
```typescript
type CachedLayout = { x: number; y: number; width: number; height: number; top?: number }
const nodeCache = new WeakMap<DOMElement, CachedLayout>()
const pendingClears = new WeakMap<DOMElement, Rectangle[]>()

addPendingClear(parent, rect, isAbsolute): void
consumeAbsoluteRemovedFlag(): boolean
```

`pendingClears` holds rectangles of removed children that must be painted over on the next render. `absoluteNodeRemoved` gates the global blit disable path for absolute-positioned removals.

---

## Terminal I/O

### `/x/Bigger-Projects/Claude-Code/src/ink/terminal.ts`

**Purpose:** Terminal capability detection and the `writeDiffToTerminal` serializer.

**`Terminal` type:** `{ stdout: NodeJS.WriteStream; stderr: NodeJS.WriteStream }`

**Capability detection:**
- `isSynchronizedOutputSupported()` — returns `true` for iTerm2, WezTerm, Warp, ghostty, kitty, VS Code, alacritty, foot, etc. Returns `false` for tmux (parses but doesn't implement DEC 2026 properly)
- `isProgressReportingAvailable()` — returns `true` for ConEmu, Ghostty 1.2.0+, iTerm2 3.6.6+; excludes Windows Terminal (interprets OSC 9;4 as notifications)
- `supportsExtendedKeys()` — detects Kitty keyboard protocol or modifyOtherKeys support
- `isXtermJs()` — set by XTVERSION probe (survives SSH, unlike TERM_PROGRAM)
- `setXtversionName(name)` — called by `App.tsx` after terminal query response

**`writeDiffToTerminal(terminal, diff, syncOutput)`:**
Serializes a `Diff` into ANSI sequences and writes to `terminal.stdout`. If `syncOutput === SYNC_OUTPUT_SUPPORTED`, wraps in BSU/ESU (DEC 2026 synchronized output). The serialization is a tight loop over the `Patch` array:
- `stdout`: write content directly
- `clear`: `eraseLines(count)` — moves cursor up and erases
- `clearTerminal`: `getClearTerminalSequence()` — erase screen + scrollback
- `cursorHide` / `cursorShow`: emit HIDE/SHOW_CURSOR sequences
- `cursorMove`: emit `cursorMove(x, y)` relative move
- `cursorTo`: emit `cursorTo(col)` absolute column
- `carriageReturn`: emit `\r`
- `hyperlink`: emit `link(uri)` or `LINK_END`
- `styleStr`: write pre-serialized ANSI transition string directly

`SYNC_OUTPUT_SUPPORTED` constant is computed once at module load.

---

## Termio Layer

### `/x/Bigger-Projects/Claude-Code/src/ink/termio.ts` — Termio Public API

Re-exports:
- `Parser` from `termio/parser.ts`
- All types: `Action`, `Color`, `CursorAction`, `CursorDirection`, `EraseAction`, `Grapheme`, `LinkAction`, `ModeAction`, `NamedColor`, `ScrollAction`, `TextSegment`, `TextStyle`, `TitleAction`, `UnderlineStyle`
- `colorsEqual`, `defaultStyle`, `stylesEqual`

### `/x/Bigger-Projects/Claude-Code/src/ink/termio/types.ts`

**Purpose:** Semantic type definitions for all ANSI parser output.

**Key types:**
```typescript
// 16-color palette
type NamedColor = 'black' | 'red' | ... | 'brightWhite'

// 3-way color union
type Color =
  | { type: 'named'; name: NamedColor }
  | { type: 'indexed'; index: number }   // 0-255
  | { type: 'rgb'; r: number; g: number; b: number }
  | { type: 'default' }

type UnderlineStyle = 'none' | 'single' | 'double' | 'curly' | 'dotted' | 'dashed'

type TextStyle = {
  bold: boolean; dim: boolean; italic: boolean; underline: UnderlineStyle
  blink: boolean; inverse: boolean; hidden: boolean; strikethrough: boolean
  overline: boolean; fg: Color; bg: Color; underlineColor: Color
}

// All parsed actions
type Action =
  | { type: 'text'; graphemes: Grapheme[]; style: TextStyle }
  | { type: 'cursor'; action: CursorAction }
  | { type: 'erase'; action: EraseAction }
  | { type: 'scroll'; action: ScrollAction }
  | { type: 'mode'; action: ModeAction }
  | { type: 'link'; action: LinkAction }
  | { type: 'title'; action: TitleAction }
  | { type: 'tabStatus'; action: TabStatusAction }
  | { type: 'sgr'; params: string }
  | { type: 'bell' }
  | { type: 'reset' }
  | { type: 'unknown'; sequence: string }
```

**`TabStatusAction`:** `{ indicator?: Color | null; status?: string | null; statusColor?: Color | null }` — for OSC 21337 tab chrome metadata.

Utility functions: `defaultStyle()`, `stylesEqual(a, b)`, `colorsEqual(a, b)`.

### `/x/Bigger-Projects/Claude-Code/src/ink/termio/ansi.ts`

**Purpose:** Base ANSI constants and C0 control character codes.

**Exports:**
- `C0` object — complete C0 control character table (NUL through DEL)
- `ESC = '\x1b'`, `BEL = '\x07'`, `SEP = ';'`
- `ESC_TYPE` — escape sequence introducers: `CSI=0x5b, OSC=0x5d, DCS=0x50, APC=0x5f, PM=0x5e, SOS=0x58, ST=0x5c`
- `isC0(byte)`, `isEscFinal(byte)`

### `/x/Bigger-Projects/Claude-Code/src/ink/termio/csi.ts`

**Purpose:** CSI (Control Sequence Introducer) sequence generation and constants.

**Exports:**
- `CSI_PREFIX = ESC + '['`
- `CSI_RANGE` — parameter/intermediate/final byte ranges
- `isCSIParam`, `isCSIIntermediate`, `isCSIFinal`
- `csi(...args)` — sequence generator: `ESC [ params... final`
- `CSI` enum — final byte codes: `CUU=0x41(A), CUD=0x42(B), CUF=0x43(C), CUB=0x44(D), CNL=0x45(E), CPL=0x46(F), CHA=0x47(G), CUP=0x48(H), ED=0x4a(J), EL=0x4b(K), SU=0x53(S), SD=0x54(T), SGR=0x6d(m), DECSTBM=0x72(r), ...`
- Pre-generated sequences: `CURSOR_HOME`, `ERASE_SCREEN`, `ERASE_SCROLLBACK`, `RESET_SCROLL_REGION`, `PASTE_START`, `PASTE_END`, `FOCUS_IN`, `FOCUS_OUT`, `ENABLE_KITTY_KEYBOARD`, `DISABLE_KITTY_KEYBOARD`, `ENABLE_MODIFY_OTHER_KEYS`, `DISABLE_MODIFY_OTHER_KEYS`
- Parameterized: `cursorMove(x,y)`, `cursorTo(col)`, `cursorPosition(row,col)`, `eraseLines(count)`, `setScrollRegion(top,bottom)`, `scrollUp(n)`, `scrollDown(n)`

### `/x/Bigger-Projects/Claude-Code/src/ink/termio/dec.ts`

**Purpose:** DEC private mode sequence generation.

**Exports:**
- `DEC` enum — mode numbers: `CURSOR_VISIBLE=25, ALT_SCREEN=47, ALT_SCREEN_CLEAR=1049, MOUSE_NORMAL=1000, MOUSE_BUTTON=1002, MOUSE_ANY=1003, MOUSE_SGR=1006, FOCUS_EVENTS=1004, BRACKETED_PASTE=2004, SYNCHRONIZED_UPDATE=2026`
- `decset(mode)` / `decreset(mode)` — `CSI ? N h` / `CSI ? N l`
- Pre-generated: `BSU` (begin synchronized update), `ESU`, `EBP/DBP` (bracketed paste), `EFE/DFE` (focus events), `SHOW_CURSOR/HIDE_CURSOR`, `ENTER_ALT_SCREEN/EXIT_ALT_SCREEN`
- `ENABLE_MOUSE_TRACKING` — combination of 1000+1002+1003+1006 set
- `DISABLE_MOUSE_TRACKING` — reverse order reset

### `/x/Bigger-Projects/Claude-Code/src/ink/termio/osc.ts`

**Purpose:** OSC (Operating System Command) sequence generation and clipboard/tab-status support.

**Exports:**
- `OSC_PREFIX = ESC + ']'`, `ST = ESC + '\\'`
- `osc(...parts)` — generates `ESC ] parts BEL` (or ST for Kitty)
- `wrapForMultiplexer(sequence)` — wraps in tmux DCS passthrough (`ESC P tmux ; ... ESC \`) or GNU screen DCS if `TMUX`/`STY` env vars set
- `link(url)` / `LINK_END` — OSC 8 hyperlink start/end
- `setClipboard(text)` — OSC 52 + optional pbcopy/tmux load-buffer; returns `ClipboardPath`
- `getClipboardPath()` — `'native' | 'tmux-buffer' | 'osc52'` without side effects
- `tmuxLoadBuffer(text)` — async: runs `tmux load-buffer [-w] -`
- `tabStatus({indicator, status, statusColor})` / `CLEAR_TAB_STATUS` — OSC 21337 tab chrome
- `supportsTabStatus()` — detects iTerm2 / Claude Code terminal from env
- `CLEAR_ITERM2_PROGRESS` — clears iTerm2 progress bar

**Clipboard path decision:**
- `native`: macOS + no SSH_CONNECTION → use `pbcopy`
- `tmux-buffer`: inside tmux → use `tmux load-buffer [-w]`
- `osc52`: fallback → write OSC 52 raw sequence

### `/x/Bigger-Projects/Claude-Code/src/ink/termio/sgr.ts`

**Purpose:** SGR (Select Graphic Rendition) parameter parser.

**`applySGR(paramStr, style)`:** Parses semicolon/colon separated SGR params and mutates a `TextStyle`. Handles:
- SGR 0: reset
- SGR 1/2/3/4/5/7/8/9/53: bold/dim/italic/underline/blink/inverse/hidden/strikethrough/overline
- SGR 21/22/23/24/25/27/28/29/55: attribute reset
- SGR 30-37/90-97: named fg colors
- SGR 40-47/100-107: named bg colors
- SGR 38/48: extended fg/bg (256-indexed via `5` or truecolor via `2`)
- SGR 39/49: default fg/bg
- SGR 58/59: underline color
- Kitty extended underline styles (SGR 4:1-4:5)

Colon-separated subparams take priority over semicolon-separated for extended colors.

### `/x/Bigger-Projects/Claude-Code/src/ink/termio/esc.ts`

**Purpose:** ESC (non-CSI, non-OSC) sequence parser. Handles `ESC c` (full reset), `ESC 7`/`ESC 8` (save/restore cursor), and other two-byte sequences.

**`parseEsc(sequence)`:** Returns an `Action | null`.

### `/x/Bigger-Projects/Claude-Code/src/ink/termio/tokenize.ts`

**Purpose:** Streaming tokenizer for terminal input — splits raw bytes into text chunks and escape sequences without interpreting semantics.

**States:** `ground`, `escape`, `escapeIntermediate`, `csi`, `ss3`, `osc`, `dcs`, `apc`

**`Token` type:** `{ type: 'text'; value: string } | { type: 'sequence'; value: string }`

**`Tokenizer` interface:**
```typescript
{
  feed(input: string): Token[]
  flush(): Token[]
  reset(): void
  buffer(): string
}
```

**`createTokenizer(options?)`:**
- `options.x10Mouse` — enables X10 legacy mouse event parsing (consume 3 extra bytes after `CSI M`)
- Maintains incremental state across `feed()` calls for streaming input

**Algorithm:** Character-by-character state machine. `ground` state: text passes through until `ESC` or C0 control chars. `csi` state: accumulates until CSI final byte (0x40–0x7e). `osc`/`dcs`/`apc` states: accumulate until BEL or ST.

### `/x/Bigger-Projects/Claude-Code/src/ink/termio/parser.ts`

**Purpose:** Semantic parser that wraps the tokenizer and interprets each sequence into a structured `Action`.

**`Parser` class:**
```typescript
class Parser {
  feed(input: string): Action[]
  flush(): Action[]
  reset(): void
  getStyle(): TextStyle
}
```

**Internal structure:**
- Holds a `Tokenizer` with `x10Mouse: true`
- Maintains current `TextStyle` (updated by SGR actions)
- Calls `parseCSI`, `parseEsc`, `parseOSC` for sequence tokens
- Calls `segmentGraphemes` for text tokens

**Grapheme width detection:**
- `isEmoji(codePoint)` — ranges: 0x2600-0x26ff, 0x2700-0x27bf, 0x1F300-0x1F9FF, 0x1FA00-0x1FAFF, 0x1F1E0-0x1F1FF
- `isEastAsianWide(codePoint)` — standard CJK/Hangul ranges
- `graphemeWidth(grapheme)` — returns 1 or 2
- `segmentGraphemes(str)` — uses `Intl.Segmenter` to split by grapheme cluster

**`parseCSI(rawSequence)`:** Dispatches by final byte:
- `m` (SGR): `{ type: 'sgr', params }`
- Cursor movement (A-H, d, f): `{ type: 'cursor', action }`
- Erase (J, K, X): `{ type: 'erase', action }`
- Scroll (S, T): `{ type: 'scroll', action }`
- DEC private modes (h/l with `?` prefix): `{ type: 'mode', action }`
- Mouse events (M/m with `<` prefix): decoded SGR mouse

---

## Event System

### `/x/Bigger-Projects/Claude-Code/src/ink/events/event.ts`

**Purpose:** Base `Event` class providing `stopImmediatePropagation()`.

```typescript
class Event {
  stopImmediatePropagation(): void
  didStopImmediatePropagation(): boolean
}
```

### `/x/Bigger-Projects/Claude-Code/src/ink/events/terminal-event.ts`

**Purpose:** `TerminalEvent` extends `Event` with DOM-style propagation properties.

**`EventTarget` type:** `{ parentNode: EventTarget | undefined; _eventHandlers?: Record<string, unknown> }`

**`TerminalEvent` properties:**
- `type: string`, `timeStamp: number`, `bubbles: boolean`, `cancelable: boolean`
- `target: EventTarget | null`, `currentTarget: EventTarget | null`
- `eventPhase: 'none' | 'capturing' | 'at_target' | 'bubbling'`
- `defaultPrevented: boolean`

**Methods:** `stopPropagation()`, `stopImmediatePropagation()` (overrides base), `preventDefault()`

Internal setters: `_setTarget`, `_setCurrentTarget`, `_setEventPhase`, `_isPropagationStopped()`, `_isImmediatePropagationStopped()`, `_prepareForTarget(target)` (hook for subclasses)

### `/x/Bigger-Projects/Claude-Code/src/ink/events/dispatcher.ts`

**Purpose:** Two-phase (capture + bubble) event dispatcher modeled after the browser DOM event model.

**`Dispatcher` class:**
- `dispatch(target, event)` — full capture+bubble cycle via `collectListeners` + `processDispatchQueue`; runs asynchronously (via `unstable_batchedUpdates` or React's scheduler for `discrete` vs `continuous` events)
- `dispatchDiscrete(target, event)` — discrete priority (keyboard, focus); triggers React's synchronous flush
- Internal `collectListeners(target, event)` — walks from target to root, prepending capture handlers (root-first) and appending bubble handlers (target-first); result: `[root-cap, ..., target-cap, target-bub, ..., root-bub]`
- `processDispatchQueue(listeners, event)` — calls each handler, checking `_isPropagationStopped()` and `_isImmediatePropagationStopped()` before each

**React event priority mapping:**
- Keyboard/focus → `DiscreteEventPriority`
- Mouse motion → `ContinuousEventPriority`
- Other → `DefaultEventPriority`

### `/x/Bigger-Projects/Claude-Code/src/ink/events/event-handlers.ts`

**Purpose:** Defines the complete set of event handler props and the reverse lookup table.

**`EventHandlerProps`:**
```typescript
{
  onKeyDown?, onKeyDownCapture?: KeyboardEventHandler
  onFocus?, onFocusCapture?, onBlur?, onBlurCapture?: FocusEventHandler
  onPaste?, onPasteCapture?: PasteEventHandler
  onResize?: ResizeEventHandler
  onClick?: ClickEventHandler
  onMouseEnter?, onMouseLeave?: HoverEventHandler
}
```

**`HANDLER_FOR_EVENT`:** Maps event type strings to `{ bubble?, capture? }` prop name pairs. Used by `Dispatcher` for O(1) handler lookup.

**`EVENT_HANDLER_PROPS`:** `Set<string>` of all handler prop names; used by the reconciler to route event props to `_eventHandlers` instead of `attributes`.

### `/x/Bigger-Projects/Claude-Code/src/ink/events/keyboard-event.ts`

**`KeyboardEvent extends TerminalEvent`:**
- Constructor takes a `ParsedKey`; type = `'keydown'`; `bubbles = true`; `cancelable = true`
- `key: string` — printable char for printable keys; multi-char name for specials (`'down'`, `'return'`, `'escape'`, `'f1'`, etc.)
- `ctrl, shift, meta, superKey, fn: boolean`

Key extraction: ctrl keys use the name (letter); single printable ASCII chars use the literal char; special keys use the parsed name.

### `/x/Bigger-Projects/Claude-Code/src/ink/events/click-event.ts`

**`ClickEvent extends Event`:**
- `col: number`, `row: number` — 0-indexed screen coordinates
- `localCol: number`, `localRow: number` — coordinates relative to the current handler's Box (updated by `dispatchClick` before each handler fires)
- `cellIsBlank: boolean` — true if the cell had no content (allows handlers to ignore clicks on empty space)

### Other event types:

**`InputEvent`** (events/input-event.ts): Legacy input event emitted on stdin data; carries `input: string` and `Key` object. `Key` type: `{ upArrow, downArrow, leftArrow, rightArrow, pageUp, pageDown, return, escape, ctrl, shift, tab, backspace, delete, meta, fn }`.

**`FocusEvent`** (events/focus-event.ts): `type = 'focus' | 'blur'`; `relatedTarget: DOMElement | null`

**`TerminalFocusEvent`** (events/terminal-focus-event.ts): `type: TerminalFocusEventType = 'terminal-focus-in' | 'terminal-focus-out'`; fired when DECSET 1004 focus events arrive.

**`EventEmitter`** (events/emitter.ts): Simple typed event emitter. `on(event, handler)`, `off(event, handler)`, `emit(event, ...args)`. Used by `App.tsx` for stdin data events.

---

## Focus Management

### `/x/Bigger-Projects/Claude-Code/src/ink/focus.ts`

**`FocusManager` class:**

Stored on `ink-root` node so any node can reach it by walking `parentNode`.

State:
- `activeElement: DOMElement | null`
- `focusStack: DOMElement[]` — history for focus restoration (max 32 entries)
- `enabled: boolean`

Methods:
- `focus(node)` — blur previous, push to stack, focus new node; dispatches `focus`/`blur` events
- `blur()` — blur `activeElement`, dispatches `blur` event
- `handleNodeRemoved(node, root)` — removes node from stack, restores focus from stack if `activeElement` was in the removed subtree
- `handleClickFocus(node)` — called by `dispatchClick`; focuses the nearest focusable ancestor
- `getNextFocusable(root, direction)` — Tab/Shift+Tab cycling; collects all nodes with `tabIndex >= 0`, sorts by order, returns next/previous
- `enable()` / `disable()` — gates all focus operations

**`getFocusManager(node)`** / **`getRootNode(node)`** — utility functions exported for the reconciler.

---

## Input Parsing

### `/x/Bigger-Projects/Claude-Code/src/ink/parse-keypress.ts`

**Purpose:** Parses raw terminal stdin bytes into structured `ParsedKey` objects. Handles standard ANSI sequences, Kitty keyboard protocol (CSI u), xterm modifyOtherKeys, SGR mouse events, and terminal response sequences.

**`ParsedKey` type:**
```typescript
{
  kind: 'key' | 'mouse' | 'terminalResponse'
  name: string        // key name: 'up', 'down', 'return', 'escape', 'f1', 'a', ...
  fn: boolean
  ctrl: boolean; meta: boolean; shift: boolean; option: boolean; super: boolean
  sequence: string   // raw escape sequence
  raw: string        // original input bytes
  isPasted?: boolean
}
```

**`ParsedMouse` type:**
```typescript
{
  kind: 'mouse'
  button: number     // SGR button code
  col: number; row: number  // 1-indexed
  press: boolean    // true = press, false = release
  isWheel: boolean
  isDrag: boolean
  modifiers: { ctrl, shift, meta, alt }
}
```

**`ParsedInput = ParsedKey | ParsedMouse | TerminalResponse`**

**Regex patterns:**
- `META_KEY_CODE_RE`: `ESC + [a-zA-Z0-9]` — meta key combos
- `FN_KEY_RE`: SS3/CSI function key sequences
- `CSI_U_RE`: Kitty protocol `ESC [ codepoint [;modifier] u`
- `MODIFY_OTHER_KEYS_RE`: xterm `ESC [ 27 ; modifier ; keycode ~`
- `DECRPM_RE`, `DA1_RE`, `DA2_RE`, `KITTY_FLAGS_RE`, `CURSOR_POSITION_RE` — terminal response patterns
- `OSC_RESPONSE_RE`: OSC sequence responses
- `XTVERSION_RE`: DCS `>|` terminal name/version
- `SGR_MOUSE_RE`: `ESC [ < btn ; col ; row M/m`

**`parseMultipleKeypresses(buffer, state)`:** Main entry point. Uses the tokenizer to split the input, then dispatches each token to the appropriate parser. Handles bracketed paste (accumulates until `PASTE_END`).

**`INITIAL_STATE`:** Initial parser state for `parseMultipleKeypresses`.

---

## Hit Testing

### `/x/Bigger-Projects/Claude-Code/src/ink/hit-test.ts`

**Purpose:** Mouse click hit-testing against the DOM tree.

**`hitTest(node, col, row)`:** DFS in reverse child order (last child = top paint layer wins). Uses `nodeCache` for bounding-box lookups. Returns the deepest `DOMElement` whose rendered rect contains `(col, row)`, or `null`.

**`dispatchClick(root, col, row, cellIsBlank?)`:**
1. Runs `hitTest` to find the deepest hit node
2. Calls `focusManager.handleClickFocus()` to click-to-focus the nearest focusable ancestor
3. Creates a `ClickEvent(col, row, cellIsBlank)`
4. Bubbles up via `parentNode` chain, calling `onClick` handlers
5. Before each handler: sets `event.localCol/localRow` relative to the handler's bounding rect
6. Stops on `stopImmediatePropagation()`
7. Returns `true` if any handler fired

**`dispatchHover(root, col, row, hoveredNodes)`:** Diff-based hover dispatch. Finds the set of nodes hit at `(col, row)`, fires `onMouseEnter` for newly-entered nodes and `onMouseLeave` for exited nodes. Mutates `hoveredNodes` in place (owned by the `Ink` instance).

---

## Text Selection

### `/x/Bigger-Projects/Claude-Code/src/ink/selection.ts`

**Purpose:** Linear text selection in screen-buffer coordinates for fullscreen mode.

**`SelectionState` type:**
```typescript
{
  anchor: { col, row } | null
  focus: { col, row } | null
  isDragging: boolean
  anchorSpan: { lo, hi, kind: 'word' | 'line' } | null  // word/line mode
  scrolledOffAbove: string[]     // rows that scrolled off the top
  scrolledOffBelow: string[]
  scrolledOffAboveSW: boolean[]  // soft-wrap flags parallel to scrolledOffAbove
  scrolledOffBelowSW: boolean[]
  virtualAnchorRow?: number      // pre-clamp anchor row (for scroll restore)
  virtualFocusRow?: number
  lastPressHadAlt: boolean
}
```

**Exported functions:**
- `createSelectionState()` — returns zeroed state
- `startSelection(s, col, row, alt?)` — initializes anchor and focus
- `updateSelection(s, col, row)` — updates focus during drag
- `finishSelection(s)` — clears `isDragging`
- `clearSelection(s)` — resets to empty
- `hasSelection(s)` — returns true if `anchor !== null && focus !== null`
- `getSelectedText(s, screen)` — extracts the selected text; handles soft-wrap (joins wrapped lines), wide chars (skips `SpacerTail`), `noSelect` regions (excluded), and `scrolledOffAbove`/`scrolledOffBelow` accumulators
- `applySelectionOverlay(s, screen, stylePool)` — inverts cell styles in the selected region
- `selectWordAt(s, col, row, screen)` — double-click: select the word under the cursor
- `selectLineAt(s, row, screen)` — triple-click: select the entire line
- `extendSelection(s, col, row, screen)` — word/line-mode drag extension: extends to word/line boundaries
- `shiftSelection(s, dRow, min, max, screenWidth)` — keyboard scroll: shifts both anchor and focus
- `shiftAnchor(s, dRow, min, max)` — shift anchor only (keyboard selection extension)
- `moveFocus(s, move, screen)` — keyboard character/line focus extension
- `captureScrolledRows(s, firstRow, lastRow, side, screen)` — saves rows about to scroll off into `scrolledOffAbove`/`scrolledOffBelow`
- `shiftSelectionForFollow(s, delta, screen)` — called after follow-scroll to keep selection anchored to text

---

## Search Highlighting

### `/x/Bigger-Projects/Claude-Code/src/ink/searchHighlight.ts`

**`applySearchHighlight(screen, query, stylePool)`:**
- Case-insensitive scan of the screen buffer
- Builds per-row char text + `codeUnitToCell` index map (handles wide chars)
- For each match: calls `setCellStyleId(screen, ..., stylePool.withInverse(cellStyleId))` to invert the cell style
- Skips `noSelect` regions (gutters, line numbers)
- Returns `true` if any match was applied (triggers full-damage flag in caller)

The `applyPositionedHighlight` (in `render-to-screen.ts`) handles the "current match" in yellow, on top of `applySearchHighlight`'s inverse.

---

## Component Library

### `/x/Bigger-Projects/Claude-Code/src/ink/components/App.tsx`

**Purpose:** The root React component. Wires together stdin input, terminal size context, focus, clock, error boundaries, and all context providers.

**Props:** `children, stdin, stdout, stderr, exitOnCtrlC, onExit, terminalColumns, terminalRows, selection, onSelectionChange, onClickAt, onHoverAt, getHyperlinkAt, onOpenHyperlink, onMultiClick, onSelectionDrag, onStdinResume?, onCursorDeclaration`

**Responsibilities:**
- Provides `AppContext`, `StdinContext`, `TerminalSizeContext`, `ClockContext`, `TerminalFocusProvider`, `CursorDeclarationContext`, `TerminalWriteProvider`
- Listens to stdin data; calls `parseMultipleKeypresses` for each chunk
- Dispatches `KeyboardEvent` through the DOM via `dispatcher.dispatchDiscrete(rootNode, event)`
- Handles mouse events: left-button selection start/update/finish; wheel → `pendingScrollDelta`; hover → `onHoverAt`; click → `onClickAt`
- Handles pasted content via bracketed paste markers
- Detects terminal focus via DECSET 1004 `FOCUS_IN`/`FOCUS_OUT` sequences
- Runs `TerminalQuerier` on mount to probe XTVERSION and extended key support
- Re-asserts terminal modes after `STDIN_RESUME_GAP_MS = 5000` ms of stdin silence
- Handles `Ctrl+C` → `onExit` when `exitOnCtrlC = true`
- Handles `Ctrl+Z` (SIGTSTP) on non-Windows platforms

**Class component** (`PureComponent`) for stable reference identity. All mouse/keyboard state is imperative (refs), not React state, to avoid re-renders.

### `/x/Bigger-Projects/Claude-Code/src/ink/components/Box.tsx`

**Purpose:** The primary layout container. Analogous to `<div style="display: flex">`.

**`Props`:** All `Styles` properties (except `textWrap`) plus:
- `ref?: Ref<DOMElement>`
- `tabIndex?: number` — focus order (>= 0 participates in Tab cycling, -1 = programmatic only)
- `autoFocus?: boolean` — focus on mount
- `onClick?: (event: ClickEvent) => void`
- `onFocus?, onFocusCapture?, onBlur?, onBlurCapture?`
- `onKeyDown?, onKeyDownCapture?`
- `onMouseEnter?, onMouseLeave?`

Renders to `<ink-box>` host element. Compiled with React Compiler (memo cache `_c(42)`).

### `/x/Bigger-Projects/Claude-Code/src/ink/components/Text.tsx`

**Purpose:** Renders styled text. Wraps children in `ink-text` and `ink-virtual-text` elements.

**`Props`:**
- `color?, backgroundColor?`
- `bold?, dim?` (mutually exclusive via TypeScript union)
- `italic?, underline?, strikethrough?, inverse?`
- `wrap?: Styles['textWrap']`
- `children?: ReactNode`

Text styling props are mapped to `TextStyles` and passed as the `textStyles` prop on the host element. Layout props (flexDirection, flexGrow, etc.) are mapped to `Styles` on the `ink-text` node.

### `/x/Bigger-Projects/Claude-Code/src/ink/components/ScrollBox.tsx`

**Purpose:** A Box with imperative scroll API and viewport culling.

**`ScrollBoxHandle`:**
```typescript
{
  scrollTo(y): void
  scrollBy(dy): void
  scrollToElement(el, offset?): void   // defers position read to render time
  scrollToBottom(): void
  getScrollTop(): number
  getPendingDelta(): number
  getScrollHeight(): number
  getFreshScrollHeight(): number       // reads Yoga directly
  getViewportHeight(): number
  getViewportTop(): number
  isSticky(): boolean
  subscribe(listener): () => void
  setClampBounds(min, max): void
}
```

**`ScrollBoxProps`:** All `Styles` except `overflow`/`overflowX`/`overflowY`, plus `stickyScroll?: boolean`.

Implementation: Sets `overflow: 'scroll'` on the underlying Box. Scroll mutations call `markDirty` + `scheduleRenderFrom` to trigger an Ink frame without going through React's reconciler. `scrollToElement` stores a `scrollAnchor` on the DOM node; `render-node-to-output` reads it at paint time (after Yoga has computed the element's position) and clears it.

### `/x/Bigger-Projects/Claude-Code/src/ink/components/AlternateScreen.tsx`

**Purpose:** Enters the terminal's alternate screen buffer for fullscreen rendering.

**Props:** `children, mouseTracking?: boolean` (default `true`)

Uses `useInsertionEffect` (not `useLayoutEffect`) to send `ENTER_ALT_SCREEN` before the first Ink render frame. On cleanup (unmount), sends `EXIT_ALT_SCREEN`. Calls `instances.get(process.stdout).setAltScreenActive(true/false)` to coordinate with the Ink instance. Renders children inside a `Box` constrained to `terminalRows` height.

### `/x/Bigger-Projects/Claude-Code/src/ink/components/Link.tsx`

**Purpose:** Renders OSC 8 terminal hyperlinks.

Wraps children in an `ink-link` host element with `href` attribute. During rendering, `squashTextNodesToSegments` propagates the hyperlink URL to contained text segments.

### `/x/Bigger-Projects/Claude-Code/src/ink/components/RawAnsi.tsx`

**Purpose:** Renders pre-formatted ANSI strings with known dimensions.

Props: `children: string, width: number, height: number`

Renders to `ink-raw-ansi` element with `rawWidth`/`rawHeight` attributes. The yoga measure function reads these dimensions directly (no string width measurement, no wrapping).

### `/x/Bigger-Projects/Claude-Code/src/ink/components/NoSelect.tsx`

**Purpose:** Marks a rectangular region as non-selectable (e.g., gutters, line numbers).

Sets the `noSelect` flag on cells in the screen buffer via `markNoSelectRegion`. Text in these cells is excluded from selection copy and search highlighting.

### `/x/Bigger-Projects/Claude-Code/src/ink/components/Newline.tsx`

**Purpose:** Renders `\n` characters (count configurable via `count` prop).

### `/x/Bigger-Projects/Claude-Code/src/ink/components/Spacer.tsx`

**Purpose:** Flexible spacer. Renders a `Box` with `flexGrow: 1`.

### `/x/Bigger-Projects/Claude-Code/src/ink/components/Button.tsx`

**Purpose:** Focusable, clickable button with keyboard activation.

**`ButtonState`:** `'idle' | 'active' | 'focus'`

**Props:** `children, onPress?, disabled?` plus styling.

Uses `useApp` for exit integration. Handles Return/Space keydown when focused.

### `/x/Bigger-Projects/Claude-Code/src/ink/components/ErrorOverview.tsx`

**Purpose:** Error boundary overlay shown when a React component throws. Renders the stack trace and error message with formatting.

### Context components:

**`AppContext.ts`** — provides `{ exit(error?): void }`. Consumed by `useApp`.

**`StdinContext.ts`** — provides `{ stdin, setRawMode, isRawModeSupported, internal_exitOnCtrlC, internal_eventEmitter }`. Consumed by `useStdin`, `useInput`.

**`TerminalSizeContext.tsx`** — provides `{ columns: number; rows: number } | null`. Consumed by `useTerminalViewport`, `AlternateScreen`.

**`ClockContext.tsx`** — provides a shared animation clock with `subscribe(cb, keepAlive?)` and `now()`. All `useAnimationFrame` instances share one clock; idle clock (no `keepAlive` subscribers) suspends to avoid waking the process.

**`CursorDeclarationContext.ts`** — provides a setter for declaring native cursor position. Consumed by `useDeclaredCursor`. The setter signature: `(decl: CursorDeclaration | null, node?: DOMElement | null) => void`.

**`TerminalFocusContext.tsx`** — provides `useTerminalFocus()` which subscribes to DECSET 1004 focus events via `useSyncExternalStore` on `terminal-focus-state.ts`.

---

## Hooks

### `/x/Bigger-Projects/Claude-Code/src/ink/hooks/use-input.ts`

**`useInput(handler, options?)`:** Subscribes to stdin input events. Calls `setRawMode(true)` via `useLayoutEffect` (synchronous, before render returns). Subscribes to `internal_eventEmitter` `'input'` events. Options: `{ isActive?: boolean }`.

### `/x/Bigger-Projects/Claude-Code/src/ink/hooks/use-app.ts`

**`default useApp()`:** Returns `{ exit(error?): void }` from `AppContext`. Throws if used outside the App tree.

### `/x/Bigger-Projects/Claude-Code/src/ink/hooks/use-stdin.ts`

**`default useStdin()`:** Returns the full `StdinContext` value.

### `/x/Bigger-Projects/Claude-Code/src/ink/hooks/use-animation-frame.ts`

**`useAnimationFrame(intervalMs?)`:** Returns `[ref, time]`. Subscribes to the shared clock and updates `time` every `intervalMs`. Pauses (unsubscribes) when `intervalMs = null` or the element is off-screen (via `useTerminalViewport`).

### `/x/Bigger-Projects/Claude-Code/src/ink/hooks/use-interval.ts`

**`useInterval(callback, delay?)`:** Calls `callback` every `delay` ms. Uses the shared clock.

**`useAnimationTimer(intervalMs)`:** Returns `time` (elapsed ms). Similar to `useAnimationFrame` but without the viewport visibility check.

### `/x/Bigger-Projects/Claude-Code/src/ink/hooks/use-terminal-viewport.ts`

**`useTerminalViewport()`:** Returns `[ref, { isVisible }]`. Computes visibility by walking the DOM ancestor chain (including `scrollTop` offsets) during `useLayoutEffect`. Does NOT cause re-renders on visibility change — callers read the current value naturally.

### `/x/Bigger-Projects/Claude-Code/src/ink/hooks/use-terminal-focus.ts`

**`useTerminalFocus()`:** Returns `boolean` — whether the terminal window is focused. Uses `useSyncExternalStore` on `terminal-focus-state.ts` module-level signal.

### `/x/Bigger-Projects/Claude-Code/src/ink/hooks/use-terminal-title.ts`

**`useTerminalTitle(title)`:** Sets the terminal window title via OSC 0/2 on mount and clears on unmount.

### `/x/Bigger-Projects/Claude-Code/src/ink/hooks/use-selection.ts`

**`useSelection()`:** Returns an API object for text selection operations. Falls back to no-ops when not in fullscreen mode. The `Ink` instance is located via `instances.get(process.stdout)`.

Methods: `copySelection()`, `copySelectionNoClear()`, `clearSelection()`, `hasSelection()`, `getState()`, `subscribe(cb)`, `shiftAnchor(dRow, min, max)`, `shiftSelection(dRow, min, max)`, `moveFocus(move)`, `captureScrolledRows(first, last, side)`, `setSelectionBgColor(color)`.

### `/x/Bigger-Projects/Claude-Code/src/ink/hooks/use-tab-status.ts`

**`useTabStatus(kind: TabStatusKind | null)`:** Emits OSC 21337 tab status sequences. `kind = 'idle' | 'busy' | 'waiting'`. Transitions to `null` emit `CLEAR_TAB_STATUS`. Wrapped for tmux passthrough. Uses `TerminalWriteContext`.

### `/x/Bigger-Projects/Claude-Code/src/ink/hooks/use-declared-cursor.ts`

**`useDeclaredCursor({ line, column, active })`:** Returns a ref callback. When active, declares the cursor position to the `Ink` instance so the native cursor parks at the text caret. Uses `useLayoutEffect` (no dep array — re-declares every commit) for correct sibling handoff. Clears on unmount via a separate `useLayoutEffect` with empty deps.

### `/x/Bigger-Projects/Claude-Code/src/ink/hooks/use-search-highlight.ts`

Internal hook for wiring search query to the Ink instance's `setSearchHighlight` / `setSearchPositions`.

---

## Utility Modules

### `/x/Bigger-Projects/Claude-Code/src/ink/Ansi.tsx`

**`Ansi` component:** Parses ANSI escape codes in a string and renders them using `Text` and `Link` components. Memoized. Accepts `children: string` and optional `dimColor: boolean`. Uses `termio.Parser` to extract spans and maps them to `Text` props + `Link` wrappers for hyperlinks.

### `/x/Bigger-Projects/Claude-Code/src/ink/bidi.ts`

**`reorderBidi(characters: ClusteredChar[])`:** Applies the Unicode Bidi Algorithm to a `ClusteredChar` array when running on Windows Terminal, WSL, or xterm.js (all lack native bidi). Uses `bidi-js` library. Detects need via `WT_SESSION` env var or `TERM_PROGRAM=vscode`. No-op on platforms with native bidi support.

### `/x/Bigger-Projects/Claude-Code/src/ink/clearTerminal.ts`

**`getClearTerminalSequence()`:** Returns `ERASE_SCREEN + ERASE_SCROLLBACK + CURSOR_HOME` on modern terminals. Windows: uses HVP (`ESC [ 0 f`) for cursor home on legacy console; includes scrollback clear for Windows Terminal, VS Code, and mintty.

**`clearTerminal`:** Pre-computed clear sequence (module load time).

### `/x/Bigger-Projects/Claude-Code/src/ink/colorize.ts`

**`colorize(text, styles)`:** Applies chalk-based color/style transforms to a text string. Detects the chalk level and adjusts for xterm.js and tmux environments.

**`applyTextStyles(text, textStyles)`:** Converts `TextStyles` to chalk method chain calls.

**Color level management:**
- `boostChalkLevelForXtermJs()` — upgrades chalk to level 3 (truecolor) when `TERM_PROGRAM=vscode` and chalk detected level 2
- `clampChalkLevelForTmux()` — downgrades to level 2 (256-color) inside tmux to avoid truecolor passthrough bugs; skipped when `CLAUDE_CODE_TMUX_TRUECOLOR=1`

### `/x/Bigger-Projects/Claude-Code/src/ink/get-max-width.ts`

**`getMaxWidth(node, offsetWidth?)`:** Computes the available render width for a node accounting for padding and border. Reads from `yogaNode.getComputedPadding`/`getComputedBorder` for each relevant edge.

### `/x/Bigger-Projects/Claude-Code/src/ink/line-width-cache.ts`

**`lineWidth(line: string)`:** Memoized string width for individual lines (no newlines). Cache is a `Map<string, number>`. Used by `measureText`.

### `/x/Bigger-Projects/Claude-Code/src/ink/measure-element.ts`

**`measureElement(node: DOMElement)`:** Returns `{ width, height }` by reading `yogaNode.getComputedWidth()/getComputedHeight()`. Throws if the node has no yoga node.

### `/x/Bigger-Projects/Claude-Code/src/ink/measure-text.ts`

**`measureText(text, maxWidth)`:** Single-pass computation of `{ width, height }` for a text string. Uses `lineWidth` per line. Height = sum of `ceil(lineWidth / maxWidth)` per line (or 1 when `noWrap`).

### `/x/Bigger-Projects/Claude-Code/src/ink/squash-text-nodes.ts`

**`squashTextNodes(node)`:** Concatenates all text content of a node tree into a plain string (no styles). Used by `measureTextNode` in `dom.ts`.

**`squashTextNodesToSegments(node, inheritedStyles?, inheritedHyperlink?, out?)`:** Walks the text node tree and produces `StyledSegment[]` — text with inherited styles and hyperlink URLs. Used by `output.ts` for structured rendering.

**`StyledSegment` type:** `{ text: string; styles: TextStyles; hyperlink?: string }`

### `/x/Bigger-Projects/Claude-Code/src/ink/stringWidth.ts`

**`stringWidth(str)`:** Terminal display width of a string (counts wide chars as 2, strips ANSI codes). Uses `@alcalzone/ansi-tokenize` + grapheme segmentation.

### `/x/Bigger-Projects/Claude-Code/src/ink/widest-line.ts`

**`widestLine(text)`:** Returns the display width of the widest line in a multi-line string.

### `/x/Bigger-Projects/Claude-Code/src/ink/wrap-text.ts`

**`wrapText(text, maxWidth, wrapType)`:** Applies the appropriate text wrap strategy:
- `'wrap'`: `wrapAnsi(text, maxWidth, { trim: false, hard: true })`
- `'wrap-trim'`: `wrapAnsi(text, maxWidth, { trim: true, hard: true })`
- `'truncate-end'` / `'truncate'`: append `…`
- `'truncate-middle'`: insert `…` in middle
- `'truncate-start'`: prepend `…`
- `'end'` / `'middle'`: same as corresponding truncate

Uses `sliceFit` to avoid wide-char boundary errors in slice operations.

### `/x/Bigger-Projects/Claude-Code/src/ink/wrapAnsi.ts`

Custom ANSI-aware word-wrap implementation. Handles wide chars, hyperlinks (OSC 8), and soft-wrap tracking. Returns wrapped text plus `softWrap[]` flags.

### `/x/Bigger-Projects/Claude-Code/src/ink/supports-hyperlinks.ts`

**`supportsHyperlinks()`:** Detects terminal support for OSC 8 hyperlinks. Returns `true` for iTerm2, VS Code, kitty, Ghostty, WezTerm, Warp, and others.

### `/x/Bigger-Projects/Claude-Code/src/ink/tabstops.ts`

**`expandTabs(text, startColumn?)`:** Expands tab characters to spaces based on 8-column tab stops. Used during text measurement (worst-case width). Actual tab expansion at render time uses the screen x-position.

### `/x/Bigger-Projects/Claude-Code/src/ink/render-border.ts`

**`renderBorder(node, output, x, y, width, height)`:** Draws box borders using `cli-boxes` glyphs (single, double, round, bold, classic, dashed). Supports `borderText` option to embed text into top/bottom border lines with start/end/center alignment. Colors applied via `chalk`.

### `/x/Bigger-Projects/Claude-Code/src/ink/terminal-focus-state.ts`

Module-level singleton signal for terminal focus state.

**`TerminalFocusState`:** `'focused' | 'blurred' | 'unknown'`

**Exports:**
- `setTerminalFocused(v)` — updates state, notifies `useSyncExternalStore` subscribers
- `getTerminalFocused()` — returns `focusState !== 'blurred'` (unknown treated as focused)
- `getTerminalFocusState()` — returns the tristate value
- `subscribeTerminalFocus(cb)` — subscribe function for `useSyncExternalStore`
- `resetTerminalFocusState()` — resets to `'unknown'`

### `/x/Bigger-Projects/Claude-Code/src/ink/terminal-querier.ts`

**Purpose:** Queries the terminal for capability information using DA1/DECRQM/XTVERSION sentinel protocol (no timeouts — DA1 is the universal sentinel).

**`TerminalQuery<T>` type:** `{ request: string; match: (r) => r is T }`

**Query builders:**
- `decrqm(mode)` — DECRQM query; response: `DecrpmResponse`
- `da1()` — Primary Device Attributes
- `da2()` — Secondary Device Attributes
- `kittyKeyboard()` — Kitty keyboard flags query
- `cursorPosition()` — DECXCPR
- `oscColor(code, index?)` — OSC 10/11/12 color queries
- `xtversion()` — DCS `>|` terminal name/version

**`TerminalQuerier` class:**
- `send<T>(query)` — returns a `Promise<T | undefined>`
- `flush()` — sends a DA1 sentinel; all pending queries resolve when DA1 response arrives (terminals answer in order)
- Internal: holds a queue of `{ query, resolve }` entries

**`xtversion`:** Stores the XTVERSION name (set by App.tsx after the query resolves; read by `isXtermJs()`).

### `/x/Bigger-Projects/Claude-Code/src/ink/useTerminalNotification.ts`

**`TerminalWriteContext`:** React context providing a `(data: string) => void` write function that bypasses the normal Ink render pipeline (direct stdout write).

**`TerminalWriteProvider`:** `= TerminalWriteContext.Provider`

**`useTerminalNotification()`:** Returns notification methods:
- `notifyITerm2({ message, title? })` — OSC 9 iTerm2 notification
- `notifyKitty({ message, title, id })` — Kitty notification via OSC 99
- `notifyGhostty({ message, title })` — Ghostty notification via OSC 99 variant
- `notifyBell()` — raw BEL character
- `progress(state, percentage?)` — OSC 9;4 progress bar (Ghostty 1.2+, iTerm2 3.6.6+, ConEmu)

### `/x/Bigger-Projects/Claude-Code/src/ink/warn.ts`

Centralized warning emitter (wraps `console.warn` with deduplication). Used by `Box.tsx` for invalid prop combinations.

---

## Architecture: The Complete Pipeline

```
[stdin bytes]
    → App.tsx handleInput()
    → parseMultipleKeypresses()  (termio tokenizer + key parser)
    → KeyboardEvent / ParsedMouse / TerminalResponse
    → dispatcher.dispatchDiscrete(rootNode, event)  // keyboard
    → React setState / component handlers

[React state change]
    → React reconciler commit
    → reconciler.resetAfterCommit(rootNode)
    → rootNode.onComputeLayout()  // yoga calculateLayout()
    → rootNode.onRender()  →  queueMicrotask(ink.onRender)

[ink.onRender()]
    → createRenderer(rootNode, stylePool)(options)
        → renderNodeToOutput(rootNode, output, { prevScreen })
            → DOM walk with blit/clip/write/scroll ops
            → output.get() → Screen (cell buffer)
        → return Frame { screen, viewport, cursor, scrollHint }
    → applySelectionOverlay(selection, frame.screen, stylePool)
    → applySearchHighlight(frame.screen, query, stylePool)
    → applyPositionedHighlight(frame.screen, positions, ...)
    → log.render(prevFrame, frame) → Diff (Patch[])
    → optimize(diff) → compressed Patch[]
    → writeDiffToTerminal(terminal, diff, syncOutput)
        → BSU (if sync supported)
        → for each Patch: write ANSI to stdout
        → ESU (if sync supported)
    → emit onFrame(FrameEvent)
    → swap frontFrame ↔ backFrame
```

### Double Buffering

The `Ink` class maintains `frontFrame` (the last displayed frame) and `backFrame` (the rendering target). After each render:
- `backFrame.screen` contains the newly rendered content
- It becomes the new `frontFrame`
- Old `frontFrame` becomes the new `backFrame` for the next render

The renderer always reads `prevScreen = frontFrame.screen` for blit operations and writes into `backFrame.screen`.

### Blit Optimization

The blit path is the dominant fast path for steady-state rendering:
1. If a node's `dirty = false` AND `nodeCache` has a valid rect AND `prevScreen` is available AND no layout shift occurred AND no removed children: call `blitRegion(backScreen, prevScreen, cachedRect)` — pure `Int32Array.copyWithin`, O(cells).
2. This means spinner ticks and clock updates only re-render the changed cell ranges; the rest of the screen is copied in bulk.

### Synchronized Output (BSU/ESU)

When `SYNC_OUTPUT_SUPPORTED = true`, each frame is wrapped in DEC mode 2026 begin/end synchronized update sequences (`BSU` / `ESU`). This prevents terminals from rendering intermediate states during the diff write. Supported terminals: iTerm2, WezTerm, Warp, ghostty, kitty, VS Code, alacritty, foot, kitty.

### DECSTBM Hardware Scroll

When a `ScrollBox`'s `scrollTop` changes and layout is otherwise stable, `render-node-to-output` sets a `ScrollHint`. `LogUpdate.render()` checks for this hint and emits:
1. `setScrollRegion(top, bottom)` — DECSTBM restricts scroll to the box's viewport
2. `SU(n)` or `SD(n)` — hardware scroll by n rows
3. `RESET_SCROLL_REGION` — restore full-screen scroll
4. Then only re-renders the narrow band of newly exposed cells

This replaces O(viewport × width) cell writes with O(exposed_rows × width) for smooth scrolling.

### Pool Generational Reset

The `StylePool` and `CharPool` live for the entire session (never reset) to ensure stable IDs for the blit optimization (IDs are comparable as integers across frames). The `HyperlinkPool` is reset every 5 minutes (hyperlinks are ephemeral) via `migrateScreenPools()`, which re-interns all active cells into fresh pool instances.
