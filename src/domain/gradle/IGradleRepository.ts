export interface GradleModule {
    path: string;
    name: string;
}

export interface GradleTask {
    path: string;
    name: string;
    group?: string | null;
    description?: string | null;
}

export interface GradleProject {
    root: string;
    is_android: boolean;
    uses_kotlin: boolean;
    wrapper_present: boolean;
    modules: GradleModule[];
    tasks: GradleTask[];
}

export interface IGradleRepository {
    detectProject(root: string): Promise<GradleProject>;
    syncProject(root: string): Promise<GradleProject>;
    listTasks(root: string): Promise<GradleTask[]>;
    runTask(root: string, task: string): Promise<string>;
}
