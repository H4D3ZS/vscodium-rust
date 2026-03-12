import { invoke } from './tauri_bridge.ts';
import { open } from '@tauri-apps/plugin-dialog';

export function initExtensions() {
    console.log("DEBUG: initExtensions called");
    const searchInput = document.getElementById("extensions-search-input") as HTMLInputElement;
    const marketplaceList = document.getElementById("marketplace-extensions-list");
    const installedList = document.getElementById("installed-extensions-list");

    if (searchInput && marketplaceList) {
        searchInput.onkeydown = async (e) => {
            if (e.key === "Enter") {
                const query = searchInput.value.trim();
                console.log("DEBUG: Extension search query:", query);
                if (!query) return;

                marketplaceList.innerHTML = '<div style="padding: 10px; color: var(--text-secondary);">Searching Marketplace...</div>';

                try {
                    const results = await invoke<any>("search_marketplace", { query });
                    console.log("DEBUG: Marketplace results:", results);
                    renderMarketplace(results.results, marketplaceList);
                } catch (err) {
                    console.error("Marketplace search failed:", err);
                    marketplaceList.innerHTML = `<div style="padding: 10px; color: #f48771;">Search error: ${err}</div>`;
                }
            }
        };
    }

    const installedHeader = document.getElementById("installed-accordion-header");
    const marketplaceHeader = document.getElementById("marketplace-accordion-header");
    const marketplaceContent = document.getElementById("marketplace-extensions-list");

    if (installedHeader && installedList) {
        installedHeader.onclick = () => {
            installedList.classList.toggle("collapsed");
            const icon = installedHeader.querySelector(".accordion-icon");
            if (icon) {
                icon.classList.toggle("codicon-chevron-down");
                icon.classList.toggle("codicon-chevron-right");
            }
        };
    }

    if (marketplaceHeader && marketplaceContent) {
        marketplaceHeader.onclick = () => {
            marketplaceContent.classList.toggle("collapsed");
            const icon = marketplaceHeader.querySelector(".accordion-icon");
            if (icon) {
                icon.classList.toggle("codicon-chevron-down");
                icon.classList.toggle("codicon-chevron-right");
            }
        };
    }

    const installVsixBtn = document.getElementById("install-vsix-btn");
    if (installVsixBtn) {
        installVsixBtn.onclick = async () => {
            try {
                const selected = await open({
                    multiple: false,
                    filters: [{
                        name: 'VSIX Extension',
                        extensions: ['vsix']
                    }]
                });

                if (selected) {
                    const filePath = Array.isArray(selected) ? selected[0] : selected;
                    console.log("Installing VSIX from:", filePath);
                    await invoke("install_vsix", { filePath });
                    console.log("VSIX installed successfully");
                    refreshInstalledExtensions();
                }
            } catch (err) {
                console.error("Failed to install VSIX:", err);
            }
        };
    }

    refreshInstalledExtensions();
}

export async function refreshInstalledExtensions() {
    const installedList = document.getElementById("installed-extensions-list");
    if (!installedList) return;

    try {
        const extensions = await invoke<any[]>("get_running_extensions");
        renderInstalled(extensions, installedList);

        // Check for icon themes
        const iconThemeMapping = await invoke<any>("get_icon_theme_mapping");
        if (iconThemeMapping) {
            (window as any).useStore.getState().setIconThemeMapping(iconThemeMapping);
        }
    } catch (err) {
        console.error("Failed to get installed extensions or icon themes:", err);
    }
}

function renderMarketplace(extensions: any[], container: HTMLElement) {
    container.innerHTML = "";
    if (!extensions || extensions.length === 0) {
        container.innerHTML = '<div style="padding: 20px; text-align: center; color: var(--text-secondary); font-size: 13px;">No extensions found in Marketplace.</div>';
        return;
    }

    extensions.forEach(ext => {
        const item = document.createElement("div");
        item.className = "extension-item";
        item.style.padding = "12px";
        item.style.display = "flex";
        item.style.gap = "12px";
        item.style.borderBottom = "1px solid var(--border-color)";
        item.style.transition = "background 0.2s";
        item.style.cursor = "pointer";
        item.onmouseenter = () => item.style.background = "rgba(255,255,255,0.03)";
        item.onmouseleave = () => item.style.background = "transparent";

        const icon = ext.files?.icon || "https://open-vsx.org/api/icons/default.png";
        const downloads = ext.downloadCount ? (ext.downloadCount > 1000 ? (ext.downloadCount / 1000).toFixed(1) + "k" : ext.downloadCount) : "0";
        const rating = ext.averageRating ? ext.averageRating.toFixed(1) : "0.0";

        item.innerHTML = `
            <img src="${icon}" style="width: 42px; height: 42px; flex-shrink: 0;" />
            <div class="extension-info" style="flex: 1; min-width: 0;">
                <div style="display: flex; justify-content: space-between; align-items: flex-start;">
                    <div class="extension-name" style="font-weight: 600; font-size: 13px; color: var(--accent-color);">${ext.displayName || ext.name}</div>
                </div>
                <div class="extension-publisher" style="font-size: 11px; opacity: 0.8;">${ext.namespace}</div>
                <div class="extension-description" style="font-size: 12px; margin-top: 4px; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; line-height: 1.3;">${ext.description || "No description provided."}</div>
                
                <div style="display: flex; align-items: center; gap: 12px; margin-top: 8px; font-size: 11px; opacity: 0.6;">
                    <span style="display: flex; align-items: center; gap: 3px;"><i class="codicon codicon-cloud-download"></i> ${downloads}</span>
                    <span style="display: flex; align-items: center; gap: 3px;"><i class="codicon codicon-star-full" style="color: #f1c40f;"></i> ${rating}</span>
                </div>

                <div style="margin-top: 8px;">
                    <button class="extension-install-btn install-btn" style="padding: 4px 12px; background: var(--accent-color); color: white; border: none; border-radius: 2px; font-size: 12px; cursor: pointer; width: 100%;">Install</button>
                </div>
            </div>
        `;

        const installBtn = item.querySelector(".install-btn") as HTMLButtonElement;
        installBtn.onclick = async (e) => {
            e.stopPropagation();
            installBtn.innerText = "Installing...";
            installBtn.disabled = true;
            try {
                const downloadUrl = ext.files.download;
                await invoke("install_extension", { downloadUrl, name: `${ext.namespace}.${ext.name}` });
                installBtn.innerText = "Installed";
                installBtn.style.background = "transparent";
                installBtn.style.border = "1px solid var(--accent-color)";
                installBtn.style.color = "var(--accent-color)";
                refreshInstalledExtensions();
            } catch (err) {
                console.error("Installation failed:", err);
                installBtn.innerText = "Error";
                installBtn.style.background = "#f44336";
                setTimeout(() => {
                    installBtn.innerText = "Install";
                    installBtn.style.background = "var(--accent-color)";
                    installBtn.disabled = false;
                }, 3000);
            }
        };

        container.appendChild(item);
    });
}

function renderInstalled(extensions: any[], container: HTMLElement) {
    container.innerHTML = "";
    if (!extensions || extensions.length === 0) {
        container.innerHTML = '<div style="padding: 10px; color: var(--text-secondary); font-size: 12px;">No extensions installed.</div>';
        return;
    }

    extensions.forEach(ext => {
        const item = document.createElement("div");
        item.className = "extension-item";
        item.style.padding = "10px";
        item.style.display = "flex";
        item.style.gap = "12px";
        item.style.position = "relative";
        item.style.transition = "background 0.2s";
        item.onmouseenter = () => item.style.background = "rgba(255,255,255,0.03)";
        item.onmouseleave = () => item.style.background = "transparent";

        const iconHtml = ext.base64_icon 
            ? `<img src="${ext.base64_icon}" style="width: 32px; height: 32px; flex-shrink: 0; border-radius: 4px;" />`
            : `<i class="codicon codicon-extensions" style="font-size: 32px; color: var(--accent-color);"></i>`;

        item.innerHTML = `
            ${iconHtml}
            <div class="extension-info" style="flex: 1; min-width: 0;">
                <div class="extension-name" style="font-weight: 600; font-size: 13px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">${ext.name}</div>
                <div class="extension-publisher" style="font-size: 11px; opacity: 0.7;">${ext.publisher} v${ext.version}</div>
                <div class="extension-description" style="font-size: 12px; margin-top: 2px; color: var(--text-secondary); height: 1.2en; overflow: hidden; text-overflow: ellipsis; display: -webkit-box; -webkit-line-clamp: 1; -webkit-box-orient: vertical;">${ext.description || ""}</div>
                <div class="extension-actions" style="margin-top: 6px; display: flex; gap: 12px; opacity: 0.5;">
                     <i class="codicon codicon-settings" style="font-size: 14px; cursor: pointer;" title="Extension Settings"></i>
                     <i class="codicon codicon-debug-pause" style="font-size: 14px; cursor: pointer;" title="Disable Extension"></i>
                     <i class="codicon codicon-trash" style="font-size: 14px; cursor: pointer;" title="Uninstall"></i>
                </div>
            </div>
        `;
        container.appendChild(item);
    });
}
