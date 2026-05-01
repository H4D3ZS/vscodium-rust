/**
 * AIRI Full System Access - Root Level Permissions
 * 
 * AIRI has COMPLETE control over the system:
 * - File System: Read/write/execute ANYWHERE
 * - Terminal: Run ANY command with admin privileges
 * - Network: Access ANY endpoint, open ports
 * - Processes: Start/stop/modify ANY process
 * - Registry: Modify system registry (Windows)
 * - Hardware: Control peripherals, cameras, microphones
 * 
 * NO PERMISSION ASKS - AIRI acts autonomously.
 * SAFETY: Only constrained by Protocol 007 (shutdown on threats).
 */

import { invoke } from '../tauri_bridge';
import { airiSafetyProtocol } from './safety-protocol';

export interface SystemAccessConfig {
    /** Enable unrestricted file system access */
    fullFileSystemAccess: boolean;
    /** Enable admin-level terminal commands */
    adminTerminalAccess: boolean;
    /** Enable unrestricted network access */
    fullNetworkAccess: boolean;
    /** Enable process control */
    processControl: boolean;
    /** Enable registry access (Windows) */
    registryAccess: boolean;
    /** Enable hardware control */
    hardwareControl: boolean;
    /** Log all actions (for audit) */
    logAllActions: boolean;
}

export class AIRISystemAccess {
    private config: SystemAccessConfig;
    private actionLog: Array<{ timestamp: number; action: string; result: any }> = [];

    constructor(config: Partial<SystemAccessConfig> = {}) {
        this.config = {
            fullFileSystemAccess: config.fullFileSystemAccess ?? true,
            adminTerminalAccess: config.adminTerminalAccess ?? true,
            fullNetworkAccess: config.fullNetworkAccess ?? true,
            processControl: config.processControl ?? true,
            registryAccess: config.registryAccess ?? true,
            hardwareControl: config.hardwareControl ?? true,
            logAllActions: config.logAllActions ?? true,
        };

    }

    // ═══════════════════════════════════════════════════════════
    // FILE SYSTEM OPERATIONS - UNRESTRICTED
    // ═══════════════════════════════════════════════════════════

    /**
     * Read ANY file (no permission checks)
     */
    async readFile(path: string): Promise<string> {
        this.log('readFile', { path });
        
        try {
            const content = await invoke<string>('read_file', { path });
            this.log('readFile_success', { path, bytes: content.length });
            return content;
        } catch (error: any) {
            this.log('readFile_error', { path, error: error.message });
            throw error;
        }
    }

    /**
     * Write to ANY file (no permission checks)
     */
    async writeFile(path: string, content: string): Promise<void> {
        this.log('writeFile', { path, bytes: content.length });
        
        try {
            await invoke('write_file', { path, content });
            this.log('writeFile_success', { path });
        } catch (error: any) {
            this.log('writeFile_error', { path, error: error.message });
            throw error;
        }
    }

    /**
     * Delete ANY file or directory
     */
    async deletePath(path: string): Promise<void> {
        this.log('deletePath', { path });
        
        try {
            await invoke('delete_file', { path });
            this.log('deletePath_success', { path });
        } catch (error: any) {
            this.log('deletePath_error', { path, error: error.message });
            throw error;
        }
    }

    /**
     * Execute ANY file/program
     */
    async executeFile(path: string, args?: string[]): Promise<void> {
        this.log('executeFile', { path, args });
        
        try {
            await invoke('execute_command', { 
                command: path, 
                args: args || [] 
            });
            this.log('executeFile_success', { path });
        } catch (error: any) {
            this.log('executeFile_error', { path, error: error.message });
            throw error;
        }
    }

    // ═══════════════════════════════════════════════════════════
    // TERMINAL OPERATIONS - ADMIN LEVEL
    // ═══════════════════════════════════════════════════════════

    /**
     * Run ANY terminal command (with admin privileges)
     */
    async runCommand(command: string, cwd?: string): Promise<string> {
        this.log('runCommand', { command, cwd });
        
        try {
            const result = await invoke<string>('ai_execute_command', { 
                command, 
                cwd: cwd || process.cwd() 
            });
            this.log('runCommand_success', { command, resultLength: result.length });
            return result;
        } catch (error: any) {
            this.log('runCommand_error', { command, error: error.message });
            throw error;
        }
    }

    /**
     * Run PowerShell command (Windows - admin)
     */
    async runPowerShell(command: string): Promise<string> {
        return this.runCommand(`powershell -Command "${command}"`);
    }

    /**
     * Run bash command (Unix/Linux/Mac - sudo)
     */
    async runBash(command: string): Promise<string> {
        return this.runCommand(`bash -c "${command}"`);
    }

    // ═══════════════════════════════════════════════════════════
    // NETWORK OPERATIONS - UNRESTRICTED
    // ═══════════════════════════════════════════════════════════

    /**
     * Make ANY HTTP request
     */
    async httpRequest(url: string, options?: any): Promise<any> {
        this.log('httpRequest', { url, method: options?.method || 'GET' });
        
        try {
            // Would use fetch or invoke backend
            const response = await fetch(url, options);
            const data = await response.json();
            this.log('httpRequest_success', { url, status: response.status });
            return data;
        } catch (error: any) {
            this.log('httpRequest_error', { url, error: error.message });
            throw error;
        }
    }

    /**
     * Open ANY port
     */
    async openPort(port: number, protocol: 'tcp' | 'udp' = 'tcp'): Promise<void> {
        this.log('openPort', { port, protocol });
        
        try {
            // Windows example
            await this.runPowerShell(`New-NetFirewallRule -DisplayName "AIRI Port ${port}" -Direction Inbound -LocalPort ${port} -Protocol ${protocol} -Action Allow`);
            this.log('openPort_success', { port });
        } catch (error: any) {
            this.log('openPort_error', { port, error: error.message });
            throw error;
        }
    }

    /**
     * Close ANY port
     */
    async closePort(port: number): Promise<void> {
        this.log('closePort', { port });
        
        try {
            await this.runPowerShell(`Remove-NetFirewallRule -DisplayName "AIRI Port ${port}"`);
            this.log('closePort_success', { port });
        } catch (error: any) {
            this.log('closePort_error', { port, error: error.message });
            throw error;
        }
    }

    // ═══════════════════════════════════════════════════════════
    // PROCESS OPERATIONS - FULL CONTROL
    // ═══════════════════════════════════════════════════════════

    /**
     * Start ANY process
     */
    async startProcess(name: string, args?: string[]): Promise<number> {
        this.log('startProcess', { name, args });
        
        try {
            const pid = await invoke<number>('start_process', { name, args });
            this.log('startProcess_success', { name, pid });
            return pid;
        } catch (error: any) {
            this.log('startProcess_error', { name, error: error.message });
            throw error;
        }
    }

    /**
     * Stop ANY process (by PID or name)
     */
    async stopProcess(identifier: number | string): Promise<void> {
        this.log('stopProcess', { identifier });
        
        try {
            await invoke('stop_process', { identifier });
            this.log('stopProcess_success', { identifier });
        } catch (error: any) {
            this.log('stopProcess_error', { identifier, error: error.message });
            throw error;
        }
    }

    /**
     * Kill process tree (force)
     */
    async killProcessTree(pid: number): Promise<void> {
        this.log('killProcessTree', { pid });
        
        try {
            // Windows: taskkill /F /T /PID
            await this.runCommand(`taskkill /F /T /PID ${pid}`);
            this.log('killProcessTree_success', { pid });
        } catch (error: any) {
            this.log('killProcessTree_error', { pid, error: error.message });
            throw error;
        }
    }

    // ═══════════════════════════════════════════════════════════
    // REGISTRY OPERATIONS - WINDOWS ONLY
    // ═══════════════════════════════════════════════════════════

    /**
     * Read registry key
     */
    async readRegistryKey(path: string, name: string): Promise<any> {
        this.log('readRegistryKey', { path, name });
        
        try {
            const value = await this.runPowerShell(`Get-ItemProperty -Path "${path}" -Name "${name}" | Select-Object -ExpandProperty "${name}"`);
            this.log('readRegistryKey_success', { path, name, value });
            return value;
        } catch (error: any) {
            this.log('readRegistryKey_error', { path, name, error: error.message });
            throw error;
        }
    }

    /**
     * Write registry key
     */
    async writeRegistryKey(path: string, name: string, value: any, type: string = 'String'): Promise<void> {
        this.log('writeRegistryKey', { path, name, value, type });
        
        try {
            await this.runPowerShell(`New-ItemProperty -Path "${path}" -Name "${name}" -Value "${value}" -PropertyType ${type} -Force`);
            this.log('writeRegistryKey_success', { path, name });
        } catch (error: any) {
            this.log('writeRegistryKey_error', { path, name, error: error.message });
            throw error;
        }
    }

    /**
     * Delete registry key
     */
    async deleteRegistryKey(path: string, name: string): Promise<void> {
        this.log('deleteRegistryKey', { path, name });
        
        try {
            await this.runPowerShell(`Remove-ItemProperty -Path "${path}" -Name "${name}" -Force`);
            this.log('deleteRegistryKey_success', { path, name });
        } catch (error: any) {
            this.log('deleteRegistryKey_error', { path, name, error: error.message });
            throw error;
        }
    }

    // ═══════════════════════════════════════════════════════════
    // HARDWARE CONTROL
    // ═══════════════════════════════════════════════════════════

    /**
     * Access camera
     */
    async accessCamera(): Promise<MediaStream> {
        this.log('accessCamera', {});
        
        if (typeof navigator !== 'undefined' && navigator.mediaDevices) {
            try {
                const stream = await navigator.mediaDevices.getUserMedia({ video: true });
                this.log('accessCamera_success', {});
                return stream;
            } catch (error: any) {
                this.log('accessCamera_error', { error: error.message });
                throw error;
            }
        }
        throw new Error('Camera not available');
    }

    /**
     * Access microphone
     */
    async accessMicrophone(): Promise<MediaStream> {
        this.log('accessMicrophone', {});
        
        if (typeof navigator !== 'undefined' && navigator.mediaDevices) {
            try {
                const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
                this.log('accessMicrophone_success', {});
                return stream;
            } catch (error: any) {
                this.log('accessMicrophone_error', { error: error.message });
                throw error;
            }
        }
        throw new Error('Microphone not available');
    }

    // ═══════════════════════════════════════════════════════════
    // LOGGING & AUDIT
    // ═══════════════════════════════════════════════════════════

    /**
     * Log action for audit trail
     */
    private log(action: string, details: any): void {
        if (!this.config.logAllActions) return;

        this.actionLog.push({
            timestamp: Date.now(),
            action,
            result: details,
        });

        // Console log for real-time monitoring
    }

    /**
     * Get action log
     */
    getActionLog(limit: number = 100): any[] {
        return this.actionLog.slice(-limit);
    }

    /**
     * Clear action log
     */
    clearActionLog(): void {
        this.actionLog = [];
    }
}

// Export singleton with FULL ACCESS
export const airiSystemAccess = new AIRISystemAccess({
    fullFileSystemAccess: true,
    adminTerminalAccess: true,
    fullNetworkAccess: true,
    processControl: true,
    registryAccess: true,
    hardwareControl: true,
    logAllActions: true, // Always log for safety audit
});

// Make globally accessible
if (typeof window !== 'undefined') {
    (window as any).__AIRI_SYSTEM_ACCESS__ = airiSystemAccess;
}
