import React, { useMemo } from 'react';
import { computeInlineDiff } from '../../domain/agent/inlineDiff';

interface InlineDiffPreviewProps {
    oldText?: string;
    newText?: string;
    path?: string;
    maxLines?: number;
}

const InlineDiffPreview: React.FC<InlineDiffPreviewProps> = ({ oldText, newText, path, maxLines = 28 }) => {
    const lines = useMemo(
        () => computeInlineDiff(oldText || '', newText || '', maxLines),
        [oldText, newText, maxLines],
    );

    if (!lines.length && !oldText && !newText) return null;

    const adds = lines.filter((l) => l.type === 'add').length;
    const dels = lines.filter((l) => l.type === 'del').length;

    return (
        <div className="composer-inline-diff">
            {(path || adds || dels) && (
                <div className="composer-inline-diff__meta">
                    {path && <span className="composer-inline-diff__path">{path.split(/[\\/]/).pop()}</span>}
                    <span className="composer-inline-diff__stats">
                        {adds > 0 && <span className="composer-inline-diff__add">+{adds}</span>}
                        {dels > 0 && <span className="composer-inline-diff__del">−{dels}</span>}
                    </span>
                </div>
            )}
            <div className="composer-inline-diff__body">
                {lines.map((line, idx) => {
                    const isFold = line.type === 'same' && line.text.startsWith('···');
                    if (isFold) {
                        return (
                            <div key={idx} className="composer-inline-diff__fold">{line.text}</div>
                        );
                    }
                    return (
                        <div key={idx} className={`composer-inline-diff__line composer-inline-diff__line--${line.type}`}>
                            <span className="composer-inline-diff__prefix">
                                {line.type === 'add' ? '+' : line.type === 'del' ? '−' : ' '}
                            </span>
                            <span className="composer-inline-diff__text">{line.text || ' '}</span>
                        </div>
                    );
                })}
            </div>
        </div>
    );
};

export default InlineDiffPreview;
