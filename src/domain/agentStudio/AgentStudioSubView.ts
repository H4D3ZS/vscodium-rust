/** Agent Studio feature surfaces (specs, steering, research, etc.). */
export type AgentStudioSubView =
    | 'dashboard'
    | 'agents'
    | 'specs'
    | 'research'
    | 'rules'
    | 'tasks'
    | 'steering'
    | 'planning';

export interface AgentStudioSubViewMeta {
    id: AgentStudioSubView;
    label: string;
    icon: string;
    hint: string;
}
