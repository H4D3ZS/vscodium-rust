import React, { Component } from "react";
import type { ErrorInfo, ReactNode } from "react";

interface Props {
  children?: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
    error: null
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error("Uncaught error in ErrorBoundary:", error, errorInfo);
  }

  public render() {
    if (this.state.hasError) {
      return this.props.fallback || (
        <div style={{ padding: '20px', color: 'var(--vscode-errorForeground, red)', backgroundColor: 'var(--vscode-sideBar-background)', height: '100%', minHeight: '100px', display: 'flex', flexDirection: 'column', gap: '8px' }}>
          <h4 style={{ margin: 0, fontSize: '14px' }}>Something went wrong.</h4>
          <p style={{ margin: 0, fontSize: '12px', opacity: 0.8 }}>An error occurred in this view.</p>
          {this.state.error && (
            <pre style={{ fontSize: '10px', overflow: 'auto', background: 'rgba(0,0,0,0.1)', padding: '8px', borderRadius: '4px' }}>
              {this.state.error.message}
            </pre>
          )}
          <button 
            onClick={() => this.setState({ hasError: false, error: null })}
            style={{ 
              marginTop: '12px', 
              alignSelf: 'flex-start',
              background: 'var(--vscode-button-background)',
              color: 'var(--vscode-button-foreground)',
              border: 'none',
              padding: '4px 12px',
              borderRadius: '2px',
              cursor: 'pointer',
              fontSize: '12px'
            }}
          >
            Try Again
          </button>
        </div>
      );
    }

    return this.props.children;
  }
}

export default ErrorBoundary;
