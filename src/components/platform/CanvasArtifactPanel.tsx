import React, { useEffect, useState } from 'react';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import { agBrainList, type BrainArtifactInfo } from '../../infrastructure/antigravity/antigravityClient';

/** Cursor-style canvas: render HTML/walkthrough artifacts from the agent brain. */
const CanvasArtifactPanel: React.FC = () => {
    const activeRoot = useStore((s) => s.activeRoot);
    const cascadeId = useStore((s) => s.activeCascadeId);
    const [artifacts, setArtifacts] = useState<BrainArtifactInfo[]>([]);

    useEffect(() => {
        if (!activeRoot || !cascadeId) {
            setArtifacts([]);
            return;
        }
        let cancelled = false;
        void (async () => {
            try {
                const rows = await agBrainList(activeRoot, cascadeId);
                if (!cancelled) setArtifacts(rows ?? []);
            } catch {
                if (!cancelled) setArtifacts([]);
            }
        })();
        return () => { cancelled = true; };
    }, [activeRoot, cascadeId]);

    const canvasArts = artifacts.filter((a) =>
        a.artifact_type.includes('WALKTHROUGH')
        || a.artifact_type.includes('HTML')
        || a.name.endsWith('.html')
        || a.name === 'walkthrough.md',
    );

    if (!activeRoot) {
        return <p className="afi-muted" style={{ fontSize: 12 }}>Open a project folder first.</p>;
    }
    if (!cascadeId) {
        return <p className="afi-muted" style={{ fontSize: 12 }}>Start an agent turn to populate canvas artifacts.</p>;
    }
    if (canvasArts.length === 0) {
        return <p className="afi-muted" style={{ fontSize: 12 }}>Walkthroughs and HTML artifacts from the current agent run appear here.</p>;
    }

    return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
            {canvasArts.slice(-3).map((a) => (
                <CanvasCard key={a.path} artifact={a} />
            ))}
        </div>
    );
};

const CanvasCard: React.FC<{ artifact: BrainArtifactInfo }> = ({ artifact }) => {
    const [html, setHtml] = useState<string>('');

    useEffect(() => {
        let cancelled = false;
        void (async () => {
            try {
                const text = await invoke<string>('read_file', { path: artifact.path });
                if (cancelled) return;
                if (artifact.name.endsWith('.html')) {
                    setHtml(text);
                } else {
                    setHtml(`<article style="font-family:system-ui;padding:12px;color:#ddd;line-height:1.5">${text.replace(/</g, '&lt;').replace(/\n/g, '<br/>')}</article>`);
                }
            } catch {
                if (!cancelled) setHtml(`<p>Could not load ${artifact.name}</p>`);
            }
        })();
        return () => { cancelled = true; };
    }, [artifact.path, artifact.name]);

    return (
        <div style={{ border: '1px solid rgba(255,255,255,0.08)', borderRadius: 8, overflow: 'hidden' }}>
            <div style={{ padding: '6px 10px', fontSize: 11, fontWeight: 600, background: 'rgba(238,76,44,0.12)' }}>
                {artifact.name}
            </div>
            <iframe
                title={artifact.name}
                sandbox="allow-same-origin"
                srcDoc={html}
                style={{ width: '100%', height: 240, border: 'none', background: '#1e1e1e' }}
            />
        </div>
    );
};

export default CanvasArtifactPanel;
