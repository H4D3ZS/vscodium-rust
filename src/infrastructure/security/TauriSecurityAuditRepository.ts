import { invoke } from '../../tauri_bridge';
import type { ISecurityAuditRepository, SecurityAuditOptions } from '../../domain/security/ISecurityAuditRepository';
import { parseSecurityAuditReport } from '../../domain/security/SecurityAuditReport';

/**
 * Adapter: invokes the Rust `deep_security_audit` tool via `call_tool`.
 * Multi-pass: secrets sweep → CWE heuristics → dependency posture notes.
 */
export class TauriSecurityAuditRepository implements ISecurityAuditRepository {
    async runDeepAudit(options: SecurityAuditOptions = {}) {
        const raw = await invoke<Record<string, unknown>>('call_tool', {
            name: 'deep_security_audit',
            arguments: {
                path: options.path ?? '.',
                depth: options.depth ?? 'deep',
                write_report: options.writeReport ?? true,
                max_findings: options.maxFindings ?? 500,
            },
        });
        return parseSecurityAuditReport(raw);
    }
}

export const securityAuditRepository = new TauriSecurityAuditRepository();
