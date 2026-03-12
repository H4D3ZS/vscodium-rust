import { invoke } from './tauri_bridge.ts';
import { openFile } from './editor.ts';

export interface FileEntry {
    name: string;
    path: string;
    is_dir: boolean;
    children?: FileEntry[];
}

let sidebarContent: HTMLElement | null = null;
let activeRoot: string | null = null;
let themeMapping: any = null;

async function ensureThemeMapping() {
    if (!themeMapping) {
        try {
            themeMapping = await invoke("get_icon_theme_mapping");
        } catch (e) {
            console.error("Failed to load icon theme mapping:", e);
        }
    }
}

function getIconHtml(name: string, isDir: boolean, isExpanded: boolean = false): string {
    if (!themeMapping || !themeMapping.iconDefinitions) {
        // Fallback to codicons if theme loading fails
        if (isDir) {
            return `<i class="codicon codicon-folder" style="color: #dcb67a; margin-right:6px; font-size:14px;"></i>`;
        } else {
            let iconColor = name.endsWith('.rs') ? '#dea584' : (name.endsWith('.ts') ? '#3178c6' : (name.endsWith('.html') || name.endsWith('.htm') ? '#e34c26' : (name.endsWith('.css') ? '#264de4' : '#519aba')));
            return `<i class="codicon codicon-file" style="color: ${iconColor}; margin-left: 17px; margin-right: 6px; font-size:14px;"></i>`;
        }
    }

    const defs = themeMapping.iconDefinitions;
    let iconDefId = "";

    if (isDir) {
        const folderName = name.toLowerCase();
        if (isExpanded) {
            iconDefId = themeMapping.folderNamesExpanded?.[folderName] || themeMapping.folderExpanded;
        } else {
            iconDefId = themeMapping.folderNames?.[folderName] || themeMapping.folder;
        }
    } else {
        const fileName = name.toLowerCase();
        const ext = name.includes('.') ? name.split('.').pop()?.toLowerCase() : "";
        iconDefId = themeMapping.fileNames?.[fileName] || themeMapping.fileExtensions?.[ext || ""] || themeMapping.file;
    }

    const def = defs[iconDefId];
    if (def && def.iconPath) {
        const margin = isDir ? "0 6px 0 0" : "0 6px 0 17px";
        return `<img src="${def.iconPath}" style="width: 14px; height: 14px; margin: ${margin}; flex-shrink: 0;" />`;
    }

    // Secondary fallback to codicon
    return isDir ? `<i class="codicon codicon-folder" style="margin-right:6px;"></i>` : `<i class="codicon codicon-file" style="margin-left:17px; margin-right:6px;"></i>`;
}

export async function createFile(path: string) {
    await invoke("create_file", { path });
}

export async function createDirectory(path: string) {
    await invoke("create_directory", { path });
}

export async function renamePath(oldPath: string, newPath: string) {
    await invoke("rename_path", { oldPath, newPath });
}

export async function deletePath(path: string) {
    await invoke("delete_path", { path });
}

export async function refreshExplorer() {
    if (activeRoot) {
        await loadDirectory(activeRoot);
    }
}

export function initExplorer(contentElement: HTMLElement) {
    sidebarContent = contentElement;

    document.getElementById("explorer-refresh")?.addEventListener("click", () => refreshExplorer());

    document.getElementById("explorer-new-file")?.addEventListener("click", async () => {
        if (!activeRoot) return;
        const name = prompt("Enter file name:");
        if (name) {
            await createFile(`${activeRoot}/${name}`);
            await refreshExplorer();
        }
    });

    document.getElementById("explorer-new-folder")?.addEventListener("click", async () => {
        if (!activeRoot) return;
        const name = prompt("Enter folder name:");
        if (name) {
            await createDirectory(`${activeRoot}/${name}`);
            await refreshExplorer();
        }
    });
}

export async function loadDirectory(path: string, container: HTMLElement = sidebarContent!) {
    try {
        if (container === sidebarContent) {
            activeRoot = path;
            (window as any).activeRoot = path;
            const folderName = path.split('/').pop() || path.split('\\').pop();
            const headerText = document.getElementById('explorer-header-text');
            if (headerText) {
                headerText.innerHTML = `<i class="codicon codicon-chevron-down" style="margin-right: 4px;"></i> ${folderName?.toUpperCase()}`;
            }
            container.innerHTML = "";
        }
        await ensureThemeMapping();
        const entries = await invoke<FileEntry[]>("list_directory", { path });
        renderExplorer(entries, container);
    } catch (e) {
        console.error("Failed to load directory:", e);
        container.innerHTML = `<div style="padding: 10px; color: red;">Failed to load: ${e}</div>`;
    }
}

function renderExplorer(entries: FileEntry[], container: HTMLElement) {
    const ul = document.createElement("ul");
    ul.className = "tree-list";
    ul.style.listStyle = "none";
    ul.style.paddingLeft = container === sidebarContent ? "0" : "12px";

    entries.sort((a, b) => {
        if (a.is_dir === b.is_dir) return a.name.localeCompare(b.name);
        return a.is_dir ? -1 : 1;
    });

    entries.forEach(entry => {
        const li = document.createElement("li");
        li.className = "tree-item-container";
        li.style.display = "block";

        const rowDiv = document.createElement("div");
        rowDiv.className = "tree-item";
        rowDiv.style.padding = "3px 5px";
        rowDiv.style.cursor = "pointer";
        rowDiv.style.display = "flex";
        rowDiv.style.alignItems = "center";

        const fontFamily = "-apple-system, BlinkMacSystemFont, 'Segoe UI', 'system-ui', sans-serif";
        const iconHtml = getIconHtml(entry.name, entry.is_dir);

        if (entry.is_dir) {
            rowDiv.innerHTML = `<i class="codicon codicon-chevron-right tree-folder-arrow" style="font-size:14px; margin-right:2px; transition: transform 0.1s;"></i>${iconHtml}<span style="font-size:13px; font-family: ${fontFamily}; letter-spacing: 0.2px;">${entry.name}</span>`;
        } else {
            rowDiv.innerHTML = `${iconHtml}<span style="font-size:13px; font-family: ${fontFamily}; letter-spacing: 0.2px;">${entry.name}</span>`;
        }

        li.appendChild(rowDiv);

        if (entry.is_dir) {
            let expanded = false;
            let childContainer: HTMLDivElement | null = null;
            rowDiv.onclick = async (e) => {
                e.stopPropagation();
                expanded = !expanded;
                const arrow = rowDiv.querySelector(".tree-folder-arrow") as HTMLElement;
                if (expanded) {
                    arrow.classList.remove("codicon-chevron-right");
                    arrow.classList.add("codicon-chevron-down");
                    
                    // Update folder icon to expanded state
                    const iconContainer = rowDiv.querySelector("img, i.codicon-folder") as HTMLElement;
                    if (iconContainer) {
                        const newIconHtml = getIconHtml(entry.name, true, true);
                        const temp = document.createElement('div');
                        temp.innerHTML = newIconHtml;
                        iconContainer.replaceWith(temp.firstChild!);
                    }

                    childContainer = document.createElement("div");
                    li.appendChild(childContainer);
                    await loadDirectory(entry.path, childContainer);
                } else {
                    arrow.classList.add("codicon-chevron-right");
                    arrow.classList.remove("codicon-chevron-down");

                    // Update folder icon to closed state
                    const iconContainer = rowDiv.querySelector("img, i.codicon-folder") as HTMLElement;
                    if (iconContainer) {
                        const newIconHtml = getIconHtml(entry.name, true, false);
                        const temp = document.createElement('div');
                        temp.innerHTML = newIconHtml;
                        iconContainer.replaceWith(temp.firstChild!);
                    }

                    if (childContainer) {
                        li.removeChild(childContainer);
                        childContainer = null;
                    }
                }
            };
        } else {
            rowDiv.onclick = (e) => {
                e.stopPropagation();
                openFile(entry.path, entry.name);
            };
        }

        ul.appendChild(li);
    });

    container.appendChild(ul);
}

// Simplified explorer.ts
