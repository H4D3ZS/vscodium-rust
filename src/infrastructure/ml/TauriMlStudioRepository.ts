import { invoke } from '../../tauri_bridge';
import type {
    IMlStudioRepository,
    MlDatasetEntry,
    MlRunMetrics,
    MlRunSummary,
    MlStudioConfig,
} from '../../domain/ml/IMlStudioRepository';

export class TauriMlStudioRepository implements IMlStudioRepository {
    init(root: string) {
        return invoke<{ ok: boolean; path?: string }>('ml_studio_init', { root });
    }
    getConfig(root: string) {
        return invoke<MlStudioConfig>('ml_studio_get_config', { root });
    }
    saveConfig(root: string, config: MlStudioConfig) {
        return invoke<void>('ml_studio_save_config', { root, config });
    }
    listData(root: string) {
        return invoke<MlDatasetEntry[]>('ml_studio_list_data', { root });
    }
    prepareDataset(root: string, csvName: string, targetColumn: string, valRatio?: number) {
        return invoke<Record<string, unknown>>('ml_studio_prepare_dataset', {
            root, csvName, targetColumn, valRatio,
        });
    }
    train(root: string) {
        return invoke<{ ok: boolean; job_id?: string; run_id?: string }>('ml_studio_train', { root });
    }
    listRuns(root: string) {
        return invoke<MlRunSummary[]>('ml_studio_list_runs', { root });
    }
    infer(root: string, runId: string, input: Record<string, number>) {
        return invoke<Record<string, unknown>>('ml_studio_infer', { root, runId, input });
    }
    installDeps() {
        return invoke<{ ok: boolean; stdout?: string; stderr?: string }>('ml_studio_install_deps');
    }
    getRunMetrics(root: string, runId: string) {
        return invoke<MlRunMetrics>('ml_studio_get_run_metrics', { root, runId });
    }
    getActiveRun(root: string) {
        return invoke<string | null>('ml_studio_get_active_run', { root });
    }
    datasetStats(root: string, csvName: string, targetColumn?: string) {
        return invoke<Record<string, unknown>>('ml_studio_dataset_stats', { root, csvName, targetColumn });
    }
    modelSummary(root: string, runId: string) {
        return invoke<Record<string, unknown>>('ml_studio_model_summary', { root, runId });
    }
    exportModel(root: string, runId: string, format: string) {
        return invoke<Record<string, unknown>>('ml_studio_export_model', { root, runId, format });
    }
    pretrainedGallery(root: string) {
        return invoke<Record<string, unknown>>('ml_studio_pretrained_gallery', { root });
    }
    hpo(root: string, mode: string, trials: number) {
        return invoke<Record<string, unknown>>('ml_studio_hpo', { root, mode, trials });
    }
    lrFinder(root: string, steps: number) {
        return invoke<Record<string, unknown>>('ml_studio_lr_finder', { root, steps });
    }
    gradCheck(root: string, runId: string) {
        return invoke<Record<string, unknown>>('ml_studio_grad_check', { root, runId });
    }
    benchmark(root: string, runId: string, iterations: number) {
        return invoke<Record<string, unknown>>('ml_studio_benchmark', { root, runId, iterations });
    }
    listExperiments(root: string) {
        return invoke<Record<string, unknown>[]>('ml_studio_list_experiments', { root });
    }
    exportReport(root: string, runId: string) {
        return invoke<string>('ml_studio_export_report', { root, runId });
    }
}

export const mlStudioRepository = new TauriMlStudioRepository();
