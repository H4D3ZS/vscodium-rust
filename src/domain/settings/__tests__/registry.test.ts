import { describe, it, expect } from 'vitest';
import {
    SETTINGS_SECTIONS,
    SETTINGS_ITEMS,
    LEGACY_CATEGORY_MAP,
    searchSettings,
    itemsForSection,
    findItem,
} from '../registry';

describe('settings registry', () => {
    it('has exactly 8 sections', () => {
        expect(SETTINGS_SECTIONS).toHaveLength(8);
    });

    it('every item belongs to a declared section', () => {
        const ids = new Set(SETTINGS_SECTIONS.map((s) => s.id));
        for (const it of SETTINGS_ITEMS) {
            expect(ids.has(it.section), `${it.id} → ${it.section}`).toBe(true);
        }
    });

    it('every section has at least one item', () => {
        for (const s of SETTINGS_SECTIONS) {
            expect(itemsForSection(s.id).length, s.id).toBeGreaterThan(0);
        }
    });

    it('item ids are unique', () => {
        const ids = SETTINGS_ITEMS.map((i) => i.id);
        expect(new Set(ids).size).toBe(ids.length);
    });

    it('every legacy category maps to a real item', () => {
        for (const [legacy, mapped] of Object.entries(LEGACY_CATEGORY_MAP)) {
            expect(findItem(mapped), `${legacy} → ${mapped}`).toBeDefined();
        }
    });

    it('search finds an Advanced setting by keyword ("thermal" → PyTorch)', () => {
        const hits = searchSettings('thermal');
        expect(hits.map((h) => h.id)).toContain('pytorch');
    });

    it('search finds providers by old section name ("api key")', () => {
        const hits = searchSettings('api key');
        expect(hits.map((h) => h.id)).toContain('models');
    });

    it('search is empty for blank query', () => {
        expect(searchSettings('   ')).toHaveLength(0);
    });
});
