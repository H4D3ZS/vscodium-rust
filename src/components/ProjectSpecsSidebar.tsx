import React from 'react';
import { useStore } from '../store';
import { Sparkles, Edit, Terminal } from 'lucide-react';
import { marked } from 'marked';

const ProjectSpecsSidebar: React.FC = () => {
    const activeProjectSpec = useStore(state => state.activeProjectSpec);
    const setSpecsWizardOpen = useStore(state => state.setSpecsWizardOpen);
    const setSpecsWizardStep = useStore(state => state.setSpecsWizardStep);
    const setCurrentSpecProjectId = useStore(state => state.setCurrentSpecProjectId);

    if (!activeProjectSpec) return null;

    const handleOpenWizard = () => {
        setCurrentSpecProjectId(activeProjectSpec.id);
        setSpecsWizardStep('status');
        setSpecsWizardOpen(true);
    };

    const handleEditSpecs = () => {
        setCurrentSpecProjectId(activeProjectSpec.id);
        setSpecsWizardStep('generator');
        setSpecsWizardOpen(true);
    };

    return (
        <div className="project-specs-sidebar" style={{ display: 'flex', flexDirection: 'column', gap: '8px', padding: '0 12px 12px' }}>
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '4px' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '6px', color: 'var(--terminator-accent)', fontSize: '11px', fontWeight: 600 }}>
                    <Sparkles size={12} />
                    <span>AI PIPELINE ACTIVE</span>
                </div>
                <div style={{ display: 'flex', gap: '8px' }}>
                    <span onClick={handleEditSpecs} title="Edit Specs" style={{ cursor: 'pointer', opacity: 0.6, display: 'flex', alignItems: 'center' }}><Edit size={12} /></span>
                    <span onClick={handleOpenWizard} title="View Pipeline Status" style={{ cursor: 'pointer', opacity: 0.6, display: 'flex', alignItems: 'center' }}><Terminal size={12} /></span>
                </div>
            </div>

            <div style={{
                background: 'rgba(255, 255, 255, 0.02)',
                border: '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.05))',
                borderRadius: '6px',
                padding: '10px',
                fontSize: '12px',
                maxHeight: '300px',
                overflowY: 'auto',
                lineHeight: '1.4'
            }}>
                <div className="markdown-content" style={{ opacity: 0.8 }} dangerouslySetInnerHTML={{ __html: marked.parse(activeProjectSpec.specs || "") as string }} />
            </div>

            <button
                className="secondary-button"
                onClick={handleOpenWizard}
                style={{
                    width: '100%',
                    padding: '4px',
                    fontSize: '11px',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    gap: '6px',
                    marginTop: '4px'
                }}
            >
                <Terminal size={12} />
                Manage Pipeline
            </button>
        </div>
    );
};

export default ProjectSpecsSidebar;
