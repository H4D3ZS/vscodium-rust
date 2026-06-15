import React, { Suspense, lazy } from 'react';

const PyTorchStudioPanel = lazy(() => import('../pytorch/PyTorchStudioPanel'));

/** Full-height PyTorch ML Studio — no bundled browser preview. */
const MlStudioWorkbench: React.FC = () => (
    <div
        className="ml-studio-workbench"
        style={{
            display: 'flex',
            flexDirection: 'column',
            flex: 1,
            minHeight: 0,
            overflow: 'hidden',
            background: 'var(--vscode-sideBar-background, #161b22)',
        }}
    >
        <Suspense fallback={null}>
            <PyTorchStudioPanel mode="dock" />
        </Suspense>
    </div>
);

export default MlStudioWorkbench;
