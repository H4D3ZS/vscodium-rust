import React, { useEffect, useMemo, useState } from 'react';
import type { MlRunMetrics } from '../../domain/ml/IMlStudioRepository';
import { getMlRunMetrics } from '../../application/ml/mlStudio';

const CHART_W = 420;
const CHART_H = 120;
const PAD = { t: 8, r: 8, b: 22, l: 36 };

type Series = { label: string; color: string; values: number[]; epochs: number[] };

const MiniLineChart: React.FC<{ series: Series[]; yLabel?: string }> = ({ series, yLabel }) => {
    const allY = series.flatMap((s) => s.values);
    const minY = Math.min(...allY, 0);
    const maxY = Math.max(...allY, minY + 1e-6);
    const maxEpoch = Math.max(...series.flatMap((s) => s.epochs), 1);

    const toX = (ep: number) =>
        PAD.l + ((ep - 1) / Math.max(maxEpoch - 1, 1)) * (CHART_W - PAD.l - PAD.r);
    const toY = (v: number) =>
        PAD.t + (1 - (v - minY) / (maxY - minY)) * (CHART_H - PAD.t - PAD.b);

    return (
        <svg width="100%" viewBox={`0 0 ${CHART_W} ${CHART_H}`} style={{ display: 'block', maxWidth: CHART_W }}>
            {[0, 0.5, 1].map((f) => {
                const y = PAD.t + f * (CHART_H - PAD.t - PAD.b);
                return (
                    <line
                        key={f}
                        x1={PAD.l}
                        x2={CHART_W - PAD.r}
                        y1={y}
                        y2={y}
                        stroke="rgba(255,255,255,0.08)"
                        strokeWidth={1}
                    />
                );
            })}
            {series.map((s) => {
                if (s.values.length < 2) return null;
                const d = s.epochs
                    .map((ep, i) => `${i === 0 ? 'M' : 'L'} ${toX(ep)} ${toY(s.values[i])}`)
                    .join(' ');
                return <path key={s.label} d={d} fill="none" stroke={s.color} strokeWidth={2} />;
            })}
            {yLabel && (
                <text x={4} y={PAD.t + 8} fill="rgba(255,255,255,0.45)" fontSize={9}>
                    {yLabel}
                </text>
            )}
        </svg>
    );
};

const Stat: React.FC<{ label: string; value: string; accent?: string }> = ({ label, value, accent }) => (
    <div
        style={{
            flex: '1 1 100px',
            padding: '8px 10px',
            borderRadius: 6,
            background: 'rgba(0,0,0,0.2)',
            border: '1px solid rgba(255,255,255,0.06)',
        }}
    >
        <div style={{ fontSize: 10, opacity: 0.55, marginBottom: 2 }}>{label}</div>
        <div style={{ fontSize: 14, fontWeight: 700, color: accent ?? 'inherit' }}>{value}</div>
    </div>
);

export interface TrainingDashboardProps {
    root: string;
    runId: string | null;
    pollMs?: number;
}

const TrainingDashboard: React.FC<TrainingDashboardProps> = ({ root, runId, pollMs = 2000 }) => {
    const [metrics, setMetrics] = useState<MlRunMetrics | null>(null);
    const [err, setErr] = useState<string | null>(null);

    useEffect(() => {
        if (!runId) {
            setMetrics(null);
            return;
        }
        let alive = true;
        const tick = async () => {
            try {
                const m = await getMlRunMetrics(root, runId);
                if (alive) {
                    setMetrics(m);
                    setErr(null);
                }
            } catch (e) {
                if (alive) setErr(e instanceof Error ? e.message : String(e));
            }
        };
        void tick();
        const id = setInterval(() => void tick(), pollMs);
        return () => {
            alive = false;
            clearInterval(id);
        };
    }, [root, runId, pollMs]);

    const history = metrics?.history ?? [];
    const last = history[history.length - 1];
    const isTraining = metrics?.status === 'training';

    const lossSeries = useMemo(
        (): Series[] => [
            {
                label: 'train',
                color: '#60a5fa',
                epochs: history.map((h) => h.epoch),
                values: history.map((h) => h.train_loss),
            },
            {
                label: 'val',
                color: '#ee4c2c',
                epochs: history.map((h) => h.epoch),
                values: history.map((h) => h.val_loss),
            },
        ],
        [history],
    );

    const accSeries = useMemo(
        (): Series[] => [
            {
                label: 'val_acc',
                color: '#4ade80',
                epochs: history.map((h) => h.epoch),
                values: history.map((h) => h.val_acc),
            },
        ],
        [history],
    );

    if (!runId) {
        return <p className="afi-muted">Start a training job to see live metrics.</p>;
    }

    if (!metrics && !err) {
        return <p className="afi-muted">Loading metrics for {runId}…</p>;
    }

    const progress =
        metrics && metrics.total_epochs > 0
            ? Math.round(((metrics.current_epoch ?? 0) / metrics.total_epochs) * 100)
            : 0;

    return (
        <div className="ml-training-dashboard">
            {err && <div style={{ color: '#ff8b80', fontSize: 12, marginBottom: 8 }}>{err}</div>}

            <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 10, flexWrap: 'wrap' }}>
                <span
                    style={{
                        fontSize: 10,
                        fontWeight: 700,
                        textTransform: 'uppercase',
                        padding: '3px 8px',
                        borderRadius: 4,
                        background: isTraining ? 'rgba(238,76,44,0.2)' : 'rgba(74,222,128,0.15)',
                        color: isTraining ? '#ee4c2c' : '#4ade80',
                    }}
                >
                    {metrics?.status ?? 'unknown'}
                </span>
                <code style={{ fontSize: 11 }}>{runId}</code>
                {metrics?.early_stop && (
                    <span style={{ fontSize: 10, color: '#fbbf24' }}>Early stopped @ epoch {metrics.best_epoch}</span>
                )}
            </div>

            <div style={{ height: 6, background: 'rgba(255,255,255,0.08)', borderRadius: 3, marginBottom: 12, overflow: 'hidden' }}>
                <div
                    style={{
                        height: '100%',
                        width: `${progress}%`,
                        background: 'linear-gradient(90deg,#ee4c2c,#f97316)',
                        transition: 'width 0.4s ease',
                    }}
                />
            </div>
            <div style={{ fontSize: 10, opacity: 0.5, marginBottom: 10 }}>
                Epoch {metrics?.current_epoch ?? 0} / {metrics?.total_epochs ?? '?'}
                {metrics?.stale_epochs != null && metrics.early_stop_patience != null && isTraining && (
                    <> · patience {metrics.stale_epochs}/{metrics.early_stop_patience}</>
                )}
            </div>

            <div style={{ display: 'flex', flexWrap: 'wrap', gap: 8, marginBottom: 14 }}>
                <Stat label="Train loss" value={last ? last.train_loss.toFixed(4) : '—'} />
                <Stat label="Val loss" value={last ? last.val_loss.toFixed(4) : '—'} accent="#ee4c2c" />
                <Stat label="Val accuracy" value={last ? `${(last.val_acc * 100).toFixed(1)}%` : '—'} accent="#4ade80" />
                <Stat label="Learning rate" value={last ? last.lr.toExponential(2) : metrics?.lr?.toExponential(2) ?? '—'} />
                <Stat label="Speed" value={last ? `${last.samples_per_sec} samp/s` : '—'} />
                <Stat
                    label="GPU memory"
                    value={last?.gpu_mem_mb != null ? `${last.gpu_mem_mb} MB` : metrics?.device?.includes('cpu') ? 'CPU' : '—'}
                />
            </div>

            {history.length > 0 && (
                <>
                    <div style={{ fontWeight: 600, fontSize: 12, marginBottom: 4 }}>Loss curves</div>
                    <MiniLineChart series={lossSeries} yLabel="loss" />
                    <div style={{ fontWeight: 600, fontSize: 12, margin: '12px 0 4px' }}>Validation accuracy</div>
                    <MiniLineChart series={accSeries} yLabel="acc" />
                </>
            )}

            {metrics?.device && (
                <p className="afi-subtle" style={{ marginTop: 10, fontSize: 11 }}>
                    Device: {metrics.device}
                    {metrics.best_val_acc != null && <> · Best val acc {(metrics.best_val_acc * 100).toFixed(2)}%</>}
                </p>
            )}
        </div>
    );
};

export { MiniLineChart };
export default TrainingDashboard;
