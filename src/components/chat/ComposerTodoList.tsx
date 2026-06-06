import React from 'react';
import type { AgentTodoItem } from '../../domain/agent/agentToolBlocks';

interface ComposerTodoListProps {
    todos: AgentTodoItem[];
    title?: string;
}

const statusIcon: Record<AgentTodoItem['status'], string> = {
    pending: 'circle-outline',
    in_progress: 'sync',
    completed: 'check',
    cancelled: 'close',
};

const ComposerTodoList: React.FC<ComposerTodoListProps> = ({ todos, title }) => {
    if (!todos.length) return null;
    const done = todos.filter((t) => t.status === 'completed').length;

    return (
        <div className="composer-todo">
            <div className="composer-todo__header">
                <i className="codicon codicon-tasklist" style={{ fontSize: 12, opacity: 0.6 }} />
                <span>{title || 'Tasks'}</span>
                <span className="composer-todo__progress">{done}/{todos.length}</span>
            </div>
            <ul className="composer-todo__list">
                {todos.map((todo) => (
                    <li key={todo.id} className={`composer-todo__item composer-todo__item--${todo.status}`}>
                        <i className={`codicon codicon-${statusIcon[todo.status]}`} />
                        <span>{todo.text}</span>
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default ComposerTodoList;
