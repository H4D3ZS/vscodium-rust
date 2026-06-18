// @ts-nocheck — work-in-progress AIRI subsystem; types stabilised once interfaces settle.
/**
 * AIRI Tool Orchestrator - External Security Tool Integration
 * 
 * Orchestrates specialized security tools:
 * - FlutterSentinel (mobile app security)
 * - DissectX_Pro (phishing/red teaming)
 * - Other specialized tools
 * 
 * Architecture: Keep tools separate, AIRI coordinates via CLI/API
 */

export interface ExternalTool {
    name: string;
    type: 'mobile' | 'phishing' | 'web' | 'network' | 'social';
    path: string;
    command: string;
    args: string[];
    outputFormat: 'json' | 'text' | 'xml';
    isActive: boolean;
}

export interface OrchestratedScan {
    id: string;
    target: string;
    tools: string[];
    startTime: number;
    endTime?: number;
    status: 'running' | 'completed' | 'failed';
    results: ToolResult[];
}

export interface ToolResult {
    tool: string;
    success: boolean;
    output: string;
    findings: number;
    severity: 'critical' | 'high' | 'medium' | 'low' | 'info';
}

export class AIRIToolOrchestrator {
    private registeredTools: Map<string, ExternalTool> = new Map();
    private activeScans: Map<string, OrchestratedScan> = new Map();

    constructor() {
    }

    /**
     * Register FlutterSentinel for mobile app security
     */
    registerFlutterSentinel(projectPath: string): void {
        const tool: ExternalTool = {
            name: 'FlutterSentinel',
            type: 'mobile',
            path: projectPath,
            command: 'flutter',
            args: ['pub', 'run', 'flutter_sentinel'],
            outputFormat: 'json',
            isActive: true,
        };

        this.registeredTools.set('flutter_sentinel', tool);
    }

    /**
     * Register DissectX_Pro for phishing/red teaming
     */
    registerDissectXPro(projectPath: string): void {
        const tool: ExternalTool = {
            name: 'DissectX_Pro',
            type: 'phishing',
            path: projectPath,
            command: 'python',
            args: ['dissectpro.py'],
            outputFormat: 'json',
            isActive: true,
        };

        this.registeredTools.set('dissectx_pro', tool);
    }

    /**
     * Run coordinated multi-tool scan
     */
    async runCoordinatedScan(
        target: string,
        toolNames: string[]
    ): Promise<OrchestratedScan> {
        const scanId = `scan_${Date.now()}`;
        

        const scan: OrchestratedScan = {
            id: scanId,
            target,
            tools: toolNames,
            startTime: Date.now(),
            status: 'running',
            results: [],
        };

        this.activeScans.set(scanId, scan);

        // Run each tool
        for (const toolName of toolNames) {
            const tool = this.registeredTools.get(toolName);
            if (!tool) {
 console.warn(` Tool not found: ${toolName}`);
                continue;
            }

            
            try {
                const result = await this.executeTool(tool, target);
                scan.results.push(result);
            } catch (error) {
 console.error(` ${tool.name} failed:`, error);
                scan.results.push({
                    tool: tool.name,
                    success: false,
                    output: `Error: ${error}`,
                    findings: 0,
                    severity: 'info',
                });
            }
        }

        scan.endTime = Date.now();
        scan.status = 'completed';

        // Summary
        const totalFindings = scan.results.reduce((sum, r) => sum + r.findings, 0);

        return scan;
    }

    /**
     * Execute external tool
     */
    private async executeTool(
        tool: ExternalTool,
        target: string
    ): Promise<ToolResult> {
        // Really spawn the external tool via the Rust backend executor and
        // parse its actual output. (Previously this slept 2s and returned
        // random findings.)
        const cmdLine = [tool.command, ...(tool.args || []), target]
            .filter(Boolean)
            .join(' ');
        try {
            const { invoke } = await import('@tauri-apps/api/core');
            const output = await invoke<string>('ai_execute_command', {
                command: cmdLine,
                cwd: tool.path || undefined,
                timeout: 300_000,
            });
            const findings = this.parseFindings(output, tool.outputFormat);
            const failed = /command not found|is not recognized|no such file|cannot find|fatal error/i
                .test(output.slice(0, 400));
            return {
                tool: tool.name,
                success: !failed,
                output,
                findings,
                severity: findings >= 5 ? 'high' : findings > 0 ? 'medium' : 'info',
            };
        } catch (e: any) {
            return {
                tool: tool.name,
                success: false,
                output: `Failed to run ${tool.name}: ${e?.message || String(e)}`,
                findings: 0,
                severity: 'info',
            };
        }
    }

    /** Count real findings from a tool's output (JSON array/field, or text keywords). */
    private parseFindings(output: string, format: 'json' | 'text' | 'xml'): number {
        if (!output) return 0;
        if (format === 'json') {
            try {
                const data = JSON.parse(output);
                if (Array.isArray(data)) return data.length;
                if (Array.isArray(data?.findings)) return data.findings.length;
                if (Array.isArray(data?.vulnerabilities)) return data.vulnerabilities.length;
                if (typeof data?.count === 'number') return data.count;
            } catch { /* fall through to text scan */ }
        }
        // Text/XML: count lines that look like findings.
        const re = /\b(vuln|vulnerabilit|finding|critical|high|medium|cve-|insecure|exposed|leak|misconfig)/i;
        return output.split(/\r?\n/).filter(l => re.test(l)).length;
    }

    /**
     * Mobile App Security Scan (FlutterSentinel)
     */
    async scanMobileApp(appPath: string): Promise<OrchestratedScan> {
        
        if (!this.registeredTools.has('flutter_sentinel')) {
            throw new Error('FlutterSentinel not registered. Call registerFlutterSentinel() first.');
        }

        return this.runCoordinatedScan(appPath, ['flutter_sentinel']);
    }

    /**
     * Phishing Campaign (DissectX_Pro)
     */
    async runPhishingCampaign(
        targetDomain: string,
        campaignType: 'spear' | 'whale' | 'bulk'
    ): Promise<OrchestratedScan> {
        
        if (!this.registeredTools.has('dissectx_pro')) {
            throw new Error('DissectX_Pro not registered. Call registerDissectXPro() first.');
        }

        return this.runCoordinatedScan(targetDomain, ['dissectx_pro']);
    }

    /**
     * Full Spectrum Assessment (all tools)
     */
    async fullSpectrumAssessment(target: string): Promise<OrchestratedScan> {

        const allTools = Array.from(this.registeredTools.keys());
        return this.runCoordinatedScan(target, allTools);
    }

    /**
     * Get scan results
     */
    getScanResults(scanId: string): OrchestratedScan | undefined {
        return this.activeScans.get(scanId);
    }

    /**
     * List registered tools
     */
    listTools(): ExternalTool[] {
        return Array.from(this.registeredTools.values());
    }

    /**
     * Generate unified report
     */
    generateUnifiedReport(scanId: string): string {
        const scan = this.activeScans.get(scanId);
        if (!scan) {
            return 'Scan not found';
        }

        
        report += `Target: ${scan.target}\n`;
        report += `Date: ${new Date(scan.startTime).toLocaleString()}\n`;
        report += `Tools Used: ${scan.tools.join(', ')}\n`;
        report += `Status: ${scan.status}\n\n`;

        report += `═══════════════════════════════════════════════════════════\n`;
        report += `TOOL RESULTS\n`;
        report += `═══════════════════════════════════════════════════════════\n\n`;

        for (const result of scan.results) {
            report += `[${result.tool}]\n`;
            report += `   Success: ${result.success}\n`;
            report += `   Findings: ${result.findings}\n`;
            report += `   Severity: ${result.severity}\n`;
            report += `   Output: ${result.output}\n\n`;
        }

        if (scan.endTime) {
            report += `═══════════════════════════════════════════════════════════\n`;
            report += `Duration: ${((scan.endTime - scan.startTime) / 1000).toFixed(1)}s\n`;
        }

        return report;
    }
}

// Export singleton
export const airiOrchestrator = new AIRIToolOrchestrator();

// Make globally accessible
if (typeof window !== 'undefined') {
    (window as any).__AIRI_ORCHESTRATOR__ = airiOrchestrator;
}
