// Shared pure utilities used by slices. No Zustand imports here.

function ollamaShouldInferScheme(s: string): boolean {
    const head = (s.split('/')[0].split('?')[0] || '').replace(/^\/+/, '');
    if (!head) return false;
    const lower = head.toLowerCase();
    if (lower.startsWith('localhost')) return true;
    const first = head.charCodeAt(0);
    if ((first >= 48 && first <= 57) || head.startsWith('[')) return head.includes('.') || head.includes(':');
    if (!head.includes('.')) return false;
    const labels = head.split('.').filter(Boolean);
    if (labels.length < 2) return false;
    const tld = labels[labels.length - 1] || '';
    return tld.length >= 2;
}

export function normalizeOllamaUrl(raw: string): string {
    const s = raw.trim().replace(/\/+$/, '');
    if (!s) return 'http://127.0.0.1:11434';
    if (/^https?:\/\//i.test(s)) return s;
    if (s.startsWith('//')) return `https:${s}`.replace(/\/+$/, '');
    if (!ollamaShouldInferScheme(s)) return s;
    const hostish = s.replace(/^\/+/, '');
    const lower = hostish.toLowerCase();
    const useHttp =
        lower.startsWith('localhost') ||
        lower.startsWith('127.') ||
        lower.startsWith('0.0.0.0') ||
        lower.startsWith('[::1]') ||
        /^192\.168\.\d+\.\d+/.test(lower) ||
        /^10\.\d+\.\d+\.\d+/.test(lower) ||
        /^172\.(1[6-9]|2\d|3[0-1])\.\d+\.\d+/.test(lower);
    return `${useHttp ? 'http' : 'https'}://${hostish}`.replace(/\/+$/, '');
}
