import React, { useRef, useState, useEffect } from 'react';
import TerminalInstance from './TerminalInstance';
import { useStore } from '../../store';

interface TerminalGroupViewProps {
    groupId: string;
    active: boolean;
}

const TerminalGroupView: React.FC<TerminalGroupViewProps> = ({ groupId, active }) => {
    const group = useStore((state) => state.terminalGroups.find((g) => g.id === groupId));
    const updateWeights = useStore((state) => state.updateTerminalSplitWeights);
    const containerRef = useRef<HTMLDivElement>(null);
    const [draggingIndex, setDraggingIndex] = useState<number | null>(null);

    if (!group) return null;

    const instances = group.instances;
    const weights = group.splitWeights || instances.map(() => 1 / instances.length);
    const isVertical = group.layout === 'split-vertical';
    const flexDirection = isVertical ? 'column' : 'row';
    const sashCursor = isVertical ? 'row-resize' : 'col-resize';

    const onMouseDown = (index: number) => (e: React.MouseEvent) => {
        e.preventDefault();
        setDraggingIndex(index);
    };

    const onMouseMove = (e: MouseEvent) => {
        if (draggingIndex === null || !containerRef.current) return;

        const rect = containerRef.current.getBoundingClientRect();
        const total = weights.reduce((a, b) => a + b, 0);
        const currentSum = weights[draggingIndex] + weights[draggingIndex + 1];

        let pairStartWeight = 0;
        for (let i = 0; i < draggingIndex; i++) pairStartWeight += weights[i];

        let newWeightLeft: number;
        if (isVertical) {
            const mouseY = e.clientY - rect.top;
            const pairStartY = (pairStartWeight / total) * rect.height;
            newWeightLeft = ((mouseY - pairStartY) / rect.height) * total;
        } else {
            const mouseX = e.clientX - rect.left;
            const pairStartX = (pairStartWeight / total) * rect.width;
            newWeightLeft = ((mouseX - pairStartX) / rect.width) * total;
        }

        if (newWeightLeft > 0.05 && currentSum - newWeightLeft > 0.05) {
            const nextWeights = [...weights];
            nextWeights[draggingIndex] = newWeightLeft;
            nextWeights[draggingIndex + 1] = currentSum - newWeightLeft;
            updateWeights(groupId, nextWeights);
        }
    };

    const onMouseUp = () => setDraggingIndex(null);

    useEffect(() => {
        if (draggingIndex !== null) {
            window.addEventListener('mousemove', onMouseMove);
            window.addEventListener('mouseup', onMouseUp);
        }
        return () => {
            window.removeEventListener('mousemove', onMouseMove);
            window.removeEventListener('mouseup', onMouseUp);
        };
    }, [draggingIndex, isVertical, weights]);

    return (
        <div
            ref={containerRef}
            className={`terminal-group-view ${active ? 'active' : ''}`}
            style={{
                display: active ? 'flex' : 'none',
                width: '100%',
                height: '100%',
                flexDirection,
                overflow: 'hidden',
                background: 'transparent',
                position: 'relative',
            }}
        >
            {instances.map((instanceId, index) => (
                <React.Fragment key={instanceId}>
                    <div style={{ flex: weights[index], position: 'relative', minWidth: '50px', minHeight: '50px', height: '100%' }}>
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
                                width: isVertical ? '100%' : '1px',
                                height: isVertical ? '1px' : '100%',
                                cursor: sashCursor,
                                background:
                                    draggingIndex === index
                                        ? 'var(--vscode-sash-hoverBorder, #007acc)'
                                        : 'var(--vscode-panel-border, rgba(128, 128, 128, 0.35))',
                                zIndex: 10,
                                position: 'relative',
                                flexShrink: 0,
                            }}
                        >
                            <div
                                style={{
                                    position: 'absolute',
                                    left: isVertical ? 0 : '-2px',
                                    top: isVertical ? '-2px' : 0,
                                    width: isVertical ? '100%' : '5px',
                                    height: isVertical ? '5px' : '100%',
                                    cursor: sashCursor,
                                }}
                            />
                        </div>
                    )}
                </React.Fragment>
            ))}
        </div>
    );
};

export default TerminalGroupView;
