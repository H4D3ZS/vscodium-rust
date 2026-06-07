import { invoke } from '../../tauri_bridge';
import type { IGradleRepository, GradleProject, GradleTask } from '../../domain/gradle/IGradleRepository';

export class TauriGradleRepository implements IGradleRepository {
    async detectProject(root: string): Promise<GradleProject> {
        return invoke<GradleProject>('gradle_detect_project', { root });
    }

    async syncProject(root: string): Promise<GradleProject> {
        return invoke<GradleProject>('gradle_sync_project', { root });
    }

    async listTasks(root: string): Promise<GradleTask[]> {
        const res = await invoke<{ tasks: GradleTask[] }>('gradle_list_tasks', { root });
        return res.tasks ?? [];
    }

    async runTask(root: string, task: string): Promise<string> {
        const res = await invoke<{ output: string }>('gradle_run_task', { root, task });
        return res.output ?? '';
    }
}

export const gradleRepository = new TauriGradleRepository();
