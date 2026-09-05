// @ts-nocheck — work-in-progress AIRI subsystem; types stabilised once interfaces settle.
/**
 * AIRI Surgical Editor
 * Autonomous code modification with vision-triggered healing
 */

import { invoke } from '@tauri-apps/api/core';
import { getModel } from './model-config';

export interface EditOperation {
  id: string;
  filePath: string;
  type: 'replace';
  search: string;
  replace: string;
  description: string;
  priority: number;
}

export interface EditProposal {
  operation: EditOperation;
  preview: { before: string; after: string; diffHunks: any[] };
  verification: { cargoCheckPassed: boolean; typecheckPassed: boolean; errors: string[]; warnings: string[] };
  score: number;
}

export interface EditResult {
  success: boolean;
  file: string;
  editsApplied: number;
  errors: string[];
}

export class AIRISurgicalEditor {
  private pendingProposals = new Map<string, EditProposal>();
  private appliedEdits: EditResult[] = [];
  private getModelName(): string {
    return getModel('code_fix') || 'huihui_ai/qwen2.5-coder-abliterate:7b';
  }
  private listeners = new Map<string, Array<(data: any) => void>>();

  constructor() {
    this.setupAutoHeal();
  }

  on(event: string, callback: (data: any) => void): void {
    if (!this.listeners.has(event)) this.listeners.set(event, []);
    this.listeners.get(event)!.push(callback);
  }

  private emit(event: string, data: any): void {
    this.listeners.get(event)?.forEach(fn => fn(data));
  }

   private async setupAutoHeal(): Promise<void> {
    const { airiVision } = await import('./vision-system');
    airiVision.on('error_detected', async (data: { analysis: any }) => {
      const analysis = data.analysis;
      if (!analysis?.code?.errors?.length) return;

      const errorMessage = analysis.code.errors[0];
      console.log('[SurgicalEditor] Auto-heal triggered:', errorMessage);

      // Broadcast error detection to HUD
      try {
        await invoke('airi_broadcast', {
          event: 'airi:error_detected',
          payload: { errors: [errorMessage] }
        });
      } catch { }

      try {
        const filePath = analysis.ui?.activeFile || await this.getActiveEditorFile();
        if (!filePath) {
          console.warn('[SurgicalEditor] No active file');
          return;
        }

        const fileContent = await this.readFile(filePath);
        if (!fileContent) return;

        const context = analysis.code.snippet || '';

        const fix = await this.generateFix(filePath, fileContent, errorMessage, context);
        if (!fix.search || !fix.replace) throw new Error('Invalid fix output');

        const patchContent = `<<<< SEARCH\n${fix.search}\n====\n${fix.replace}\n>>>> REPLACE`;

        // Apply patch directly
        const applyResult = await invoke('call_tool', {
          name: 'search_replace_edit',
          arguments: { path: filePath, content: patchContent, direct_apply: true }
        }) as any;

        if (!applyResult || applyResult.status !== 'success') {
          throw new Error(applyResult?.message || 'search_replace_edit failed');
        }

        // Verify with cargo check
        const diags = await invoke('call_tool', { name: 'dev_cargo_diagnostics', arguments: {} }) as any;
        if (diags?.error_count === 0) {
          await invoke('call_tool', { name: 'git_add', arguments: { path: filePath } });
          await invoke('call_tool', { name: 'git_commit', arguments: { message: `Auto-fix: ${errorMessage.substring(0, 30)}...` } });
          this.emit('edit_committed', { file: filePath, success: true });
          const { airiConsciousness } = await import('./consciousness');
          airiConsciousness.addThought('success', `Fixed ${filePath.split('/').pop()}`);
          // Broadcast successful edit
          try {
            await invoke('airi_broadcast', {
              event: 'airi:edit_committed',
              payload: { file: filePath, success: true }
            });
          } catch { }
        } else {
          try { await invoke('call_tool', { name: 'revert_checkpoint', arguments: { path: filePath } }); } catch {}
          this.emit('edit_failed', { file: filePath, errors: [diags?.summary || 'Verification failed'] });
          // Broadcast failure
          try {
            await invoke('airi_broadcast', {
              event: 'airi:edit_failed',
              payload: { file: filePath, errors: [diags?.summary || 'Verification failed'] }
            });
          } catch { }
        }
      } catch (err: any) {
        console.error('[SurgicalEditor] Auto-heal error:', err.message);
        this.emit('edit_failed', { file: '', errors: [err.message] });
        try {
          await invoke('airi_broadcast', {
            event: 'airi:edit_failed',
            payload: { file: '', errors: [err.message] }
          });
        } catch { }
      }
    });
  }

  private async readFile(path: string): Promise<string> {
    try {
      const content = await invoke('call_tool', { name: 'view_file', arguments: { path } }) as string;
      return content;
    } catch (error: any) {
      console.error(`[SurgicalEditor] read_file failed (${path}):`, error.message);
      return '';
    }
  }

  private async getActiveEditorFile(): Promise<string | null> {
    try {
      const result = await invoke('call_tool', { name: 'editor_get_active_file', arguments: {} }) as any;
      return result?.path || null;
     } catch {
       return null;
     }
   }

   private async generateFix(
    filePath: string,
    fileContent: string,
    errorMessage: string,
    context: string
  ): Promise<{ search: string; replace: string; description: string }> {
    const { createSharedOllama } = await import('./shared-ollama');
    const ollama = createSharedOllama();

    const prompt = `Fix the Rust compiler error using EXACT SEARCH/REPLACE.

FILE: ${filePath}
ERROR: ${errorMessage}
${context ? `CONTEXT:\n${context}\n` : ''}
CODE:
${fileContent}

Return ONLY:
SEARCH:
<exact existing code (2-3 lines context)>
REPLACE:
<fixed code (same line count)>
DESCRIPTION: <brief explanation>`;

    const response = await ollama.generate({
      model: this.getModelName(),
      messages: [{ role: 'user', content: prompt }],
      stream: false,
      options: { temperature: 0.1, num_predict: 1024 },
    });

    const raw = response.message?.content || response.response || '';
    return this.parseSearchReplace(raw);
  }

  private parseSearchReplace(text: string): { search: string; replace: string; description: string } {
    const searchMatch = text.match(/SEARCH:\s*([\s\S]*?)(?=\nREPLACE:|\nDESCRIPTION:)/i);
    const replaceMatch = text.match(/REPLACE:\s*([\s\S]*?)(?=\nDESCRIPTION:)/i);
    const descMatch = text.match(/DESCRIPTION:\s*([\s\S]*)/i);

    return {
      search: searchMatch ? searchMatch[1].trim() : '',
      replace: replaceMatch ? replaceMatch[1].trim() : '',
      description: descMatch ? descMatch[1].trim() : 'Auto-fix generated',
    };
  }

  async proposeEdit(operation: EditOperation): Promise<EditProposal> {
    const { airiConsciousness } = await import('./consciousness');
    airiConsciousness.addThought('planning', `Proposing edit: ${operation.description}`);

    const currentContent = await this.readFile(operation.filePath);
    const modifiedContent = currentContent.replace(operation.search, operation.replace);

    try {
      const result = await invoke('call_tool', {
        name: 'search_replace_edit',
        arguments: { path: operation.filePath, content: `<<<< SEARCH\n${operation.search}\n====\n${operation.replace}\n>>>> REPLACE`, direct_apply: false }
      }) as any;

      const verification = { cargoCheckPassed: false, typecheckPassed: false, errors: [], warnings: [] };
      const score = 0.8;

      const proposal: EditProposal = {
        operation,
        preview: { before: currentContent, after: modifiedContent, diffHunks: [] },
        verification,
        score,
      };

      this.pendingProposals.set(operation.id, proposal);
      this.emit('edit_proposed', { id: operation.id, file: operation.filePath, description: operation.description, score });
      // Broadcast to HUD
      try {
        await invoke('airi_broadcast', {
          event: 'airi:edit_proposed',
          payload: { id: operation.id, file: operation.filePath, description: operation.description, score }
        });
      } catch { }

      return proposal;
    } catch (error: any) {
      airiConsciousness.addThought('error', `Propose failed: ${error.message}`);
      throw error;
    }
  }

  async commitEdit(proposalId: string, force: boolean = false): Promise<EditResult> {
    const proposal = this.pendingProposals.get(proposalId);
    if (!proposal) {
      return { success: false, file: '', editsApplied: 0, errors: ['Proposal not found'] };
    }

    try {
      await invoke('call_tool', {
        name: 'apply_shadow_patch',
        arguments: { path: proposal.operation.filePath }
      });

      const { airiConsciousness } = await import('./consciousness');
      airiConsciousness.addThought('execution', `✅ Edit committed: ${proposal.operation.description}`);

      const result: EditResult = { success: true, file: proposal.operation.filePath, editsApplied: 1, errors: [] };
      this.appliedEdits.push(result);
      this.pendingProposals.delete(proposalId);

      this.emit('edit_committed', { id: proposalId, file: proposal.operation.filePath, success: true });
      try {
        await invoke('airi_broadcast', {
          event: 'airi:edit_committed',
          payload: { id: proposalId, file: proposal.operation.filePath, success: true }
        });
      } catch { }

      return result;
    } catch (error: any) {
      const err = error instanceof Error ? error.message : String(error);
      return { success: false, file: proposal.operation.filePath, editsApplied: 0, errors: [err] };
    }
  }

  getPending(): EditProposal[] {
    return Array.from(this.pendingProposals.values());
  }

  getHistory(): EditResult[] {
    return [...this.appliedEdits];
  }
}

// Singleton instance
export const airiSurgicalEditor = new AIRISurgicalEditor();
