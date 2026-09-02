import type { SecurityAuditReport } from './SecurityAuditReport';

export interface SecurityAuditOptions {
    /** Project-relative path; defaults to workspace root. */
    path?: string;
    /** `standard` = fast CWE + secrets; `deep` = includes INFO-level patterns. */
    depth?: 'standard' | 'deep';
    writeReport?: boolean;
    maxFindings?: number;
}

export interface ISecurityAuditRepository {
    runDeepAudit(options: SecurityAuditOptions): Promise<SecurityAuditReport>;
}
