// CSS-only stand-in for framer-motion (Milestone B animation purge —
// docs/overhaul/MASTER_PLAN.md). Drop-in for the `motion.div` /
// `AnimatePresence` subset this codebase used: framer props are stripped,
// entry gets a single ≤150ms opacity fade via .motion-fade (styles.css).
// Exit animations are intentionally gone — VSCode-native UIs don't animate
// unmounts.

import React from 'react';

type MotionDivProps = React.HTMLAttributes<HTMLDivElement> & {
    initial?: unknown;
    animate?: unknown;
    exit?: unknown;
    transition?: unknown;
    whileHover?: unknown;
    whileTap?: unknown;
    whileDrag?: unknown;
    drag?: unknown;
    layout?: unknown;
    layoutId?: unknown;
    variants?: unknown;
};

const FRAMER_PROPS = [
    'initial', 'animate', 'exit', 'transition', 'whileHover', 'whileTap',
    'whileDrag', 'drag', 'layout', 'layoutId', 'variants',
] as const;

function stripFramerProps(props: MotionDivProps): React.HTMLAttributes<HTMLDivElement> {
    const rest: Record<string, unknown> = { ...props };
    for (const k of FRAMER_PROPS) delete rest[k];
    return rest as React.HTMLAttributes<HTMLDivElement>;
}

export const motion = {
    div: React.forwardRef<HTMLDivElement, MotionDivProps>(function MotionDiv(props, ref) {
        const rest = stripFramerProps(props);
        const className = [props.className, 'motion-fade'].filter(Boolean).join(' ');
        return <div ref={ref} {...rest} className={className} />;
    }),
};

/** Passthrough: children render/unmount immediately, no exit animation. */
export const AnimatePresence: React.FC<{
    children?: React.ReactNode;
    mode?: string;
    initial?: boolean;
}> = ({ children }) => <>{children}</>;
