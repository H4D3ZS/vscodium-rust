import React, { useCallback, useEffect, useState } from 'react';
import { useStore } from '../../store';
import { detectGradleProject, runGradleTask, syncGradleProject } from '../../application/gradle/syncGradleProject';
import type { GradleProject, GradleTask } from '../../domain/gradle/IGradleRepository';

const GradleToolsPanel: React.FC = () => {
    const activeRoot = useStore((s) => s.activeRoot);
    const [project, setProject] = useState<GradleProject | null>(null);
    const [busy, setBusy] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [lastOutput, setLastOutput] = useState('');

    const refresh = useCallback(async () => {
        if (!activeRoot) return;
        setBusy(true);
        setError(null);
        try {
            const p = await syncGradleProject(activeRoot);
            setProject(p);
        } catch (e) {
            try {
                setProject(await detectGradleProject(activeRoot));
            } catch (e2) {
                setError(String(e2));
                setProject(null);
            }
        } finally {
            setBusy(false);
        }
    }, [activeRoot]);

    useEffect(() => {
        void refresh();
    }, [refresh]);

    const onRun = async (task: GradleTask) => {
        if (!activeRoot) return;
        setBusy(true);
        setError(null);
        try {
            const out = await runGradleTask(activeRoot, task.name);
            setLastOutput(out);
            useStore.getState().setActivePanelTab('OUTPUT');
        } catch (e) {
            setError(String(e));
        } finally {
            setBusy(false);
        }
    };

    if (!activeRoot) {
        return <p style={{ padding: 12, opacity: 0.6 }}>Open a folder to sync Gradle.</p>;
    }

    const androidTasks = (project?.tasks ?? []).filter((t) =>
        t.name.includes('assemble') || t.name.includes('install') || t.name.includes('test') || t.group?.toLowerCase().includes('build'),
    ).slice(0, 40);

    return (
        <div style={{ padding: 12, fontSize: 12, lineHeight: 1.5 }}>
            <div style={{ display: 'flex', gap: 8, marginBottom: 10, flexWrap: 'wrap' }}>
                <button type="button" disabled={busy} onClick={() => void refresh()}>Sync Gradle</button>
                {project?.is_android && <span style={{ opacity: 0.7 }}>Android project · {project.modules.length} modules</span>}
            </div>
            {error && <pre style={{ color: '#f85149', fontSize: 11 }}>{error}</pre>}
            {project && (
                <>
                    <div style={{ marginBottom: 8, opacity: 0.75 }}>
                        Wrapper: {project.wrapper_present ? 'yes' : 'missing'} · Kotlin: {project.uses_kotlin ? 'yes' : 'no'}
                    </div>
                    <div style={{ display: 'flex', flexDirection: 'column', gap: 4, maxHeight: 320, overflow: 'auto' }}>
                        {androidTasks.map((t) => (
                            <div key={t.name} style={{ display: 'flex', justifyContent: 'space-between', gap: 8, padding: '4px 6px', border: '1px solid rgba(255,255,255,0.06)', borderRadius: 4 }}>
                                <span>{t.name}</span>
                                <button type="button" disabled={busy} onClick={() => void onRun(t)}>Run</button>
                            </div>
                        ))}
                    </div>
                </>
            )}
            {lastOutput && (
                <pre style={{ marginTop: 10, fontSize: 10, maxHeight: 120, overflow: 'auto', opacity: 0.8 }}>{lastOutput.slice(-4000)}</pre>
            )}
        </div>
    );
};

export default GradleToolsPanel;
