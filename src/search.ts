import { invoke } from './tauri_bridge.ts';
import { openFile } from './editor.ts';

export function initSearch() {
    const searchInput = document.getElementById("search-input") as HTMLInputElement;
    const searchResults = document.getElementById("search-results");

    if (searchInput && searchResults) {
        searchInput.onkeydown = async (e) => {
            if (e.key === "Enter") {
                const query = searchInput.value.trim();
                if (!query) {
                    searchResults.innerHTML = "";
                    return;
                }

                searchResults.innerHTML = `
                    <div style="padding: 20px; text-align: center; color: var(--vscode-descriptionForeground);">
                        <i class="codicon codicon-loading codicon-modifier-spin" style="font-size: 20px; display: block; margin-bottom: 8px;"></i>
                        <span style="font-size: 12px;">Searching...</span>
                    </div>
                `;

                try {
                    const results = await invoke<any[]>("search_project", { query });
                    renderSearchResults(results, searchResults);
                } catch (err) {
                    console.error("Search failed:", err);
                    searchResults.innerHTML = `<div style="padding: 10px; color: #f48771;">Search error: ${err}</div>`;
                }
            }
        };
    }
}

function renderSearchResults(results: any[], container: HTMLElement) {
    if (results.length === 0) {
        container.innerHTML = '<div style="padding: 20px; text-align: center; color: var(--vscode-descriptionForeground); font-size: 13px;">No results found.</div>';
        return;
    }

    container.innerHTML = "";
    container.style.padding = "4px 0";
    container.style.overflowX = "hidden";

    const grouped: { [key: string]: any[] } = {};
    results.forEach(res => {
        if (!grouped[res.path]) grouped[res.path] = [];
        grouped[res.path].push(res);
    });

    for (const [path, matches] of Object.entries(grouped)) {
        const fileName = path.split('/').pop() || path;
        const relativePath = path.replace((window as any).activeRoot || "", "").slice(1);

        const fileSection = document.createElement("div");
        fileSection.className = "search-file-section";
        fileSection.style.marginBottom = "8px";

        const fileHeader = document.createElement("div");
        fileHeader.className = "search-file-header hoverable";
        fileHeader.style.padding = "4px 12px";
        fileHeader.style.cursor = "pointer";
        fileHeader.style.display = "flex";
        fileHeader.style.alignItems = "center";
        fileHeader.style.gap = "8px";
        fileHeader.style.background = "rgba(255,255,255,0.03)";
        fileHeader.style.fontSize = "12px";

        fileHeader.innerHTML = `
            <i class="codicon codicon-chevron-down" style="font-size: 14px; opacity: 0.8;"></i>
            <i class="codicon codicon-file" style="color: #519aba; font-size: 14px;"></i>
            <span style="font-weight: 500; color: var(--vscode-sideBar-foreground);">${fileName}</span>
            <span style="color: var(--vscode-descriptionForeground); opacity: 0.6; font-size: 11px;">${relativePath || '.'}</span>
            <span style="margin-left: auto; background: var(--vscode-activityBar-background); padding: 0 6px; border-radius: 10px; font-size: 10px; font-weight: 600;">${matches.length}</span>
        `;

        const matchesList = document.createElement("div");
        matchesList.className = "search-matches-list";

        matches.forEach(match => {
            const item = document.createElement("div");
            item.className = "search-match-item hoverable";
            item.style.padding = "3px 32px";
            item.style.fontSize = "12px";
            item.style.cursor = "pointer";
            item.style.whiteSpace = "nowrap";
            item.style.overflow = "hidden";
            item.style.textOverflow = "ellipsis";
            item.style.display = "flex";
            item.style.alignItems = "center";
            item.style.gap = "10px";
            item.style.color = "var(--vscode-sideBar-foreground)";
            item.style.opacity = "0.9";

            const highlightedContent = match.content.replace(
                new RegExp(match.query, 'gi'),
                (m: string) => `<span class="search-match-highlight">${m}</span>`
            );

            item.innerHTML = `
                <span style="color: var(--vscode-descriptionForeground); min-width: 25px; text-align: right; opacity: 0.6; font-size: 11px;">${match.line}</span>
                <span style="font-family: var(--font-mono); overflow: hidden; text-overflow: ellipsis;">${highlightedContent}</span>
            `;

            item.onclick = () => openFile(path, fileName);
            matchesList.appendChild(item);
        });

        fileHeader.onclick = () => {
            const isHidden = matchesList.style.display === "none";
            matchesList.style.display = isHidden ? "block" : "none";
            const arrow = fileHeader.querySelector(".codicon") as HTMLElement;
            arrow.className = isHidden ? "codicon codicon-chevron-down" : "codicon codicon-chevron-right";
        };

        fileSection.appendChild(fileHeader);
        fileSection.appendChild(matchesList);
        container.appendChild(fileSection);
    }
}
