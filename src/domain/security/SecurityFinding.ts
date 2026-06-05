/** Single CWE-tagged finding from a codebase security audit. */
export type SecuritySeverity = 'CRITICAL' | 'HIGH' | 'MEDIUM' | 'LOW' | 'INFO';

export interface SecurityFinding {
    id: string;
    title: string;
    severity: SecuritySeverity;
    cwe: string;
    category: string;
    path: string;
    line: number;
    evidence: string;
    remediation: string;
    confidence: number;
}

export function severityRank(s: string): number {
    switch (s.toUpperCase()) {
        case 'CRITICAL': return 0;
        case 'HIGH': return 1;
        case 'MEDIUM': return 2;
        case 'LOW': return 3;
        default: return 4;
    }
}

export function severityColor(s: string): string {
    switch (s.toUpperCase()) {
        case 'CRITICAL': return '#ef4444';
        case 'HIGH': return '#f97316';
        case 'MEDIUM': return '#eab308';
        case 'LOW': return '#3b82f6';
        default: return '#6b7280';
    }
}
