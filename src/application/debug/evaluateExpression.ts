import { sendDapRequest } from './sendDapRequest';
import { useStore } from '../../store';

/** Send a DAP evaluate request; result arrives via bootstrapDebugRuntime → lastEvaluateResult. */
export async function evaluateDebugExpression(
    expression: string,
    context: 'repl' | 'watch' = 'repl',
): Promise<string> {
    const frameId = useStore.getState().debugStackFrames[0]?.id ?? 1;
    useStore.getState().setLastEvaluateResult(null);
    await sendDapRequest('evaluate', { expression, frameId, context });

    for (let i = 0; i < 30; i++) {
        await new Promise((r) => setTimeout(r, 100));
        const result = useStore.getState().lastEvaluateResult;
        if (result != null) return result;
    }
    return '(no response)';
}
