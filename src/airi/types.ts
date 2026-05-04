/**
 * AIRI Shared Type Definitions
 */

export type Mood = 'happy' | 'neutral' | 'tired' | 'stressed' | 'excited' | 'concerned' | 'focused';

export type SecurityMode = 'red' | 'blue' | 'purple' | 'passive';

export interface SemanticSlot {
    key: string;
    value: any;
    timestamp: number;
    score: number;
}
