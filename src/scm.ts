import { invoke } from './tauri_bridge.ts';

interface GitFileStatus {
    path: string;
    status: string;
}

function getRoot(): string {
    return (window as any).activeRoot || '.';
}

function renderScmStatus(statuses: GitFileStatus[], container: HTMLElement) {
    container.innerHTML = '';

    if (!statuses || statuses.length === 0) {
        container.innerHTML = '<div style="padding: 10px; font-size: 12px; opacity: 0.7;">No changes</div>';
        return;
    }

    const list = document.createElement('div');
    statuses.forEach((file) => {
        const row = document.createElement('div');
        row.className = 'scm-item';
        row.style.display = 'flex';
        row.style.alignItems = 'center';
        row.style.padding = '2px 10px';
        row.style.fontSize = '12px';

        const label = document.createElement('span');
        label.textContent = file.path;
        row.appendChild(label);

        const badge = document.createElement('span');
        badge.className = 'scm-badge ' + file.status.trim();
        badge.textContent = file.status.trim();
        row.appendChild(badge);

        row.onclick = () => {
            // For now just open file; later can show diff view
            const fullPath = `${getRoot()}/${file.path}`;
            (window as any).monacoEditor && (window as any).monacoEditor.focus();
            // Reuse openFile helper if present
            import('./editor').then(mod => mod.openFile(fullPath, file.path.split('/').pop() || file.path));
        };

        list.appendChild(row);
    });

    container.appendChild(list);
}

export async function refreshScm() {
    const root = getRoot();
    const container = document.getElementById('scm-changes');
    if (!container) return;

    try {
        const statuses = await invoke<GitFileStatus[]>('git_status', { path: root });
        renderScmStatus(statuses, container);
    } catch (e) {
        container.innerHTML = `<div style="padding: 10px; color: #f48771; font-size: 12px;">Git error: ${e}</div>`;
    }
}

export function initScm() {
    const msgInput = document.getElementById('scm-input') as HTMLInputElement | null;
    if (!msgInput) return;

    msgInput.onkeydown = async (e) => {
        if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
            e.preventDefault();
            const message = msgInput.value.trim();
            if (!message) return;
            try {
                await invoke('git_commit', { path: getRoot(), message });
                msgInput.value = '';
                await refreshScm();
            } catch (err) {
                alert(`Commit failed: ${err}`);
            }
        }
    };

    refreshScm();
}

