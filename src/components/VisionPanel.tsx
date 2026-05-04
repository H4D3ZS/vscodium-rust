import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store';

interface VisionResult {
  status: 'healthy' | 'error' | 'unknown';
  error_message: string | null;
  ui_elements: string[];
  suggested_action: string;
  raw_analysis: string;
}

const VisionPanel: React.FC = () => {
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [result, setResult] = useState<VisionResult | null>(null);
  const [customPrompt, setCustomPrompt] = useState('');
  const ollamaUrl = useStore(state => state.ollamaUrl);

  const handleAnalyzeScreen = async () => {
    setIsAnalyzing(true);
    setResult(null);

    try {
      const prompt = customPrompt || 'Analyze this mobile app screen for UI issues and errors.';
      
      const analysisResult = await invoke<VisionResult>('airi_vision_analyze_screen', {
        prompt: prompt,
        ollamaUrl: ollamaUrl || 'http://localhost:11434',
      });

      setResult(analysisResult);
    } catch (error: any) {
      setResult({
        status: 'error',
        error_message: error.message || 'Analysis failed',
        ui_elements: [],
        suggested_action: 'Check if Ollama is running and qwen2.5-vl model is pulled',
        raw_analysis: `Error: ${error.message}`,
      });
    } finally {
      setIsAnalyzing(false);
    }
  };

  const handleQuickAction = (action: string) => {
    setCustomPrompt(action);
  };

  return (
    <div style={{
      display: 'flex',
      flexDirection: 'column',
      height: '100%',
      background: 'var(--vscode-editor-background)',
      overflow: 'auto'
    }}>
      {/* Header */}
      <div style={{
        padding: '12px',
        borderBottom: '1px solid var(--vscode-panel-border)',
        background: 'var(--vscode-sideBar-background)'
      }}>
        <div style={{ 
          display: 'flex', 
          alignItems: 'center', 
          gap: '8px',
          marginBottom: '8px'
        }}>
          <span style={{ fontSize: '16px' }}>👁️</span>
          <h3 style={{ 
            margin: 0, 
            fontSize: '14px', 
            fontWeight: 600,
            color: 'var(--vscode-foreground)'
          }}>
            AIRI Vision - Screen Analysis
          </h3>
        </div>
        <p style={{ 
          fontSize: '11px', 
          color: 'var(--vscode-descriptionForeground)',
          margin: 0
        }}>
          AI-powered mobile QA using Qwen2.5-VL
        </p>
      </div>

      {/* Quick Actions */}
      <div style={{ padding: '12px', borderBottom: '1px solid var(--vscode-panel-border)' }}>
        <div style={{ fontSize: '11px', fontWeight: 600, marginBottom: '6px', color: 'var(--vscode-foreground)' }}>
          Quick Actions:
        </div>
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px' }}>
          <button
            onClick={() => handleQuickAction('Find any error messages or crash dialogs on this screen')}
            style={quickActionBtnStyle}
          >
            🔍 Find Errors
          </button>
          <button
            onClick={() => handleQuickAction('List all interactive UI elements and their states')}
            style={quickActionBtnStyle}
          >
            📱 List UI Elements
          </button>
          <button
            onClick={() => handleQuickAction('Is this app screen healthy or showing problems?')}
            style={quickActionBtnStyle}
          >
            ✅ Health Check
          </button>
          <button
            onClick={() => handleQuickAction('What should I test next on this screen?')}
            style={quickActionBtnStyle}
          >
            🧪 Test Suggestions
          </button>
        </div>
      </div>

      {/* Custom Prompt */}
      <div style={{ padding: '12px', borderBottom: '1px solid var(--vscode-panel-border)' }}>
        <div style={{ fontSize: '11px', fontWeight: 600, marginBottom: '6px', color: 'var(--vscode-foreground)' }}>
          Custom Prompt:
        </div>
        <textarea
          value={customPrompt}
          onChange={(e) => setCustomPrompt(e.target.value)}
          placeholder="Describe what you want AIRI to analyze..."
          style={{
            width: '100%',
            minHeight: '60px',
            padding: '8px',
            fontSize: '12px',
            background: 'var(--vscode-input-background)',
            color: 'var(--vscode-input-foreground)',
            border: '1px solid var(--vscode-input-border)',
            borderRadius: '4px',
            resize: 'vertical',
            fontFamily: 'var(--vscode-editor-font-family)',
          }}
        />
        <button
          onClick={handleAnalyzeScreen}
          disabled={isAnalyzing}
          style={{
            ...analyzeButtonStyle,
            opacity: isAnalyzing ? 0.6 : 1,
            cursor: isAnalyzing ? 'not-allowed' : 'pointer',
          }}
        >
          {isAnalyzing ? (
            <>
              <span className="codicon codicon-loading codicon-modifier-spin" style={{ display: 'inline-block' }}></span>
              Analyzing Screen...
            </>
          ) : (
            <>
              <span>👁️</span>
              Analyze Screen
            </>
          )}
        </button>
      </div>

      {/* Results */}
      {result && (
        <div style={{ padding: '12px', flex: 1, overflow: 'auto' }}>
          {/* Status Badge */}
          <div style={{
            display: 'inline-block',
            padding: '4px 10px',
            borderRadius: '12px',
            fontSize: '11px',
            fontWeight: 600,
            marginBottom: '12px',
            background: result.status === 'healthy' ? '#34C759' : result.status === 'error' ? '#FF3B30' : '#FF9500',
            color: '#fff',
          }}>
            {result.status === 'healthy' ? '✅ Healthy' : result.status === 'error' ? '❌ Error Detected' : '⚠️ Unknown'}
          </div>

          {/* Error Message */}
          {result.error_message && (
            <div style={{
              padding: '10px',
              background: 'rgba(255, 59, 48, 0.1)',
              border: '1px solid #FF3B30',
              borderRadius: '4px',
              marginBottom: '12px',
              fontSize: '12px',
              color: '#FF3B30',
            }}>
              <strong>Error Found:</strong> {result.error_message}
            </div>
          )}

          {/* UI Elements */}
          {result.ui_elements.length > 0 && (
            <div style={{
              padding: '10px',
              background: 'var(--vscode-textBlockQuote-background)',
              border: '1px solid var(--vscode-panel-border)',
              borderRadius: '4px',
              marginBottom: '12px',
            }}>
              <div style={{ fontSize: '11px', fontWeight: 600, marginBottom: '6px', color: 'var(--vscode-foreground)' }}>
                📱 UI Elements Detected:
              </div>
              <ul style={{ 
                margin: 0, 
                paddingLeft: '20px',
                fontSize: '11px',
                color: 'var(--vscode-foreground)',
              }}>
                {result.ui_elements.map((el, idx) => (
                  <li key={idx}>{el}</li>
                ))}
              </ul>
            </div>
          )}

          {/* Suggested Action */}
          <div style={{
            padding: '10px',
            background: 'var(--vscode-textBlockQuote-background)',
            border: '1px solid var(--vscode-panel-border)',
            borderRadius: '4px',
            marginBottom: '12px',
          }}>
            <div style={{ fontSize: '11px', fontWeight: 600, marginBottom: '6px', color: 'var(--vscode-foreground)' }}>
              💡 Suggested Action:
            </div>
            <div style={{ fontSize: '12px', color: 'var(--vscode-foreground)' }}>
              {result.suggested_action}
            </div>
          </div>

          {/* Raw Analysis */}
          <div style={{
            padding: '10px',
            background: 'var(--vscode-editor-background)',
            border: '1px solid var(--vscode-panel-border)',
            borderRadius: '4px',
            fontSize: '11px',
            fontFamily: 'var(--vscode-editor-font-family)',
            whiteSpace: 'pre-wrap',
            color: 'var(--vscode-foreground)',
          }}>
            <div style={{ fontSize: '10px', fontWeight: 600, marginBottom: '6px', opacity: 0.7 }}>
              📄 Full Analysis:
            </div>
            {result.raw_analysis}
          </div>
        </div>
      )}

      {/* Empty State */}
      {!result && !isAnalyzing && (
        <div style={{
          flex: 1,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          color: 'var(--vscode-descriptionForeground)',
          fontSize: '12px',
          textAlign: 'center',
          padding: '20px',
        }}>
          <div>
            <div style={{ fontSize: '48px', marginBottom: '16px' }}>👁️</div>
            <div>Capture and analyze emulator screens</div>
            <div style={{ fontSize: '11px', opacity: 0.6, marginTop: '8px' }}>
              Uses Qwen2.5-VL for AI-powered mobile QA
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

const quickActionBtnStyle: React.CSSProperties = {
  padding: '6px 12px',
  fontSize: '11px',
  fontWeight: 500,
  background: 'var(--vscode-button-secondaryBackground)',
  color: 'var(--vscode-button-secondaryForeground)',
  border: '1px solid var(--vscode-panel-border)',
  borderRadius: '3px',
  cursor: 'pointer',
};

const analyzeButtonStyle: React.CSSProperties = {
  width: '100%',
  padding: '10px',
  fontSize: '12px',
  fontWeight: 600,
  background: '#007AFF',
  color: 'white',
  border: 'none',
  borderRadius: '4px',
  marginTop: '8px',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  gap: '8px',
};

export default VisionPanel;
