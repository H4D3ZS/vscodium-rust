import { describe, expect, it } from 'vitest';
import {
    buildTaskCommand,
    substituteVars,
    tryParseJsonc,
} from '../runConfigUtils';

describe('runConfigUtils', () => {
    it('parses jsonc with line comments and trailing commas', () => {
        const raw = `{
            // launch config
            "version": "0.2.0",
            "configurations": [{ "name": "App", "type": "node", },],
        }`;
        const parsed = tryParseJsonc<{ configurations: { name: string }[] }>(raw);
        expect(parsed?.configurations[0].name).toBe('App');
    });

    it('substitutes workspace and file variables', () => {
        const ctx = {
            workspaceFolder: 'C:/proj',
            file: 'C:/proj/src/main.ts',
        };
        expect(substituteVars('${workspaceFolder}/out', ctx)).toBe('C:/proj/out');
        expect(substituteVars('${fileBasenameNoExtension}', ctx)).toBe('main');
    });

    it('builds task command with quoted args', () => {
        const cmd = buildTaskCommand(
            { command: 'npm', args: ['run', 'test --watch'] },
            { workspaceFolder: '/root' },
        );
        expect(cmd).toContain('npm');
        expect(cmd).toContain('"test --watch"');
    });
});
