import * as diff from 'diff';

export type InlineDiffLine = { type: 'add' | 'del' | 'same'; text: string };

const CONTEXT = 2;

function collapseSameRuns(lines: InlineDiffLine[], maxLines: number): InlineDiffLine[] {
    const out: InlineDiffLine[] = [];
    let sameBuf: InlineDiffLine[] = [];

    const flushSame = () => {
        if (sameBuf.length <= CONTEXT * 2) {
            out.push(...sameBuf);
        } else {
            out.push(...sameBuf.slice(0, CONTEXT));
            out.push({ type: 'same', text: `··· ${sameBuf.length - CONTEXT * 2} unchanged lines ···` });
            out.push(...sameBuf.slice(-CONTEXT));
        }
        sameBuf = [];
    };

    for (const line of lines) {
        if (line.type === 'same') {
            sameBuf.push(line);
        } else {
            flushSame();
            out.push(line);
        }
        if (out.length >= maxLines) break;
    }
    flushSame();
    return out.slice(0, maxLines);
}

/** Compact red/green diff for Composer-style edit cards. */
export function computeInlineDiff(oldText: string, newText: string, maxLines = 28): InlineDiffLine[] {
    const old = oldText || '';
    const neu = newText || '';
    if (!old && neu) {
        return neu.split('\n').slice(0, maxLines).map((text) => ({ type: 'add' as const, text }));
    }
    if (old && !neu) {
        return old.split('\n').slice(0, maxLines).map((text) => ({ type: 'del' as const, text }));
    }
    if (old === neu) return [];

    const changes = diff.diffLines(old, neu);
    const lines: InlineDiffLine[] = [];
    for (const change of changes) {
        const type: InlineDiffLine['type'] = change.added ? 'add' : change.removed ? 'del' : 'same';
        const raw = change.value.endsWith('\n') ? change.value.slice(0, -1) : change.value;
        if (raw === '' && change.value.endsWith('\n')) {
            lines.push({ type, text: '' });
            continue;
        }
        for (const text of raw.split('\n')) {
            lines.push({ type, text });
        }
    }
    return collapseSameRuns(lines, maxLines);
}
