export interface MlStudioConfig {
    epochs: number;
    learning_rate: number;
    hidden_size: number;
    val_ratio: number;
    embed_model: string;
}

export interface MlDatasetEntry {
    name: string;
    path: string;
    size_bytes: number;
    columns: string[];
}

export interface MlRunSummary {
    id: string;
    model_path?: string | null;
    metrics_path?: string | null;
    val_acc?: number | null;
    created_at: number;
}

export interface MlEpochMetric {
    epoch: number;
    train_loss: number;
    val_loss: number;
    val_acc: number;
    lr: number;
    samples_per_sec: number;
    gpu_mem_mb?: number | null;
    epoch_secs: number;
}

export interface MlRunMetrics {
    status: string;
    run_id: string;
    total_epochs: number;
    current_epoch?: number | null;
    lr?: number | null;
    device?: string | null;
    best_val_acc?: number | null;
    best_val_loss?: number | null;
    best_epoch?: number | null;
    stale_epochs?: number | null;
    early_stop_patience?: number | null;
    early_stop?: boolean | null;
    history: MlEpochMetric[];
    confusion_matrix?: { labels: string[]; matrix: number[][] };
}

export interface IMlStudioRepository {
    init(root: string): Promise<{ ok: boolean; path?: string }>;
    getConfig(root: string): Promise<MlStudioConfig>;
    saveConfig(root: string, config: MlStudioConfig): Promise<void>;
    listData(root: string): Promise<MlDatasetEntry[]>;
    prepareDataset(root: string, csvName: string, targetColumn: string, valRatio?: number): Promise<Record<string, unknown>>;
    train(root: string): Promise<{ ok: boolean; job_id?: string; run_id?: string }>;
    listRuns(root: string): Promise<MlRunSummary[]>;
    infer(root: string, runId: string, input: Record<string, number>): Promise<Record<string, unknown>>;
    installDeps(): Promise<{ ok: boolean; stdout?: string; stderr?: string }>;
    getRunMetrics(root: string, runId: string): Promise<MlRunMetrics>;
    getActiveRun(root: string): Promise<string | null>;
    datasetStats(root: string, csvName: string, targetColumn?: string): Promise<Record<string, unknown>>;
    modelSummary(root: string, runId: string): Promise<Record<string, unknown>>;
    exportModel(root: string, runId: string, format: string): Promise<Record<string, unknown>>;
    pretrainedGallery(root: string): Promise<Record<string, unknown>>;
    hpo(root: string, mode: string, trials: number): Promise<Record<string, unknown>>;
    lrFinder(root: string, steps: number): Promise<Record<string, unknown>>;
    gradCheck(root: string, runId: string): Promise<Record<string, unknown>>;
    benchmark(root: string, runId: string, iterations: number): Promise<Record<string, unknown>>;
    listExperiments(root: string): Promise<Record<string, unknown>[]>;
    exportReport(root: string, runId: string): Promise<string>;
}
