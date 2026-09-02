<script lang="ts">
    import { store } from '../../store';
    import { llamaCppService } from '../../llama-cpp-service';
    import {
        profileModel,
        defaultProfilePath,
        loadProfile,
        quickPlan,
        summarizePlan,
        type Backend as KortexBackend,
        type TierPlan,
    } from '../../kortex/gac-orchestrator';

    let backend: 'ollama' | 'llama-cpp' | 'openai' = $state($store.inferenceBackend);
    let llamaCppUrl = $state($store.llamaCppUrl);
    let llamaCppModelPath = $state($store.llamaCppModelPath);
    let llamaCppNgl = $state($store.llamaCppNgl);
    let hadesEnabled = $state($store.llamaCppHadesEnabled);
    let llamaCppStatus = $state($store.llamaCppStatus);

    // Kortex GAC settings — these live in localStorage via the llama-cpp-service.
    const kortexSaved = (() => {
        try { return JSON.parse(localStorage.getItem('llamaCppConfig') ?? '{}'); }
        catch { return {}; }
    })();
    let kortexEnabled = $state(kortexSaved.kortexEnabled ?? true);
    let vramTotalMb = $state(kortexSaved.vramTotalMb ?? 8192);
    let kortexTheta = $state(kortexSaved.kortexTheta ?? 0.85);
    let kortexBackend: KortexBackend = $state(kortexSaved.kortexBackend ?? 'vulkan');
    let serverBinary = $state(kortexSaved.serverBinary ?? '');

    let kortexBusy = $state(false);
    let kortexLastPlan: TierPlan | null = $state(null);
    let kortexPlanSummary = $state('');
    let kortexProfilePath = $state('');
    let kortexError = $state('');

    function saveSettings() {
        $store.setInferenceBackend(backend);
        $store.setLlamaCppUrl(llamaCppUrl);
        $store.setLlamaCppModelPath(llamaCppModelPath);
        $store.setLlamaCppNgl(llamaCppNgl);
        $store.setLlamaCppHadesEnabled(hadesEnabled);

        llamaCppService.configure({
            enabled: backend === 'llama-cpp',
            modelPath: llamaCppModelPath,
            ngl: llamaCppNgl,
            hadesEnabled,
            kortexEnabled,
            vramTotalMb,
            kortexTheta,
            kortexBackend,
            serverBinary,
        });
    }

    async function checkStatus() {
        if (backend === 'llama-cpp') {
            await $store.checkLlamaCppStatus();
        } else {
            await $store.checkOllamaStatus();
        }
    }

    async function profileNow() {
        kortexError = '';
        kortexBusy = true;
        try {
            kortexProfilePath = await profileModel(llamaCppModelPath);
            const profile = await loadProfile(kortexProfilePath);
            kortexPlanSummary = `profiled ${profile.global.n_profiled} tensors · `
                + `d_eff_global=${profile.global.d_eff_global.toFixed(1)} · `
                + `d̄_mean=${profile.global.d_bar_mean.toFixed(3)}`;
        } catch (e) {
            kortexError = String(e);
        } finally {
            kortexBusy = false;
        }
    }

    async function previewPlan() {
        kortexError = '';
        kortexBusy = true;
        try {
            const plan = await quickPlan(llamaCppModelPath, {
                vram_total_mb: vramTotalMb,
                theta: kortexTheta,
                backend: kortexBackend,
            }, false);
            kortexLastPlan = plan;
            kortexPlanSummary = summarizePlan(plan);
        } catch (e) {
            kortexError = String(e);
        } finally {
            kortexBusy = false;
        }
    }

    async function startKortex() {
        kortexError = '';
        kortexBusy = true;
        try {
            saveSettings();
            const result = await llamaCppService.startServer();
            if (result && 'plan' in result) {
                kortexLastPlan = result.plan;
                kortexPlanSummary = summarizePlan(result.plan);
            }
            llamaCppStatus = 'running';
        } catch (e) {
            kortexError = String(e);
        } finally {
            kortexBusy = false;
        }
    }

    async function stopKortex() {
        kortexBusy = true;
        try {
            await llamaCppService.stopServer();
            llamaCppStatus = 'disconnected';
        } finally {
            kortexBusy = false;
        }
    }

    async function showProfilePath() {
        if (!llamaCppModelPath) return;
        kortexProfilePath = await defaultProfilePath(llamaCppModelPath);
    }

    $effect(() => {
        llamaCppStatus = $store.llamaCppStatus;
    });
    $effect(() => {
        if (llamaCppModelPath) showProfilePath();
    });
</script>

<div class="inference-backend-settings">
    <h3>Inference Backend</h3>
    <p class="description">
        Choose your AI inference backend. Ollama is recommended for most users. 
        llama.cpp with HADES Bridge provides optimized 8GB VRAM inference.
    </p>

    <div class="backend-selector">
        <label>
            <input 
                type="radio" 
                name="backend" 
                value="ollama" 
                bind:checked={backend === 'ollama'}
            />
            <div class="backend-option">
                <strong>Ollama</strong>
                <span>Recommended for most users. Easy setup with automatic model management.</span>
            </div>
        </label>

        <label>
            <input 
                type="radio" 
                name="backend" 
                value="llama-cpp" 
                bind:checked={backend === 'llama-cpp'}
            />
            <div class="backend-option">
                <strong>llama.cpp + HADES</strong>
                <span>Direct llama.cpp with HADES Bridge for 8GB VRAM optimization.</span>
            </div>
        </label>

        <label>
            <input 
                type="radio" 
                name="backend" 
                value="openai" 
                bind:checked={backend === 'openai'}
            />
            <div class="backend-option">
                <strong>OpenAI API</strong>
                <span>Cloud-based inference with GPT-4 and other models.</span>
            </div>
        </label>
    </div>

    {#if backend === 'llama-cpp'}
        <div class="llama-cpp-config">
            <h4>llama.cpp Configuration</h4>

            <div class="form-group">
                <label>Server URL</label>
                <input 
                    type="text" 
                    bind:value={llamaCppUrl}
                    placeholder="http://localhost:8080"
                />
                <small>Default: http://localhost:8080</small>
            </div>

            <div class="form-group">
                <label>Model Path</label>
                <input 
                    type="text" 
                    bind:value={llamaCppModelPath}
                    placeholder="C:/models/llama-3.2-3b.gguf"
                />
                <small>Path to GGUF model file</small>
            </div>

            <div class="form-group">
                <label>GPU Layers (NGPU)</label>
                <input 
                    type="number" 
                    bind:value={llamaCppNgl}
                    min="0"
                    max="99"
                />
                <small>Number of layers to offload to GPU (99 = all layers)</small>
            </div>

            <div class="form-group checkbox">
                <label>
                    <input 
                        type="checkbox" 
                        bind:checked={hadesEnabled}
                    />
                    Enable HADES Bridge
                </label>
                <small>
                    Activates 8GB VRAM optimization with JIT decompression and thermal governance.
                    Recommended for AMD RX 580 and other consumer GPUs.
                </small>
            </div>

            <hr style="opacity: 0.2; margin: 16px 0" />

            <h4>Kortex GAC — Geometry-Aware Scheduling</h4>
            <p class="description" style="margin: 0 0 12px 0">
                Profiles each weight tensor in your GGUF, then chooses GPU vs CPU
                placement using the geometry-of-consolidation theorem.
                <strong>Spread</strong> tensors (every direction matters) win the
                GPU. <strong>Tight</strong> tensors (high redundancy) ship to
                CPU. Result: a stronger 35B–70B fit on an 8GB card than naive
                <code>--n-gpu-layers</code> can ever produce.
            </p>

            <div class="form-group checkbox">
                <label>
                    <input type="checkbox" bind:checked={kortexEnabled} />
                    Use Kortex GAC scheduler when starting llama-server
                </label>
            </div>

            <div class="form-group">
                <label>Total VRAM (MB)</label>
                <input type="number" bind:value={vramTotalMb} min="1024" max="49152" step="512" />
                <small>RX 580 8GB → 8192 · RTX 3070 8GB → 8192 · RTX 4090 24GB → 24576</small>
            </div>

            <div class="form-group">
                <label>θ (retrieval threshold)</label>
                <input type="range" bind:value={kortexTheta} min="0.5" max="0.95" step="0.01" />
                <small>θ = {kortexTheta.toFixed(2)}. Higher θ = stricter cap, more tensors flagged spread, larger GPU footprint. Default 0.85 (paper recommendation).</small>
            </div>

            <div class="form-group">
                <label>GPU backend</label>
                <select bind:value={kortexBackend}>
                    <option value="vulkan">Vulkan (RX 580, generic AMD/Intel)</option>
                    <option value="cuda">CUDA (NVIDIA)</option>
                    <option value="rocm">ROCm (modern AMD only)</option>
                    <option value="metal">Metal (Apple)</option>
                    <option value="sycl">SYCL (Intel)</option>
                </select>
                <small>Buffer name used in llama.cpp <code>--override-tensor</code> rules.</small>
            </div>

            <div class="form-group">
                <label>llama-server binary (optional)</label>
                <input type="text" bind:value={serverBinary} placeholder="leave blank to auto-detect on PATH" />
                <small>Override only if you have a custom build (e.g. with HIP for RX 580).</small>
            </div>

            <div class="form-group">
                <label>Geometry profile path</label>
                <input type="text" value={kortexProfilePath} readonly placeholder="(set a model path to compute)" />
                <small>Generated next to the GGUF as <code>&lt;model&gt;.geometry.aim</code>. Reusable across runs.</small>
            </div>

            <div class="actions" style="margin-top: 8px">
                <button class="btn btn-secondary" disabled={kortexBusy || !llamaCppModelPath} on:click={profileNow}>
                    {kortexBusy ? 'Working…' : 'Profile model'}
                </button>
                <button class="btn btn-secondary" disabled={kortexBusy || !llamaCppModelPath} on:click={previewPlan}>
                    Preview plan
                </button>
                <button class="btn btn-primary" disabled={kortexBusy || !llamaCppModelPath} on:click={startKortex}>
                    Start Kortex inference
                </button>
                <button class="btn btn-secondary" disabled={kortexBusy} on:click={stopKortex}>
                    Stop
                </button>
            </div>

            {#if kortexPlanSummary}
                <div class="status-indicator" style="margin-top: 12px; flex-direction: column; align-items: flex-start; gap: 4px">
                    <span class="status-label">Last plan</span>
                    <code style="font-size: 11px; word-break: break-all">{kortexPlanSummary}</code>
                </div>
            {/if}

            {#if kortexError}
                <div class="status-indicator" style="margin-top: 8px; background: rgba(255,80,80,0.10)">
                    <span class="status-label" style="color: var(--vscode-errorForeground)">Error</span>
                    <code style="font-size: 11px">{kortexError}</code>
                </div>
            {/if}

            <div class="status-indicator">
                <span class="status-label">Status:</span>
                <span class={`status-badge ${llamaCppStatus}`}>
                    {llamaCppStatus}
                </span>
            </div>
        </div>
    {:else if backend === 'ollama'}
        <div class="ollama-config">
            <h4>Ollama Configuration</h4>
            <p>Ollama is configured in the main settings panel.</p>
        </div>
    {:else if backend === 'openai'}
        <div class="openai-config">
            <h4>OpenAI API Configuration</h4>
            <p>Configure your OpenAI API key in the API Keys section.</p>
        </div>
    {/if}

    <div class="actions">
        <button class="btn btn-primary" on:click={saveSettings}>
            Save Settings
        </button>
        <button class="btn btn-secondary" on:click={checkStatus}>
            Check Connection
        </button>
    </div>

    <style>
        .inference-backend-settings {
            padding: 20px;
        }

        h3 {
            margin-bottom: 8px;
            color: var(--vscode-foreground);
        }

        .description {
            color: var(--vscode-descriptionForeground);
            margin-bottom: 20px;
            line-height: 1.5;
        }

        .backend-selector {
            display: flex;
            flex-direction: column;
            gap: 12px;
            margin-bottom: 24px;
        }

        .backend-selector label {
            display: flex;
            gap: 12px;
            cursor: pointer;
            padding: 12px;
            border: 1px solid var(--vscode-widget-border);
            border-radius: 6px;
            transition: all 0.2s;
        }

        .backend-selector label:hover {
            background: var(--vscode-list-hoverBackground);
        }

        .backend-selector input[type="radio"] {
            margin-top: 4px;
        }

        .backend-option {
            display: flex;
            flex-direction: column;
            gap: 4px;
        }

        .backend-option strong {
            color: var(--vscode-foreground);
        }

        .backend-option span {
            color: var(--vscode-descriptionForeground);
            font-size: 12px;
        }

        .llama-cpp-config,
        .ollama-config,
        .openai-config {
            background: var(--vscode-textBlockQuoteBackground);
            border: 1px solid var(--vscode-widget-border);
            border-radius: 6px;
            padding: 16px;
            margin-bottom: 20px;
        }

        h4 {
            margin: 0 0 16px 0;
            color: var(--vscode-foreground);
        }

        .form-group {
            margin-bottom: 16px;
        }

        .form-group label {
            display: block;
            margin-bottom: 6px;
            color: var(--vscode-foreground);
            font-weight: 500;
        }

        .form-group input[type="text"],
        .form-group input[type="number"] {
            width: 100%;
            padding: 8px 12px;
            background: var(--vscode-input-background);
            border: 1px solid var(--vscode-input-border);
            color: var(--vscode-input-foreground);
            border-radius: 4px;
        }

        .form-group small {
            display: block;
            margin-top: 4px;
            color: var(--vscode-descriptionForeground);
            font-size: 12px;
        }

        .form-group.checkbox {
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .form-group.checkbox label {
            margin: 0;
            display: flex;
            align-items: center;
            gap: 8px;
            cursor: pointer;
        }

        .status-indicator {
            display: flex;
            align-items: center;
            gap: 8px;
            margin-top: 16px;
            padding: 8px 12px;
            background: var(--vscode-textBlockQuoteBackground);
            border-radius: 4px;
        }

        .status-label {
            font-weight: 500;
        }

        .status-badge {
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 12px;
            font-weight: 500;
        }

        .status-badge.running {
            background: var(--vscode-terminal-ansiGreen);
            color: white;
        }

        .status-badge.error {
            background: var(--vscode-terminal-ansiRed);
            color: white;
        }

        .status-badge.checking,
        .status-badge.idle {
            background: var(--vscode-terminal-ansiYellow);
            color: black;
        }

        .status-badge.disconnected {
            background: var(--vscode-terminal-ansiBlue);
            color: white;
        }

        .actions {
            display: flex;
            gap: 12px;
        }

        .btn {
            padding: 8px 16px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-weight: 500;
            transition: all 0.2s;
        }

        .btn-primary {
            background: var(--vscode-button-background);
            color: var(--vscode-button-foreground);
        }

        .btn-primary:hover {
            background: var(--vscode-button-hoverBackground);
        }

        .btn-secondary {
            background: var(--vscode-button-secondaryBackground);
            color: var(--vscode-button-secondaryForeground);
        }

        .btn-secondary:hover {
            background: var(--vscode-button-secondaryHoverBackground);
        }
    </style>
</div>
