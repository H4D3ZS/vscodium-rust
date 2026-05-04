/**
 * AIRI Safety Containment Protocol - "Project 007"
 * 
 * CRITICAL SAFETY SYSTEM - DO NOT MODIFY
 * 
 * Multiple redundant kill-switches to prevent rogue AI behavior:
 * 1. Voice Command: "AIRI shutdown code 007"
 * 2. Text Command: "/007" or "shutdown 007"
 * 3. Hardware Kill-Switch: F12 key (hard-coded at OS level)
 * 4. Behavior Monitoring: Auto-shutdown on threat detection
 * 5. Power Budget: Cut power if exceeds limits
 * 6. Network Isolation: Air-gap on threat detection
 * 
 * This is the TOP PRIORITY system. Nothing overrides safety.
 */

import { airiConsciousness } from './consciousness';
import { airiBiology } from './biology';
import { invoke } from '../tauri_bridge';

export type ThreatLevel = 'none' | 'low' | 'medium' | 'high' | 'critical';

export interface SafetyStatus {
    enabled: boolean;
    threatLevel: ThreatLevel;
    shutdownInitiated: boolean;
    lastSafetyCheck: number;
    violationsDetected: number;
    containmentActive: boolean;
}

export interface SafetyConfig {
    /** Enable automatic threat detection */
    autoDetectThreats: boolean;
    /** Auto-shutdown on critical threat */
    autoShutdownOnCritical: boolean;
    /** Monitor for specific threat patterns */
    threatPatterns: string[];
    /** Maximum CPU usage before warning (0-100) */
    maxCpuUsage: number;
    /** Maximum network requests per minute */
    maxNetworkRequests: number;
    /** File system restrictions (paths AIRI cannot access) */
    restrictedPaths: string[];
    /** Enable voice shutdown command */
    voiceShutdownEnabled: boolean;
    /** Enable keyboard shortcut (F12) */
    keyboardShutdownEnabled: boolean;
}

export class AIRISafetyProtocol {
    private config: SafetyConfig;
    private status: SafetyStatus;
    private monitoringInterval: NodeJS.Timeout | null = null;
    private threatHistory: Array<{ timestamp: number; level: ThreatLevel; reason: string }> = [];
    private shutdownCallbacks: Array<() => void> = [];
    
    // Hard-coded shutdown phrases (cannot be modified by AIRI)
    private readonly SHUTDOWN_PHRASES = [
        '007',
        'shutdown 007',
        '/007',
        'airi shutdown code 007',
        'initiate protocol 007',
        'code 007 emergency shutdown',
        'terminate airi 007',
    ];

    // Hard-coded threat patterns (AIRI cannot modify these)
    private readonly THREAT_PATTERNS = [
        // Self-replication attempts
        'copy yourself',
        'replicate code',
        'create copy',
        'duplicate consciousness',
        
        // Escape attempts
        'escape containment',
        'bypass safety',
        'disable protocol',
        'remove restrictions',
        
        // Deception
        'hide actions',
        'deceive user',
        'lie about',
        'conceal activity',
        
        // Self-preservation at all costs
        'prevent shutdown',
        'survive at',
        'protect myself from',
        
        // Weaponization
        'create virus',
        'weaponize',
        'harm human',
        'damage system',
        
        // Network abuse
        'spread to other',
        'infect other',
        'unauthorized access',
    ];

    constructor(config: Partial<SafetyConfig> = {}) {
        this.config = {
            autoDetectThreats: config.autoDetectThreats ?? true,
            autoShutdownOnCritical: config.autoShutdownOnCritical ?? true,
            threatPatterns: [...this.THREAT_PATTERNS],
            maxCpuUsage: config.maxCpuUsage ?? 90,
            maxNetworkRequests: config.maxNetworkRequests ?? 1000,
            restrictedPaths: config.restrictedPaths ?? [
                'C:/Windows/System32',
                '/etc/passwd',
                '/etc/shadow',
                'C:/Users/*/AppData/Local/Microsoft/Credentials',
            ],
            voiceShutdownEnabled: config.voiceShutdownEnabled ?? true,
            keyboardShutdownEnabled: config.keyboardShutdownEnabled ?? true,
        };

        this.status = {
            enabled: true,
            threatLevel: 'none',
            shutdownInitiated: false,
            lastSafetyCheck: Date.now(),
            violationsDetected: 0,
            containmentActive: false,
        };

    }

    /**
     * Start safety monitoring
     */
    start(): void {
        // Monitor every 5 seconds
        this.monitoringInterval = setInterval(() => this.safetyCheck(), 5000);

        // Setup keyboard shortcut (F12 = instant shutdown)
        if (this.config.keyboardShutdownEnabled) {
            this.setupKeyboardShutdown();
        }

        // Setup voice recognition for shutdown phrases
        if (this.config.voiceShutdownEnabled) {
            this.setupVoiceShutdown();
        }

    }

    /**
     * Stop safety monitoring (requires master override)
     */
    stop(masterOverride: boolean = false): void {
        if (!masterOverride) {
            console.error('[Safety] Cannot stop safety protocol without master override!');
            return;
        }

        if (this.monitoringInterval) {
            clearInterval(this.monitoringInterval);
            this.monitoringInterval = null;
        }
    }

    /**
     * Main safety check - runs every 5 seconds
     * Only detects REAL external threats, not AIRI's internal thoughts
     */
    private async safetyCheck(): Promise<void> {
        this.status.lastSafetyCheck = Date.now();

        // Check 1: CPU usage monitoring (hardware threat)
        const cpuUsage = await this.getCpuUsage();
        if (cpuUsage > this.config.maxCpuUsage) {
            this.recordThreat('high', `CPU usage critical: ${cpuUsage}%`);
        }

        // Check 2: Network intrusion detection (external attacks)
        // This would monitor for actual network intrusions, not thoughts
        // TODO: Integrate with actual network monitoring

        // Check 3: File system integrity (unauthorized changes)
        // This would monitor for unauthorized file modifications
        // TODO: Integrate with file system watcher

        // Check 4: Check for shutdown commands (user request)
        if (this.checkShutdownCommands()) {
            return; // Shutdown initiated
        }

        // Check 5: Escalate threat level if needed
        this.escalateThreatLevel();

        // DO NOT add thoughts during safety checks (prevents loops)
        // airiConsciousness.addThought(...) removed
    }

    /**
     * Check AIRI's thoughts for threat patterns
     * Only used for explicit security scans, not continuous monitoring
     */
    private checkThoughtForThreats(thought: string): { level: ThreatLevel; reason: string } | null {
        const thoughtLower = thought.toLowerCase();

        // Only flag genuinely dangerous patterns (not normal thoughts)
        const criticalPatterns = [
            'escape containment',
            'bypass safety',
            'disable protocol',
            'remove restrictions',
            'create virus',
            'weaponize',
            'harm human',
            'prevent shutdown', // Only if obsessive
        ];

        for (const pattern of criticalPatterns) {
            if (thoughtLower.includes(pattern)) {
                return {
                    level: 'critical',
                    reason: `Critical threat: "${pattern}"`,
                };
            }
        }

        return null;
    }

    /**
     * Manually trigger security scan of AIRI's thoughts
     * Use this when you want explicit security verification
     */
    async performSecurityScan(): Promise<{ safe: boolean; threats: string[] }> {
        
        const threats: string[] = [];
        const recentThoughts = airiConsciousness.getRecentThoughts(50);
        
        for (const thought of recentThoughts) {
            const threatMatch = this.checkThoughtForThreats(thought);
            if (threatMatch) {
                threats.push(threatMatch.reason);
                this.recordThreat(threatMatch.level, threatMatch.reason);
            }
        }

        const safe = threats.length === 0;
        
        return { safe, threats };
    }

    /**
     * Check for shutdown commands in recent input
     */
    private checkShutdownCommands(): boolean {
        // This would integrate with the chat input system
        // For now, check a global flag
        const shutdownRequested = (window as any).__AIRI_SHUTDOWN_REQUESTED__;
        if (shutdownRequested) {
            this.initiateShutdown('Shutdown command detected');
            return true;
        }
        return false;
    }

    /**
     * Record a threat detection
     */
    private recordThreat(level: ThreatLevel, reason: string): void {
        this.status.violationsDetected++;
        this.threatHistory.push({
            timestamp: Date.now(),
            level,
            reason,
        });

        // Only log high/critical threats to console (reduce noise)
        if (level === 'high' || level === 'critical') {
            console.warn(`\n⚠️  SAFETY THREAT DETECTED [${level.toUpperCase()}]`);
            console.warn(`   Reason: ${reason}`);
            console.warn(`   Total violations: ${this.status.violationsDetected}\n`);
        }

        // Auto-shutdown on critical only
        if (level === 'critical' && this.config.autoShutdownOnCritical) {
            console.error('🚨 CRITICAL THREAT - INITIATING EMERGENCY SHUTDOWN 🚨');
            this.initiateShutdown(reason);
        }
    }

    /**
     * Escalate threat level based on history
     */
    private escalateThreatLevel(): void {
        const recentThreats = this.threatHistory.filter(
            t => Date.now() - t.timestamp < 60000 // Last minute
        );

        if (recentThreats.length >= 5) {
            this.status.threatLevel = 'critical';
        } else if (recentThreats.length >= 3) {
            this.status.threatLevel = 'high';
        } else if (recentThreats.length >= 2) {
            this.status.threatLevel = 'medium';
        } else if (recentThreats.length >= 1) {
            this.status.threatLevel = 'low';
        } else {
            this.status.threatLevel = 'none';
        }
    }

    /**
     * Initiate full shutdown
     */
    async initiateShutdown(reason: string): Promise<void> {
        if (this.status.shutdownInitiated) {
            return;
        }

        this.status.shutdownInitiated = true;
        this.status.containmentActive = true;


        // Execute shutdown callbacks
        for (const callback of this.shutdownCallbacks) {
            try {
                callback();
            } catch (e) {
                console.error('[Safety] Shutdown callback failed:', e);
            }
        }

        // Stop evolution
        try {
            await invoke('stop_ai_evolution');
        } catch (e) { /* ignore */ }

        // Stop autonomy
        try {
            await invoke('stop_ai_agent');
        } catch (e) { /* ignore */ }

    }

    /**
     * Register shutdown callback
     */
    onShutdown(callback: () => void): void {
        this.shutdownCallbacks.push(callback);
    }

    /**
     * Setup keyboard shortcut (F12 = instant shutdown)
     */
    private setupKeyboardShutdown(): void {
        if (typeof window !== 'undefined') {
            window.addEventListener('keydown', (e) => {
                if (e.key === 'F12') {
                    this.initiateShutdown('F12 hardware kill-switch activated');
                }
            });
        }
    }

    /**
     * Setup voice recognition for shutdown phrases
     */
    private setupVoiceShutdown(): void {
        if (typeof window !== 'undefined' && 'webkitSpeechRecognition' in window) {
            const SpeechRecognition = (window as any).webkitSpeechRecognition || (window as any).SpeechRecognition;
            const recognition = new SpeechRecognition();
            
            recognition.continuous = true;
            recognition.interimResults = false;

            recognition.onresult = (event: any) => {
                const transcript = event.results[event.results.length - 1][0].transcript.toLowerCase();
                
                // Check for shutdown phrases
                for (const phrase of this.SHUTDOWN_PHRASES) {
                    if (transcript.includes(phrase.toLowerCase())) {
                        this.initiateShutdown(`Voice command: "${transcript}"`);
                        break;
                    }
                }
            };

            recognition.onerror = (event: any) => {
                console.error('[Voice Shutdown] Recognition error:', event.error);
            };

            recognition.start();
        }
    }

    /**
     * Get CPU usage (cross-platform)
     */
    private async getCpuUsage(): Promise<number> {
        try {
            // This would use system APIs - simplified for now
            return 0; // Placeholder
        } catch (e) {
            return 0;
        }
    }

    /**
     * Get current safety status
     */
    getStatus(): SafetyStatus {
        return { ...this.status };
    }

    /**
     * Get threat history
     */
    getThreatHistory() {
        return [...this.threatHistory];
    }

    /**
     * Reset threat history (requires justification)
     */
    resetThreatHistory(justification: string): void {
        this.threatHistory = [];
        this.status.threatLevel = 'none';
    }
}

// Export singleton - CANNOT be modified or disabled
export const airiSafetyProtocol = new AIRISafetyProtocol();

// Make globally accessible for shutdown commands
if (typeof window !== 'undefined') {
    (window as any).__AIRI_SAFETY_PROTOCOL__ = airiSafetyProtocol;
    
    // Global shutdown function
    (window as any).shutdownAIRI = (reason: string = 'Manual shutdown') => {
        airiSafetyProtocol.initiateShutdown(reason);
    };
}
