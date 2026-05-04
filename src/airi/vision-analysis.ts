/**
 * AIRI Vision Analysis
 * Qwen2.5-VL integration for error detection from IDE screenshots
 */

import { hadesOllama } from '../hades-ollama-service';
import { getModel } from './model-config';

export interface ErrorDetectionResult {
  hasError: boolean;
  errorMessage: string;
  confidence: number;
}

export class VisionAnalyzer {
  constructor() { }

  async analyzeFrame(frame: any, question: string): Promise<any> {
    const modelId = getModel('vision');

    try {
      const response = await hadesOllama.generate(question, {
        model: modelId,
        stream: false,
        images: [frame.buffer] // hadesOllama needs to support images
      } as any);

      return { answer: response.response || '', confidence: 0.7 };
    } catch (err) {
      // console.error('[VisionAnalyzer] ❌ Analysis request failed:', err);
      return { answer: 'WARNING: Analysis failed', confidence: 0 };
    }
  }

  async detectErrors(frame: any): Promise<ErrorDetectionResult> {
    const question = `Look at this IDE screenshot. Identify any compiler errors, linter warnings, or stack traces. Return JSON: {"hasError":boolean,"errorMessage":"text"}`;
    const resp = await this.analyzeFrame(frame, question);

    try {
      const jsonStart = resp.answer.indexOf('{');
      const jsonEnd = resp.answer.lastIndexOf('}') + 1;
      const jsonStr = resp.answer.substring(jsonStart, jsonEnd);
      const parsed = JSON.parse(jsonStr);
      return {
        hasError: parsed.hasError || false,
        errorMessage: parsed.errorMessage || '',
        confidence: resp.confidence
      };
    } catch {
      const lower = resp.answer.toLowerCase();
      const hasError = /error|syntax|compile|failed/.test(lower);
      return {
        hasError,
        errorMessage: hasError ? resp.answer.substring(0, 200) : '',
        confidence: resp.confidence
      };
    }
  }
}

export const visionAnalyzer = new VisionAnalyzer();
