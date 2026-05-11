/**
 * AIRI Vision Analysis
 * Qwen2.5-VL-72B integration for error detection from IDE screenshots
 */

import { Ollama } from 'ollama';

export interface ErrorDetectionResult {
  hasError: boolean;
  errorMessage: string;
  confidence: number;
}

export class VisionAnalyzer {
  private ollama: any;
  private model: string = 'qwen2.5-vl:72b';

  constructor(host: string = 'http://localhost:11434') {
    this.ollama = new Ollama({ host });
  }

  async analyzeFrame(frame: any, question: string): Promise<any> {
    const response = await this.ollama.generate({
      model: this.model,
      prompt: question,
      images: [frame.buffer],
      stream: false,
      options: { temperature: 0.3, num_predict: 512 },
    });
    return { answer: response.response, confidence: 0.7 };
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
         errorMessage: hasError ? resp.answer.substring(0,200) : '', 
         confidence: resp.confidence 
      };
    }
  }
}

export const visionAnalyzer = new VisionAnalyzer();
