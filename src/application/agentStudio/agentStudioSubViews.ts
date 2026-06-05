import type { AgentStudioSubViewMeta } from '../../domain/agentStudio/AgentStudioSubView';

export const AGENT_STUDIO_SUB_VIEWS: AgentStudioSubViewMeta[] = [
    { id: 'dashboard', label: 'Dashboard', icon: 'dashboard', hint: 'Mission control & performance' },
    { id: 'agents', label: 'Agents', icon: 'server-process', hint: 'Parallel background agents — spawn, logs, cancel' },
    { id: 'research', label: 'Web Agent', icon: 'globe', hint: 'Full web mission — search, browser, scrape, audit, terminal' },
    { id: 'specs', label: 'Specs', icon: 'book', hint: '.kiro/specs — spec-driven development' },
    { id: 'tasks', label: 'Tasks', icon: 'tasklist', hint: 'Executable tasks from specs' },
    { id: 'steering', label: 'Steering', icon: 'symbol-keyword', hint: '.agent/steering guidance files' },
    { id: 'rules', label: 'Rules', icon: 'law', hint: 'Project & workspace rules' },
    { id: 'planning', label: 'Session', icon: 'notebook', hint: 'task_plan / findings / progress notes' },
];
