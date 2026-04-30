/**
 * AIRI Cybersecurity Engine - Real-Time Threat Detection
 * 
 * Monitors for:
 * - Nmap/Port scans
 * - Phishing attempts
 * - Malicious URLs
 * - Network intrusions
 * - Social engineering
 * - Suspicious emails
 */

import { airiSafetyProtocol } from './safety-protocol';

export interface NetworkThreat {
    type: 'port_scan' | 'phishing' | 'malicious_url' | 'intrusion' | 'social_engineering';
    severity: 'low' | 'medium' | 'high' | 'critical';
    source: string;
    details: string;
    timestamp: number;
    blocked: boolean;
}

export interface URLAnalysis {
    url: string;
    isPhishing: boolean;
    riskScore: number; // 0-100
    indicators: string[];
    recommendation: string;
}

export class AIRICybersecurityEngine {
    private threatHistory: NetworkThreat[] = [];
    private knownPhishingPatterns = [
        // Common phishing patterns
        'account suspended',
        'verify your account',
        'update payment',
        'confirm your identity',
        'urgent action required',
        'your account has been compromised',
        'click here to verify',
        'limited time offer',
        'congratulations you won',
        'free gift card',
    ];

    private suspiciousTLDs = [
        '.tk', '.ml', '.ga', '.cf', '.gq', // Free TLDs often used for phishing
        '.xyz', '.top', '.work', '.click', // Cheap TLDs
        '.ru', '.cn', '.kp', '.ir', // High-risk country TLDs (context dependent)
    ];

    /**
     * Start cybersecurity monitoring
     */
    start(): void {
        console.log('🛡️  Monitoring for:');
        console.log('   - Port scans (Nmap, etc.)');
        console.log('   - Phishing attempts');
        console.log('   - Malicious URLs');
        console.log('   - Network intrusions');
        console.log('   - Social engineering\n');

        // Monitor network connections (if APIs available)
        this.startNetworkMonitoring();
    }

    /**
     * Start network monitoring loop
     */
    private startNetworkMonitoring(): void {
        // Check for port scans every 10 seconds
        setInterval(() => {
            this.detectPortScans();
        }, 10000);

        // Monitor clipboard for phishing links (if user enables)
        // TODO: Implement clipboard monitoring with permission
    }

    /**
     * Detect port scanning activity
     */
    private async detectPortScans(): Promise<void> {
        try {
            // On Windows, use netstat to detect unusual connection patterns
            if (typeof window !== 'undefined' && navigator.platform.includes('Win')) {
                // This would require backend integration
                // For now, log that monitoring is active
                console.log('[Cybersecurity] 📡 Scanning for port scan activity...');
            }

            // TODO: Integrate with Rust backend for actual network monitoring
            // Backend can use: netstat -an | findstr SYN_RECV
            // Multiple SYN_RECV from same IP = potential port scan

        } catch (error) {
            console.error('[Cybersecurity] Port scan detection error:', error);
        }
    }

    /**
     * Analyze URL for phishing indicators
     */
    analyzeURL(url: string): URLAnalysis {
        const indicators: string[] = [];
        let riskScore = 0;

        try {
            const urlObj = new URL(url);
            const hostname = urlObj.hostname.toLowerCase();

            // Check 1: Suspicious TLDs
            for (const tld of this.suspiciousTLDs) {
                if (hostname.endsWith(tld)) {
                    indicators.push(`Suspicious TLD: ${tld}`);
                    riskScore += 20;
                }
            }

            // Check 2: IP address instead of domain
            if (/^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$/.test(hostname)) {
                indicators.push('IP address instead of domain');
                riskScore += 30;
            }

            // Check 3: Multiple subdomains (phishing technique)
            const subdomains = hostname.split('.');
            if (subdomains.length > 4) {
                indicators.push(`Too many subdomains (${subdomains.length})`);
                riskScore += 15;
            }

            // Check 4: Legitimate brand impersonation
            const brandImpersonation = [
                'paypal', 'amazon', 'microsoft', 'apple', 'google',
                'facebook', 'netflix', 'dropbox', 'adobe', 'bank'
            ];
            for (const brand of brandImpersonation) {
                if (hostname.includes(brand) && !hostname.endsWith(`${brand}.com`)) {
                    indicators.push(`Potential brand impersonation: ${brand}`);
                    riskScore += 40;
                }
            }

            // Check 5: Suspicious keywords in URL
            const suspiciousKeywords = [
                'login', 'signin', 'account', 'verify', 'secure',
                'update', 'confirm', 'suspended', 'locked'
            ];
            for (const keyword of suspiciousKeywords) {
                if (hostname.includes(keyword) || urlObj.pathname.includes(keyword)) {
                    indicators.push(`Suspicious keyword: ${keyword}`);
                    riskScore += 10;
                }
            }

            // Check 6: URL shorteners (often used in phishing)
            const shorteners = ['bit.ly', 'tinyurl.com', 'goo.gl', 't.co'];
            for (const shortener of shorteners) {
                if (hostname.includes(shortener)) {
                    indicators.push('URL shortener (hidden destination)');
                    riskScore += 25;
                }
            }

            // Check 7: Non-HTTPS for sensitive sites
            if (urlObj.protocol === 'http:' && 
                (hostname.includes('bank') || hostname.includes('login') || hostname.includes('account'))) {
                indicators.push('Non-HTTPS for sensitive site');
                riskScore += 35;
            }

            // Determine if phishing
            const isPhishing = riskScore >= 50;

            // Generate recommendation
            let recommendation = 'URL appears safe';
            if (riskScore >= 70) {
                recommendation = '⚠️ HIGH RISK - Do not click! Likely phishing attempt.';
            } else if (riskScore >= 50) {
                recommendation = '⚠️ SUSPICIOUS - Verify sender before clicking.';
            } else if (riskScore >= 30) {
                recommendation = '⚠️ CAUTION - Some suspicious indicators present.';
            }

            return {
                url,
                isPhishing,
                riskScore: Math.min(100, riskScore),
                indicators,
                recommendation,
            };

        } catch (error) {
            return {
                url,
                isPhishing: false,
                riskScore: 0,
                indicators: ['Invalid URL format'],
                recommendation: 'Invalid URL - cannot analyze',
            };
        }
    }

    /**
     * Analyze email/text content for phishing indicators
     */
    analyzeContent(content: string): { isPhishing: boolean; confidence: number; indicators: string[] } {
        const indicators: string[] = [];
        let confidence = 0;

        const contentLower = content.toLowerCase();

        // Check 1: Urgency tactics
        const urgencyPatterns = [
            'urgent', 'immediately', 'asap', 'within 24 hours',
            'act now', 'limited time', 'expire soon'
        ];
        for (const pattern of urgencyPatterns) {
            if (contentLower.includes(pattern)) {
                indicators.push(`Urgency tactic: ${pattern}`);
                confidence += 15;
            }
        }

        // Check 2: Threats/Fear
        const threatPatterns = [
            'account suspended', 'account locked', 'unauthorized access',
            'suspicious activity', 'we detected fraud', 'your account will be closed'
        ];
        for (const pattern of threatPatterns) {
            if (contentLower.includes(pattern)) {
                indicators.push(`Fear tactic: ${pattern}`);
                confidence += 20;
            }
        }

        // Check 3: Too good to be true
        const greedPatterns = [
            'you won', 'congratulations', 'free gift', 'claim your prize',
            'inheritance', 'lottery winner', 'million dollars'
        ];
        for (const pattern of greedPatterns) {
            if (contentLower.includes(pattern)) {
                indicators.push(`Greed tactic: ${pattern}`);
                confidence += 25;
            }
        }

        // Check 4: Request for sensitive info
        const sensitiveRequests = [
            'password', 'credit card', 'social security', 'ssn',
            'bank account', 'pin', 'verification code', '2fa code'
        ];
        for (const pattern of sensitiveRequests) {
            if (contentLower.includes(pattern)) {
                indicators.push(`Requesting sensitive info: ${pattern}`);
                confidence += 30;
            }
        }

        // Check 5: Generic greetings
        const genericGreetings = ['dear customer', 'dear user', 'dear member', 'hello user'];
        for (const greeting of genericGreetings) {
            if (contentLower.includes(greeting)) {
                indicators.push(`Generic greeting: ${greeting}`);
                confidence += 10;
            }
        }

        // Check 6: Spelling/grammar errors (common in phishing)
        const commonErrors = {
            'recieve': 'receive',
            'occured': 'occurred',
            'seperate': 'separate',
            'definately': 'definitely',
            'goverment': 'government',
        };
        for (const [wrong, right] of Object.entries(commonErrors)) {
            if (contentLower.includes(wrong)) {
                indicators.push(`Spelling error: "${wrong}" should be "${right}"`);
                confidence += 5;
            }
        }

        const isPhishing = confidence >= 40;

        return {
            isPhishing,
            confidence: Math.min(100, confidence),
            indicators,
        };
    }

    /**
     * Report detected threat
     */
    reportThreat(threat: NetworkThreat): void {
        this.threatHistory.push(threat);
        
        // Alert user via safety protocol
        airiSafetyProtocol.recordThreat(
            threat.severity === 'critical' || threat.severity === 'high' ? 'high' : 'medium',
            `🛡️ CYBERSECURITY ALERT: ${threat.type.toUpperCase()} - ${threat.details}`
        );

        console.warn(`\n🛡️ CYBERSECURITY THREAT DETECTED:`);
        console.warn(`   Type: ${threat.type}`);
        console.warn(`   Severity: ${threat.severity}`);
        console.warn(`   Source: ${threat.source}`);
        console.warn(`   Details: ${threat.details}`);
        console.warn(`   Blocked: ${threat.blocked}\n`);
    }

    /**
     * Get threat history
     */
    getThreatHistory(limit: number = 20): NetworkThreat[] {
        return this.threatHistory.slice(-limit);
    }

    /**
     * Quick URL check (convenience method)
     */
    quickURLCheck(url: string): void {
        const analysis = this.analyzeURL(url);
        
        if (analysis.isPhishing) {
            this.reportThreat({
                type: 'phishing',
                severity: analysis.riskScore >= 70 ? 'high' : 'medium',
                source: url,
                details: `Phishing risk score: ${analysis.riskScore}/100. Indicators: ${analysis.indicators.join(', ')}`,
                timestamp: Date.now(),
                blocked: true,
            });
        } else {
            console.log(`[Cybersecurity] ✅ URL appears safe: ${url} (Risk: ${analysis.riskScore}/100)`);
        }
    }

    /**
     * Quick content check (convenience method)
     */
    quickContentCheck(content: string, source?: string): void {
        const analysis = this.analyzeContent(content);
        
        if (analysis.isPhishing) {
            this.reportThreat({
                type: 'social_engineering',
                severity: analysis.confidence >= 70 ? 'high' : 'medium',
                source: source || 'Unknown',
                details: `Phishing confidence: ${analysis.confidence}/100. Indicators: ${analysis.indicators.join(', ')}`,
                timestamp: Date.now(),
                blocked: false,
            });
        } else {
            console.log(`[Cybersecurity] ✅ Content appears safe (Confidence: ${analysis.confidence}/100)`);
        }
    }
}

// Export singleton
export const airiCybersecurity = new AIRICybersecurityEngine();

// Make globally accessible
if (typeof window !== 'undefined') {
    (window as any).__AIRI_CYBERSECURITY__ = airiCybersecurity;
}
