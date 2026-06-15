export type TestFramework = 'vitest' | 'jest' | 'bun' | 'cargo' | 'pytest' | 'go' | 'gradle' | 'unknown';

export interface TestCase {
    path: string;
    name: string;
    framework: TestFramework | string;
}

export interface TestRunResult {
    ok: boolean;
    exit_code?: number | null;
    stdout: string;
    stderr: string;
}

export interface ITestRepository {
    sniffFramework(root: string): Promise<TestFramework>;
    discover(root: string): Promise<TestCase[]>;
    runFile(root: string, path: string): Promise<TestRunResult>;
    runAll(root: string): Promise<TestRunResult>;
}
