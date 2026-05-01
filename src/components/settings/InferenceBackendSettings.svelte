<script lang="ts">
    import { store } from '../../store';
    import { llamaCppService } from '../../llama-cpp-service';

    let backend: 'ollama' | 'llama-cpp' | 'openai' = $state($store.inferenceBackend);
    let llamaCppUrl = $state($store.llamaCppUrl);
    let llamaCppModelPath = $state($store.llamaCppModelPath);
    let llamaCppNgl = $state($store.llamaCppNgl);
    let hadesEnabled = $state($store.llamaCppHadesEnabled);
    let llamaCppStatus = $state($store.llamaCppStatus);

    function saveSettings() {
        $store.setInferenceBackend(backend);
        $store.setLlamaCppUrl(llamaCppUrl);
        $store.setLlamaCppModelPath(llamaCppModelPath);
        $store.setLlamaCppNgl(llamaCppNgl);
        $store.setLlamaCppHadesEnabled(hadesEnabled);
        
        // Update llama.cpp service config
        llamaCppService.configure({
            enabled: backend === 'llama-cpp',
            modelPath: llamaCppModelPath,
            ngl: llamaCppNgl,
            hadesEnabled,
        });
    }

    async function checkStatus() {
        if (backend === 'llama-cpp') {
            await $store.checkLlamaCppStatus();
        } else {
            await $store.checkOllamaStatus();
        }
    }

    $effect(() => {
        llamaCppStatus = $store.llamaCppStatus;
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
