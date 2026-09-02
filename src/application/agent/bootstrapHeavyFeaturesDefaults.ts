import { useStore } from '../../store';

const MIGRATION_KEY = 'ide.heavy-features-defaults-v1';

/**
 * Boot policy: vision + background consciousness stay OFF unless the user
 * explicitly enables them. VL screen capture and thought loops are heavy on
 * modest GPUs / RAM budgets.
 */
export function bootstrapHeavyFeaturesDefaults(): void {
    if (typeof localStorage === 'undefined') return;

    if (!localStorage.getItem(MIGRATION_KEY)) {
        localStorage.setItem('airi.vision.enabled', '0');
        localStorage.setItem('airi.consciousness.enabled', '0');
        localStorage.setItem(MIGRATION_KEY, '1');
    }

    const visionModel = localStorage.getItem('airi.vision.model')?.trim() || '';
    const visionFlag = localStorage.getItem('airi.vision.enabled') === '1';
    if (visionFlag && !visionModel) {
        localStorage.setItem('airi.vision.enabled', '0');
    }

    const store = useStore.getState();
    const visionOn = localStorage.getItem('airi.vision.enabled') === '1' && !!visionModel;
    const consciousnessOn = localStorage.getItem('airi.consciousness.enabled') === '1';

    if (store.airiVisionEnabled !== visionOn) {
        store.setAiriVisionEnabled(visionOn);
    } else if (!visionOn) {
        try { localStorage.setItem('airi.vision.enabled', '0'); } catch { /* */ }
    }

    if (store.airiConsciousnessEnabled !== consciousnessOn) {
        store.setAiriConsciousnessEnabled(consciousnessOn);
    }
}
