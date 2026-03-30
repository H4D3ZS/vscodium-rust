import React from 'react';
import { useStore } from '../store';

export const TrustDialog: React.FC = () => {
    const request = useStore(state => state.extensionTrustRequest);
    const resolve = useStore(state => state.resolveExtensionTrust);
    const [alwaysTrust, setAlwaysTrust] = React.useState(false);

    if (!request) return null;

    return (
        <div className="modal-overlay" style={{
            position: 'fixed',
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
            background: 'rgba(0, 0, 0, 0.65)',
            backdropFilter: 'blur(12px)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            zIndex: 11000,
            animation: 'fadeIn 0.2s ease-out'
        }}>
            <div className="modal-content" style={{
                width: '420px',
                background: 'rgba(30, 30, 30, 0.85)',
                border: '1px solid rgba(255, 255, 255, 0.1)',
                borderRadius: '12px',
                padding: '24px',
                boxShadow: '0 20px 40px rgba(0, 0, 0, 0.4), 0 0 0 1px rgba(255, 255, 255, 0.05)',
                color: '#e0e0e0',
                display: 'flex',
                flexDirection: 'column',
                gap: '20px',
                transform: 'scale(1)',
                animation: 'slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1)'
            }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
                    <div style={{
                        width: '48px',
                        height: '48px',
                        borderRadius: '10px',
                        background: 'linear-gradient(135deg, #f59e0b, #d97706)',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        boxShadow: '0 4px 12px rgba(245, 158, 11, 0.3)'
                    }}>
                        <i className="codicon codicon-warning" style={{ fontSize: '24px', color: 'white' }}></i>
                    </div>
                    <div>
                        <h3 style={{ margin: 0, fontSize: '18px', fontWeight: 600, color: '#fff' }}>Trust this extension?</h3>
                        <p style={{ margin: '4px 0 0 0', fontSize: '13px', opacity: 0.7 }}>Installation from unknown publishers</p>
                    </div>
                </div>

                <div style={{
                    background: 'rgba(255, 255, 255, 0.03)',
                    padding: '16px',
                    borderRadius: '8px',
                    border: '1px solid rgba(255, 255, 255, 0.05)',
                    fontSize: '14px',
                    lineHeight: '1.5'
                }}>
                    TERMINATOR is about to install <strong>{request.publisher}.{request.name}</strong> (v{request.version}).
                    Installing extensions from untrusted sources can potentially harm your system.
                </div>

                <div style={{ display: 'flex', alignItems: 'center', gap: '8px', fontSize: '13px' }}>
                    <input
                        type="checkbox"
                        id="always-trust-checkbox"
                        checked={alwaysTrust}
                        onChange={(e) => setAlwaysTrust(e.target.checked)}
                        style={{ cursor: 'pointer' }}
                    />
                    <label htmlFor="always-trust-checkbox" style={{ cursor: 'pointer', opacity: 0.8 }}>
                        Always trust publisher "<strong>{request.publisher}</strong>"
                    </label>
                </div>

                <div style={{ display: 'flex', justifyContent: 'flex-end', gap: '12px' }}>
                    <button
                        onClick={() => resolve(false)}
                        style={{
                            background: 'transparent',
                            color: '#ccc',
                            border: '1px solid rgba(255, 255, 255, 0.1)',
                            padding: '10px 20px',
                            borderRadius: '6px',
                            fontSize: '13px',
                            fontWeight: 500,
                            cursor: 'pointer',
                            transition: 'all 0.2s'
                        }}
                    >
                        Don't Install
                    </button>
                    <button
                        onClick={() => resolve(true, alwaysTrust)}
                        style={{
                            background: 'linear-gradient(135deg, #3b82f6, #2563eb)',
                            color: 'white',
                            border: 'none',
                            padding: '10px 24px',
                            borderRadius: '6px',
                            fontSize: '13px',
                            fontWeight: 600,
                            cursor: 'pointer',
                            boxShadow: '0 4px 12px rgba(37, 99, 235, 0.3)',
                            transition: 'all 0.2s'
                        }}
                    >
                        Trust and Install
                    </button>
                </div>
            </div>

            <style>{`
                @keyframes fadeIn {
                    from { opacity: 0; }
                    to { opacity: 1; }
                }
                @keyframes slideUp {
                    from { opacity: 0; transform: translateY(20px) scale(0.98); }
                    to { opacity: 1; transform: translateY(0) scale(1); }
                }
            `}</style>
        </div>
    );
};

