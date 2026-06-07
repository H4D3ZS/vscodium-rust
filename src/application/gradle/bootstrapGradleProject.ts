import { detectGradleProject, syncGradleProject } from './syncGradleProject';

/**
 * Auto-sync Gradle metadata when a workspace folder opens (Android Studio–style project import).
 * Non-blocking: failures are ignored when the folder is not a Gradle project.
 */
export async function bootstrapGradleProject(root: string): Promise<void> {
    if (!root?.trim()) return;
    try {
        const detected = await detectGradleProject(root);
        if (detected.wrapper_present || detected.is_android || detected.modules.length > 0) {
            await syncGradleProject(root);
        }
    } catch {
        /* not a Gradle workspace */
    }
}
