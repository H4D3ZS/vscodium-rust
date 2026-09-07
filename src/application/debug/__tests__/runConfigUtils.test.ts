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

    it('substitutes the full VS Code variable set', () => {
        const ctx = {
            workspaceFolder: 'C:/proj',
            file: 'C:/proj/src/app/main.ts',
            lineNumber: 42,
            selectedText: 'foo',
        };
        expect(substituteVars('${workspaceFolder}/out', ctx)).toBe('C:/proj/out');
        expect(substituteVars('${workspaceFolderBasename}', ctx)).toBe('proj');
        expect(substituteVars('${fileBasename}', ctx)).toBe('main.ts');
        expect(substituteVars('${fileBasenameNoExtension}', ctx)).toBe('main');
        expect(substituteVars('${fileExtname}', ctx)).toBe('.ts');
        expect(substituteVars('${fileDirname}', ctx)).toBe('C:/proj/src/app');
        expect(substituteVars('${relativeFile}', ctx)).toBe('src/app/main.ts');
        expect(substituteVars('${relativeFileDirname}', ctx)).toBe('src/app');
        expect(substituteVars('${lineNumber}', ctx)).toBe('42');
        expect(substituteVars('${selectedText}', ctx)).toBe('foo');
        // unknown + unresolvable tokens pass through
        expect(substituteVars('${command:foo.bar}', ctx)).toBe('${command:foo.bar}');
        expect(substituteVars('${notAThing}', ctx)).toBe('${notAThing}');
    });

    it('resolves ${env:NAME}', () => {
        process.env.RC_TEST_VAR = 'hello';
        expect(substituteVars('x-${env:RC_TEST_VAR}-y', {})).toBe('x-hello-y');
        expect(substituteVars('${env:RC_MISSING_VAR}', {})).toBe('');
        delete process.env.RC_TEST_VAR;
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
