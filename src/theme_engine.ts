import { loader } from '@monaco-editor/react';
import { invoke } from './tauri_bridge';

export interface VscodeTheme {
    id: string;
    label: string;
    path: string;
    uiTheme: string;
    extensionName: string;
}

export async function getThemes(): Promise<VscodeTheme[]> {
    return await invoke<VscodeTheme[]>('get_installed_themes');
}

export async function applyTheme(themePath: string) {
    try {
        const themeJson = await invoke<any>('load_extension_theme', { path: themePath });
        const colors = themeJson.colors || {};
        const tokenColors = themeJson.tokenColors || [];

        // Map VS Code color keys to our CSS variables
        const root = document.documentElement;
        for (const [key, value] of Object.entries(colors)) {
            if (typeof value === 'string') {
                const cssVar = `--vscode-${key.replace(/\./g, '-')}`;
                root.style.setProperty(cssVar, value);

                // Also provide RGB version for alpha transparency in CSS
                if (value.startsWith('#')) {
                    const r = parseInt(value.slice(1, 3), 16);
                    const g = parseInt(value.slice(3, 5), 16);
                    const b = parseInt(value.slice(5, 7), 16);
                    if (!isNaN(r) && !isNaN(g) && !isNaN(b)) {
                        root.style.setProperty(`${cssVar}-rgb`, `${r}, ${g}, ${b}`);
                    }
                }
            }
        }

        const bg = colors['editor.background'] || '#1e1e1e';
        const fg = colors['editor.foreground'] || '#d4d4d4';
        const sidebarBg = colors['sideBar.background'] || bg;
        const isDark = isColorDark(bg);

        // --- Comprehensive fallback generation ---
        // Derive missing colors from the editor/sidebar backgrounds to prevent unstyled surfaces.
        const fallbacks: Record<string, string> = {
            'sideBar.background': sidebarBg,
            'sideBar.foreground': colors['sideBar.foreground'] || fg,
            'activityBar.background': colors['activityBar.background'] || adjustBrightness(bg, isDark ? 10 : -10),
            'activityBar.foreground': colors['activityBar.foreground'] || fg,
            'panel.background': colors['panel.background'] || bg,
            'panel.border': colors['panel.border'] || adjustBrightness(bg, isDark ? 8 : -8),
            'titleBar.activeBackground': colors['titleBar.activeBackground'] || bg,
            'titleBar.activeForeground': colors['titleBar.activeForeground'] || fg,
            'editorGroupHeader.tabsBackground': colors['editorGroupHeader.tabsBackground'] || sidebarBg,
            'tab.activeBackground': colors['tab.activeBackground'] || bg,
            'tab.activeForeground': colors['tab.activeForeground'] || fg,
            'tab.inactiveBackground': colors['tab.inactiveBackground'] || adjustBrightness(bg, isDark ? 6 : -6),
            'tab.inactiveForeground': colors['tab.inactiveForeground'] || (isDark ? 'rgba(255,255,255,0.5)' : 'rgba(0,0,0,0.5)'),
            'tab.border': colors['tab.border'] || sidebarBg,
            'list.hoverBackground': colors['list.hoverBackground'] || adjustBrightness(bg, isDark ? 8 : -6),
            'list.activeSelectionBackground': colors['list.activeSelectionBackground'] || (isDark ? '#094771' : '#0060c0'),
            'list.activeSelectionForeground': colors['list.activeSelectionForeground'] || '#ffffff',
            'list.inactiveSelectionBackground': colors['list.inactiveSelectionBackground'] || adjustBrightness(bg, isDark ? 14 : -10),
            'input.background': colors['input.background'] || adjustBrightness(bg, isDark ? 12 : -8),
            'input.foreground': colors['input.foreground'] || fg,
            'input.placeholderForeground': colors['input.placeholderForeground'] || (isDark ? '#a6a6a6' : '#767676'),
            'focusBorder': colors['focusBorder'] || '#007acc',
            'menu.background': colors['menu.background'] || sidebarBg,
            'menu.foreground': colors['menu.foreground'] || fg,
            'menu.border': colors['menu.border'] || adjustBrightness(sidebarBg, isDark ? 15 : -15),
            'menu.selectionBackground': colors['menu.selectionBackground'] || (isDark ? '#094771' : '#0060c0'),
            'menu.selectionForeground': colors['menu.selectionForeground'] || '#ffffff',
            'toolbar.hoverBackground': colors['toolbar.hoverBackground'] || adjustBrightness(bg, isDark ? 14 : -10),
            'button.background': colors['button.background'] || '#0e639c',
            'button.foreground': colors['button.foreground'] || '#ffffff',
            'button.hoverBackground': colors['button.hoverBackground'] || '#1177bb',
            'debugToolBar.background': colors['debugToolBar.background'] || sidebarBg,
            'debugToolBar.border': colors['debugToolBar.border'] || adjustBrightness(sidebarBg, isDark ? 15 : -15),
            'statusBar.background': colors['statusBar.background'] || '#007acc',
            'statusBar.foreground': colors['statusBar.foreground'] || '#ffffff',
            'terminal.background': colors['terminal.background'] || sidebarBg,
            'terminal.foreground': colors['terminal.foreground'] || fg,
            'sideBarSectionHeader.background': colors['sideBarSectionHeader.background'] || sidebarBg,
            'sideBarSectionHeader.foreground': colors['sideBarSectionHeader.foreground'] || fg,
            'sideBar.border': colors['sideBar.border'] || adjustBrightness(sidebarBg, isDark ? 15 : -15),
            'editorGroupHeader.noTabsBackground': colors['editorGroupHeader.noTabsBackground'] || sidebarBg,
            'tab.activeBorder': colors['tab.activeBorder'] || colors['focusBorder'] || '#007acc',
        };

        for (const [key, value] of Object.entries(fallbacks)) {
            const cssVar = `--vscode-${key.replace(/\./g, '-')}`;
            // Only set if the theme didn't already provide it
            if (!colors[key]) {
                root.style.setProperty(cssVar, value);
            }
        }

        root.style.setProperty('--vscode-is-dark', isDark ? 'true' : 'false');
        document.body.setAttribute('data-vscode-theme-kind', isDark ? 'vscode-dark' : 'vscode-light');

        // Transform tokenColors to Monaco rules
        const rules: any[] = [];
        tokenColors.forEach((tc: any) => {
            if (!tc.settings) return;
            const scopes = Array.isArray(tc.scope) ? tc.scope : (tc.scope ? tc.scope.split(',') : []);
            scopes.forEach((scope: string) => {
                const rule: any = { token: scope.trim() };
                if (tc.settings.foreground) rule.foreground = tc.settings.foreground.replace('#', '');
                if (tc.settings.fontStyle) {
                    if (tc.settings.fontStyle.includes('italic')) rule.fontStyle = 'italic';
                    if (tc.settings.fontStyle.includes('bold')) rule.fontStyle = (rule.fontStyle || '') + ' bold';
                }
                rules.push(rule);
            });
        });

        // Map standard Monaco tokens if not covered (best effort mapping)
        const monacoThemeName = `vscode-theme-${themePath.replace(/[/\\:.]/g, '-')}`;

        console.log(`[ThemeEngine] Registering Monaco theme: ${monacoThemeName} (isDark: ${isDark}, bg: ${bg})`);

        const monaco = await loader.init();

        // applyDokiAssets if applicable
        const isDoki = themeJson.name && (themeJson.name.includes('Doki') || themeJson.name.match(/^[A-Z][a-z]+/));

        // Ensure we explicitly set the background color in the theme definition to match our variable
        monaco.editor.defineTheme(monacoThemeName as any, {
            base: (isDark || isDoki) ? 'vs-dark' : 'vs', // Doki themes are typically dark or custom
            inherit: true,
            rules: rules,
            colors: {
                ...colors,
                'editor.background': (isDoki || isDark) ? '#00000000' : colors['editor.background'], // Force transparency for themed backgrounds
            }
        });

        console.log(`[ThemeEngine] Registered theme ${monacoThemeName} successfully.`);


        if (isDoki) {
            applyDokiAssets(themeJson.name);
        } else {
            // Clear assets if switching to a non-Doki theme
            const root = document.documentElement;
            root.style.setProperty('--airi-sticker', 'none');
            root.style.setProperty('--airi-wallpaper', 'none');
            document.body.classList.remove('has-airi-wallpaper');
        }

        // Persist theme choice
        localStorage.setItem('active-theme-path', themePath);
        localStorage.setItem('active-monaco-theme', monacoThemeName);

        return monacoThemeName;
    } catch (e) {
        console.error("Failed to apply theme:", e);
        return 'vs-dark';
    }
}

async function applyDokiAssets(themeName: string) {
    // Doki Theme Asset CDN Base URLs
    const STICKER_BASE = 'https://doki.assets.unthrottled.io/stickers/vscode/v2';
    const WALLPAPER_BASE = 'https://doki.assets.unthrottled.io/backgrounds/wallpapers/transparent';

    // This is a simplified mapper. In a full implementation, we'd use the DokiThemeDefinitions.js
    // but for now we'll match some popular ones and provide a mechanism to expand.
    const themeToAssetPath: Record<string, { sticker: string, wallpaper: string }> = {
        'Essex': { sticker: '/azurLane/essex/dark/essex_dark.png', wallpaper: '/essex_dark.png' },
        'Maika': { sticker: '/blendS/maika/dark/maika_dark.png', wallpaper: '/maika_dark.png' },
        'Mai Dark': { sticker: '/bunnySenpai/mai/dark/mai_dark.png', wallpaper: '/mai_dark.png' },
        'Makise Kurisu': { sticker: '/steinsGate/kurisu/dark/kurisu_dark.png', wallpaper: '/kurisu_dark.png' },
        'Zero Two Dark': { sticker: '/franxx/zeroTwo/dark/zero_two_dark.png', wallpaper: '/zero_two_dark.png' },
        'Rias Crimson': { sticker: '/highSchoolDxD/rias/dark/rias_dark.png', wallpaper: '/rias_dark.png' },
        'Rem': { sticker: '/reZero/rem/dark/rem_dark.png', wallpaper: '/rem_dark.png' },
        'Ram': { sticker: '/reZero/ram/dark/ram_dark.png', wallpaper: '/ram_dark.png' },
        'Emilia': { sticker: '/reZero/emilia/dark/emilia_dark.png', wallpaper: '/emilia_dark.png' },
        'Beatrice': { sticker: '/reZero/beatrice/dark/beatrice_dark.png', wallpaper: '/beatrice_dark.png' },
        'Megumin': { sticker: '/konosuba/megumin/dark/megumin_dark.png', wallpaper: '/megumin_dark.png' },
        'Aqua': { sticker: '/konosuba/aqua/dark/aqua_dark.png', wallpaper: '/aqua_dark.png' },
        'Darkness': { sticker: '/konosuba/darkness/dark/darkness_dark.png', wallpaper: '/darkness_dark.png' },
        'Tohru': { sticker: '/maidDragon/tohru/dark/tohru_dark.png', wallpaper: '/tohru_dark.png' },
        'Kanna': { sticker: '/maidDragon/kanna/dark/kanna_dark.png', wallpaper: '/kanna_dark.png' },
        'Rin': { sticker: '/fate/rin/dark/rin_dark.png', wallpaper: '/rin_dark.png' },
        'Saber': { sticker: '/fate/saber/dark/saber_dark.png', wallpaper: '/saber_dark.png' },
        'Monika': { sticker: '/ddlc/monika/dark/monika_dark.png', wallpaper: '/monika_dark.png' },
        'Sayori': { sticker: '/ddlc/sayori/dark/sayori_dark.png', wallpaper: '/sayori_dark.png' },
        'Natsuki': { sticker: '/ddlc/natsuki/dark/natsuki_dark.png', wallpaper: '/natsuki_dark.png' },
        'Yuri': { sticker: '/ddlc/yuri/dark/yuri_dark.png', wallpaper: '/yuri_dark.png' },
    };

    // Clean theme name (remove known suffixes for better matching)
    const cleanName = themeName
        .replace('Doki Theme ', '')
        .replace(' (Wallpaper)', '')
        .replace(' (Sticker)', '')
        .replace(/ \((Dark|Light)\)$/, '')
        .trim();

    // Fuzzy matching to find the closest asset key
    const assetKey = Object.keys(themeToAssetPath).find(k => k === cleanName || cleanName.includes(k));
    const assets = assetKey ? themeToAssetPath[assetKey] : null;

    const root = document.documentElement;
    if (assets) {
        root.style.setProperty('--airi-sticker', `url('${STICKER_BASE}${assets.sticker}')`);
        root.style.setProperty('--airi-wallpaper', `url('${WALLPAPER_BASE}${assets.wallpaper}')`);
        document.body.classList.add('has-airi-wallpaper');
    } else {
        // Fallback or Clear
        root.style.setProperty('--airi-sticker', 'none');
        root.style.setProperty('--airi-wallpaper', 'none');
        document.body.classList.remove('has-airi-wallpaper');
    }
}

export async function initTheme() {
    const themePath = localStorage.getItem('active-theme-path');
    if (themePath) {
        return await applyTheme(themePath);
    }
    return 'vs-dark';
}

/** Shift a hex color's brightness by ±amount (0–255 range). */
function adjustBrightness(hex: string, amount: number): string {
    if (!hex || hex[0] !== '#') return hex;
    let r: number, g: number, b: number;
    if (hex.length <= 5) {
        r = parseInt(hex[1] + hex[1], 16);
        g = parseInt(hex[2] + hex[2], 16);
        b = parseInt(hex[3] + hex[3], 16);
    } else {
        r = parseInt(hex.slice(1, 3), 16);
        g = parseInt(hex.slice(3, 5), 16);
        b = parseInt(hex.slice(5, 7), 16);
    }
    r = Math.max(0, Math.min(255, r + amount));
    g = Math.max(0, Math.min(255, g + amount));
    b = Math.max(0, Math.min(255, b + amount));
    return `#${r.toString(16).padStart(2, '0')}${g.toString(16).padStart(2, '0')}${b.toString(16).padStart(2, '0')}`;
}

function isColorDark(hex: string): boolean {
    if (!hex) return true;

    // Fast check for known safe defaults or keywords (sometimes theme JSON can be weird)
    const lower = hex.toLowerCase();
    if (lower === 'transparent' || lower === 'inherit') return true;

    // Handle rgba or rgb
    if (hex.startsWith('rgb')) {
        const match = hex.match(/\d+/g);
        if (match && match.length >= 3) {
            const [r, g, b] = match.map(Number);
            const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
            return luminance < 0.6; // Slightly more aggressive on dark detection
        }
        return true;
    }

    if (hex[0] !== '#') return true;

    // Handle #RGB
    if (hex.length === 4) {
        const r = parseInt(hex[1] + hex[1], 16);
        const g = parseInt(hex[2] + hex[2], 16);
        const b = parseInt(hex[3] + hex[3], 16);
        const brightness = (r * 299 + g * 587 + b * 114) / 1000;
        return brightness < 150; // Threshold for dark detection
    }

    // Handle #RRGGBB or #RRGGBBAA
    const r = parseInt(hex.slice(1, 3), 16);
    const g = parseInt(hex.slice(3, 5), 16);
    const b = parseInt(hex.slice(5, 7), 16);
    const brightness = (r * 299 + g * 587 + b * 114) / 1000;
    return brightness < 150;
}
