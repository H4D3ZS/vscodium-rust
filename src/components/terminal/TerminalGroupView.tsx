import React, { useRef, useState, useEffect } from 'react';
import TerminalInstance from './TerminalInstance';
import { useStore } from '../../store';

interface TerminalGroupViewProps {
    groupId: string;
    active: boolean;
}

const TerminalGroupView: React.FC<TerminalGroupViewProps> = ({ groupId, active }) => {
    const group = useStore(state => state.terminalGroups.find(g => g.id === groupId));
    const updateWeights = useStore(state => state.updateTerminalSplitWeights);
    const containerRef = useRef<HTMLDivElement>(null);
    const [draggingIndex, setDraggingIndex] = useState<number | null>(null);

    if (!group) return null;

    const instances = group.instances;
    const weights = group.splitWeights || instances.map(() => 1 / instances.length);

    const onMouseDown = (index: number) => (e: React.MouseEvent) => {
        e.preventDefault();
        setDraggingIndex(index);
    };

    const onMouseMove = (e: MouseEvent) => {
        if (draggingIndex === null || !containerRef.current) return;

        const rect = containerRef.current.getBoundingClientRect();
        const mouseX = e.clientX - rect.left;
        const totalWidth = rect.width;

        // Calculate new weights based on mouse position
        // We look at draggingIndex (left instance) and draggingIndex + 1 (right instance)
        const currentSum = weights[draggingIndex] + weights[draggingIndex + 1];
        
        // Find the cumulative start of the dragging pair
        let pairStartWeight = 0;
        for (let i = 0; i < draggingIndex; i++) pairStartWeight += weights[i];
        
        const pairStartX = (pairStartWeight / weights.reduce((a, b) => a + b, 0)) * totalWidth;
        const newWeightLeft = ((mouseX - pairStartX) / totalWidth) * weights.reduce((a, b) => a + b, 0);
        
        if (newWeightLeft > 0.05 && (currentSum - newWeightLeft) > 0.05) {
            const nextWeights = [...weights];
            nextWeights[draggingIndex] = newWeightLeft;
            nextWeights[draggingIndex + 1] = currentSum - newWeightLeft;
            updateWeights(groupId, nextWeights);
        }
    };

    const onMouseUp = () => {
        setDraggingIndex(null);
    };

    useEffect(() => {
        if (draggingIndex !== null) {
            window.addEventListener('mousemove', onMouseMove);
            window.addEventListener('mouseup', onMouseUp);
        }
        return () => {
            window.removeEventListener('mousemove', onMouseMove);
            window.removeEventListener('mouseup', onMouseUp);
        };
    }, [draggingIndex]);

    return (
        <div 
            ref={containerRef}
            className={`terminal-group-view ${active ? 'active' : ''}`}
            style={{ 
                display: active ? 'flex' : 'none',
                width: '100%', 
                height: '100%', 
                flexDirection: 'row', 
                overflow: 'hidden',
                background: 'transparent',
                position: 'relative'
            }}
        >
            {instances.map((instanceId, index) => (
                <React.Fragment key={instanceId}>
                    <div style={{ flex: weights[index], position: 'relative', minWidth: '50px', height: '100%' }}>
                        <TerminalInstance 
                            id={instanceId} 
                            groupId={groupId}
                            active={group.activeInstanceId === instanceId} 
                        />
                    </div>
                    {index < instances.length - 1 && (
                        <div 
                            onMouseDown={onMouseDown(index)}
                            style={{
                                width: '1px',
                                cursor: 'col-resize',
                                background: draggingIndex === index ? 'var(--vscode-sash-hoverBorder, #007acc)' : 'var(--vscode-panel-border, rgba(128, 128, 128, 0.35))',
                                zIndex: 10,
                                position: 'relative'
                            }}
                        >
                            <div style={{
                                position: 'absolute',
                                left: '-2px',
                                width: '5px',
                                height: '100%',
                                cursor: 'col-resize'
                            }} />
                        </div>
                    )}
                </React.Fragment>
            ))}
        </div>
    );
};

export default TerminalGroupView;
