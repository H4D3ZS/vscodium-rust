import { invoke } from '../../tauri_bridge';
import type { IPatchRepository } from '../../domain/editor/IPatchRepository';

export class TauriPatchRepository implements IPatchRepository {
    acceptPatch(path: string): Promise<void> {
        return invoke('accept_sentient_patch', { path });
    }

    rejectPatch(path: string): Promise<void> {
        return invoke('reject_sentient_patch', { path });
    }

    revertContent(path: string, content: string): Promise<void> {
        return invoke('revert_file_content', { path, content });
    }
}

export const patchRepository = new TauriPatchRepository();
