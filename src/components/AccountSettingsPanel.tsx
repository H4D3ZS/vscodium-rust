import React from 'react';

const AccountSettingsPanel: React.FC = () => {
    return (
        <div style={{ padding: 16 }}>
            <h3 style={{ marginBottom: 8 }}>Community Edition</h3>
            <p style={{ color: '#888', lineHeight: 1.6 }}>
                You are running the open-source community edition of VSCodium-Rust.
                All features are available without restrictions.
            </p>
            <p style={{ color: '#888', lineHeight: 1.6, marginTop: 12 }}>
                API keys can be configured in the <strong>Cloud API Keys</strong> settings panel.
            </p>
        </div>
    );
};

export default AccountSettingsPanel;
