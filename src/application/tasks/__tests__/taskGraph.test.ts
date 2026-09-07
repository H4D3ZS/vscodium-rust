import { describe, it, expect } from 'vitest';
import { resolveBuildTask, taskRunOrder, type VscTask } from '../taskGraph';

describe('resolveBuildTask', () => {
    it('prefers an explicit default build task', () => {
        const tasks: VscTask[] = [
            { label: 'lint', group: 'test' },
            { label: 'build', group: { kind: 'build', isDefault: true } },
            { label: 'watch', group: { kind: 'build' } },
        ];
        expect(resolveBuildTask(tasks)?.label).toBe('build');
    });

    it('falls back to the sole build-group task', () => {
        const tasks: VscTask[] = [
            { label: 'x', group: 'none' },
            { label: 'compile', group: 'build' },
        ];
        expect(resolveBuildTask(tasks)?.label).toBe('compile');
    });

    it('returns null when there is no build task', () => {
        expect(resolveBuildTask([{ label: 'x', group: 'test' }])).toBeNull();
        expect(resolveBuildTask([])).toBeNull();
    });
});

describe('taskRunOrder', () => {
    const all: VscTask[] = [
        { label: 'a', command: 'echo a' },
        { label: 'b', command: 'echo b', dependsOn: 'a' },
        { label: 'main', command: 'echo main', dependsOn: ['a', 'b'] },
    ];

    it('runs deps before the task, in order, once each', () => {
        expect(taskRunOrder(all[2], all)).toEqual(['a', 'b', 'main']);
    });

    it('is cycle-safe', () => {
        const cyclic: VscTask[] = [
            { label: 'x', dependsOn: 'y' },
            { label: 'y', dependsOn: 'x' },
        ];
        expect(taskRunOrder(cyclic[0], cyclic)).toEqual(['y', 'x']);
    });

    it('a task with no deps is just itself', () => {
        expect(taskRunOrder(all[0], all)).toEqual(['a']);
    });
});
