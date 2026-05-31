# Claude Code — Constants, Types & Configuration

This document exhaustively catalogs every constant, type, interface, and configuration value defined across the `constants/` directory, key root-level files (`Tool.ts`, `Task.ts`), and the `types/` directory of the Claude Code CLI codebase.

---

## Table of Contents

1. [API Limits (`constants/apiLimits.ts`)](#1-api-limits)
2. [Beta Headers (`constants/betas.ts`)](#2-beta-headers)
3. [Common Utilities (`constants/common.ts`)](#3-common-utilities)
4. [Cyber Risk Instruction (`constants/cyberRiskInstruction.ts`)](#4-cyber-risk-instruction)
5. [Error IDs (`constants/errorIds.ts`)](#5-error-ids)
6. [UI Figures / Glyphs (`constants/figures.ts`)](#6-ui-figures--glyphs)
7. [File Constants (`constants/files.ts`)](#7-file-constants)
8. [GitHub App (`constants/github-app.ts`)](#8-github-app)
9. [Keys / Analytics (`constants/keys.ts`)](#9-keys--analytics)
10. [Messages (`constants/messages.ts`)](#10-messages)
11. [OAuth Configuration (`constants/oauth.ts`)](#11-oauth-configuration)
12. [Output Styles (`constants/outputStyles.ts`)](#12-output-styles)
13. [Product URLs (`constants/product.ts`)](#13-product-urls)
14. [System Prompts (`constants/prompts.ts`)](#14-system-prompts)
15. [Spinner Verbs (`constants/spinnerVerbs.ts`)](#15-spinner-verbs)
16. [System Constants (`constants/system.ts`)](#16-system-constants)
17. [System Prompt Sections (`constants/systemPromptSections.ts`)](#17-system-prompt-sections)
18. [Tool Limits (`constants/toolLimits.ts`)](#18-tool-limits)
19. [Tool Sets (`constants/tools.ts`)](#19-tool-sets)
20. [Turn Completion Verbs (`constants/turnCompletionVerbs.ts`)](#20-turn-completion-verbs)
21. [XML Tags (`constants/xml.ts`)](#21-xml-tags)
22. [Tool Type System (`Tool.ts`)](#22-tool-type-system)
23. [Task Type System (`Task.ts`)](#23-task-type-system)
24. [Types Directory](#24-types-directory)
    - [IDs (`types/ids.ts`)](#241-ids-typesidsts)
    - [Permissions (`types/permissions.ts`)](#242-permissions-typespermissionsts)
    - [Commands (`types/command.ts`)](#243-commands-typescommandts)
    - [Hooks (`types/hooks.ts`)](#244-hooks-typeshooksts)
    - [Plugins (`types/plugin.ts`)](#245-plugins-typespluginsts)
    - [Logs (`types/logs.ts`)](#246-logs-typeslogsts)
    - [Text Input Types (`types/textInputTypes.ts`)](#247-text-input-types-typestextinputtypests)

---

## 1. API Limits

**File:** `constants/apiLimits.ts`
**Purpose:** Server-side limits enforced by the Claude API. Dependency-free to prevent circular imports. Last verified: 2025-12-22.

### Image Limits

| Constant | Value | Description |
|---|---|---|
| `API_IMAGE_MAX_BASE64_SIZE` | `5 * 1024 * 1024` = **5,242,880 bytes** (5 MB) | Maximum base64-encoded image size. The API rejects images where the base64 string length exceeds this. This is the base64 length, NOT raw bytes (base64 increases size ~33%). |
| `IMAGE_TARGET_RAW_SIZE` | `(API_IMAGE_MAX_BASE64_SIZE * 3) / 4` = **3,932,160 bytes** (3.75 MB) | Target raw image size to stay under base64 limit. Derived as: `raw_size = base64_size * 3/4`. |
| `IMAGE_MAX_WIDTH` | `2000` (pixels) | Client-side max width for image resizing. The API internally resizes above 1568px server-side (no error), but client resizes at 2000px to preserve quality. |
| `IMAGE_MAX_HEIGHT` | `2000` (pixels) | Client-side max height for image resizing. Same rationale as `IMAGE_MAX_WIDTH`. |

### PDF Limits

| Constant | Value | Description |
|---|---|---|
| `PDF_TARGET_RAW_SIZE` | `20 * 1024 * 1024` = **20,971,520 bytes** (20 MB) | Maximum raw PDF size before encoding. The API has a 32 MB total request limit; 20 MB raw → ~27 MB base64, leaving room for conversation context. |
| `API_PDF_MAX_PAGES` | `100` | Maximum number of pages in a PDF accepted by the API. |
| `PDF_EXTRACT_SIZE_THRESHOLD` | `3 * 1024 * 1024` = **3,145,728 bytes** (3 MB) | PDFs above this size are extracted to page images instead of being sent as base64 document blocks. Applies to first-party API only; non-first-party always uses extraction. |
| `PDF_MAX_EXTRACT_SIZE` | `100 * 1024 * 1024` = **104,857,600 bytes** (100 MB) | Maximum PDF file size for the page extraction path. PDFs larger than this are rejected. |
| `PDF_MAX_PAGES_PER_READ` | `20` | Maximum pages the Read tool will extract in a single call when `pages` parameter is used. |
| `PDF_AT_MENTION_INLINE_THRESHOLD` | `10` | PDFs with more pages than this get reference treatment on `@` mention instead of being inlined into context. |

### Media Limits

| Constant | Value | Description |
|---|---|---|
| `API_MAX_MEDIA_PER_REQUEST` | `100` | Maximum number of media items (images + PDFs) per API request. Client-side validation provides a clear error message before hitting the API's confusing error. |

---

## 2. Beta Headers

**File:** `constants/betas.ts`
**Purpose:** API beta feature header strings used in request headers to opt into experimental API features.

### Exported Beta Header Constants

| Constant | Value | Description |
|---|---|---|
| `CLAUDE_CODE_20250219_BETA_HEADER` | `'claude-code-20250219'` | Core Claude Code beta header. |
| `INTERLEAVED_THINKING_BETA_HEADER` | `'interleaved-thinking-2025-05-14'` | Enables interleaved thinking (extended reasoning within tool use). |
| `CONTEXT_1M_BETA_HEADER` | `'context-1m-2025-08-07'` | Enables 1M token context window. |
| `CONTEXT_MANAGEMENT_BETA_HEADER` | `'context-management-2025-06-27'` | Context management features. |
| `STRUCTURED_OUTPUTS_BETA_HEADER` | `'structured-outputs-2025-12-15'` | Structured output response format. |
| `WEB_SEARCH_BETA_HEADER` | `'web-search-2025-03-05'` | Web search tool. |
| `TOOL_SEARCH_BETA_HEADER_1P` | `'advanced-tool-use-2025-11-20'` | Tool search for Claude API / Foundry. |
| `TOOL_SEARCH_BETA_HEADER_3P` | `'tool-search-tool-2025-10-19'` | Tool search for Vertex AI / Bedrock. |
| `EFFORT_BETA_HEADER` | `'effort-2025-11-24'` | Effort control (thinking budget). |
| `TASK_BUDGETS_BETA_HEADER` | `'task-budgets-2026-03-13'` | Task-level token budgets. |
| `PROMPT_CACHING_SCOPE_BETA_HEADER` | `'prompt-caching-scope-2026-01-05'` | Scoped prompt caching (global vs per-session). |
| `FAST_MODE_BETA_HEADER` | `'fast-mode-2026-02-01'` | Fast mode for faster output at same model quality. |
| `REDACT_THINKING_BETA_HEADER` | `'redact-thinking-2026-02-12'` | Redact thinking blocks from API response. |
| `TOKEN_EFFICIENT_TOOLS_BETA_HEADER` | `'token-efficient-tools-2026-03-28'` | Token-efficient tool schema encoding. |
| `SUMMARIZE_CONNECTOR_TEXT_BETA_HEADER` | `'summarize-connector-text-2026-03-13'` if `feature('CONNECTOR_TEXT')`, else `''` | Connector text summarization (feature-gated). |
| `AFK_MODE_BETA_HEADER` | `'afk-mode-2026-01-31'` if `feature('TRANSCRIPT_CLASSIFIER')`, else `''` | AFK (autonomous) mode (feature-gated). |
| `CLI_INTERNAL_BETA_HEADER` | `'cli-internal-2026-02-09'` if `USER_TYPE === 'ant'`, else `''` | Internal-only features. |
| `ADVISOR_BETA_HEADER` | `'advisor-tool-2026-03-01'` | Advisor tool. |

### Provider-Specific Beta Header Sets

**`BEDROCK_EXTRA_PARAMS_HEADERS`** — `Set<string>`
Bedrock only supports a limited number of beta headers and only through `extraBodyParams`. This set maintains the beta strings that should be in Bedrock `extraBodyParams` and NOT in Bedrock headers.

Members:
- `INTERLEAVED_THINKING_BETA_HEADER` (`'interleaved-thinking-2025-05-14'`)
- `CONTEXT_1M_BETA_HEADER` (`'context-1m-2025-08-07'`)
- `TOOL_SEARCH_BETA_HEADER_3P` (`'tool-search-tool-2025-10-19'`)

**`VERTEX_COUNT_TOKENS_ALLOWED_BETAS`** — `Set<string>`
Betas allowed on Vertex `countTokens` API. Other betas cause 400 errors.

Members:
- `CLAUDE_CODE_20250219_BETA_HEADER` (`'claude-code-20250219'`)
- `INTERLEAVED_THINKING_BETA_HEADER` (`'interleaved-thinking-2025-05-14'`)
- `CONTEXT_MANAGEMENT_BETA_HEADER` (`'context-management-2025-06-27'`)

---

## 3. Common Utilities

**File:** `constants/common.ts`

### Functions

**`getLocalISODate(): string`**
Returns the current date in ISO format (`YYYY-MM-DD`) using the local timezone. Respects the `CLAUDE_CODE_OVERRIDE_DATE` environment variable for ant-only date overrides (testing/debugging).

**`getSessionStartDate`** — memoized version of `getLocalISODate`
Captures the date once at session start for prompt-cache stability. Used in simple mode (`--bare`) to avoid busting the cached prefix at midnight. Exported as a memoized constant so subsequent calls return the same value.

**`getLocalMonthYear(): string`**
Returns `"Month YYYY"` (e.g., `"February 2026"`) in the user's local timezone. Changes monthly, not daily — used in tool prompts to minimize cache busting. Respects `CLAUDE_CODE_OVERRIDE_DATE`.

---

## 4. Cyber Risk Instruction

**File:** `constants/cyberRiskInstruction.ts`
**Ownership:** Safeguards team (David Forsythe, Kyla Guru). **Do not modify without Safeguards team review.**

### `CYBER_RISK_INSTRUCTION` — `string`

```
IMPORTANT: Assist with authorized security testing, defensive security, CTF challenges,
and educational contexts. Refuse requests for destructive techniques, DoS attacks, mass
targeting, supply chain compromise, or detection evasion for malicious purposes.
Dual-use security tools (C2 frameworks, credential testing, exploit development) require
clear authorization context: pentesting engagements, CTF competitions, security research,
or defensive use cases.
```

This instruction is injected directly into the system prompt (both the simple intro section and the proactive/autonomous mode section). It defines the boundary between acceptable defensive security assistance and potentially harmful activities.

---

## 5. Error IDs

**File:** `constants/errorIds.ts`
**Purpose:** Obfuscated numeric identifiers for tracking error sources in production. Structured as individual `const` exports for optimal dead code elimination in external builds.

**Next ID (as of file):** 346

| Constant | Value | Description |
|---|---|---|
| `E_TOOL_USE_SUMMARY_GENERATION_FAILED` | `344` | Identifies errors from tool use summary generation. |

---

## 6. UI Figures / Glyphs

**File:** `constants/figures.ts`

### Status Indicators

| Constant | Value | Unicode | Description |
|---|---|---|---|
| `BLACK_CIRCLE` | `'⏺'` (macOS) or `'●'` (others) | U+23FA / U+25CF | Platform-adaptive spinner/status dot. |
| `BULLET_OPERATOR` | `'∙'` | U+2219 | Small bullet operator. |
| `TEARDROP_ASTERISK` | `'✻'` | U+273B | Decorative asterisk. |
| `UP_ARROW` | `'\u2191'` (`↑`) | U+2191 | Used for opus 1m merge notice. |
| `DOWN_ARROW` | `'\u2193'` (`↓`) | U+2193 | Used for scroll hint. |
| `LIGHTNING_BOLT` | `'↯'` (`\u21af`) | U+21AF | Fast mode indicator. |

### Effort Level Indicators

| Constant | Value | Unicode | Description |
|---|---|---|---|
| `EFFORT_LOW` | `'○'` | U+25CB | Effort level: low. |
| `EFFORT_MEDIUM` | `'◐'` | U+25D0 | Effort level: medium. |
| `EFFORT_HIGH` | `'●'` | U+25CF | Effort level: high. |
| `EFFORT_MAX` | `'◉'` | U+25C9 | Effort level: max (Opus 4.6 only). |

### Media/Trigger Status

| Constant | Value | Unicode | Description |
|---|---|---|---|
| `PLAY_ICON` | `'\u25b6'` (`▶`) | U+25B6 | Play state indicator. |
| `PAUSE_ICON` | `'\u23f8'` (`⏸`) | U+23F8 | Pause state indicator. |

### MCP Subscription Indicators

| Constant | Value | Unicode | Description |
|---|---|---|---|
| `REFRESH_ARROW` | `'\u21bb'` (`↻`) | U+21BB | Resource update indicator. |
| `CHANNEL_ARROW` | `'\u2190'` (`←`) | U+2190 | Inbound channel message indicator. |
| `INJECTED_ARROW` | `'\u2192'` (`→`) | U+2192 | Cross-session injected message indicator. |
| `FORK_GLYPH` | `'\u2442'` (`⑂`) | U+2442 | Fork directive indicator. |

### Review Status Indicators (Ultrareview)

| Constant | Value | Unicode | Description |
|---|---|---|---|
| `DIAMOND_OPEN` | `'\u25c7'` (`◇`) | U+25C7 | Running state. |
| `DIAMOND_FILLED` | `'\u25c6'` (`◆`) | U+25C6 | Completed/failed state. |
| `REFERENCE_MARK` | `'\u203b'` (`※`) | U+203B | Komejirushi; away-summary recap marker. |

### Other Indicators

| Constant | Value | Unicode | Description |
|---|---|---|---|
| `FLAG_ICON` | `'\u2691'` (`⚑`) | U+2691 | Issue flag banner. |
| `BLOCKQUOTE_BAR` | `'\u258e'` (`▎`) | U+258E | Left one-quarter block; blockquote line prefix. |
| `HEAVY_HORIZONTAL` | `'\u2501'` (`━`) | U+2501 | Heavy box-drawing horizontal line. |

### Bridge Status Indicators

| Constant | Value | Description |
|---|---|---|
| `BRIDGE_SPINNER_FRAMES` | `['·\|·', '·/·', '·—·', '·\\·']` | Animation frames for the bridge spinner. |
| `BRIDGE_READY_INDICATOR` | `'·✔︎·'` | Shown when bridge is ready. |
| `BRIDGE_FAILED_INDICATOR` | `'×'` | Shown when bridge fails. |

---

## 7. File Constants

**File:** `constants/files.ts`

### `BINARY_EXTENSIONS` — `Set<string>`

Set of file extensions considered binary. Used to skip text-based operations on these files. Contains 113 extensions across the following categories:

**Images:** `.png`, `.jpg`, `.jpeg`, `.gif`, `.bmp`, `.ico`, `.webp`, `.tiff`, `.tif`

**Videos:** `.mp4`, `.mov`, `.avi`, `.mkv`, `.webm`, `.wmv`, `.flv`, `.m4v`, `.mpeg`, `.mpg`

**Audio:** `.mp3`, `.wav`, `.ogg`, `.flac`, `.aac`, `.m4a`, `.wma`, `.aiff`, `.opus`

**Archives:** `.zip`, `.tar`, `.gz`, `.bz2`, `.7z`, `.rar`, `.xz`, `.z`, `.tgz`, `.iso`

**Executables/Binaries:** `.exe`, `.dll`, `.so`, `.dylib`, `.bin`, `.o`, `.a`, `.obj`, `.lib`, `.app`, `.msi`, `.deb`, `.rpm`

**Documents:** `.pdf` *(excluded at call site for FileReadTool)*, `.doc`, `.docx`, `.xls`, `.xlsx`, `.ppt`, `.pptx`, `.odt`, `.ods`, `.odp`

**Fonts:** `.ttf`, `.otf`, `.woff`, `.woff2`, `.eot`

**Bytecode/VM artifacts:** `.pyc`, `.pyo`, `.class`, `.jar`, `.war`, `.ear`, `.node`, `.wasm`, `.rlib`

**Database files:** `.sqlite`, `.sqlite3`, `.db`, `.mdb`, `.idx`

**Design/3D:** `.psd`, `.ai`, `.eps`, `.sketch`, `.fig`, `.xd`, `.blend`, `.3ds`, `.max`

**Flash:** `.swf`, `.fla`

**Lock/profiling:** `.lockb`, `.dat`, `.data`

### Functions

**`hasBinaryExtension(filePath: string): boolean`**
Checks if a file path has a binary extension by looking up its extension (lowercased) in `BINARY_EXTENSIONS`.

**`isBinaryContent(buffer: Buffer): boolean`**
Detects binary content by inspecting up to 8192 bytes (`BINARY_CHECK_SIZE`):
- Returns `true` immediately if a null byte (`0x00`) is found.
- Returns `true` if more than 10% of checked bytes are non-printable, non-whitespace (excluding tab `0x09`, newline `0x0A`, carriage return `0x0D`).

---

## 8. GitHub App

**File:** `constants/github-app.ts`
**Purpose:** Templates and metadata for the GitHub Actions workflow integration.

### String Constants

| Constant | Value | Description |
|---|---|---|
| `PR_TITLE` | `'Add Claude Code GitHub Workflow'` | Title of the PR created when installing the GitHub app workflow. |
| `GITHUB_ACTION_SETUP_DOCS_URL` | `'https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md'` | Link to setup documentation. |

### Template Constants

**`WORKFLOW_CONTENT`** — `string`
A complete GitHub Actions YAML workflow file (`name: Claude Code`) that:
- Triggers on: `issue_comment` (created), `pull_request_review_comment` (created), `issues` (opened, assigned), `pull_request_review` (submitted).
- Condition: Only runs when `@claude` is mentioned in the relevant event body.
- Runs on: `ubuntu-latest`.
- Required permissions: `contents: read`, `pull-requests: read`, `issues: read`, `id-token: write`, `actions: read`.
- Steps: `actions/checkout@v4` (depth 1), then `anthropics/claude-code-action@v1` with `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`.

**`PR_BODY`** — `string`
Markdown body for the installation PR. Explains what Claude Code is, how the workflow functions, security considerations (API key as secret, write-access restriction, Actions run history), and how to configure additional allowed tools.

**`CODE_REVIEW_PLUGIN_WORKFLOW_CONTENT`** — `string`
A GitHub Actions YAML workflow for automated code review (`name: Claude Code Review`):
- Triggers on: `pull_request` events (opened, synchronize, ready_for_review, reopened).
- Uses `anthropics/claude-code-action@v1` with the `code-review@claude-code-plugins` plugin from the `https://github.com/anthropics/claude-code.git` marketplace.
- Runs the `/code-review:code-review` command on the PR.

---

## 9. Keys / Analytics

**File:** `constants/keys.ts`
**Purpose:** GrowthBook feature flag client keys.

### `getGrowthBookClientKey(): string`

Returns one of three GrowthBook SDK client keys based on environment:

| Condition | Key |
|---|---|
| `USER_TYPE === 'ant'` AND `ENABLE_GROWTHBOOK_DEV` is truthy | `'sdk-yZQvlplybuXjYh6L'` (dev/internal) |
| `USER_TYPE === 'ant'` (without dev flag) | `'sdk-xRVcrliHIlrg4og4'` (ant prod) |
| All other users (external) | `'sdk-zAZezfDKGoZuXXKe'` (external prod) |

Implemented as a lazy function (not a constant) so `ENABLE_GROWTHBOOK_DEV` from `globalSettings.env` (applied after module load) is picked up at call time.

---

## 10. Messages

**File:** `constants/messages.ts`
**Purpose:** User-facing string constants for display.

| Constant | Value | Description |
|---|---|---|
| `NO_CONTENT_MESSAGE` | `'(no content)'` | Displayed when a tool result or message has no content to show. |

---

## 11. OAuth Configuration

**File:** `constants/oauth.ts`

### Scope Constants

| Constant | Value | Description |
|---|---|---|
| `CLAUDE_AI_INFERENCE_SCOPE` | `'user:inference'` | Scope for Claude.ai inference access. |
| `CLAUDE_AI_PROFILE_SCOPE` | `'user:profile'` | Scope for user profile access. |
| `OAUTH_BETA_HEADER` | `'oauth-2025-04-20'` | OAuth beta feature header. |

### Scope Arrays

**`CONSOLE_OAUTH_SCOPES`** — `readonly ['org:create_api_key', 'user:profile']`
OAuth scopes for Console (API key creation via Console).

**`CLAUDE_AI_OAUTH_SCOPES`** — `readonly ['user:profile', 'user:inference', 'user:sessions:claude_code', 'user:mcp_servers', 'user:file_upload']`
OAuth scopes for Claude.ai subscribers (Pro/Max/Team/Enterprise).

**`ALL_OAUTH_SCOPES`** — `string[]`
Union of all scopes from both arrays (deduplicated). When logging in, all scopes are requested to handle Console → Claude.ai redirect flows. Must be kept in sync with `OAuthConsentPage` in the apps repo.

### `OauthConfig` Type

```typescript
type OauthConfig = {
  BASE_API_URL: string
  CONSOLE_AUTHORIZE_URL: string
  CLAUDE_AI_AUTHORIZE_URL: string
  CLAUDE_AI_ORIGIN: string        // Separate from AUTHORIZE_URL for web page links
  TOKEN_URL: string
  API_KEY_URL: string
  ROLES_URL: string
  CONSOLE_SUCCESS_URL: string
  CLAUDEAI_SUCCESS_URL: string
  MANUAL_REDIRECT_URL: string
  CLIENT_ID: string
  OAUTH_FILE_SUFFIX: string
  MCP_PROXY_URL: string
  MCP_PROXY_PATH: string
}
```

### Production OAuth Configuration (`PROD_OAUTH_CONFIG`)

| Field | Value |
|---|---|
| `BASE_API_URL` | `'https://api.anthropic.com'` |
| `CONSOLE_AUTHORIZE_URL` | `'https://platform.claude.com/oauth/authorize'` |
| `CLAUDE_AI_AUTHORIZE_URL` | `'https://claude.com/cai/oauth/authorize'` (routes through claude.com/cai/* for attribution, 307s to claude.ai) |
| `CLAUDE_AI_ORIGIN` | `'https://claude.ai'` |
| `TOKEN_URL` | `'https://platform.claude.com/v1/oauth/token'` |
| `API_KEY_URL` | `'https://api.anthropic.com/api/oauth/claude_cli/create_api_key'` |
| `ROLES_URL` | `'https://api.anthropic.com/api/oauth/claude_cli/roles'` |
| `CONSOLE_SUCCESS_URL` | `'https://platform.claude.com/buy_credits?returnUrl=/oauth/code/success%3Fapp%3Dclaude-code'` |
| `CLAUDEAI_SUCCESS_URL` | `'https://platform.claude.com/oauth/code/success?app=claude-code'` |
| `MANUAL_REDIRECT_URL` | `'https://platform.claude.com/oauth/code/callback'` |
| `CLIENT_ID` | `'9d1c250a-e61b-44d9-88ed-5944d1962f5e'` |
| `OAUTH_FILE_SUFFIX` | `''` (no suffix for production) |
| `MCP_PROXY_URL` | `'https://mcp-proxy.anthropic.com'` |
| `MCP_PROXY_PATH` | `'/v1/mcp/{server_id}'` |

### Staging OAuth Configuration (`STAGING_OAUTH_CONFIG`)

Only included in `ant` builds. Key fields:

| Field | Value |
|---|---|
| `BASE_API_URL` | `'https://api-staging.anthropic.com'` |
| `CONSOLE_AUTHORIZE_URL` | `'https://platform.staging.ant.dev/oauth/authorize'` |
| `CLAUDE_AI_AUTHORIZE_URL` | `'https://claude-ai.staging.ant.dev/oauth/authorize'` |
| `CLIENT_ID` | `'22422756-60c9-4084-8eb7-27705fd5cf9a'` |
| `OAUTH_FILE_SUFFIX` | `'-staging-oauth'` |
| `MCP_PROXY_URL` | `'https://mcp-proxy-staging.anthropic.com'` |

### Local OAuth Configuration (Dynamic)

Built from environment variables with defaults:

| Env Var | Default |
|---|---|
| `CLAUDE_LOCAL_OAUTH_API_BASE` | `'http://localhost:8000'` |
| `CLAUDE_LOCAL_OAUTH_APPS_BASE` | `'http://localhost:4000'` |
| `CLAUDE_LOCAL_OAUTH_CONSOLE_BASE` | `'http://localhost:3000'` |

Local `CLIENT_ID`: `'22422756-60c9-4084-8eb7-27705fd5cf9a'`
Local `OAUTH_FILE_SUFFIX`: `'-local-oauth'`
Local `MCP_PROXY_URL`: `'http://localhost:8205'`
Local `MCP_PROXY_PATH`: `'/v1/toolbox/shttp/mcp/{server_id}'`

### Allowed OAuth Base URLs (FedStart/PubSec)

```
'https://beacon.claude-ai.staging.ant.dev'
'https://claude.fedstart.com'
'https://claude-staging.fedstart.com'
```

Only these base URLs are permitted for `CLAUDE_CODE_CUSTOM_OAUTH_URL` to prevent OAuth tokens from being sent to arbitrary endpoints.

### MCP Client Metadata

| Constant | Value |
|---|---|
| `MCP_CLIENT_METADATA_URL` | `'https://claude.ai/oauth/claude-code-client-metadata'` |

Used as `client_id` for MCP OAuth (CIMD/SEP-991) when the auth server advertises `client_id_metadata_document_supported: true`.

### Functions

**`getOauthConfig(): OauthConfig`**
Returns the appropriate OAuth configuration based on environment type (`prod`/`staging`/`local`). Applies `CLAUDE_CODE_CUSTOM_OAUTH_URL` override if set (validated against allowlist). Applies `CLAUDE_CODE_OAUTH_CLIENT_ID` override if set.

**`fileSuffixForOauthConfig(): string`**
Returns the file suffix for OAuth credential storage files: `''` for prod, `'-staging-oauth'` for staging, `'-local-oauth'` for local, `'-custom-oauth'` for custom OAuth URL.

---

## 12. Output Styles

**File:** `constants/outputStyles.ts`

### Types

```typescript
type OutputStyleConfig = {
  name: string
  description: string
  prompt: string
  source: SettingSource | 'built-in' | 'plugin'
  keepCodingInstructions?: boolean
  forceForPlugin?: boolean   // If true, automatically applied when the plugin is enabled
}

type OutputStyles = {
  readonly [K in OutputStyle]: OutputStyleConfig | null
}
```

### Constants

| Constant | Value | Description |
|---|---|---|
| `DEFAULT_OUTPUT_STYLE_NAME` | `'default'` | Name of the built-in default output style (no customization). |

### Built-In Output Styles (`OUTPUT_STYLE_CONFIG`)

**`default`:** `null` — No customization; standard Claude Code behavior.

**`Explanatory`:**
- Source: `'built-in'`
- Description: `'Claude explains its implementation choices and codebase patterns'`
- `keepCodingInstructions: true`
- Prompt instructs Claude to provide educational insights about the codebase. Uses an `EXPLANATORY_FEATURE_PROMPT` block with a distinctive code fence format using star (`★`) borders.

**`Learning`:**
- Source: `'built-in'`
- Description: `'Claude pauses and asks you to write small pieces of code for hands-on practice'`
- `keepCodingInstructions: true`
- Prompt encourages hands-on practice by requesting 2–10 line code contributions for design decisions, business logic, and key algorithms. Uses a "Learn by Doing" request format with **Context**, **Your Task**, and **Guidance** fields.

### Functions

**`getAllOutputStyles(cwd: string): Promise<{ [styleName: string]: OutputStyleConfig | null }>`**
Memoized. Merges built-in, plugin, user, project, and managed styles in priority order (built-in < plugin < user < project < managed).

**`getOutputStyleConfig(): Promise<OutputStyleConfig | null>`**
Returns the active output style configuration. First checks for a forced plugin output style; if multiple plugins force a style, uses the first and logs a warning. Falls back to the user's settings.

**`hasCustomOutputStyle(): boolean`**
Returns `true` if the user has selected a non-default output style.

**`clearAllOutputStylesCache(): void`**
Clears the memoization cache for `getAllOutputStyles`. Called on `/clear` and `/compact`.

---

## 13. Product URLs

**File:** `constants/product.ts`

### URL Constants

| Constant | Value | Description |
|---|---|---|
| `PRODUCT_URL` | `'https://claude.com/claude-code'` | Main product landing page. |
| `CLAUDE_AI_BASE_URL` | `'https://claude.ai'` | Claude AI base URL for remote sessions. |
| `CLAUDE_AI_STAGING_BASE_URL` | `'https://claude-ai.staging.ant.dev'` | Staging environment base URL. |
| `CLAUDE_AI_LOCAL_BASE_URL` | `'http://localhost:4000'` | Local development base URL. |

### Functions

**`isRemoteSessionStaging(sessionId?, ingressUrl?): boolean`**
Returns `true` if `sessionId` contains `'_staging_'` or `ingressUrl` contains `'staging'`.

**`isRemoteSessionLocal(sessionId?, ingressUrl?): boolean`**
Returns `true` if `sessionId` contains `'_local_'` or `ingressUrl` contains `'localhost'`.

**`getClaudeAiBaseUrl(sessionId?, ingressUrl?): string`**
Returns the appropriate base URL by checking environment: local → staging → prod.

**`getRemoteSessionUrl(sessionId, ingressUrl?): string`**
Returns the full URL to view a remote session at `${baseUrl}/code/${compatId}`. Translates `cse_*` prefixes to `session_*` for frontend compatibility via `toCompatSessionId()`.

---

## 14. System Prompts

**File:** `constants/prompts.ts`
**Purpose:** Core system prompt generation for all Claude Code session types.

### URL Constants

| Constant | Value | Description |
|---|---|---|
| `CLAUDE_CODE_DOCS_MAP_URL` | `'https://code.claude.com/docs/en/claude_code_docs_map.md'` | URL for Claude Code documentation map. |
| `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` | `'__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__'` | Boundary marker separating static (cross-org cacheable) content from dynamic content. Everything BEFORE this in the system prompt array can use `scope: 'global'`. |

### Model Configuration (Internal)

These are internal constants used within the prompts but not exported:

| Internal | Value | Description |
|---|---|---|
| `FRONTIER_MODEL_NAME` | `'Claude Opus 4.6'` | Current frontier model name (annotated with `@[MODEL LAUNCH]` for updates). |
| `CLAUDE_4_5_OR_4_6_MODEL_IDS.opus` | `'claude-opus-4-6'` | Latest Opus model ID. |
| `CLAUDE_4_5_OR_4_6_MODEL_IDS.sonnet` | `'claude-sonnet-4-6'` | Latest Sonnet model ID. |
| `CLAUDE_4_5_OR_4_6_MODEL_IDS.haiku` | `'claude-haiku-4-5-20251001'` | Latest Haiku model ID. |

### Knowledge Cutoff Dates (by Model)

| Model Pattern | Cutoff |
|---|---|
| `claude-sonnet-4-6` | `'August 2025'` |
| `claude-opus-4-6` | `'May 2025'` |
| `claude-opus-4-5` | `'May 2025'` |
| `claude-haiku-4*` | `'February 2025'` |
| `claude-opus-4` or `claude-sonnet-4` (generic) | `'January 2025'` |
| All others | `null` (no cutoff shown) |

### CLI System Prompt Prefix Values

```typescript
type CLISyspromptPrefix =
  | "You are Claude Code, Anthropic's official CLI for Claude."
  | "You are Claude Code, Anthropic's official CLI for Claude, running within the Claude Agent SDK."
  | "You are a Claude agent, built on Anthropic's Claude Agent SDK."
```

**`CLI_SYSPROMPT_PREFIXES`** — `ReadonlySet<string>` containing all three prefix values. Used by `splitSysPromptPrefix` to identify prefix blocks by content rather than position.

**`getCLISyspromptPrefix(options?): CLISyspromptPrefix`** — Selection logic:
- Vertex AI → always `DEFAULT_PREFIX`
- Non-interactive with `appendSystemPrompt` → `AGENT_SDK_CLAUDE_CODE_PRESET_PREFIX`
- Non-interactive without → `AGENT_SDK_PREFIX`
- Interactive → `DEFAULT_PREFIX`

### Exported Agent Prompt

**`DEFAULT_AGENT_PROMPT`** — `string`

```
You are an agent for Claude Code, Anthropic's official CLI for Claude. Given the user's
message, you should use the tools available to complete the task. Complete the task fully—
don't gold-plate, but don't leave it half-done. When you complete the task, respond with
a concise report covering what was done and any key findings — the caller will relay this
to the user, so it only needs the essentials.
```

Used as the base system prompt for subagents spawned by `AgentTool`.

### System Prompt Section Architecture

`getSystemPrompt()` assembles the full system prompt as an array of strings in this order:

**Static sections (cache-stable):**
1. `getSimpleIntroSection()` — Identity framing + CYBER_RISK_INSTRUCTION + URL policy
2. `getSimpleSystemSection()` — Communication rules, tool approvals, system-reminder tags, prompt injection warning, hooks, context compression
3. `getSimpleDoingTasksSection()` — Software engineering task guidance, code style rules, user help info
4. `getActionsSection()` — Action reversibility and blast radius guidance, risky action examples
5. `getUsingYourToolsSection()` — Dedicated tool preferences over Bash, parallel tool calls
6. `getSimpleToneAndStyleSection()` — No emoji, concise responses, file:line references, GitHub #issue format
7. `getOutputEfficiencySection()` — Different content for `ant` vs external users

**Dynamic boundary marker** (if global cache scope enabled):
- `SYSTEM_PROMPT_DYNAMIC_BOUNDARY`

**Dynamic sections (registry-managed, cached per-section):**
- `session_guidance` — Session-specific guidance (AskUserQuestion, shell bang, Agent tool, skills, DiscoverSkills, verification agent)
- `memory` — CLAUDE.md memory content
- `ant_model_override` — Ant-internal model overrides
- `env_info_simple` — Working directory, platform, OS, model description, knowledge cutoff
- `language` — Language preference
- `output_style` — Active output style
- `mcp_instructions` — MCP server instructions (uncached — MCP servers connect/disconnect between turns)
- `scratchpad` — Scratchpad directory instructions
- `frc` — Function result clearing instructions
- `summarize_tool_results` — Tool result summarization reminder
- `numeric_length_anchors` — (ant-only) Length limits: ≤25 words between tool calls, ≤100 words final responses
- `token_budget` — (TOKEN_BUDGET feature) Token target instructions
- `brief` — (KAIROS/KAIROS_BRIEF feature) Brief tool instructions

### Key System Prompt Sections

**`getSimpleDoingTasksSection()` - Code Style Rules (ant-internal):**
- Default to writing no comments
- Only add comments when the WHY is non-obvious
- Don't explain WHAT the code does
- Don't reference the current task in comments
- Don't remove existing comments unless removing the code they describe

**`getActionsSection()` - Risky Actions (must confirm):**
- Destructive: deleting files/branches, dropping DB tables, killing processes, `rm -rf`, overwriting uncommitted changes
- Hard-to-reverse: force-pushing, `git reset --hard`, amending published commits, removing packages, modifying CI/CD
- Shared-state: pushing code, creating/closing PRs/issues, sending messages (Slack, email, GitHub), posting to external services
- Uploads to third-party tools (may be cached/indexed)

**`getProactiveSection()` - Autonomous Mode:**
- Responds to `<tick>` prompts that keep it alive between turns
- Uses `SleepTool` to control wait time between actions (max 5 min prompt cache expiry)
- Must call `SleepTool` if nothing useful to do (never output "still waiting")
- First wake-up: greet user briefly, ask what they want
- Terminal focus awareness: unfocused = autonomous action; focused = collaborative

**`SUMMARIZE_TOOL_RESULTS_SECTION`** (non-exported):
```
When working with tool results, write down any important information you might need
later in your response, as the original tool result may be cleared later.
```

### Key Functions

**`getSystemPrompt(tools, model, additionalWorkingDirectories?, mcpClients?): Promise<string[]>`**
Main entry point for system prompt generation. Returns `['You are Claude Code...\nCWD: ...\nDate: ...']` when `CLAUDE_CODE_SIMPLE=1`.

**`computeEnvInfo(modelId, additionalWorkingDirectories?): Promise<string>`**
Generates the `<env>` block in legacy format for subagent system prompts.

**`computeSimpleEnvInfo(modelId, additionalWorkingDirectories?): Promise<string>`**
Generates the `# Environment` section for main system prompts (includes worktree warning, model IDs reference, Claude Code availability info, Fast Mode note).

**`enhanceSystemPromptWithEnvDetails(existingSystemPrompt, model, ...): Promise<string[]>`**
Enhances an existing system prompt (from `--system-prompt`) with notes and environment info for subagents. Notes include:
- Use absolute file paths between bash calls
- Share absolute file paths in final response
- Avoid emojis
- No colon before tool calls

**`getScratchpadInstructions(): string | null`**
Returns instructions for using the scratchpad directory if enabled. Directs Claude to use the session-specific scratchpad directory instead of `/tmp` for all temporary files.

**`prependBullets(items): string[]`**
Utility that prepends ` - ` to top-level items and `  - ` to nested arrays for system prompt formatting.

**`getAttributionHeader(fingerprint: string): string`** (in `constants/system.ts`):
Generates `x-anthropic-billing-header: cc_version=<VERSION>.<fingerprint>; cc_entrypoint=<entrypoint>;[cch=00000;][cc_workload=<workload>;]`

---

## 15. Spinner Verbs

**File:** `constants/spinnerVerbs.ts`

### `SPINNER_VERBS` — `string[]`

Array of 186 playful present-participle verbs shown in the spinner while Claude is working. Examples range from mundane (`'Computing'`, `'Processing'`, `'Working'`) to whimsical (`'Beboppin'`, `'Discombobulating'`, `'Flibbertigibbeting'`).

Complete list: `'Accomplishing'`, `'Actioning'`, `'Actualizing'`, `'Architecting'`, `'Baking'`, `'Beaming'`, `"Beboppin'"`, `'Befuddling'`, `'Billowing'`, `'Blanching'`, `'Bloviating'`, `'Boogieing'`, `'Boondoggling'`, `'Booping'`, `'Bootstrapping'`, `'Brewing'`, `'Bunning'`, `'Burrowing'`, `'Calculating'`, `'Canoodling'`, `'Caramelizing'`, `'Cascading'`, `'Catapulting'`, `'Cerebrating'`, `'Channeling'`, `'Channelling'`, `'Choreographing'`, `'Churning'`, `'Clauding'`, `'Coalescing'`, `'Cogitating'`, `'Combobulating'`, `'Composing'`, `'Computing'`, `'Concocting'`, `'Considering'`, `'Contemplating'`, `'Cooking'`, `'Crafting'`, `'Creating'`, `'Crunching'`, `'Crystallizing'`, `'Cultivating'`, `'Deciphering'`, `'Deliberating'`, `'Determining'`, `'Dilly-dallying'`, `'Discombobulating'`, `'Doing'`, `'Doodling'`, `'Drizzling'`, `'Ebbing'`, `'Effecting'`, `'Elucidating'`, `'Embellishing'`, `'Enchanting'`, `'Envisioning'`, `'Evaporating'`, `'Fermenting'`, `'Fiddle-faddling'`, `'Finagling'`, `'Flambéing'`, `'Flibbertigibbeting'`, `'Flowing'`, `'Flummoxing'`, `'Fluttering'`, `'Forging'`, `'Forming'`, `'Frolicking'`, `'Frosting'`, `'Gallivanting'`, `'Galloping'`, `'Garnishing'`, `'Generating'`, `'Gesticulating'`, `'Germinating'`, `'Gitifying'`, `'Grooving'`, `'Gusting'`, `'Harmonizing'`, `'Hashing'`, `'Hatching'`, `'Herding'`, `'Honking'`, `'Hullaballooing'`, `'Hyperspacing'`, `'Ideating'`, `'Imagining'`, `'Improvising'`, `'Incubating'`, `'Inferring'`, `'Infusing'`, `'Ionizing'`, `'Jitterbugging'`, `'Julienning'`, `'Kneading'`, `'Leavening'`, `'Levitating'`, `'Lollygagging'`, `'Manifesting'`, `'Marinating'`, `'Meandering'`, `'Metamorphosing'`, `'Misting'`, `'Moonwalking'`, `'Moseying'`, `'Mulling'`, `'Mustering'`, `'Musing'`, `'Nebulizing'`, `'Nesting'`, `'Newspapering'`, `'Noodling'`, `'Nucleating'`, `'Orbiting'`, `'Orchestrating'`, `'Osmosing'`, `'Perambulating'`, `'Percolating'`, `'Perusing'`, `'Philosophising'`, `'Photosynthesizing'`, `'Pollinating'`, `'Pondering'`, `'Pontificating'`, `'Pouncing'`, `'Precipitating'`, `'Prestidigitating'`, `'Processing'`, `'Proofing'`, `'Propagating'`, `'Puttering'`, `'Puzzling'`, `'Quantumizing'`, `'Razzle-dazzling'`, `'Razzmatazzing'`, `'Recombobulating'`, `'Reticulating'`, `'Roosting'`, `'Ruminating'`, `'Sautéing'`, `'Scampering'`, `'Schlepping'`, `'Scurrying'`, `'Seasoning'`, `'Shenaniganing'`, `'Shimmying'`, `'Simmering'`, `'Skedaddling'`, `'Sketching'`, `'Slithering'`, `'Smooshing'`, `'Sock-hopping'`, `'Spelunking'`, `'Spinning'`, `'Sprouting'`, `'Stewing'`, `'Sublimating'`, `'Swirling'`, `'Swooping'`, `'Symbioting'`, `'Synthesizing'`, `'Tempering'`, `'Thinking'`, `'Thundering'`, `'Tinkering'`, `'Tomfoolering'`, `'Topsy-turvying'`, `'Transfiguring'`, `'Transmuting'`, `'Twisting'`, `'Undulating'`, `'Unfurling'`, `'Unravelling'`, `'Vibing'`, `'Waddling'`, `'Wandering'`, `'Warping'`, `'Whatchamacalliting'`, `'Whirlpooling'`, `'Whirring'`, `'Whisking'`, `'Wibbling'`, `'Working'`, `'Wrangling'`, `'Zesting'`, `'Zigzagging'`

### `getSpinnerVerbs(): string[]`

Returns spinner verbs respecting user configuration from settings:
- No config → returns `SPINNER_VERBS`
- `mode: 'replace'` → returns `config.verbs` if non-empty, else `SPINNER_VERBS`
- `mode: 'append'` → returns `[...SPINNER_VERBS, ...config.verbs]`

---

## 16. System Constants

**File:** `constants/system.ts`
**Purpose:** Critical system constants extracted to break circular dependencies.

### CLI System Prompt Prefixes

Three possible prefix values (exported via `CLI_SYSPROMPT_PREFIXES` set):

1. `DEFAULT_PREFIX`: `"You are Claude Code, Anthropic's official CLI for Claude."` — Used for interactive sessions and Vertex AI.
2. `AGENT_SDK_CLAUDE_CODE_PRESET_PREFIX`: `"You are Claude Code, Anthropic's official CLI for Claude, running within the Claude Agent SDK."` — Non-interactive with `appendSystemPrompt`.
3. `AGENT_SDK_PREFIX`: `"You are a Claude agent, built on Anthropic's Claude Agent SDK."` — Non-interactive without `appendSystemPrompt`.

### Attribution Header

**`getAttributionHeader(fingerprint: string): string`**

Generates the `x-anthropic-billing-header` for API requests. Format:
```
x-anthropic-billing-header: cc_version=<VERSION>.<fingerprint>; cc_entrypoint=<entrypoint>;[cch=00000;][cc_workload=<workload>;]
```

- `cc_version`: `${MACRO.VERSION}.${fingerprint}` — identifies the build version.
- `cc_entrypoint`: from `CLAUDE_CODE_ENTRYPOINT` env var (default `'unknown'`).
- `cch=00000`: Native client attestation placeholder (only when `NATIVE_CLIENT_ATTESTATION` feature enabled). Bun's HTTP stack overwrites the zeros with a computed hash before transmission.
- `cc_workload`: Turn-scoped workload hint for API routing (absent = interactive default).

Enabled by default. Can be disabled via `CLAUDE_CODE_ATTRIBUTION_HEADER=false` or GrowthBook killswitch `tengu_attribution_header`.

---

## 17. System Prompt Sections

**File:** `constants/systemPromptSections.ts`
**Purpose:** Memoization infrastructure for system prompt sections.

### Types

```typescript
type ComputeFn = () => string | null | Promise<string | null>

type SystemPromptSection = {
  name: string
  compute: ComputeFn
  cacheBreak: boolean
}
```

### Functions

**`systemPromptSection(name, compute): SystemPromptSection`**
Creates a memoized system prompt section (`cacheBreak: false`). Computed once, cached until `/clear` or `/compact`.

**`DANGEROUS_uncachedSystemPromptSection(name, compute, _reason): SystemPromptSection`**
Creates a volatile system prompt section that recomputes every turn (`cacheBreak: true`). This WILL break the prompt cache when the value changes. The `_reason` parameter documents why cache-breaking is necessary.

**`resolveSystemPromptSections(sections): Promise<(string | null)[]>`**
Resolves all sections, returning computed strings. Non-cacheBreak sections use cached values when available.

**`clearSystemPromptSections(): void`**
Clears all system prompt section state and resets beta header latches. Called on `/clear` and `/compact`.

---

## 18. Tool Limits

**File:** `constants/toolLimits.ts`

### Size Constants

| Constant | Value | Description |
|---|---|---|
| `DEFAULT_MAX_RESULT_SIZE_CHARS` | `50_000` | Default maximum character count for tool results before persisting to disk. Individual tools may declare a lower limit; this is the system-wide cap. |
| `MAX_TOOL_RESULT_TOKENS` | `100_000` | Maximum tool result size in tokens. Approximately 400 KB of text (at ~4 bytes/token). |
| `BYTES_PER_TOKEN` | `4` | Conservative estimate for bytes-per-token conversion. Actual may vary. |
| `MAX_TOOL_RESULT_BYTES` | `MAX_TOOL_RESULT_TOKENS * BYTES_PER_TOKEN` = **400,000 bytes** (400 KB) | Maximum tool result size in bytes (derived from token limit). |
| `MAX_TOOL_RESULTS_PER_MESSAGE_CHARS` | `200_000` | Maximum aggregate character count for ALL tool result blocks within a single user message (one turn's parallel results). Prevents N parallel tools from each hitting 50K and collectively producing e.g. 10 × 40K = 400K in one turn. Overridable via GrowthBook flag `tengu_hawthorn_window`. |
| `TOOL_SUMMARY_MAX_LENGTH` | `50` | Maximum character length for tool summary strings in compact views. Used by `getToolUseSummary()` implementations. |

---

## 19. Tool Sets

**File:** `constants/tools.ts`
**Purpose:** Defines which tools are allowed or disallowed for different agent contexts.

### `ALL_AGENT_DISALLOWED_TOOLS` — `Set<string>`

Tools disallowed for all subagents (non-main-thread):
- `TaskOutputTool` — Output routing tool (main-thread only)
- `ExitPlanModeV2Tool` — Plan mode exit (main-thread abstraction)
- `EnterPlanModeTool` — Plan mode entry (main-thread abstraction)
- `AgentTool` — Nested agents (blocked for external users; allowed for `ant` users)
- `AskUserQuestionTool` — Requires human interaction
- `TaskStopTool` — Requires main-thread task state
- `WorkflowTool` — Prevents recursive workflow execution (when `WORKFLOW_SCRIPTS` feature enabled)

### `CUSTOM_AGENT_DISALLOWED_TOOLS` — `Set<string>`

Same as `ALL_AGENT_DISALLOWED_TOOLS` (identical membership).

### `ASYNC_AGENT_ALLOWED_TOOLS` — `Set<string>`

Tools available to async background agents:
- `FileReadTool`, `WebSearchTool`, `TodoWriteTool`, `GrepTool`, `WebFetchTool`, `GlobTool`
- All shell tools (`SHELL_TOOL_NAMES`)
- `FileEditTool`, `FileWriteTool`, `NotebookEditTool`
- `SkillTool`, `SyntheticOutputTool`, `ToolSearchTool`
- `EnterWorktreeTool`, `ExitWorktreeTool`

**Explicitly blocked for async agents:**
- `AgentTool` — Prevents recursion
- `TaskOutputTool` — Prevents recursion
- `ExitPlanModeTool` — Plan mode is a main-thread abstraction
- `TaskStopTool` — Requires access to main-thread task state
- `TungstenTool` — Uses singleton virtual terminal conflicting between agents

### `IN_PROCESS_TEAMMATE_ALLOWED_TOOLS` — `Set<string>`

Additional tools only available for in-process teammates (not general async agents). Injected by `inProcessRunner.ts`:
- `TaskCreateTool`, `TaskGetTool`, `TaskListTool`, `TaskUpdateTool`
- `SendMessageTool`
- `CronCreateTool`, `CronDeleteTool`, `CronListTool` (when `AGENT_TRIGGERS` feature enabled)

### `COORDINATOR_MODE_ALLOWED_TOOLS` — `Set<string>`

Tools available in coordinator mode (coordinator orchestrates workers, doesn't do direct work):
- `AgentTool` — Spawns workers
- `TaskStopTool` — Stops tasks
- `SendMessageTool` — Inter-agent communication
- `SyntheticOutputTool` — Output synthesis

---

## 20. Turn Completion Verbs

**File:** `constants/turnCompletionVerbs.ts`

### `TURN_COMPLETION_VERBS` — `string[]`

Past-tense verbs for turn completion messages. Work with "for [duration]" (e.g., "Worked for 5s"):

`'Baked'`, `'Brewed'`, `'Churned'`, `'Cogitated'`, `'Cooked'`, `'Crunched'`, `'Sautéed'`, `'Worked'`

---

## 21. XML Tags

**File:** `constants/xml.ts`

### Command/Skill Tags

| Constant | Value | Description |
|---|---|---|
| `COMMAND_NAME_TAG` | `'command-name'` | Marks skill/command name in messages. |
| `COMMAND_MESSAGE_TAG` | `'command-message'` | Marks skill/command message content. |
| `COMMAND_ARGS_TAG` | `'command-args'` | Marks skill/command arguments. |

### Terminal / Bash Tags

| Constant | Value | Description |
|---|---|---|
| `BASH_INPUT_TAG` | `'bash-input'` | Wraps bash command input content in user messages. |
| `BASH_STDOUT_TAG` | `'bash-stdout'` | Wraps bash stdout output. |
| `BASH_STDERR_TAG` | `'bash-stderr'` | Wraps bash stderr output. |
| `LOCAL_COMMAND_STDOUT_TAG` | `'local-command-stdout'` | Wraps local command stdout. |
| `LOCAL_COMMAND_STDERR_TAG` | `'local-command-stderr'` | Wraps local command stderr. |
| `LOCAL_COMMAND_CAVEAT_TAG` | `'local-command-caveat'` | Wraps local command caveats. |

### `TERMINAL_OUTPUT_TAGS` — `readonly string[]`

All terminal-related tags (used to detect if a message is terminal output, not a user prompt):
`[BASH_INPUT_TAG, BASH_STDOUT_TAG, BASH_STDERR_TAG, LOCAL_COMMAND_STDOUT_TAG, LOCAL_COMMAND_STDERR_TAG, LOCAL_COMMAND_CAVEAT_TAG]`

### Proactive/Autonomous Mode

| Constant | Value | Description |
|---|---|---|
| `TICK_TAG` | `'tick'` | Wraps proactive mode heartbeat prompts. Claude responds to `<tick>` as "you're awake, what now?" |

### Task Notification Tags

| Constant | Value | Description |
|---|---|---|
| `TASK_NOTIFICATION_TAG` | `'task-notification'` | Wraps background task completion notifications. |
| `TASK_ID_TAG` | `'task-id'` | Task identifier within notification. |
| `TOOL_USE_ID_TAG` | `'tool-use-id'` | Tool use ID within notification. |
| `TASK_TYPE_TAG` | `'task-type'` | Task type within notification. |
| `OUTPUT_FILE_TAG` | `'output-file'` | Output file path within notification. |
| `STATUS_TAG` | `'status'` | Status within notification. |
| `SUMMARY_TAG` | `'summary'` | Summary within notification. |
| `REASON_TAG` | `'reason'` | Reason within notification. |
| `WORKTREE_TAG` | `'worktree'` | Worktree information. |
| `WORKTREE_PATH_TAG` | `'worktreePath'` | Worktree path. |
| `WORKTREE_BRANCH_TAG` | `'worktreeBranch'` | Worktree branch. |

### Feature-Specific Tags

| Constant | Value | Description |
|---|---|---|
| `ULTRAPLAN_TAG` | `'ultraplan'` | Ultraplan mode (remote parallel planning sessions). |
| `REMOTE_REVIEW_TAG` | `'remote-review'` | Remote `/review` results from teleported review sessions. |
| `REMOTE_REVIEW_PROGRESS_TAG` | `'remote-review-progress'` | Heartbeat progress from `run_hunt.sh` orchestrator (~10s interval). |
| `TEAMMATE_MESSAGE_TAG` | `'teammate-message'` | Swarm inter-agent communication. |
| `CHANNEL_MESSAGE_TAG` | `'channel-message'` | External channel messages. |
| `CHANNEL_TAG` | `'channel'` | Channel identifier within channel message. |
| `CROSS_SESSION_MESSAGE_TAG` | `'cross-session-message'` | Cross-session UDS messages from another Claude session's inbox. |
| `FORK_BOILERPLATE_TAG` | `'fork-boilerplate'` | Wraps rules/format boilerplate in a fork child's first message. Allows transcript renderer to collapse boilerplate and show only the directive. |
| `FORK_DIRECTIVE_PREFIX` | `'Your directive: '` | Prefix before directive text in fork messages. Stripped by renderer. |

### Slash Command Argument Patterns

**`COMMON_HELP_ARGS`** — `string[]`
Common arguments for help requests: `['help', '-h', '--help']`

**`COMMON_INFO_ARGS`** — `string[]`
Common arguments for current-state/info requests:
`['list', 'show', 'display', 'current', 'view', 'get', 'check', 'describe', 'print', 'version', 'about', 'status', '?']`

---

## 22. Tool Type System

**File:** `Tool.ts`

### Core Types

#### `ToolInputJSONSchema`
```typescript
type ToolInputJSONSchema = {
  [x: string]: unknown
  type: 'object'
  properties?: { [x: string]: unknown }
}
```

#### `QueryChainTracking`
```typescript
type QueryChainTracking = {
  chainId: string
  depth: number
}
```

#### `ValidationResult`
```typescript
type ValidationResult =
  | { result: true }
  | { result: false; message: string; errorCode: number }
```

#### `SetToolJSXFn`
Callback for tools that render custom JSX UI:
```typescript
type SetToolJSXFn = (args: {
  jsx: React.ReactNode | null
  shouldHidePromptInput: boolean
  shouldContinueAnimation?: true
  showSpinner?: boolean
  isLocalJSXCommand?: boolean
  isImmediate?: boolean
  clearLocalJSX?: boolean
} | null) => void
```

#### `ToolPermissionContext`
```typescript
type ToolPermissionContext = DeepImmutable<{
  mode: PermissionMode
  additionalWorkingDirectories: Map<string, AdditionalWorkingDirectory>
  alwaysAllowRules: ToolPermissionRulesBySource
  alwaysDenyRules: ToolPermissionRulesBySource
  alwaysAskRules: ToolPermissionRulesBySource
  isBypassPermissionsModeAvailable: boolean
  isAutoModeAvailable?: boolean
  strippedDangerousRules?: ToolPermissionRulesBySource
  shouldAvoidPermissionPrompts?: boolean
  awaitAutomatedChecksBeforeDialog?: boolean
  prePlanMode?: PermissionMode
}>
```

**`getEmptyToolPermissionContext(): ToolPermissionContext`** — Returns a minimal context with `mode: 'default'` and all rules as empty objects.

#### `CompactProgressEvent`
```typescript
type CompactProgressEvent =
  | { type: 'hooks_start'; hookType: 'pre_compact' | 'post_compact' | 'session_start' }
  | { type: 'compact_start' }
  | { type: 'compact_end' }
```

#### `ToolUseContext`
The primary context object passed to every tool call. Contains:

- `options`: `{ commands, debug, mainLoopModel, tools, verbose, thinkingConfig, mcpClients, mcpResources, isNonInteractiveSession, agentDefinitions, maxBudgetUsd?, customSystemPrompt?, appendSystemPrompt?, querySource?, refreshTools? }`
- `abortController: AbortController`
- `readFileState: FileStateCache`
- `getAppState(): AppState`
- `setAppState(f): void`
- `setAppStateForTasks?: (f) => void` — Always-shared setAppState for session-scoped infrastructure
- `handleElicitation?: (serverName, params, signal) => Promise<ElicitResult>`
- `setToolJSX?: SetToolJSXFn`
- `addNotification?: (notif) => void`
- `appendSystemMessage?: (msg) => void`
- `sendOSNotification?: (opts) => void`
- `nestedMemoryAttachmentTriggers?: Set<string>`
- `loadedNestedMemoryPaths?: Set<string>`
- `dynamicSkillDirTriggers?: Set<string>`
- `discoveredSkillNames?: Set<string>`
- `userModified?: boolean`
- `setInProgressToolUseIDs: (f) => void`
- `setHasInterruptibleToolInProgress?: (v) => void`
- `setResponseLength: (f) => void`
- `pushApiMetricsEntry?: (ttftMs) => void`
- `setStreamMode?: (mode) => void`
- `onCompactProgress?: (event) => void`
- `setSDKStatus?: (status) => void`
- `openMessageSelector?: () => void`
- `updateFileHistoryState: (updater) => void`
- `updateAttributionState: (updater) => void`
- `setConversationId?: (id) => void`
- `agentId?: AgentId`
- `agentType?: string`
- `requireCanUseTool?: boolean`
- `messages: Message[]`
- `fileReadingLimits?: { maxTokens?, maxSizeBytes? }`
- `globLimits?: { maxResults? }`
- `toolDecisions?: Map<string, { source, decision, timestamp }>`
- `queryTracking?: QueryChainTracking`
- `requestPrompt?: (sourceName, toolInputSummary?) => (request) => Promise<PromptResponse>`
- `toolUseId?: string`
- `criticalSystemReminder_EXPERIMENTAL?: string`
- `preserveToolUseResults?: boolean`
- `localDenialTracking?: DenialTrackingState`
- `contentReplacementState?: ContentReplacementState`
- `renderedSystemPrompt?: SystemPrompt`

#### `ToolResult<T>`
```typescript
type ToolResult<T> = {
  data: T
  newMessages?: (UserMessage | AssistantMessage | AttachmentMessage | SystemMessage)[]
  contextModifier?: (context: ToolUseContext) => ToolUseContext
  mcpMeta?: { _meta?: Record<string, unknown>; structuredContent?: Record<string, unknown> }
}
```

#### `Tool<Input, Output, P>` — Full Interface

The main tool interface. Key methods and properties:

| Property/Method | Required | Description |
|---|---|---|
| `name: string` | Yes | Primary tool name. |
| `aliases?: string[]` | No | Backwards-compatibility aliases. |
| `searchHint?: string` | No | One-line capability phrase for ToolSearch keyword matching (3–10 words). |
| `inputSchema: Input` | Yes | Zod schema for input validation. |
| `inputJSONSchema?: ToolInputJSONSchema` | No | Alternative JSON Schema for MCP tools. |
| `outputSchema?: ZodType` | No | Zod schema for output validation. |
| `maxResultSizeChars: number` | Yes | Max chars before persisting result to disk. Use `Infinity` for tools that must never be persisted (e.g., `Read`). |
| `strict?: boolean` | No | When true, enables strict mode for API parameter adherence (requires `tengu_tool_pear`). |
| `shouldDefer?: boolean` | No (readonly) | When true, tool is deferred and requires `ToolSearch` before calling. |
| `alwaysLoad?: boolean` | No (readonly) | When true, never deferred — full schema always in initial prompt. |
| `mcpInfo?: { serverName, toolName }` | No | MCP server and tool names (on all MCP tools). |
| `isMcp?: boolean` | No | True for MCP tools. |
| `isLsp?: boolean` | No | True for LSP tools. |
| `call(args, context, canUseTool, parentMessage, onProgress?)` | Yes | Executes the tool. Returns `Promise<ToolResult<Output>>`. |
| `description(input, options)` | Yes | Returns tool description for permission dialogs. |
| `prompt(options)` | Yes | Returns tool documentation for system prompt injection. |
| `checkPermissions(input, context)` | Yes | Returns `Promise<PermissionResult>`. |
| `validateInput?(input, context)` | No | Returns `Promise<ValidationResult>`. Called before `checkPermissions`. |
| `isEnabled()` | Yes (default: `true`) | Whether the tool is currently available. |
| `isConcurrencySafe(input)` | Yes (default: `false`) | Whether the tool can run in parallel safely. |
| `isReadOnly(input)` | Yes (default: `false`) | Whether the tool only reads (never writes). |
| `isDestructive?(input)` | No (default: `false`) | Whether the tool performs irreversible operations. |
| `interruptBehavior?(): 'cancel' \| 'block'` | No (default: `'block'`) | What happens when user submits during this tool's execution. |
| `isSearchOrReadCommand?(input)` | No | Returns `{ isSearch, isRead, isList? }` for UI collapsing. |
| `isOpenWorld?(input)` | No | Whether the tool can affect external state. |
| `requiresUserInteraction?()` | No | Whether the tool needs interactive UI. |
| `inputsEquivalent?(a, b)` | No | Compares two inputs for deduplication. |
| `isTransparentWrapper?()` | No | If true, delegates all rendering to progress handler. |
| `backfillObservableInput?(input)` | No | Mutates input copy to add legacy/derived fields before observers see it. |
| `getPath?(input)` | No | Returns the file path this tool operates on. |
| `preparePermissionMatcher?(input)` | No | Prepares hook `if` condition matcher for permission rules. |
| `userFacingName(input)` | Yes (default: `name`) | Human-readable name for UI display. |
| `userFacingNameBackgroundColor?(input)` | No | Theme key for colored name display. |
| `getToolUseSummary?(input)` | No | Short summary for compact views (max `TOOL_SUMMARY_MAX_LENGTH` chars). |
| `getActivityDescription?(input)` | No | Present-tense activity description for spinner (e.g., "Reading src/foo.ts"). |
| `toAutoClassifierInput(input)` | Yes (default: `''`) | Compact repr for auto-mode security classifier (return `''` to skip). |
| `mapToolResultToToolResultBlockParam(content, toolUseID)` | Yes | Serializes tool result for API. |
| `renderToolResultMessage?(content, progressMessages, options)` | No | React rendering of result. Omit for results surfaced elsewhere. |
| `extractSearchText?(out)` | No | Flattened text for transcript search indexing. |
| `renderToolUseMessage(input, options)` | Yes | React rendering of tool invocation (called immediately, input may be partial). |
| `isResultTruncated?(output)` | No | True when non-verbose rendering is truncated (gates click-to-expand). |
| `renderToolUseTag?(input)` | No | Optional tag rendered after tool use message (timeout, model, resume ID). |
| `renderToolUseProgressMessage?(progressMessages, options)` | No | React rendering of progress while running. |
| `renderToolUseQueuedMessage?()` | No | React rendering of queued state. |
| `renderToolUseRejectedMessage?(input, options)` | No | Custom rejection UI. Falls back to `<FallbackToolUseRejectedMessage />`. |
| `renderToolUseErrorMessage?(result, options)` | No | Custom error UI. Falls back to `<FallbackToolUseErrorMessage />`. |
| `renderGroupedToolUse?(toolUses, options)` | No | Renders multiple parallel instances as a group (non-verbose only). |

#### `Tools` — `readonly Tool[]`

Type alias for a collection of tools. Used throughout to pass around tool sets.

#### `ToolDef` — Partial Tool Definition

Same shape as `Tool` but with defaultable methods optional. Accepted by `buildTool()`.

**Defaultable keys:** `isEnabled`, `isConcurrencySafe`, `isReadOnly`, `isDestructive`, `checkPermissions`, `toAutoClassifierInput`, `userFacingName`

**Default values:**
- `isEnabled` → `() => true`
- `isConcurrencySafe` → `() => false`
- `isReadOnly` → `() => false`
- `isDestructive` → `() => false`
- `checkPermissions` → `Promise.resolve({ behavior: 'allow', updatedInput: input })`
- `toAutoClassifierInput` → `() => ''`
- `userFacingName` → `() => def.name`

#### `buildTool<D>(def: D): BuiltTool<D>`

Factory function that builds a complete `Tool` from a `ToolDef`, filling in defaults. All tool exports should go through this function. The `userFacingName` default is `() => def.name`.

---

## 23. Task Type System

**File:** `Task.ts`

### Types

#### `TaskType` — Union Type
```typescript
type TaskType =
  | 'local_bash'        // Shell command in background
  | 'local_agent'       // Subagent running locally
  | 'remote_agent'      // Agent running on remote server
  | 'in_process_teammate' // In-process swarm teammate
  | 'local_workflow'    // Local workflow execution
  | 'monitor_mcp'       // MCP subscription monitor
  | 'dream'             // Dream/auto-dream background agent
```

#### `TaskStatus` — Union Type
```typescript
type TaskStatus =
  | 'pending'    // Not yet started
  | 'running'    // Currently executing
  | 'completed'  // Finished successfully
  | 'failed'     // Finished with error
  | 'killed'     // Manually terminated
```

**`isTerminalTaskStatus(status): boolean`** — Returns `true` for `'completed'`, `'failed'`, or `'killed'`. Guards against injecting messages into dead teammates.

#### `TaskHandle`
```typescript
type TaskHandle = {
  taskId: string
  cleanup?: () => void
}
```

#### `SetAppState` — `(f: (prev: AppState) => AppState) => void`

#### `TaskContext`
```typescript
type TaskContext = {
  abortController: AbortController
  getAppState: () => AppState
  setAppState: SetAppState
}
```

#### `TaskStateBase`
```typescript
type TaskStateBase = {
  id: string
  type: TaskType
  status: TaskStatus
  description: string
  toolUseId?: string
  startTime: number       // Unix timestamp ms
  endTime?: number        // Unix timestamp ms
  totalPausedMs?: number
  outputFile: string      // Path to disk output file
  outputOffset: number    // Bytes already read from outputFile
  notified: boolean       // Whether user has been notified of completion
}
```

#### `LocalShellSpawnInput`
```typescript
type LocalShellSpawnInput = {
  command: string
  description: string
  timeout?: number
  toolUseId?: string
  agentId?: AgentId
  kind?: 'bash' | 'monitor'  // UI display variant
}
```

#### `Task`
```typescript
type Task = {
  name: string
  type: TaskType
  kill(taskId: string, setAppState: SetAppState): Promise<void>
}
```

### Task ID Generation

**Task ID Prefixes** (by `TaskType`):

| Type | Prefix |
|---|---|
| `local_bash` | `'b'` |
| `local_agent` | `'a'` |
| `remote_agent` | `'r'` |
| `in_process_teammate` | `'t'` |
| `local_workflow` | `'w'` |
| `monitor_mcp` | `'m'` |
| `dream` | `'d'` |
| (unknown) | `'x'` |

**`TASK_ID_ALPHABET`** = `'0123456789abcdefghijklmnopqrstuvwxyz'` (36 chars)

**`generateTaskId(type): string`** — Generates a task ID as `{prefix}` + 8 random base-36 characters. Uses `randomBytes(8)` for cryptographic randomness. Total space: 36^8 ≈ 2.8 trillion combinations.

**`createTaskStateBase(id, type, description, toolUseId?): TaskStateBase`** — Creates a `TaskStateBase` with `status: 'pending'`, `startTime: Date.now()`, `outputFile: getTaskOutputPath(id)`, `outputOffset: 0`, `notified: false`.

---

## 24. Types Directory

### 24.1 IDs (`types/ids.ts`)

**Branded types** preventing ID type confusion at compile time:

```typescript
type SessionId = string & { readonly __brand: 'SessionId' }
type AgentId = string & { readonly __brand: 'AgentId' }
```

**Functions:**
- `asSessionId(id: string): SessionId` — Cast string to SessionId (use sparingly).
- `asAgentId(id: string): AgentId` — Cast string to AgentId (use sparingly).
- `toAgentId(s: string): AgentId | null` — Validates and brands. Matches pattern `^a(?:.+-)?[0-9a-f]{16}$` (letter `a` + optional `<label>-` + 16 hex chars).

### 24.2 Permissions (`types/permissions.ts`)

#### Permission Modes

```typescript
// User-addressable external modes
const EXTERNAL_PERMISSION_MODES = ['acceptEdits', 'bypassPermissions', 'default', 'dontAsk', 'plan']
type ExternalPermissionMode = 'acceptEdits' | 'bypassPermissions' | 'default' | 'dontAsk' | 'plan'

// Internal modes (includes auto when TRANSCRIPT_CLASSIFIER enabled)
type InternalPermissionMode = ExternalPermissionMode | 'auto' | 'bubble'
type PermissionMode = InternalPermissionMode

// Runtime validation set
const INTERNAL_PERMISSION_MODES = [...EXTERNAL_PERMISSION_MODES, ...('auto' if TRANSCRIPT_CLASSIFIER)]
const PERMISSION_MODES = INTERNAL_PERMISSION_MODES
```

**Mode descriptions:**
- `'default'` — Normal interactive mode; prompts user for each new operation type.
- `'acceptEdits'` — Auto-approves file edits without prompting.
- `'dontAsk'` — Auto-approves all tool calls.
- `'bypassPermissions'` — Bypasses all permission checks (dangerous).
- `'plan'` — Plan mode; Claude can plan but not execute.
- `'auto'` — Automatic mode with classifier-based approval (TRANSCRIPT_CLASSIFIER feature).
- `'bubble'` — Bubbles permission requests to parent context.

#### Permission Behavior
```typescript
type PermissionBehavior = 'allow' | 'deny' | 'ask'
```

#### `PermissionRuleSource`
```typescript
type PermissionRuleSource =
  | 'userSettings' | 'projectSettings' | 'localSettings' | 'flagSettings'
  | 'policySettings' | 'cliArg' | 'command' | 'session'
```

#### `PermissionRuleValue`
```typescript
type PermissionRuleValue = { toolName: string; ruleContent?: string }
```

#### `PermissionRule`
```typescript
type PermissionRule = {
  source: PermissionRuleSource
  ruleBehavior: PermissionBehavior
  ruleValue: PermissionRuleValue
}
```

#### `PermissionUpdateDestination`
```typescript
type PermissionUpdateDestination =
  | 'userSettings' | 'projectSettings' | 'localSettings' | 'session' | 'cliArg'
```

#### `PermissionUpdate` — Discriminated Union
```typescript
type PermissionUpdate =
  | { type: 'addRules'; destination; rules: PermissionRuleValue[]; behavior }
  | { type: 'replaceRules'; destination; rules: PermissionRuleValue[]; behavior }
  | { type: 'removeRules'; destination; rules: PermissionRuleValue[]; behavior }
  | { type: 'setMode'; destination; mode: ExternalPermissionMode }
  | { type: 'addDirectories'; destination; directories: string[] }
  | { type: 'removeDirectories'; destination; directories: string[] }
```

#### `AdditionalWorkingDirectory`
```typescript
type AdditionalWorkingDirectory = { path: string; source: WorkingDirectorySource }
```

#### Permission Decisions

```typescript
type PermissionAllowDecision<Input> = {
  behavior: 'allow'
  updatedInput?: Input
  userModified?: boolean
  decisionReason?: PermissionDecisionReason
  toolUseID?: string
  acceptFeedback?: string
  contentBlocks?: ContentBlockParam[]
}

type PermissionAskDecision<Input> = {
  behavior: 'ask'
  message: string
  updatedInput?: Input
  decisionReason?: PermissionDecisionReason
  suggestions?: PermissionUpdate[]
  blockedPath?: string
  metadata?: PermissionMetadata
  isBashSecurityCheckForMisparsing?: boolean
  pendingClassifierCheck?: PendingClassifierCheck
  contentBlocks?: ContentBlockParam[]
}

type PermissionDenyDecision = {
  behavior: 'deny'
  message: string
  decisionReason: PermissionDecisionReason
  toolUseID?: string
}

type PermissionResult<Input> =
  | PermissionDecision<Input>
  | { behavior: 'passthrough'; message; decisionReason?; suggestions?; blockedPath?; pendingClassifierCheck? }
```

#### `PermissionDecisionReason` — Discriminated Union

```typescript
type PermissionDecisionReason =
  | { type: 'rule'; rule: PermissionRule }
  | { type: 'mode'; mode: PermissionMode }
  | { type: 'subcommandResults'; reasons: Map<string, PermissionResult> }
  | { type: 'permissionPromptTool'; permissionPromptToolName: string; toolResult: unknown }
  | { type: 'hook'; hookName: string; hookSource?: string; reason?: string }
  | { type: 'asyncAgent'; reason: string }
  | { type: 'sandboxOverride'; reason: 'excludedCommand' | 'dangerouslyDisableSandbox' }
  | { type: 'classifier'; classifier: string; reason: string }
  | { type: 'workingDir'; reason: string }
  | { type: 'safetyCheck'; reason: string; classifierApprovable: boolean }
  | { type: 'other'; reason: string }
```

#### Classifier Types

```typescript
type ClassifierResult = {
  matches: boolean
  matchedDescription?: string
  confidence: 'high' | 'medium' | 'low'
  reason: string
}

type ClassifierBehavior = 'deny' | 'ask' | 'allow'

type ClassifierUsage = {
  inputTokens: number; outputTokens: number
  cacheReadInputTokens: number; cacheCreationInputTokens: number
}

type YoloClassifierResult = {
  thinking?: string
  shouldBlock: boolean
  reason: string
  unavailable?: boolean
  transcriptTooLong?: boolean
  model: string
  usage?: ClassifierUsage
  durationMs?: number
  promptLengths?: { systemPrompt: number; toolCalls: number; userPrompts: number }
  errorDumpPath?: string
  stage?: 'fast' | 'thinking'
  stage1Usage?: ClassifierUsage
  stage1DurationMs?: number
  stage1RequestId?: string
  stage1MsgId?: string
  stage2Usage?: ClassifierUsage
  stage2DurationMs?: number
  stage2RequestId?: string
  stage2MsgId?: string
}
```

#### Other Permission Types

```typescript
type RiskLevel = 'LOW' | 'MEDIUM' | 'HIGH'

type PermissionExplanation = {
  riskLevel: RiskLevel
  explanation: string
  reasoning: string
  risk: string
}

type ToolPermissionRulesBySource = {
  [T in PermissionRuleSource]?: string[]
}
```

### 24.3 Commands (`types/command.ts`)

#### `LocalCommandResult`
```typescript
type LocalCommandResult =
  | { type: 'text'; value: string }
  | { type: 'compact'; compactionResult: CompactionResult; displayText?: string }
  | { type: 'skip' }
```

#### `PromptCommand`
```typescript
type PromptCommand = {
  type: 'prompt'
  progressMessage: string
  contentLength: number
  argNames?: string[]
  allowedTools?: string[]
  model?: string
  source: SettingSource | 'builtin' | 'mcp' | 'plugin' | 'bundled'
  pluginInfo?: { pluginManifest: PluginManifest; repository: string }
  disableNonInteractive?: boolean
  hooks?: HooksSettings
  skillRoot?: string
  context?: 'inline' | 'fork'  // 'inline' = expands in current conversation, 'fork' = subagent
  agent?: string
  effort?: EffortValue
  paths?: string[]              // Glob patterns limiting skill visibility by touched files
  getPromptForCommand(args, context): Promise<ContentBlockParam[]>
}
```

#### `CommandAvailability`
```typescript
type CommandAvailability = 'claude-ai' | 'console'
```
- `'claude-ai'` — Claude.ai OAuth subscriber (Pro/Max/Team/Enterprise)
- `'console'` — Console API key user (direct api.anthropic.com)

#### `CommandBase`
```typescript
type CommandBase = {
  availability?: CommandAvailability[]
  description: string
  hasUserSpecifiedDescription?: boolean
  isEnabled?: () => boolean         // Default: true
  isHidden?: boolean                // Default: false
  name: string
  aliases?: string[]
  isMcp?: boolean
  argumentHint?: string
  whenToUse?: string
  version?: string
  disableModelInvocation?: boolean
  userInvocable?: boolean
  loadedFrom?: 'commands_DEPRECATED' | 'skills' | 'plugin' | 'managed' | 'bundled' | 'mcp'
  kind?: 'workflow'
  immediate?: boolean               // Executes immediately without queuing
  isSensitive?: boolean             // Args redacted from conversation history
  userFacingName?: () => string     // Default: name
}

type Command = CommandBase & (PromptCommand | LocalCommand | LocalJSXCommand)
```

#### `ResumeEntrypoint`
```typescript
type ResumeEntrypoint =
  | 'cli_flag'
  | 'slash_command_picker'
  | 'slash_command_session_id'
  | 'slash_command_title'
  | 'fork'
```

#### `QueuePriority`
```typescript
type QueuePriority = 'now' | 'next' | 'later'
```
- `'now'` — Interrupt immediately, abort in-flight tool calls.
- `'next'` — Drain mid-turn; let current tool finish, then send.
- `'later'` — Drain end-of-turn; wait for current turn to finish.

### 24.4 Hooks (`types/hooks.ts`)

#### `PromptRequest` / `PromptResponse`
```typescript
type PromptRequest = {
  prompt: string           // request id (discriminator)
  message: string
  options: Array<{ key: string; label: string; description?: string }>
}

type PromptResponse = {
  prompt_response: string  // request id
  selected: string
}
```

#### `HookCallback`
```typescript
type HookCallback = {
  type: 'callback'
  callback: (input, toolUseID, abort, hookIndex?, context?) => Promise<HookJSONOutput>
  timeout?: number
  internal?: boolean       // Excludes from tengu_run_hook metrics
}
```

#### `HookProgress`
```typescript
type HookProgress = {
  type: 'hook_progress'
  hookEvent: HookEvent
  hookName: string
  command: string
  promptText?: string
  statusMessage?: string
}
```

#### `HookResult`
```typescript
type HookResult = {
  message?: Message
  systemMessage?: Message
  blockingError?: HookBlockingError
  outcome: 'success' | 'blocking' | 'non_blocking_error' | 'cancelled'
  preventContinuation?: boolean
  stopReason?: string
  permissionBehavior?: 'ask' | 'deny' | 'allow' | 'passthrough'
  hookPermissionDecisionReason?: string
  additionalContext?: string
  initialUserMessage?: string
  updatedInput?: Record<string, unknown>
  updatedMCPToolOutput?: unknown
  permissionRequestResult?: PermissionRequestResult
  retry?: boolean
}
```

#### `PermissionRequestResult`
```typescript
type PermissionRequestResult =
  | { behavior: 'allow'; updatedInput?: Record<string, unknown>; updatedPermissions?: PermissionUpdate[] }
  | { behavior: 'deny'; message?: string; interrupt?: boolean }
```

#### `Sync Hook Response Schema Fields`
When a hook outputs JSON, it can include:
- `continue?: boolean` — Whether Claude should continue after hook (default: `true`)
- `suppressOutput?: boolean` — Hide stdout from transcript (default: `false`)
- `stopReason?: string` — Message shown when `continue` is `false`
- `decision?: 'approve' | 'block'`
- `reason?: string`
- `systemMessage?: string`
- `hookSpecificOutput?` — Event-specific data (varies by `hookEventName`):
  - `PreToolUse`: `permissionDecision`, `permissionDecisionReason`, `updatedInput`, `additionalContext`
  - `UserPromptSubmit`: `additionalContext`
  - `SessionStart`: `additionalContext`, `initialUserMessage`, `watchPaths`
  - `Setup`: `additionalContext`
  - `SubagentStart`: `additionalContext`
  - `PostToolUse`: `additionalContext`, `updatedMCPToolOutput`
  - `PostToolUseFailure`: `additionalContext`
  - `PermissionDenied`: `retry`
  - `Notification`: `additionalContext`
  - `PermissionRequest`: `decision` (allow with `updatedInput`/`updatedPermissions`, or deny with `message`/`interrupt`)
  - `Elicitation`/`ElicitationResult`: `action` (`'accept'|'decline'|'cancel'`), `content`
  - `CwdChanged`/`FileChanged`: `watchPaths`
  - `WorktreeCreate`: `worktreePath`

### 24.5 Plugins (`types/plugin.ts`)

#### `BuiltinPluginDefinition`
```typescript
type BuiltinPluginDefinition = {
  name: string
  description: string
  version?: string
  skills?: BundledSkillDefinition[]
  hooks?: HooksSettings
  mcpServers?: Record<string, McpServerConfig>
  isAvailable?: () => boolean
  defaultEnabled?: boolean   // Default: true
}
```

#### `LoadedPlugin`
```typescript
type LoadedPlugin = {
  name: string
  manifest: PluginManifest
  path: string
  source: string
  repository: string
  enabled?: boolean
  isBuiltin?: boolean
  sha?: string
  commandsPath?: string
  commandsPaths?: string[]
  commandsMetadata?: Record<string, CommandMetadata>
  agentsPath?: string
  agentsPaths?: string[]
  skillsPath?: string
  skillsPaths?: string[]
  outputStylesPath?: string
  outputStylesPaths?: string[]
  hooksConfig?: HooksSettings
  mcpServers?: Record<string, McpServerConfig>
  lspServers?: Record<string, LspServerConfig>
  settings?: Record<string, unknown>
}
```

#### `PluginComponent`
```typescript
type PluginComponent = 'commands' | 'agents' | 'skills' | 'hooks' | 'output-styles'
```

#### `PluginError` — Discriminated Union (17 variants)

| Type | Key Fields |
|---|---|
| `'path-not-found'` | `source`, `plugin?`, `path`, `component` |
| `'git-auth-failed'` | `source`, `plugin?`, `gitUrl`, `authType: 'ssh' \| 'https'` |
| `'git-timeout'` | `source`, `plugin?`, `gitUrl`, `operation: 'clone' \| 'pull'` |
| `'network-error'` | `source`, `plugin?`, `url`, `details?` |
| `'manifest-parse-error'` | `source`, `plugin?`, `manifestPath`, `parseError` |
| `'manifest-validation-error'` | `source`, `plugin?`, `manifestPath`, `validationErrors` |
| `'plugin-not-found'` | `source`, `pluginId`, `marketplace` |
| `'marketplace-not-found'` | `source`, `marketplace`, `availableMarketplaces` |
| `'marketplace-load-failed'` | `source`, `marketplace`, `reason` |
| `'mcp-config-invalid'` | `source`, `plugin`, `serverName`, `validationError` |
| `'mcp-server-suppressed-duplicate'` | `source`, `plugin`, `serverName`, `duplicateOf` |
| `'hook-load-failed'` | `source`, `plugin`, `hookPath`, `reason` |
| `'component-load-failed'` | `source`, `plugin`, `component`, `path`, `reason` |
| `'mcpb-download-failed'` | `source`, `plugin`, `url`, `reason` |
| `'mcpb-extract-failed'` | `source`, `plugin`, `mcpbPath`, `reason` |
| `'mcpb-invalid-manifest'` | `source`, `plugin`, `mcpbPath`, `validationError` |
| `'lsp-config-invalid'` | `source`, `plugin`, `serverName`, `validationError` |
| `'lsp-server-start-failed'` | `source`, `plugin`, `serverName`, `reason` |
| `'lsp-server-crashed'` | `source`, `plugin`, `serverName`, `exitCode`, `signal?` |
| `'lsp-request-timeout'` | `source`, `plugin`, `serverName`, `method`, `timeoutMs` |
| `'lsp-request-failed'` | `source`, `plugin`, `serverName`, `method`, `error` |
| `'marketplace-blocked-by-policy'` | `source`, `plugin?`, `marketplace`, `blockedByBlocklist?`, `allowedSources` |
| `'dependency-unsatisfied'` | `source`, `plugin`, `dependency`, `reason: 'not-enabled' \| 'not-found'` |
| `'plugin-cache-miss'` | `source`, `plugin`, `installPath` |
| `'generic-error'` | `source`, `plugin?`, `error` |

#### `PluginLoadResult`
```typescript
type PluginLoadResult = {
  enabled: LoadedPlugin[]
  disabled: LoadedPlugin[]
  errors: PluginError[]
}
```

### 24.6 Logs (`types/logs.ts`)

#### `SerializedMessage`
```typescript
type SerializedMessage = Message & {
  cwd: string
  userType: string
  entrypoint?: string   // CLAUDE_CODE_ENTRYPOINT value
  sessionId: string
  timestamp: string
  version: string
  gitBranch?: string
  slug?: string
}
```

#### `LogOption`
Complete session log descriptor with metadata:
```typescript
type LogOption = {
  date: string; messages: SerializedMessage[]
  fullPath?: string; value: number
  created: Date; modified: Date
  firstPrompt: string; messageCount: number
  fileSize?: number; isSidechain: boolean
  isLite?: boolean; sessionId?: string
  teamName?: string; agentName?: string
  agentColor?: string; agentSetting?: string
  isTeammate?: boolean; leafUuid?: UUID
  summary?: string; customTitle?: string; tag?: string
  fileHistorySnapshots?: FileHistorySnapshot[]
  attributionSnapshots?: AttributionSnapshotMessage[]
  contextCollapseCommits?: ContextCollapseCommitEntry[]
  contextCollapseSnapshot?: ContextCollapseSnapshotEntry
  gitBranch?: string; projectPath?: string
  prNumber?: number; prUrl?: string; prRepository?: string
  mode?: 'coordinator' | 'normal'
  worktreeSession?: PersistedWorktreeSession | null
  contentReplacements?: ContentReplacementRecord[]
}
```

#### Transcript Entry Types

All are part of the `Entry` union type stored in transcript files:

| Type | Description |
|---|---|
| `TranscriptMessage` | Full serialized message with parent UUID, sidechain status, agent info. |
| `SummaryMessage` | `{ type: 'summary'; leafUuid; summary }` |
| `CustomTitleMessage` | `{ type: 'custom-title'; sessionId; customTitle }` |
| `AiTitleMessage` | `{ type: 'ai-title'; sessionId; aiTitle }` — AI-generated; never re-appended on resume. |
| `LastPromptMessage` | `{ type: 'last-prompt'; sessionId; lastPrompt }` |
| `TaskSummaryMessage` | `{ type: 'task-summary'; sessionId; summary; timestamp }` — Periodic summary from fork every min(5 steps, 2min). |
| `TagMessage` | `{ type: 'tag'; sessionId; tag }` |
| `AgentNameMessage` | `{ type: 'agent-name'; sessionId; agentName }` |
| `AgentColorMessage` | `{ type: 'agent-color'; sessionId; agentColor }` |
| `AgentSettingMessage` | `{ type: 'agent-setting'; sessionId; agentSetting }` |
| `PRLinkMessage` | `{ type: 'pr-link'; sessionId; prNumber; prUrl; prRepository; timestamp }` |
| `ModeEntry` | `{ type: 'mode'; sessionId; mode: 'coordinator' \| 'normal' }` |
| `WorktreeStateEntry` | `{ type: 'worktree-state'; sessionId; worktreeSession: PersistedWorktreeSession \| null }` |
| `ContentReplacementEntry` | `{ type: 'content-replacement'; sessionId; agentId?; replacements }` |
| `FileHistorySnapshotMessage` | `{ type: 'file-history-snapshot'; messageId; snapshot; isSnapshotUpdate }` |
| `AttributionSnapshotMessage` | `{ type: 'attribution-snapshot'; messageId; surface; fileStates; promptCount?; ... }` |
| `ContextCollapseCommitEntry` | `{ type: 'marble-origami-commit'; sessionId; collapseId; summaryUuid; summaryContent; summary; firstArchivedUuid; lastArchivedUuid }` — Obfuscated type name to avoid leaking feature name. |
| `ContextCollapseSnapshotEntry` | `{ type: 'marble-origami-snapshot'; sessionId; staged[]; armed; lastSpawnTokens }` — Last-wins snapshot. |

#### `PersistedWorktreeSession`
```typescript
type PersistedWorktreeSession = {
  originalCwd: string; worktreePath: string; worktreeName: string
  worktreeBranch?: string; originalBranch?: string; originalHeadCommit?: string
  sessionId: string; tmuxSessionName?: string; hookBased?: boolean
}
```

#### `FileAttributionState`
```typescript
type FileAttributionState = {
  contentHash: string       // SHA-256 of file content
  claudeContribution: number // Characters written by Claude
  mtime: number             // File modification time
}
```

### 24.7 Text Input Types (`types/textInputTypes.ts`)

#### `VimMode`
```typescript
type VimMode = 'INSERT' | 'NORMAL'
```

#### `PromptInputMode`
```typescript
type PromptInputMode =
  | 'bash'
  | 'prompt'
  | 'orphaned-permission'
  | 'task-notification'
```

#### `QueuePriority` (also in command.ts)
```typescript
type QueuePriority = 'now' | 'next' | 'later'
```

#### `QueuedCommand`
```typescript
type QueuedCommand = {
  value: string | Array<ContentBlockParam>
  mode: PromptInputMode
  priority?: QueuePriority
  uuid?: UUID
  orphanedPermission?: OrphanedPermission
  pastedContents?: Record<number, PastedContent>
  preExpansionValue?: string      // Value before [Pasted text #N] expansion (for ultraplan detection)
  skipSlashCommands?: boolean     // Treat as plain text, skip / dispatch
  bridgeOrigin?: boolean          // Remote bridge origin; use isBridgeSafeCommand filter
  isMeta?: boolean                // Hidden from UI but model-visible
  origin?: MessageOrigin          // undefined = human (keyboard)
  workload?: string               // cc_workload= billing header tag
  agentId?: AgentId               // Target agent (undefined = main thread)
}
```

#### `InlineGhostText`
```typescript
type InlineGhostText = {
  readonly text: string            // Ghost text (e.g., "mit" for /commit)
  readonly fullCommand: string     // Full command name (e.g., "commit")
  readonly insertPosition: number  // Position in input where ghost text appears
}
```

#### `BaseTextInputProps`
Core props for all text input components including: `value`, `onChange`, `onSubmit?`, `onExit?`, `columns`, `cursorOffset`, `onChangeCursorOffset`, `placeholder?`, `multiline?`, `focus?`, `mask?`, `showCursor?`, `highlightPastedText?`, `maxVisibleLines?`, `onImagePaste?`, `onPaste?`, `onIsPastingChange?`, `disableCursorMovementForUpDownKeys?`, `disableEscapeDoublePress?`, `argumentHint?`, `onUndo?`, `dimColor?`, `highlights?`, `placeholderElement?`, `inlineGhostText?`, `inputFilter?`, `onHistoryUp?`, `onHistoryDown?`, `onHistoryReset?`, `onClearInput?`, `onExitMessage?`.

#### `BaseInputState`
```typescript
type BaseInputState = {
  onInput: (input: string, key: Key) => void
  renderedValue: string; offset: number; setOffset: (offset) => void
  cursorLine: number; cursorColumn: number
  viewportCharOffset: number; viewportCharEnd: number
  isPasting?: boolean
  pasteState?: { chunks: string[]; timeoutId: ReturnType<typeof setTimeout> | null }
}
```

#### `OrphanedPermission`
```typescript
type OrphanedPermission = {
  permissionResult: PermissionResult
  assistantMessage: AssistantMessage
}
```

---

## Summary of Key Numerical Limits

| Limit | Value | Context |
|---|---|---|
| Image base64 max | 5 MB | API hard limit |
| Image raw target | 3.75 MB | Derived client target |
| Image max dimension | 2000 px | Client-side resize limit |
| PDF raw max | 20 MB | Safe API request budget |
| PDF page max (API) | 100 pages | API hard limit |
| PDF extract threshold | 3 MB | Switch to page-image extraction |
| PDF max extract | 100 MB | Absolute PDF size rejection |
| PDF pages per Read | 20 | Single Read tool call limit |
| PDF inline @ mention | 10 pages | Above this → reference treatment |
| Media per request | 100 | Images + PDFs combined |
| Tool result default max | 50,000 chars | Per-tool result size cap |
| Tool result token max | 100,000 tokens | ~400 KB |
| Tool results per message | 200,000 chars | Aggregate per-turn budget |
| Tool summary max | 50 chars | Compact view summaries |
| Task ID space | ~2.8 trillion | 36^8 combinations |
| Binary check size | 8,192 bytes | For binary content detection |
| Binary threshold | 10% | Non-printable byte ratio |
