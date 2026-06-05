import { invoke } from '../../tauri_bridge';
import { spawnTerminalGroup } from '../terminal/spawnTerminal';

export interface StealthPageSummary {
    text?: string;
    links?: { text?: string; href?: string }[];
    headers?: string[];
}

/** Start the invisible_playwright stealth-Firefox sidecar (no-op if already running). */
export async function ensureStealthBrowser(): Promise<void> {
    await invoke<string>('browser_open').catch(() => {});
}

/** Navigate with invisible_playwright — JS runs, Cloudflare/challenge pages render. */
export async function navigateStealth(url: string): Promise<string> {
    await ensureStealthBrowser();
    return invoke<string>('browser_navigate', { url: url.trim() });
}

/** Read rendered page text + links from the live stealth browser session. */
export async function readStealthPageSummary(): Promise<StealthPageSummary> {
    return invoke<StealthPageSummary>('browser_get_content_summary');
}

/**
 * Scrape a URL through invisible_playwright (stealth Firefox).
 * Bypasses Cloudflare / bot gates that block plain HTTP fetch.
 */
export async function scrapeUrl(url: string): Promise<string> {
    await navigateStealth(url);
    const summary = await readStealthPageSummary();
    const html = await invoke<string>('browser_read_dom').catch(() => '');
    const text = summary.text?.trim() || '';
    if (text.length > 200) {
        const links = (summary.links || [])
            .slice(0, 12)
            .map(l => `- ${l.text || l.href}: ${l.href}`)
            .join('\n');
        return [text, links ? `\n### Links\n${links}` : ''].filter(Boolean).join('\n');
    }
    return html.slice(0, 12000);
}

/** DuckDuckGo JSON API — fast metadata; does not hit Cloudflare-protected pages. */
export async function searchWeb(query: string): Promise<string> {
    const results = await invoke<unknown>('web_search', { query });
    return typeof results === 'string' ? results : JSON.stringify(results, null, 2);
}

/**
 * Search via invisible_playwright (Google in real browser).
 * Use when target sites or chatbot gates block API/metadata fetch.
 */
export async function searchWebStealth(query: string): Promise<string> {
    const searchUrl = `https://www.google.com/search?q=${encodeURIComponent(query)}`;
    await navigateStealth(searchUrl);
    const summary = await readStealthPageSummary();
    const parts = [
        summary.text?.slice(0, 6000) || '',
        '',
        '### Result links',
        ...(summary.links || []).slice(0, 15).map(l => `- ${l.text || '(link)'}: ${l.href}`),
    ];
    return parts.join('\n');
}

export async function auditUrlSecurity(url: string): Promise<string> {
    const report = await invoke<unknown>('call_tool', {
        name: 'apex_scan_url',
        arguments: { url },
    });
    return typeof report === 'string' ? report : JSON.stringify(report, null, 2);
}

export async function openBrowserTo(url: string): Promise<void> {
    if (url.trim()) {
        await navigateStealth(url);
    } else {
        await ensureStealthBrowser();
    }
}

/** Capture a PNG screenshot from the live stealth browser (base64). */
export async function captureStealthScreenshot(): Promise<string> {
    await ensureStealthBrowser();
    return invoke<string>('browser_screenshot').catch(() => '');
}

/** Open bottom terminal panel for follow-up shell work during research. */
export async function openResearchTerminal(): Promise<string> {
    return spawnTerminalGroup();
}
