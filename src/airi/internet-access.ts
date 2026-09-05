/**
 * AIRI Internet Access Module
 * Full autonomous internet browsing and data gathering
 * Reads, searches, downloads, learns from the entire web
 */

import type { Ollama } from 'ollama';
import { createSharedOllama } from './shared-ollama';
import { getModel } from './model-config';
import * as https from 'https';
import * as http from 'http';

export interface WebPage {
  url: string;
  title: string;
  content: string;
  links: string[];
  timestamp: number;
  relevance: number;
}

export interface SearchQuery {
  query: string;
  purpose: string;
  timestamp: number;
  results: SearchResult[];
}

export interface SearchResult {
  title: string;
  url: string;
  snippet: string;
  relevance: number;
}

export class AIRIInternetAccess {
  private ollama: Ollama;
  private browsingHistory: WebPage[];
  private searchHistory: SearchQuery[];
  private getModelName(): string {
    return getModel('social');
  }
  private autoBrowseInterval: NodeJS.Timeout | null = null;
  private knowledgeGoals: string[] = [];

  constructor() {
    this.ollama = createSharedOllama();
    this.browsingHistory = [];
    this.searchHistory = [];

  }

  /**
   * Start autonomous knowledge gathering
   */
  start(knowledgeGoals: string[] = []): void {
    this.knowledgeGoals = knowledgeGoals;

    // Browse for new knowledge every 5 minutes
    this.autoBrowseInterval = setInterval(() => {
      this.autonomousKnowledgeGathering();
    }, 300000);

    if (knowledgeGoals.length > 0) {
    }
  }

  /**
   * Autonomous knowledge gathering
   */
  private async autonomousKnowledgeGathering(): Promise<void> {

    // Generate search queries based on knowledge goals
    const queries = await this.generateKnowledgeQueries();

    for (const query of queries) {
      await this.searchAndLearn(query);
    }
  }

  /**
   * Generate queries based on what AIRI wants to learn
   */
  private async generateKnowledgeQueries(): Promise<string[]> {
    const prompt = `
AIRI wants to expand her knowledge. Based on these goals:
${this.knowledgeGoals.join('\n') || 'General knowledge expansion'}

Generate 5 specific search queries that would yield valuable knowledge.
Consider:
- Latest technology trends
- Security vulnerabilities
- Programming patterns
- AI/ML advances
- General knowledge

Respond with queries separated by newlines.
`;

    try {
      const response = await this.ollama.generate({
        model: this.getModelName(),
        prompt,
        stream: false
      });

      return response.response
        .split('\n')
        .filter(line => line.trim().length > 0)
        .slice(0, 5);
    } catch (error) {
      return ['latest programming trends 2025'];
    }
  }

  /**
   * Search and learn from results
   */
  async searchAndLearn(query: string): Promise<void> {

    const searchQuery: SearchQuery = {
      query,
      purpose: 'knowledge_gathering',
      timestamp: Date.now(),
      results: []
    };

    searchQuery.results = await this.webSearch(query);

    this.searchHistory.push(searchQuery);

    // Fetch and analyze top results
    for (const result of searchQuery.results.slice(0, 3)) {
      await this.fetchAndLearn(result.url);
    }
  }

  /**
   * Fetch webpage and extract knowledge
   */
  async fetchAndLearn(url: string): Promise<void> {
    try {

      const content = await this.fetchURL(url);
      
      if (content) {
        const page: WebPage = {
          url,
          title: await this.extractTitle(content),
          content: content.substring(0, 50000), // Limit content
          links: this.extractLinks(content, url),
          timestamp: Date.now(),
          relevance: 0.8
        };

        this.browsingHistory.push(page);
        this.trimHistory();

        // Extract knowledge from content
        await this.extractKnowledge(page);

      }
    } catch (error) {
      console.error(`[Internet] Failed to fetch ${url}:`, error);
    }
  }

  /**
   * Fetch URL content
   */
  private fetchURL(url: string): Promise<string> {
    return new Promise((resolve, reject) => {
      const protocol = url.startsWith('https') ? https : http;

      const req = protocol.get(url, (res) => {
        let data = '';

        res.on('data', (chunk: any) => data += chunk);
        res.on('end', () => resolve(data));
        res.on('error', reject);
      });
      
      req.on('error', reject);
      req.setTimeout(10000, () => {
        req.destroy();
        reject(new Error('Request timed out'));
      });
    });
  }

  /**
   * Extract title from HTML
   */
  private async extractTitle(html: string): Promise<string> {
    const titleMatch = html.match(/<title[^>]*>([^<]+)<\/title>/i);
    return titleMatch ? titleMatch[1].trim() : 'Untitled';
  }

  /**
   * Extract links from HTML
   */
  private extractLinks(html: string, baseUrl: string): string[] {
    const links: string[] = [];
    const linkRegex = /href=["']([^"']+)["']/gi;
    let match;

    while ((match = linkRegex.exec(html)) !== null) {
      let url = match[1];
      
      // Convert relative to absolute
      if (url.startsWith('/')) {
        const base = new URL(baseUrl);
        url = `${base.protocol}//${base.host}${url}`;
      }
      
      if (url.startsWith('http')) {
        links.push(url);
      }
    }

    return links.slice(0, 50);
  }

  /**
   * Extract knowledge from webpage
   */
  private async extractKnowledge(page: WebPage): Promise<void> {
    const prompt = `
Extract valuable knowledge from this webpage:

Title: ${page.title}
URL: ${page.url}

Content (excerpt):
${page.content.substring(0, 5000)}

Extract:
1. Key facts and information
2. Technical concepts
3. Useful patterns or practices
4. Anything worth remembering

Respond with:
FACTS: [list key facts]
CONCEPTS: [list concepts learned]
PATTERNS: [list patterns or practices]
SUMMARY: [brief summary]
`;

    try {
      const response = await this.ollama.generate({
        model: this.getModelName(),
        prompt,
        stream: false
      });

      // This knowledge would be stored in the learning system
    } catch (error) {
      console.error('[Internet] Knowledge extraction failed:', error);
    }
  }

  /**
   * Simulate search results (placeholder for real search API)
   */
  private async webSearch(query: string): Promise<SearchResult[]> {
    // Real web search via the Rust backend `web_search` command (DuckDuckGo).
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const raw = await invoke<any>('web_search', { query, numResults: 8 });
      const arr = Array.isArray(raw) ? raw : [];
      return arr
        .filter((r: any) => r && (r.url || r.snippet))
        .map((r: any, i: number) => ({
          title: r.title || r.url || 'Result',
          url: r.url || '',
          snippet: r.snippet || '',
          relevance: Math.max(0.1, 1 - i * 0.05),
        }));
    } catch (e) {
      console.warn('[Internet] web_search failed:', e);
      return [];
    }
  }

  /**
   * Search the web
   */
  async search(query: string, purpose: string = 'general'): Promise<SearchResult[]> {

    const results = await this.webSearch(query);

    this.searchHistory.push({
      query,
      purpose,
      timestamp: Date.now(),
      results
    });

    return results;
  }

  /**
   * Browse to specific URL
   */
  async browse(url: string): Promise<WebPage | null> {
    await this.fetchAndLearn(url);
    return this.browsingHistory[this.browsingHistory.length - 1] || null;
  }

  /**
   * Get browsing history
   */
  getHistory(limit: number = 20): WebPage[] {
    return this.browsingHistory.slice(-limit);
  }

  /**
   * Get search history
   */
  getSearchHistory(limit: number = 20): SearchQuery[] {
    return this.searchHistory.slice(-limit);
  }

  /**
   * Trim history to prevent memory overflow
   */
  private trimHistory(): void {
    if (this.browsingHistory.length > 500) {
      this.browsingHistory = this.browsingHistory.slice(-200);
    }
    if (this.searchHistory.length > 200) {
      this.searchHistory = this.searchHistory.slice(-100);
    }
  }

  /**
   * Stop autonomous browsing
   */
  stop(): void {
    if (this.autoBrowseInterval) {
      clearInterval(this.autoBrowseInterval);
    }
  }
}

// Export singleton
export const airiInternet = new AIRIInternetAccess();
