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
}
