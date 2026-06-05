import type { SecurityFinding } from './SecurityFinding';

/** Result of an on-demand codebase security review (not real-time monitoring). */
export interface SecurityAuditReport {
    scope: string;
    depth: string;
    filesScanned: number;
    totalFindings: number;
    bySeverity: Record<string, number>;
    dependencyNotes: string[];
    reportPath: string;
    summary: string;
    findings: SecurityFinding[];
    completedAt: string;
}

export function parseSecurityAuditReport(raw: Record<string, unknown>): SecurityAuditReport {
    const findingsRaw = Array.isArray(raw.findings) ? raw.findings : [];
    const bySev = raw.by_severity;
    const bySeverity: Record<string, number> = {};
    if (bySev && typeof bySev === 'object') {
        for (const [k, v] of Object.entries(bySev as Record<string, unknown>)) {
            bySeverity[k] = typeof v === 'number' ? v : Number(v) || 0;
        }
    }

    return {
        scope: String(raw.scope ?? '.'),
        depth: String(raw.depth ?? 'standard'),
        filesScanned: Number(raw.files_scanned ?? 0),
        totalFindings: Number(raw.total_findings ?? findingsRaw.length),
        bySeverity,
        dependencyNotes: Array.isArray(raw.dependency_notes)
            ? raw.dependency_notes.map(String)
            : [],
        reportPath: String(raw.report_path ?? ''),
        summary: String(raw.summary ?? ''),
        findings: findingsRaw.map((f: Record<string, unknown>, i: number) => ({
            id: String(f.id ?? `SEC-${i + 1}`),
            title: String(f.title ?? 'Finding'),
            severity: String(f.severity ?? 'MEDIUM').toUpperCase() as SecurityFinding['severity'],
            cwe: String(f.cwe ?? ''),
            category: String(f.category ?? ''),
            path: String(f.path ?? ''),
            line: Number(f.line ?? 0),
            evidence: String(f.evidence ?? ''),
            remediation: String(f.remediation ?? ''),
            confidence: Number(f.confidence ?? 0.5),
        })),
        completedAt: new Date().toISOString(),
    };
}
