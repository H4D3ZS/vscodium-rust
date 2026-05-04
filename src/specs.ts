import { invoke } from './tauri_bridge.ts';
import { openFile } from './editor.ts';

export interface Spec {
    name: string;
    path: string;
}

let specsList: Spec[] = [];

export async function initSpecs() {
    const newSpecBtn = document.getElementById("new-spec");
    if (newSpecBtn) {
        newSpecBtn.onclick = () => createNewSpec();
    }
}

export async function refreshSpecs() {
    const listContainer = document.getElementById("specs-list");
    const emptyMsg = document.getElementById("specs-empty");

    try {
        const workspaceRoot = (window as any).activeRoot;
        if (!workspaceRoot) return;

        const specsPath = `${workspaceRoot}/.kiro/specs`;

        // Use a recursive list check or just create_dir
        await invoke("create_directory", { path: specsPath });

        const entries = await invoke<any[]>("list_directory", { path: specsPath });
        specsList = entries.filter(e => e.is_dir).map(e => ({
            name: e.name,
            path: e.path
        }));

        if (listContainer) {
            listContainer.innerHTML = "";
            if (specsList.length > 0) {
                if (emptyMsg) emptyMsg.classList.add("hidden");
                specsList.forEach(spec => {
                    const item = document.createElement("div");
                    item.className = "spec-item";
                    item.onclick = () => openSpec(spec);
                    item.innerHTML = `
                        <div class="spec-item-title">${spec.name}</div>
                        <div class="spec-item-path">.kiro/specs/${spec.name}</div>
                    `;
                    listContainer.appendChild(item);
                });
            } else {
                if (emptyMsg) emptyMsg.classList.remove("hidden");
            }
        }
    } catch (e) {
        console.error("Failed to refresh specs:", e);
    }
}

async function createNewSpec() {
    const name = prompt("Enter spec name (e.g. 'auth-system'):");
    if (!name) return;

    const workspaceRoot = (window as any).activeRoot;
    if (!workspaceRoot) {
        alert("Please open a workspace folder first.");
        return;
    }

    const type = prompt("Type (1: Feature, 2: Bugfix):", "1");
    if (!type) return;

    const specPath = `${workspaceRoot}/.kiro/specs/${name}`;

    try {
        await invoke("create_directory", { path: specPath });

        if (type === "1") {
            // Feature Spec
            await invoke("create_file", { path: `${specPath}/requirements.md` });
            await invoke("create_file", { path: `${specPath}/design.md` });
            await invoke("create_file", { path: `${specPath}/tasks.md` });

            const reqBoilerplate = `# Requirements: ${name}\n\n## User Stories\n- [ ] As a user, I want...\n\n## Acceptance Criteria\n- [ ] ...\n\n### EARS Requirements\nWHEN ... THE SYSTEM SHALL ...\n`;
            const designBoilerplate = `# Design: ${name}\n\n## Architecture\n...`;
            const tasksBoilerplate = `# Tasks: ${name}\n\n- [ ] Task 1`;

            // Note: ai_modify_file uses replace, so since file is empty, we might need a better 'write_file' 
            // but for now let's hope it works or I'll implement 'write_file' in Rust.
            // Actually ai_modify_file checks content.contains(target). 
            // I'll add a 'write_file' command to lib.rs.
            await invoke("write_file", { path: `${specPath}/requirements.md`, content: reqBoilerplate });
            await invoke("write_file", { path: `${specPath}/design.md`, content: designBoilerplate });
            await invoke("write_file", { path: `${specPath}/tasks.md`, content: tasksBoilerplate });

        } else {
            // Bugfix Spec
            await invoke("create_file", { path: `${specPath}/bugfix.md` });
            await invoke("create_file", { path: `${specPath}/tasks.md` });

            const bugBoilerplate = `# Bug Analysis: ${name}\n\n## Current Behavior\n...\n\n## Expected Behavior\n...\n`;
            await invoke("write_file", { path: `${specPath}/bugfix.md`, content: bugBoilerplate });
            await invoke("write_file", { path: `${specPath}/tasks.md`, content: "# Tasks\n- [ ] Fix bug" });
        }

        await refreshSpecs();
        alert(`Spec '${name}' created!`);
    } catch (e) {
        alert(`Failed to create spec: ${e}`);
    }
}

async function openSpec(spec: Spec) {
    // Open the primary file of the spec
    // Find files in spec dir
    try {
        const entries = await invoke<any[]>("list_directory", { path: spec.path });
        const primary = entries.find(e => e.name === "requirements.md" || e.name === "bugfix.md");
        if (primary) {
            await openFile(primary.path, primary.name);
        }
    } catch (e) {
        console.error("Failed to open spec:", e);
    }
}
