import { invoke } from '../../tauri_bridge';
import type { ITestRepository, TestCase, TestFramework, TestRunResult } from '../../domain/test/ITestRepository';

export class TauriTestRepository implements ITestRepository {
    async sniffFramework(root: string): Promise<TestFramework> {
        const res = await invoke<{ framework: TestFramework }>('test_sniff_framework', { root });
        return res.framework ?? 'unknown';
    }

    async discover(root: string): Promise<TestCase[]> {
        const res = await invoke<{ tests: TestCase[] }>('test_discover', { root });
        return res.tests ?? [];
    }

    async runFile(root: string, path: string): Promise<TestRunResult> {
        return invoke<TestRunResult>('test_run_file', { root, path });
    }

    async runAll(root: string): Promise<TestRunResult> {
        return invoke<TestRunResult>('test_run_all', { root });
    }
}

export const testRepository = new TauriTestRepository();
