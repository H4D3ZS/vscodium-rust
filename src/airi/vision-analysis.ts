/**
 * AIRI Vision Analysis
 * 
 * Moondream VLM integration for UI understanding
 */

import { Ollama } from 'ollama';
import { FrameData, VisionResponse, VerificationResult } from './vision-system';

export interface AnalysisOptions {
  temperature?: number;
  maxTokens?: number;
  stream?: boolean;
}

export class VisionAnalyzer {
  private ollama: Ollama;
  private model: string = 'moondream';

  constructor(host: string = 'http://localhost:11434') {
    this.ollama = new Ollama({ host });
    console.log('🧠 [AIRI Vision Analyzer] Moondream analyzer ready');
  }

  /**
   * Analyze a frame with a question
   */
  async analyzeFrame(
    frame: FrameData,
    question: string,
    options: AnalysisOptions = {}
  ): Promise<VisionResponse> {
    // Convert frame to base64
    const frameBase64 = frame.buffer.toString('base64');

    const response = await this.ollama.generate({
      model: this.model,
      prompt: question,
      images: [frameBase64],
      stream: options.stream ?? false,
      options: {
        temperature: options.temperature ?? 0.7,
        num_predict: options.maxTokens ?? 256,
      },
    });

    return {
      answer: response.response,
      description: response.response,
      confidence: this.estimateConfidence(response.response),
      timestamp: Date.now(),
    };
  }

  /**
   * Verify UI color
   */
  async verifyUIColor(
    frame: FrameData,
    expectedColor: string,
    elementDescription: string
  ): Promise<VerificationResult> {
    const question = `
In this Android app screenshot, look at the ${elementDescription}.
Is the background color "${expectedColor}"?

Answer YES or NO, then briefly describe what color you actually see.
`;

    const response = await this.analyzeFrame(frame, question);
    
    const passed = response.answer.toUpperCase().includes('YES');
    
    return {
      passed,
      description: response.description,
      confidence: response.confidence,
    };
  }

  /**
   * Verify text content
   */
  async verifyText(
    frame: FrameData,
    expectedText: string,
    elementDescription: string
  ): Promise<VerificationResult> {
    const question = `
Read the text from ${elementDescription} in this screenshot.
What does it say exactly?
`;

    const response = await this.analyzeFrame(frame, question);
    
    const passed = response.answer.toLowerCase().includes(expectedText.toLowerCase());
    
    return {
      passed,
      description: `Text reads: "${response.answer.trim()}"`,
      confidence: response.confidence,
    };
  }

  /**
   * Verify element visibility
   */
  async verifyVisibility(
    frame: FrameData,
    shouldBeVisible: boolean,
    elementDescription: string
  ): Promise<VerificationResult> {
    const question = `
Is the ${elementDescription} visible in this screenshot?
Answer YES or NO.
`;

    const response = await this.analyzeFrame(frame, question);
    const isVisible = response.answer.toUpperCase().includes('YES');
    
    const passed = isVisible === shouldBeVisible;
    
    return {
      passed,
      description: shouldBeVisible 
        ? (isVisible ? 'Element is visible ✓' : 'Element NOT visible ✗')
        : (isVisible ? 'Element still visible ✗' : 'Element hidden ✓'),
      confidence: response.confidence,
    };
  }

  /**
   * Describe UI layout
   */
  async describeLayout(frame: FrameData): Promise<VisionResponse> {
    const question = `
Describe the layout of this Android app screen.
What elements do you see? How are they arranged?
`;

    return await this.analyzeFrame(frame, question);
  }

  /**
   * Detect errors in UI
   */
  async detectErrors(frame: FrameData): Promise<VisionResponse> {
    const question = `
Look at this Android app screenshot carefully.
Do you see any error messages, crash dialogs, or visual glitches?
If yes, describe them. If no, say "No errors detected".
`;

    return await this.analyzeFrame(frame, question);
  }

  /**
   * Estimate confidence from response
   */
  private estimateConfidence(response: string): number {
    // Simple heuristic based on response certainty
    const certainWords = ['yes', 'no', 'definitely', 'clearly', 'obviously'];
    const uncertainWords = ['maybe', 'possibly', 'might', 'could', 'unclear', 'hard to tell'];

    const lowerResponse = response.toLowerCase();
    
    let confidence = 0.7;  // Base confidence

    for (const word of certainWords) {
      if (lowerResponse.includes(word)) {
        confidence += 0.15;
      }
    }

    for (const word of uncertainWords) {
      if (lowerResponse.includes(word)) {
        confidence -= 0.15;
      }
    }

    return Math.max(0.1, Math.min(0.99, confidence));
  }
}

// Singleton instance
export const visionAnalyzer = new VisionAnalyzer();
