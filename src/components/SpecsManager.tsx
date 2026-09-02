import React, { useState, useEffect } from 'react';
import { invoke } from '../tauri_bridge';
import { useStore } from '../store';
import { sendAgentMessage } from '../application/agent';

interface SpecsManagerProps {
    onClose?: () => void;
}

export const SpecsManager: React.FC<SpecsManagerProps> = ({ onClose }) => {
    const [prompt, setPrompt] = useState('');
    const [isGenerating, setIsGenerating] = useState(false);
    const [isScaffolding, setIsScaffolding] = useState(false);
    const [isGeneratingTests, setIsGeneratingTests] = useState(false);
    const [generatedSpecs, setGeneratedSpecs] = useState('');
    const [projects, setProjects] = useState<any[]>([]);
    const [activeProject, setActiveProject] = useState<any | null>(null);
    const [tasks, setTasks] = useState<any[]>([]);

    const specsPrompt = useStore(state => state.specsPrompt);
    const setSpecsPrompt = useStore(state => state.setSpecsPrompt);

    useEffect(() => {
        loadProjects();
    }, []);

    useEffect(() => {
        if (specsPrompt && specsPrompt.trim()) {
            setPrompt(specsPrompt);
            setSpecsPrompt('');
            generateSpecsAndTasks(specsPrompt);
        }
    }, [specsPrompt]);

    const loadProjects = async () => {
        try {
            const p = await invoke<any[]>('cmd_specs_get_projects');
            setProjects(p || []);
        } catch (e) {
            console.error('Failed to load projects', e);
        }
    };

    const loadTasks = async (projectId: number) => {
        try {
            const t = await invoke<any[]>('cmd_specs_get_project_tasks', { projectId });
            setTasks(t || []);
        } catch (e) {
            console.error('Failed to load tasks', e);
        }
    };

    const generateSpecsAndTasks = async (overridePrompt?: string) => {
        const val = (overridePrompt || prompt).trim();
        if (!val) return;
        setIsGenerating(true);
        setGeneratedSpecs('Analyzing prompt and converting to EARS notation requirements...');
        
        try {
            // Step 1: Generate EARS Requirements via AI
            const earsPrompt = `You are the Spec Engine. Convert the following natural language request into clear requirements and acceptance criteria in EARS (Easy Approach to Requirements Syntax) notation. Keep it concise but thorough. Request: "${val}"`;
            
            let earsResult = '';
            await new Promise<void>((resolve, reject) => {
                sendAgentMessage(earsPrompt, (msg) => {
                    earsResult = msg;
                    setGeneratedSpecs(msg);
                }).then(() => resolve()).catch(reject);
            });

            // Step 2: Save to Specs DB
            const projectId = await invoke<number>('cmd_specs_create_project', {
                name: `Feature: ${val.slice(0, 20)}...`,
                specs: earsResult,
                provider: null
            });
            
            // Step 3: Trigger Architectural Breakdown (Tasks)
            setGeneratedSpecs((prev) => prev + '\n\n---\nGenerating architectural design and discrete tasks...');
            await invoke('cmd_specs_generate_layout', { projectId });
            
            await loadProjects();
            const newProj = { id: projectId, name: `Feature: ${val.slice(0, 20)}...`, description: earsResult };
            setActiveProject(newProj);
            await loadTasks(projectId);

        } catch (error: any) {
            console.error(error);
            setGeneratedSpecs((prev) => prev + `\n\nError: ${error.message || String(error)}`);
        } finally {
            setIsGenerating(false);
        }
    };

    const scaffoldProject = async () => {
        if (!activeProject) return;
        setIsScaffolding(true);
        const scaffoldPrompt = `You are an autonomous project architect. Based on the following specification, generate the complete file/directory scaffold for the project.

SPEC:
${activeProject.description}

TASKS:
${tasks.map((t: any) => `- ${t.title}`).join('\n')}

Instructions:
1. List every file path that needs to be created (relative to project root).
2. For each file, write the starter implementation — not stubs, real code.
3. Use the existing codebase style where applicable (Tauri v2 + React + TypeScript or Rust).
4. After listing each file, use the write_file tool to create it.
5. Announce "SCAFFOLD COMPLETE" when done.

Begin scaffolding now.`;
        try {
            await sendAgentMessage(scaffoldPrompt, () => {});
        } catch (e: any) {
            console.error('Scaffold failed', e);
        } finally {
            setIsScaffolding(false);
        }
    };

    const generateTests = async () => {
        if (!activeProject) return;
        setIsGeneratingTests(true);
        const testPrompt = `You are a test engineer. Based on the following specification and task list, generate a comprehensive test suite.

SPEC:
${activeProject.description}

TASKS:
${tasks.map((t: any) => `- ${t.title}`).join('\n')}

Instructions:
1. Write unit tests for each acceptance criterion in the spec.
2. Write integration tests for the full feature flow.
3. Use the testing framework appropriate for the detected language (Vitest for TS, cargo test for Rust).
4. Each test must have a clear description tied to an acceptance criterion.
5. Use write_file to create the test files.
6. Announce "TEST GENERATION COMPLETE" when done.

Begin generating tests now.`;
        try {
            await sendAgentMessage(testPrompt, () => {});
        } catch (e: any) {
            console.error('Test gen failed', e);
        } finally {
            setIsGeneratingTests(false);
        }
    };

    return (
        <div style={{ padding: '16px', color: 'var(--vscode-foreground)', height: '100%', overflowY: 'auto' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '16px' }}>
                <h2 style={{ margin: 0, fontSize: '14px', textTransform: 'uppercase', letterSpacing: '1px' }}>Spec Engine</h2>
                {onClose && <button onClick={onClose} style={{ background: 'none', border: 'none', color: 'inherit', cursor: 'pointer' }}>×</button>}
            </div>

            {!activeProject ? (
                <>
                    <div style={{ marginBottom: '16px' }}>
                        <div style={{ fontSize: '11px', opacity: 0.7, marginBottom: '8px' }}>
                            Describe your feature in natural language. The agent will convert it to structured EARS requirements, determine the optimal architecture, and break it into discrete executable tasks.
                        </div>
                        <textarea
                            value={prompt}
                            onChange={e => setPrompt(e.target.value)}
                            placeholder="e.g., I need a new user authentication system with email/password and OAuth support..."
                            style={{
                                width: '100%',
                                minHeight: '80px',
                                background: 'rgba(0,0,0,0.2)',
                                border: '1px solid rgba(255,255,255,0.1)',
                                color: 'var(--vscode-editor-foreground, #fff)',
                                padding: '8px',
                                borderRadius: '4px',
                                fontFamily: 'inherit',
                                resize: 'vertical'
                            }}
                        />
                        <button 
                            onClick={() => generateSpecsAndTasks()}
                            disabled={isGenerating || !prompt.trim()}
                            style={{
                                marginTop: '8px',
                                padding: '8px 16px',
                                background: 'var(--terminator-accent)',
                                color: 'var(--vscode-editor-foreground, #fff)',
                                border: 'none',
                                borderRadius: '4px',
                                cursor: isGenerating ? 'wait' : 'pointer',
                                fontWeight: 600,
                                width: '100%'
                            }}
                        >
                            {isGenerating ? 'Synthesizing Requirements...' : 'Generate Specs & Plan'}
                        </button>
                    </div>

                    {generatedSpecs && (
                        <div style={{ background: 'rgba(0,0,0,0.3)', padding: '12px', borderRadius: '4px', marginBottom: '16px' }}>
                            <div style={{ fontSize: '10px', textTransform: 'uppercase', opacity: 0.5, marginBottom: '8px' }}>Output Log</div>
                            <pre style={{ margin: 0, fontSize: '11px', whiteSpace: 'pre-wrap', fontFamily: 'var(--vscode-editor-font-family)' }}>
                                {generatedSpecs}
                            </pre>
                        </div>
                    )}

                    <div style={{ borderTop: '1px solid rgba(255,255,255,0.1)', paddingTop: '16px' }}>
                        <div style={{ fontSize: '12px', fontWeight: 600, marginBottom: '8px' }}>Recent Specs Projects</div>
                        {projects.map(p => (
                            <div 
                                key={p.id} 
                                onClick={() => { setActiveProject(p); loadTasks(p.id); }}
                                style={{
                                    padding: '8px',
                                    background: 'rgba(255,255,255,0.02)',
                                    marginBottom: '4px',
                                    borderRadius: '4px',
                                    cursor: 'pointer'
                                }}
                            >
                                <div style={{ fontWeight: 600, fontSize: '12px' }}>{p.name}</div>
                            </div>
                        ))}
                    </div>
                </>
            ) : (
                <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                    <div>
                        <button 
                            onClick={() => setActiveProject(null)}
                            style={{ background: 'none', border: 'none', color: 'var(--terminator-accent)', cursor: 'pointer', padding: 0, fontSize: '11px' }}
                        >
                            ← Back to Generator
                        </button>
                    </div>
                    <div>
                        <h3 style={{ margin: '0 0 8px 0', fontSize: '13px' }}>{activeProject.name}</h3>
                        <div style={{ fontSize: '11px', background: 'rgba(0,0,0,0.2)', padding: '8px', borderRadius: '4px', whiteSpace: 'pre-wrap' }}>
                            {activeProject.description}
                        </div>
                    </div>
                    {/* Scaffold + Test Gen actions */}
                    <div style={{ display: 'flex', gap: '8px', marginBottom: '4px' }}>
                        <button
                            onClick={scaffoldProject}
                            disabled={isScaffolding || tasks.length === 0}
                            style={{
                                flex: 1, padding: '7px 10px',
                                background: isScaffolding ? 'rgba(0,198,255,0.12)' : 'rgba(0,198,255,0.18)',
                                border: '1px solid rgba(0,198,255,0.4)',
                                color: '#00c6ff', borderRadius: '5px', cursor: isScaffolding ? 'wait' : 'pointer',
                                fontSize: '11px', fontWeight: 600,
                            }}
                        >
                            {isScaffolding ? 'Scaffolding...' : 'Scaffold Project'}
                        </button>
                        <button
                            onClick={generateTests}
                            disabled={isGeneratingTests || tasks.length === 0}
                            style={{
                                flex: 1, padding: '7px 10px',
                                background: isGeneratingTests ? 'rgba(74,222,128,0.08)' : 'rgba(74,222,128,0.12)',
                                border: '1px solid rgba(74,222,128,0.3)',
                                color: '#4ade80', borderRadius: '5px', cursor: isGeneratingTests ? 'wait' : 'pointer',
                                fontSize: '11px', fontWeight: 600,
                            }}
                        >
                            {isGeneratingTests ? 'Generating...' : 'Generate Tests'}
                        </button>
                    </div>

                    <div>
                        <h4 style={{ margin: '0 0 8px 0', fontSize: '12px' }}>Architectural Tasks</h4>
                        {tasks.length === 0 ? (
                            <div style={{ opacity: 0.5, fontSize: '11px' }}>No tasks generated yet.</div>
                        ) : (
                            tasks.map(t => (
                                <div key={t.id} style={{
                                    padding: '8px',
                                    background: 'rgba(255,255,255,0.05)',
                                    marginBottom: '4px',
                                    borderRadius: '4px',
                                    display: 'flex',
                                    alignItems: 'center',
                                    gap: '8px'
                                }}>
                                    <div style={{
                                        width: '8px',
                                        height: '8px',
                                        borderRadius: '50%',
                                        background: t.status === 'Completed' ? '#4ade80' : t.status === 'Running' ? '#60a5fa' : 'var(--terminator-accent)'
                                    }} />
                                    <div style={{ flex: 1 }}>
                                        <div style={{ fontSize: '12px', fontWeight: 500 }}>{t.title}</div>
                                        <div style={{ fontSize: '10px', opacity: 0.6 }}>Status: {t.status}</div>
                                    </div>
                                    <button 
                                        style={{ background: 'var(--terminator-accent)', color: 'white', border: 'none', padding: '4px 8px', borderRadius: '4px', fontSize: '10px', cursor: 'pointer' }}
                                    >
                                        Execute Agent
                                    </button>
                                </div>
                            ))
                        )}
                    </div>
                </div>
            )}
        </div>
    );
};

export default SpecsManager;
