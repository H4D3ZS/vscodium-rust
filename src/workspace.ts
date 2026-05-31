export interface Tab {
    path: string;
    name: string;
    active: boolean;
}

let openTabs: Tab[] = [];
let onTabSwitch: (path: string) => void = () => { };

export function setOnTabSwitch(cb: (path: string) => void) {
    onTabSwitch = cb;
}

export function addTab(path: string, name: string) {
    openTabs.forEach(t => t.active = false);
    const existing = openTabs.find(t => t.path === path);
    if (existing) {
        existing.active = true;
    } else {
        openTabs.push({ path, name, active: true });
    }
    renderTabs();
}

export function renderTabs() {
    const tabsRow = document.querySelector(".tabs-row");
    if (!tabsRow) return;

    tabsRow.innerHTML = "";
    openTabs.forEach(tab => {
        const div = document.createElement("div");
        div.className = `tab ${tab.active ? 'active' : ''}`;
        div.innerHTML = `
            <span class="tab-label">${tab.name}</span>
            <i class="codicon codicon-close tab-close"></i>
        `;
        div.onclick = () => switchTab(tab.path);

        const closeBtn = div.querySelector(".tab-close") as HTMLElement;
        closeBtn.onclick = (e) => {
            e.stopPropagation();
            closeTab(tab.path);
        };

        tabsRow.appendChild(div);
    });
}

export function switchTab(path: string) {
    openTabs.forEach(t => t.active = (t.path === path));
    renderTabs();
    if (onTabSwitch) onTabSwitch(path);
}

export function closeTab(path: string) {
    openTabs = openTabs.filter(t => t.path !== path);
    if (openTabs.length > 0 && !openTabs.find(t => t.active)) {
        openTabs[openTabs.length - 1].active = true;
    }
    renderTabs();
}
