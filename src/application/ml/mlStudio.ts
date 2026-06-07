import { mlStudioRepository } from '../../infrastructure/ml/TauriMlStudioRepository';
import type { MlStudioConfig } from '../../domain/ml/IMlStudioRepository';

export const initMlStudio = (root: string) => mlStudioRepository.init(root);
export const getMlConfig = (root: string) => mlStudioRepository.getConfig(root);
export const saveMlConfig = (root: string, config: MlStudioConfig) => mlStudioRepository.saveConfig(root, config);
export const listMlData = (root: string) => mlStudioRepository.listData(root);
export const prepareMlDataset = (
    root: string, csvName: string, targetColumn: string, valRatio?: number,
) => mlStudioRepository.prepareDataset(root, csvName, targetColumn, valRatio);
export const trainMlModel = (root: string) => mlStudioRepository.train(root);
export const listMlRuns = (root: string) => mlStudioRepository.listRuns(root);
export const runMlInference = (root: string, runId: string, input: Record<string, number>) =>
    mlStudioRepository.infer(root, runId, input);
export const installMlDeps = () => mlStudioRepository.installDeps();
export const getMlRunMetrics = (root: string, runId: string) => mlStudioRepository.getRunMetrics(root, runId);
export const getMlActiveRun = (root: string) => mlStudioRepository.getActiveRun(root);
export const getMlDatasetStats = (root: string, csvName: string, targetColumn?: string) =>
    mlStudioRepository.datasetStats(root, csvName, targetColumn);
export const getMlModelSummary = (root: string, runId: string) => mlStudioRepository.modelSummary(root, runId);
export const exportMlModel = (root: string, runId: string, format: string) =>
    mlStudioRepository.exportModel(root, runId, format);
export const getMlPretrainedGallery = (root: string) => mlStudioRepository.pretrainedGallery(root);
export const runMlHpo = (root: string, mode: string, trials: number) => mlStudioRepository.hpo(root, mode, trials);
export const runMlLrFinder = (root: string, steps: number) => mlStudioRepository.lrFinder(root, steps);
export const runMlGradCheck = (root: string, runId: string) => mlStudioRepository.gradCheck(root, runId);
export const runMlBenchmark = (root: string, runId: string, iterations: number) =>
    mlStudioRepository.benchmark(root, runId, iterations);
export const listMlExperiments = (root: string) => mlStudioRepository.listExperiments(root);
export const exportMlReport = (root: string, runId: string) => mlStudioRepository.exportReport(root, runId);
