import { invoke } from '../../tauri_bridge';
import type {
    IMlStudioRepository,
    MlDatasetEntry,
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
            root,
            csvName,
            targetColumn,
            valRatio,
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
}

export const mlStudioRepository = new TauriMlStudioRepository();
