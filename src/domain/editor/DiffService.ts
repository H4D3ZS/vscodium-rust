import * as diff from 'diff';

export interface DiffBlock {
    id: string;
    type: 'added' | 'removed' | 'modified';
    oldStartLine: number;
    oldEndLine: number;
    newStartLine: number;
    newEndLine: number;
    oldContent: string;
    newContent: string;
}

function hashContent(value: string): string {
    let hash = 5381;
    for (let i = 0; i < value.length; i++) {
        hash = ((hash << 5) + hash) ^ value.charCodeAt(i);
    }
    return (hash >>> 0).toString(36);
}

function createDiffBlockId(block: Omit<DiffBlock, 'id'>): string {
    return [
        block.type,
        `${block.oldStartLine}-${block.oldEndLine}`,
        `${block.newStartLine}-${block.newEndLine}`,
        hashContent(block.oldContent),
        hashContent(block.newContent),
    ].join(':');
}

export const computeDiffBlocks = (oldContent: string, newContent: string): DiffBlock[] => {
    const changes = diff.diffLines(oldContent, newContent);
    const hunks: DiffBlock[] = [];
    
    let oldLine = 1;
    let newLine = 1;
    
    let currentHunk: Partial<DiffBlock> | null = null;
    
    const flushHunk = () => {
        if (currentHunk) {
            const block = {
                type: currentHunk.type || 'modified',
                oldStartLine: currentHunk.oldStartLine!,
                oldEndLine: currentHunk.oldEndLine!,
                newStartLine: currentHunk.newStartLine!,
                newEndLine: currentHunk.newEndLine!,
                oldContent: currentHunk.oldContent || '',
                newContent: currentHunk.newContent || '',
            };
            hunks.push({
                id: createDiffBlockId(block),
                ...block,
            });
            currentHunk = null;
        }
    };

    for (const change of changes) {
        if (change.added || change.removed) {
            if (!currentHunk) {
                currentHunk = {
                    type: change.added ? 'added' : 'removed',
                    oldStartLine: oldLine,
                    oldEndLine: oldLine - 1,
                    newStartLine: newLine,
                    newEndLine: newLine - 1,
                    oldContent: '',
                    newContent: '',
                };
            } else {
                currentHunk.type = 'modified';
            }
            
            if (change.added) {
                currentHunk.newEndLine = (currentHunk.newEndLine || newLine - 1) + (change.count || 0);
                currentHunk.newContent += change.value;
                newLine += (change.count || 0);
            } else {
                currentHunk.oldEndLine = (currentHunk.oldEndLine || oldLine - 1) + (change.count || 0);
                currentHunk.oldContent += change.value;
                oldLine += (change.count || 0);
            }
        } else {
            flushHunk();
            oldLine += (change.count || 0);
            newLine += (change.count || 0);
        }
    }
    
    flushHunk();
    return hunks;
};

/**
 * Reconstructs content by applying only those changes that haven't been rejected.
 */
export function patchContentSelective(original: string, proposed: string, rejectedHunkIds: string[]): string {
    const originalLines = original.split('\n');
    const proposedLines = proposed.split('\n');
    
    // Let's re-run the hunk grouping logic but emit lines selectively.
    let resultLines: string[] = [];
    
    const hunks = computeDiffBlocks(original, proposed);
    
    // Sort hunks by their position in PROPOSED content
    hunks.sort((a, b) => a.newStartLine - b.newStartLine);
    
    let lastProposedLineHandled = 0;
    
    for (const hunk of hunks) {
        // Add lines from proposed that are BEFORE this hunk
        while (lastProposedLineHandled < hunk.newStartLine - 1) {
            resultLines.push(proposedLines[lastProposedLineHandled]);
            lastProposedLineHandled++;
        }
        
        if (rejectedHunkIds.includes(hunk.id)) {
            // Use original lines for this hunk
            for (let i = hunk.oldStartLine - 1; i < hunk.oldEndLine; i++) {
                if (i < originalLines.length) {
                    resultLines.push(originalLines[i]);
                }
            }
        } else {
            // Use proposed lines for this hunk
            for (let i = hunk.newStartLine - 1; i < hunk.newEndLine; i++) {
                if (i < proposedLines.length) {
                    resultLines.push(proposedLines[i]);
                }
            }
        }
        lastProposedLineHandled = hunk.newEndLine;
    }
    
    // Add remaining proposed lines
    while (lastProposedLineHandled < proposedLines.length) {
        resultLines.push(proposedLines[lastProposedLineHandled]);
        lastProposedLineHandled++;
    }
    
    return resultLines.join('\n');
}

/** Apply only explicitly accepted hunks; unlisted hunks keep original content. */
export function patchContentAccepted(
    original: string,
    proposed: string,
    acceptedHunkIds: string[],
): string {
    const hunks = computeDiffBlocks(original, proposed);
    const rejected = hunks.filter((h) => !acceptedHunkIds.includes(h.id)).map((h) => h.id);
    return patchContentSelective(original, proposed, rejected);
}
