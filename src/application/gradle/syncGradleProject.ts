import { gradleRepository } from '../../infrastructure/gradle/TauriGradleRepository';

export const detectGradleProject = (root: string) => gradleRepository.detectProject(root);
export const syncGradleProject = (root: string) => gradleRepository.syncProject(root);
export const runGradleTask = (root: string, task: string) => gradleRepository.runTask(root, task);
