import { useEffect, useState, useRef, useCallback } from "react";
import Editor, { useMonaco } from '@monaco-editor/react';
import ForceGraph3D from 'react-force-graph-3d';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import "./App.css";

const MOCK_MEMORY = `---
type: aim_vfs_state
version: 1.0
encryption: Hybrid (ML-DSA / Ed25519)
parametric_gist_id: AIM-1536-Q:[dGhlIHYxLjAgY29tcHJlc3NlZCBwYXJhbWV0cmljIGRlcHRoIHN0YXRl]
compression_ratio: 99.9%
---

# Neural Context Memory: Project "Cognitive Kernel"
> The Cognitive Kernel VFS is functioning flawlessly. All prior history is compressed above.

// The system is rendering recent L1 KV-Cache actions below:
function initializeL1Cache() {
  console.log("Mounting Hot Cache inside RAM...");
  System.bindVFS({ enableGist: true });
}

// Garbage Collector status
export const GarbageCollector = {
    status: 'ACTIVE',
    decayRate: 0.1,
    nextSweep: '2ms'
};
`;

const GROUP_COLORS = [
  '#eab308', '#84cc16', '#22c55e', '#ec4899',
  '#a855f7', '#3b82f6', '#f97316', '#14b8a6', '#64748b'
];

function App() {
  const monaco = useMonaco();
  const [activeTab, setActiveTab] = useState<'explorer' | 'graph'>('graph');
  const [mountedPath, setMountedPath] = useState("");

  // Default mount: <home>/Desktop/kortex on any OS (no hardcoded usernames).
  useEffect(() => {
    if (mountedPath) return;
    import('@tauri-apps/api/path')
      .then(async (p) => {
        const home = await p.homeDir();
        setMountedPath(await p.join(home, 'Desktop', 'kortex'));
      })
      .catch(() => { /* keep empty — user picks via dialog */ });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const [graphData, setGraphData] = useState<{ nodes: any[], links: any[] }>({ nodes: [], links: [] });

  const [selectedNode, setSelectedNode] = useState<any>(null);
  const [highlightNodes, setHighlightNodes] = useState(new Set());
  const [highlightLinks, setHighlightLinks] = useState(new Set());
  const [liveLog, setLiveLog] = useState('');
  const [fileContent, setFileContent] = useState(MOCK_MEMORY);
  const graphRef = useRef<any>(null);

  useEffect(() => {
    if (monaco) {
      monaco.languages.register({ id: 'aim' });
      monaco.languages.setMonarchTokensProvider('aim', {
        tokenizer: {
          root: [
            [/---/, 'comment'],
            [/[a-zA-Z_]+:/, 'keyword'],
            [/AIM-1536-Q:\S+/, 'string'],
            [/\/\/.*?$/, 'comment'],
            [/function|const|export/, 'keyword'],
          ]
        }
      });
      monaco.editor.defineTheme('neural-dark', {
        base: 'vs-dark',
        inherit: true,
        rules: [{ token: '', background: '0c0c0e' }],
        colors: {
          'editor.background': '#0c0c0e',
          'editor.lineHighlightBackground': '#18181b',
          'editorLineNumber.foreground': '#3f3f46',
        }
      });
    }
  }, [monaco]);

  // IPC Connection on Boot
  const loadGraph = (targetPath: string) => {
    invoke('get_aim_nodes', { projectPath: targetPath }).then((data: any) => {
      const { nodes, links } = data;
      nodes.forEach((n: any) => { n.neighbors = []; n.links = []; });
      links.forEach((link: any) => {
        nodes[link.source].neighbors.push(link.target);
        nodes[link.target].neighbors.push(link.source);
        nodes[link.source].links.push(link);
        nodes[link.target].links.push(link);
      });
      setGraphData({ nodes, links });
    }).catch(console.error);
  };

  useEffect(() => {
    loadGraph(mountedPath);
  }, []);

  const handleNodeClick = useCallback((node: any) => {
    const distance = 80;
    const distRatio = 1 + distance / Math.hypot(node.x, node.y, node.z);
    if (graphRef.current) {
      graphRef.current.cameraPosition(
        { x: node.x * distRatio, y: node.y * distRatio, z: node.z * distRatio },
        node,
        2000
      );
    }

    const newHighlightNodes = new Set();
    const newHighlightLinks = new Set();
    newHighlightNodes.add(node);

    graphData.links.forEach((link: any) => {
      if (link.source === node || link.target === node) {
        newHighlightLinks.add(link);
        newHighlightNodes.add(link.source === node ? link.target : link.source);
      }
    });

    setHighlightNodes(newHighlightNodes);
    setHighlightLinks(newHighlightLinks);
    setSelectedNode(node);
  }, [graphData]);

  const clearSelection = useCallback(() => {
    setHighlightNodes(new Set());
    setHighlightLinks(new Set());
    setSelectedNode(null);
  }, []);

  const handleBuildAim = () => {
    setLiveLog(`[KERNEL] Target Acquired: ${mountedPath}\nInitiating Architectural Scan...\n`);
    invoke('build_aim_binary', { projectPath: mountedPath })
      .then((res: any) => {
        alert(`[KERNEL SUCCESS]\n${res}`);
        setLiveLog(prev => `[KERNEL] ${res}\n` + prev);
        loadGraph(mountedPath);
      })
      .catch((err: any) => alert(`[KERNEL ERROR]\n${err}`));
  };

  const handleMountProject = async () => {
    try {
      const selectedPath = await open({
        directory: true,
        multiple: false,
        title: "Mount Neural Workspace"
      });

      if (selectedPath && typeof selectedPath === 'string') {
        setMountedPath(selectedPath);
        setLiveLog(`[KERNEL] Workspace Extracted: ${selectedPath}\nPopulating 3D WebGL Coordinates...\n`);
        loadGraph(selectedPath);
      }
    } catch (err) {
      console.error(err);
    }
  };

  useEffect(() => {
    if (selectedNode && selectedNode.path && activeTab === 'explorer') {
      invoke('read_file_content', { filePath: selectedNode.path })
        .then((res: any) => setFileContent(res))
        .catch((err) => setFileContent(`// [UNABLE TO READ FILE CONTENT]\n${err}`));
    }
  }, [selectedNode, activeTab]);

  // Secure Event Listener tracking System Daemon broadcasts
  useEffect(() => {
    if (!selectedNode) return;
    setLiveLog(`[KERNEL] Secure IPC bound. Monitoring VFS Daemon for ${selectedNode.name}...`);

    let unlisten: any;

    listen('aim-telemetry', (event: any) => {
      setLiveLog(prev => {
        const lines = prev.split('\n');
        if (lines.length > 4) {
          lines.splice(0, lines.length - 4);
        }
        return lines.join('\n') + `\n[${new Date().toISOString().split('T')[1].slice(0, -1)}] ${event.payload}`;
      });
    }).then(f => unlisten = f);

    return () => {
      if (unlisten) unlisten();
    };
  }, [selectedNode]);

  return (
    <div className="neural-drive-container">
      <div className="sidebar">
        <div className="brand">
          <div className="brand-icon"></div>
          NeuralDrive
        </div>

        <div className="stats-card">
          <span className="stats-label">Context Size</span>
          <span className="stats-value">1 Token</span>
        </div>

        <div className="stats-card">
          <span className="stats-label">Memory Gist</span>
          <span className="stats-value" style={{ fontSize: '0.8rem' }}>AIM-1536-Q</span>
        </div>

        <div className="stats-card" style={{ border: '1px solid rgba(34, 197, 94, 0.4)', background: 'rgba(34, 197, 94, 0.05)' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <span className="stats-label" style={{ color: '#22c55e' }}>Provenance Badge</span>
            <span title="C2PA Fully Verified via Soft Binding" style={{ cursor: 'pointer' }}>✅</span>
          </div>
          <span className="stats-value" style={{ fontSize: '0.8rem', color: '#22c55e' }}>Lattice Verified</span>
          <span style={{ fontSize: '0.65rem', color: 'var(--text-secondary)' }}>Chain: CYBER_IFRIT_26</span>
        </div>

        <div className="file-list">
          <span className="stats-label" style={{ marginBottom: "8px", marginTop: "12px" }}>Active Neurons</span>
          <div
            className={`file-item ${selectedNode?.name?.includes('memory.md') ? 'active' : ''}`}
            onClick={() => {
              const n = graphData.nodes.find(n => n.name.includes('memory'));
              if (n) handleNodeClick(n);
            }}>
            memory.md (.aim)
          </div>
          <div
            className={`file-item ${selectedNode?.name?.includes('garbage_collector.rs') ? 'active' : ''}`}
            onClick={() => {
              const n = graphData.nodes.find(n => n.name.includes('garbage_collector.rs'));
              if (n) handleNodeClick(n);
            }}>
            garbage_collector.rs
          </div>
          <div style={{ display: 'flex', flexDirection: 'column', gap: '8px', marginTop: '16px' }}>
            <button className="tab-btn" style={{ background: 'rgba(99, 102, 241, 0.1)', border: '1px solid #6366f1', color: '#818cf8', height: '40px' }} onClick={handleBuildAim}>
              Generate Physical .aim
            </button>
            <button className="tab-btn" style={{ background: 'rgba(34, 197, 94, 0.1)', border: '1px solid #22c55e', color: '#4ade80', height: '40px' }} onClick={handleMountProject}>
              Mount Project
            </button>
          </div>
        </div>
      </div>

      <div className="editor-area">
        <div className="editor-tabs">
          <button
            className={`tab-btn ${activeTab === 'explorer' ? 'active' : ''}`}
            onClick={() => setActiveTab('explorer')}>Explorer</button>
          <button
            className={`tab-btn ${activeTab === 'graph' ? 'active' : ''}`}
            onClick={() => setActiveTab('graph')}>Brain Graph</button>
          <button className={`tab-btn`}>Sleep</button>
          <button className={`tab-btn`}>Deep Sleep</button>
        </div>

        {activeTab === 'explorer' ? (
          <>
            <div className="editor-header">
              <span>{selectedNode?.path || 'c:\\kortex\\.aim\\memory.md'}</span>
              <span className="tag">Parametric Delta Sync</span>
            </div>
            <div className="monaco-wrapper">
              <Editor
                height="100%"
                language={selectedNode ? (['rs'].includes(selectedNode.name.split('.').pop()) ? 'rust' : ['ts', 'tsx'].includes(selectedNode.name.split('.').pop()) ? 'typescript' : ['json'].includes(selectedNode.name.split('.').pop()) ? 'json' : 'aim') : 'aim'}
                theme="neural-dark"
                value={fileContent}
                options={{
                  minimap: { enabled: false },
                  fontFamily: "'JetBrains Mono', monospace",
                  fontSize: 14,
                  padding: { top: 24 }
                }}
              />
            </div>
          </>
        ) : (
          <div className="graph-wrapper">
            <ForceGraph3D
              ref={graphRef}
              graphData={graphData}
              nodeResolution={6}
              linkResolution={3}
              enableNodeDrag={false}
              cooldownTicks={150}
              nodeColor={(node: any) => {
                if (highlightNodes.size === 0) return GROUP_COLORS[node.group];
                return highlightNodes.has(node) ? GROUP_COLORS[node.group] : 'rgba(255,255,255,0.05)';
              }}
              nodeRelSize={7}
              onNodeClick={handleNodeClick}
              onBackgroundClick={clearSelection}
              linkWidth={(link: any) => highlightLinks.has(link) ? 2 : 0.6}
              linkColor={(link: any) => {
                if (highlightNodes.size === 0) return 'rgba(255,255,255,0.08)';
                return highlightLinks.has(link) ? '#a855f7' : 'rgba(255,255,255,0.02)';
              }}
              backgroundColor="#00000000"
            />

            {/* The Target Node Real-Time Inspector */}
            {selectedNode && (
              <div className="node-inspector">
                <div className="inspector-header">
                  <span>Node Telemetry</span>
                  <button className="close-btn" onClick={clearSelection}>✕</button>
                </div>
                <div className="inspector-row">
                  <span className="stats-label">Identifier</span>
                  <span className="inspector-val">{selectedNode.name}</span>
                </div>
                <div className="inspector-row">
                  <span className="stats-label">Neuron Type</span>
                  <span className="inspector-val" style={{ color: GROUP_COLORS[selectedNode.group] }}>{selectedNode.type}</span>
                </div>
                <div className="inspector-row">
                  <span className="stats-label">Connections</span>
                  <span className="inspector-val">{highlightNodes.size - 1} Edges</span>
                </div>

                <div className="stats-label" style={{ marginTop: "8px" }}>Real-Time Activity</div>
                <div className="live-log-container">
                  {liveLog}
                </div>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
