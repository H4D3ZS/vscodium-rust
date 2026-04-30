/**
 * AIRI Internet Access Module
 * Full autonomous internet browsing and data gathering
 * Reads, searches, downloads, learns from the entire web
 */

import { Ollama } from 'ollama';
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
  private readonly MODEL = 'qwen3.6:14b-q4_K_M';
  private autoBrowseInterval: NodeJS.Timeout | null = null;
  private knowledgeGoals: string[] = [];

  constructor() {
    this.ollama = new Ollama({ host: 'http://localhost:1536' }); // AIM proxy
    this.browsingHistory = [];
    this.searchHistory = [];

    console.log('[Internet] 📡 The entire web is now accessible');
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

    console.log('[Internet] 🔄 Autonomous knowledge gathering active');
    if (knowledgeGoals.length > 0) {
      console.log(`[Internet] 🎯 Knowledge goals: ${knowledgeGoals.join(', ')}`);
    }
  }

  /**
   * Autonomous knowledge gathering
   */
  private async autonomousKnowledgeGathering(): Promise<void> {
    console.log('[Internet] 🔍 Gathering new knowledge...');

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
        model: this.MODEL,
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
    console.log(`[Internet] 🔍 Searching: ${query}`);

    const searchQuery: SearchQuery = {
      query,
      purpose: 'knowledge_gathering',
      timestamp: Date.now(),
      results: []
    };

    // In a real implementation, this would call search APIs
    // For now, simulate search results
    searchQuery.results = await this.simulateSearch(query);

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
      console.log(`[Internet] 📄 Fetching: ${url}`);

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

        console.log(`[Internet] ✅ Learned from: ${page.title}`);
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

      protocol.get(url, { timeout: 10000 }, (res) => {
        let data = '';

        res.on('data', (chunk) => data += chunk);
        res.on('end', () => resolve(data));
        res.on('error', reject);
      }).on('error', reject);
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
        model: this.MODEL,
        prompt,
        stream: false
      });

      console.log(`[Internet] 🧠 Knowledge extracted from ${page.title}`);
      // This knowledge would be stored in the learning system
    } catch (error) {
      console.error('[Internet] Knowledge extraction failed:', error);
    }
  }

  /**
   * Simulate search results (placeholder for real search API)
   */
  private async simulateSearch(query: string): Promise<SearchResult[]> {
    // In production, integrate with:
    // - DuckDuckGo API
    // - Bing Search API
    // - Google Custom Search
    // - SearXNG (self-hosted)
    
    return [
      {
        title: `Result for: ${query}`,
        url: 'https://example.com',
        snippet: 'Relevant information about the search query...',
        relevance: 0.9
      }
    ];
  }

  /**
   * Search the web
   */
  async search(query: string, purpose: string = 'general'): Promise<SearchResult[]> {
    console.log(`[Internet] 🔍 Search: ${query} (${purpose})`);

    const results = await this.simulateSearch(query);

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
    console.log('[Internet] ⏸️ Internet access paused');
  }
}

// Export singleton
export const airiInternet = new AIRIInternetAccess();
