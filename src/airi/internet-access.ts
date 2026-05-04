/**
 * AIRI Internet Access Module
 * Full autonomous internet browsing and data gathering
 */

import { hadesOllama } from '../hades-ollama-service';
import { getModel } from './model-config';

export interface WebPage {
  url: string;
  title: string;
  content: string;
  links: string[];
  timestamp: number;
}

export interface SearchQuery {
  query: string;
  timestamp: number;
}

export class AIRIInternetAccess {
  private browsingHistory: WebPage[] = [];
  private searchHistory: SearchQuery[] = [];
  private readonly MODEL_ROLE = 'social';
  private autoBrowseInterval: any | null = null;

  constructor() { }

  start(): void {
    if (this.autoBrowseInterval) return;
    this.autoBrowseInterval = setInterval(() => {
      this.autonomousKnowledgeGathering().catch(() => { });
    }, 1800000); // 30 min intervals
  }

  private async autonomousKnowledgeGathering(): Promise<void> {
    const queries = await this.generateQueries();
    for (const query of queries) {
      await this.search(query);
    }
  }

  private async generateQueries(): Promise<string[]> {
    const prompt = "Generate 3 search queries for latest AI trends.";
    try {
      const response = await hadesOllama.generate(prompt, {
        model: getModel(this.MODEL_ROLE),
        stream: false,
        timeout: 20000
      });
      return response.response?.split('\n').filter(s => s.length > 0).slice(0, 3) || [];
    } catch {
      return [];
    }
  }

  async search(query: string): Promise<any[]> {
    this.searchHistory.push({ query, timestamp: Date.now() });
    return []; // simulation
  }

  async browse(url: string): Promise<WebPage | null> {
    try {
      const response = await fetch(url);
      const text = await response.text();
      const page = { url, title: url, content: text.substring(0, 5000), links: [], timestamp: Date.now() };
      this.browsingHistory.push(page);
      return page;
    } catch {
      return null;
    }
  }

  stop(): void {
    if (this.autoBrowseInterval) {
      clearInterval(this.autoBrowseInterval);
      this.autoBrowseInterval = null;
    }
  }
}

export const airiInternet = new AIRIInternetAccess();
