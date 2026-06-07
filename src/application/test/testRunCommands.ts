import type { TestFramework } from '../../domain/test/ITestRepository';

/** Map framework → shell command for "run all" (mirrors Rust `run_all_command`). */
export function runAllCommand(framework: TestFramework): string {
    switch (framework) {
        case 'jest':   return 'npx jest';
        case 'vitest': return 'npx vitest run';
        case 'bun':    return 'bun test';
        case 'pytest': return 'pytest';
        case 'go':     return 'go test ./...';
        case 'cargo':  return 'cargo test';
        case 'gradle': return './gradlew test';
        default:       return 'npm test';
    }
}

export function formatTestOutput(result: {
    ok: boolean;
    stdout?: string;
    stderr?: string;
}): string {
    return [result.stdout, result.stderr].filter(Boolean).join('\n')
        || (result.ok ? 'Passed' : 'Failed');
}
