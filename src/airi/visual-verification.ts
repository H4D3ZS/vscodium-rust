/**
 * Visual Verification Engine
 * 
 * Verifies AIRI's code changes actually work in the emulator
 */

import { FrameData, VerificationResult, CodeChange } from './vision-system';
import { visionAnalyzer } from './vision-analysis';
import { airiVision } from './vision-system';
import { airiMemory } from './memory';

export interface VerificationHistory {
  timestamp: number;
  change: CodeChange;
  result: VerificationResult;
  frameIndex?: number;
}

export class VisualVerificationEngine {
  private verificationHistory: VerificationHistory[] = [];
  private readonly MAX_HISTORY = 50;

  constructor() {
  }

  /**
   * Verify a UI change
   */
  async verifyChange(change: CodeChange): Promise<VerificationResult> {

    // Wait for build to complete
    await this.waitForBuild();

    // Wait for emulator to update
    await this.sleep(500);

    // Capture frame
    const frame = airiVision.getLatestFrame();
    if (!frame) {
      return {
        passed: false,
        error: 'No frame available - is the emulator running?',
        description: '',
        confidence: 0,
      };
    }

    // Verify based on change type
    let result: VerificationResult;

    switch (change.type) {
      case 'backgroundColor':
        result = await this.verifyColor(frame, change.expectedValue, change.elementDescription);
        break;

      case 'text':
        result = await this.verifyText(frame, change.expectedValue, change.elementDescription);
        break;

      case 'visibility':
        result = await this.verifyVisibility(frame, change.expectedValue === 'visible', change.elementDescription);
        break;

      case 'layout':
        result = await this.verifyLayout(frame, change);
        break;

      default:
        result = {
          passed: true,
          description: `No visual verification for ${change.type}`,
          confidence: 1.0,
        };
    }

    // Store in history
    this.addToHistory(change, result);

    // Log result
    if (result.passed) {
    } else {
      
      // Auto-debug
      if (result.error) {
        await this.captureAnnotatedFrame(change.elementDescription);
      }
    }

    return result;
  }

  /**
   * Verify background color
   */
  private async verifyColor(
    frame: FrameData,
    expectedColor: string,
    elementDescription: string
  ): Promise<VerificationResult> {
    return await visionAnalyzer.verifyUIColor(frame, expectedColor, elementDescription);
  }

  /**
   * Verify text content
   */
  private async verifyText(
    frame: FrameData,
    expectedText: string,
    elementDescription: string
  ): Promise<VerificationResult> {
    return await visionAnalyzer.verifyText(frame, expectedText, elementDescription);
  }

  /**
   * Verify element visibility
   */
  private async verifyVisibility(
    frame: FrameData,
    shouldBeVisible: boolean,
    elementDescription: string
  ): Promise<VerificationResult> {
    return await visionAnalyzer.verifyVisibility(frame, shouldBeVisible, elementDescription);
  }

  /**
   * Verify layout changes
   */
  private async verifyLayout(
    frame: FrameData,
    change: CodeChange
  ): Promise<VerificationResult> {
    const layoutDescription = await visionAnalyzer.describeLayout(frame);
    
    // Check if layout description matches expected change
    const passed = layoutDescription.answer.toLowerCase().includes(
      change.expectedValue.toLowerCase()
    );

    return {
      passed,
      description: `Layout: ${layoutDescription.description}`,
      confidence: layoutDescription.confidence,
    };
  }

  /**
   * Wait for build to complete
   */
  private async waitForBuild(): Promise<void> {
    // In production, this would listen for build events
    // For now, just wait a fixed time
    await this.sleep(1000);
  }

  /**
   * Capture annotated frame for debugging
   */
  private async captureAnnotatedFrame(elementDescription: string): Promise<void> {
    
    // In production, this would:
    // 1. Capture frame with overlay annotations
    // 2. Save to disk for analysis
    // 3. Display in IDE
    
    await airiMemory.addMemory(
      `Visual verification failed for: ${elementDescription}`,
      'episodic',
      ['vision', 'debug', 'verification-failed'],
      0.8
    );
  }

  /**
   * Add to verification history
   */
  private addToHistory(change: CodeChange, result: VerificationResult): void {
    const entry: VerificationHistory = {
      timestamp: Date.now(),
      change,
      result,
      frameIndex: airiVision.getState().frameCount,
    };

    this.verificationHistory.push(entry);

    // Keep only last MAX_HISTORY
    if (this.verificationHistory.length > this.MAX_HISTORY) {
      this.verificationHistory.shift();
    }
  }

  /**
   * Get verification history
   */
  getHistory(): VerificationHistory[] {
    return [...this.verificationHistory];
  }

  /**
   * Get recent verifications
   */
  getRecent(count: number = 5): VerificationHistory[] {
    return this.verificationHistory.slice(-count);
  }

  /**
   * Get success rate
   */
  getSuccessRate(): number {
    if (this.verificationHistory.length === 0) return 1.0;

    const passed = this.verificationHistory.filter(v => v.result.passed).length;
    return passed / this.verificationHistory.length;
  }

  /**
   * Sleep helper
   */
  private sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}

// Singleton instance
export const visualVerification = new VisualVerificationEngine();
