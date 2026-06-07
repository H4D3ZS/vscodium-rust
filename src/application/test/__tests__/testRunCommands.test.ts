import { describe, expect, it } from 'vitest';
import { formatTestOutput, runAllCommand } from '../testRunCommands';

describe('testRunCommands', () => {
    it('maps frameworks to run-all shell commands', () => {
        expect(runAllCommand('vitest')).toBe('npx vitest run');
        expect(runAllCommand('gradle')).toBe('./gradlew test');
        expect(runAllCommand('cargo')).toBe('cargo test');
    });

    it('formats stdout/stderr from backend result', () => {
        expect(formatTestOutput({ ok: true, stdout: 'ok', stderr: '' })).toBe('ok');
        expect(formatTestOutput({ ok: false, stdout: '', stderr: '' })).toBe('Failed');
    });
});
