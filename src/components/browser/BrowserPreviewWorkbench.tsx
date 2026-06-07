import React, { Suspense, lazy } from 'react';

const BrowserSurface = lazy(() => import('../BrowserSurface'));

/** In-IDE dev-server preview + agent vision mirror (optional layout). */
const BrowserPreviewWorkbench: React.FC = () => (
    <div
        className="browser-preview-workbench"
        style={{
            display: 'flex',
            flexDirection: 'column',
            flex: 1,
            minHeight: 0,
            overflow: 'hidden',
            background: 'var(--vscode-editor-background)',
        }}
    >
        <Suspense fallback={null}>
            <BrowserSurface />
        </Suspense>
    </div>
);

export default BrowserPreviewWorkbench;
