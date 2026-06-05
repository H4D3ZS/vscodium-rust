/** How instances within a terminal group are arranged. */
export type TerminalSplitLayout = 'single' | 'split-horizontal' | 'split-vertical';

/** Presentation state for one terminal tab group (Zustand slice). */
export interface TerminalGroupState {
    id: string;
    name: string;
    instances: string[];
    activeInstanceId: string;
    splitWeights?: number[];
    layout?: TerminalSplitLayout;
}
