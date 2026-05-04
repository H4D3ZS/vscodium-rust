import { invoke, listen } from './tauri_bridge.ts';

export function initDebugUI() {
    const startBtn = document.getElementById('start-debug');
    const toolbar = document.getElementById('debug-toolbar');
    const btnContinue = document.getElementById('debug-continue');
    const btnStop = document.getElementById('debug-stop');

    if (startBtn) {
        startBtn.onclick = async () => {
            const adapterPath = prompt('Path to debug adapter executable:');
            if (!adapterPath) return;
            try {
                await invoke('debug_start', { adapterPath });
                toolbar && toolbar.classList.remove('hidden');
            } catch (e) {
                alert(`Failed to start debug adapter: ${e}`);
            }
        };
    }

    if (btnContinue) {
        btnContinue.onclick = () => sendDap({ command: 'continue' });
    }
    if (btnStop) {
        btnStop.onclick = async () => {
            await invoke('debug_stop');
            toolbar && toolbar.classList.add('hidden');
        };
    }

    // Hook F5 basic handling
    document.addEventListener('keydown', (e) => {
        if (e.key === 'F5') {
            e.preventDefault();
            sendDap({ command: 'continue' });
        }
    });

    // Listen for DAP messages (for now just log)
    listen('dap-message', (event) => {
        console.log('DAP:', event.payload);
    });
    listen('debug-log', (event) => {
        console.log('DEBUG:', event.payload);
    });
}

function sendDap(command: any) {
    const msg = JSON.stringify(command);
    invoke('debug_send', { msg }).catch((e) => console.error('DAP send failed', e));
}

