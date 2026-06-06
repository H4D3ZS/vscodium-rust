import React from 'react';
import MacIOSSimulatorPanel from './MacIOSSimulatorPanel';
import IPhoneAcheronPanel from './IPhoneAcheronPanel';

function isMacOS(): boolean {
    if (typeof navigator === 'undefined') return false;
    const platform = navigator.platform || '';
    const ua = navigator.userAgent || '';
    return /Mac/i.test(platform) || /Mac OS X/i.test(ua);
}

/** Routes to Xcode headless mirror on macOS; acheron/vPhone hypervisor elsewhere. */
const IPhoneEmulatorPanel: React.FC = () => {
    if (isMacOS()) {
        return <MacIOSSimulatorPanel />;
    }
    return <IPhoneAcheronPanel />;
};

export default IPhoneEmulatorPanel;
