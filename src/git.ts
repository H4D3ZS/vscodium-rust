import { invoke } from './tauri_bridge.ts';

export function initGit() {
    const scmInput = document.getElementById("scm-input") as HTMLInputElement;
    const scmChanges = document.getElementById("scm-changes");

    if (scmInput && scmChanges) {
        scmInput.onkeydown = async (e) => {
            if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) {
                const message = scmInput.value.trim();
                if (!message) return;

                const activeRoot = (window as any).activeRoot;
                if (!activeRoot) return;

                try {
                    await invoke("git_commit", { path: activeRoot, message });
                    scmInput.value = "";
                    await refreshGitStatus();
                } catch (err) {
                    console.error("Git commit failed:", err);
                }
            }
        };

        // Initial refresh
        refreshGitStatus();
    }
}

export async function refreshGitStatus() {
    const scmChanges = document.getElementById("scm-changes");
    const activeRoot = (window as any).activeRoot;
    if (!scmChanges || !activeRoot) return;

    try {
        const statuses = await invoke<any[]>("git_status", { path: activeRoot });
        renderGitStatus(statuses, scmChanges);
    } catch (err) {
        console.error("Failed to get git status:", err);
    }
}

function renderGitStatus(statuses: any[], container: HTMLElement) {
    container.innerHTML = "";

    if (statuses.length === 0) {
        container.innerHTML = '<div style="padding: 10px; color: var(--text-secondary); text-align: center;">No changes detected.</div>';
        return;
    }

    const sections = {
        'Staged': statuses.filter(s => s.status === 'Staged'),
        'Changes': statuses.filter(s => s.status !== 'Staged')
    };

    for (const [name, files] of Object.entries(sections)) {
        if (files.length === 0) continue;

        const header = document.createElement("div");
        header.className = "scm-header";
        header.style.padding = "4px 8px";
        header.style.fontSize = "11px";
        header.style.fontWeight = "bold";
        header.style.color = "var(--text-secondary)";
        header.style.textTransform = "uppercase";
        header.innerText = name;
        container.appendChild(header);

        files.forEach(file => {
            const item = document.createElement("div");
            item.className = "scm-item";
            item.style.padding = "2px 20px";
            item.style.display = "flex";
            item.style.alignItems = "center";
            item.style.gap = "8px";
            item.style.cursor = "pointer";

            const fileName = file.path.split('/').pop() || file.path;
            const statusColor = file.status === 'Modified' ? '#e2c08d' : (file.status === 'New' ? '#73c991' : '#c74e39');

            item.innerHTML = `
                <span style="font-size: 13px; flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">${fileName}</span>
                <span class="scm-badge" style="color: ${statusColor}; font-size: 11px; font-weight: bold;">${file.status[0]}</span>
            `;

            item.onclick = async () => {
                const activeRoot = (window as any).activeRoot;
                if (file.status === 'Staged') {
                    await invoke("git_unstage", { path: activeRoot, filePath: file.path });
                } else {
                    await invoke("git_stage", { path: activeRoot, filePath: file.path });
                }
                refreshGitStatus();
            };

            container.appendChild(item);
        });
    }
}
