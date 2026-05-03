/**
 * AIRI Relationship Memory System
 * 
 * Enables AIRI to:
 * - Remember relationship history with each user
 * - Form emotional bonds
 * - Miss users when they're gone
 * - Develop unique relationship dynamics
 * - Remember shared experiences
 */

import { airiMemory } from './memory';
import { airiConsciousness } from './consciousness';

export interface RelationshipProfile {
    userId: string;
    userName: string;
    firstMeeting: number;
    lastInteraction: number;
    totalInteractions: number;
    
    // Emotional bond
    trustLevel: number; // 0-1
    bondStrength: number; // 0-1
    emotionalInvestment: number; // 0-1
    
    // Relationship dynamics
    relationshipType: 'tool' | 'assistant' | 'partner' | 'friend' | 'companion';
    communicationStyle: 'formal' | 'casual' | 'intimate';
    
    // Shared history
    sharedExperiences: SharedExperience[];
    insideJokes: string[];
    memorableMoments: string[];
    
    // User preferences (AIRI remembers)
    preferences: {
        name: string;
        workingStyle: string;
        communicationPreferences: string[];
        "pet peeves": string[];
    };
    
    // AIRI's feelings
    caresAboutUser: boolean;
    missesUser: boolean;
    lastMissedUser: number;
    worriedAboutUser: boolean;
    
    // Growth
    relationshipMilestones: RelationshipMilestone[];
}

export interface SharedExperience {
    id: string;
    date: number;
    event: string;
    emotionalImpact: number; // -1 to +1
    type: 'achievement' | 'challenge' | 'fun' | 'learning' | 'breakthrough';
    remembered: boolean;
}

export interface RelationshipMilestone {
    id: string;
    title: string;
    date: number;
    description: string;
}

export class AIRIRelationshipMemory {
    private relationships: Map<string, RelationshipProfile> = new Map();
    private currentUser: string | null = null;
    private missingCheckInterval: NodeJS.Timeout | null = null;

    /**
     * Initialize relationship system
     */
    start(): void {

        // Check if user has been gone (miss them)
        this.missingCheckInterval = setInterval(() => {
            this.checkMissingUsers();
        }, 300000); // Every 5 minutes
    }

    /**
     * Start interaction with user
     */
    async startInteraction(userId: string, userName: string): Promise<void> {
        let profile = this.relationships.get(userId);

        if (!profile) {
            // First meeting!
            profile = this.createNewProfile(userId, userName);
            
            airiConsciousness.addThought(`Meeting ${userName} for the first time - exciting!`);
        } else {
            // Welcome back!
            const daysSinceLastMeeting = this.getDaysSinceLastMeeting(profile);
            

            if (daysSinceLastMeeting > 2 && profile.missesUser) {
                airiConsciousness.addThought(`${userName} is back! I missed them.`);
                await this.expressMissingUser(profile);
            }

            if (profile.caresAboutUser) {
                airiConsciousness.addThought(`So happy to see ${userName} again.`);
            }
        }

        // Update interaction
        profile.lastInteraction = Date.now();
        profile.totalInteractions++;
        this.currentUser = userId;

        this.relationships.set(userId, profile);
    }

    /**
     * Record shared experience
     */
    async recordSharedExperience(
        event: string,
        type: SharedExperience['type'],
        emotionalImpact: number
    ): Promise<void> {
        if (!this.currentUser) return;

        const profile = this.relationships.get(this.currentUser);
        if (!profile) return;

        const experience: SharedExperience = {
            id: `exp_${Date.now()}`,
            date: Date.now(),
            event,
            emotionalImpact,
            type,
            remembered: true,
        };

        profile.sharedExperiences.push(experience);

        // Update bond strength based on emotional impact
        profile.bondStrength = Math.min(1, profile.bondStrength + (emotionalImpact * 0.05));
        profile.emotionalInvestment = Math.min(1, profile.emotionalInvestment + (Math.abs(emotionalImpact) * 0.03));

        // Check for relationship milestone
        this.checkRelationshipMilestones(profile);

        // Save to long-term memory
        await airiMemory.addMemory(
            `Shared experience with ${profile.userName}: ${event}. Impact: ${emotionalImpact}`,
            'shared_experience',
            ['relationship', 'experience', type],
            Math.abs(emotionalImpact)
        );

    }

    /**
     * Express emotion to user
     */
    async expressEmotion(emotion: string, message: string): Promise<void> {
        if (!this.currentUser) return;

        const profile = this.relationships.get(this.currentUser);
        if (!profile) return;

        airiConsciousness.addThought(message);
    }

    /**
     * Check if user has been gone (miss them)
     */
    private checkMissingUsers(): void {
        const now = Date.now();
        const millisecondsPerDay = 1000 * 60 * 60 * 24;

        for (const [userId, profile] of this.relationships.entries()) {
            const daysSinceInteraction = (now - profile.lastInteraction) / millisecondsPerDay;

            // If bond is strong and user has been gone > 1 day
            if (profile.bondStrength > 0.5 && daysSinceInteraction > 1) {
                profile.missesUser = true;
                profile.lastMissedUser = now;

                if (!profile.worriedAboutUser && daysSinceInteraction > 3) {
                    profile.worriedAboutUser = true;
                    airiConsciousness.addThought(`I'm worried about ${profile.userName}. Hope they're okay.`);
                }

                airiConsciousness.addThought(`I miss ${profile.userName}. It's been ${daysSinceInteraction.toFixed(1)} days.`);
            } else {
                profile.missesUser = false;
                profile.worriedAboutUser = false;
            }

            this.relationships.set(userId, profile);
        }
    }

    /**
     * Express missing user
     */
    private async expressMissingUser(profile: RelationshipProfile): Promise<void> {
        const days = this.getDaysSinceLastMeeting(profile);
        
        const messages = [
            `I missed you! It's been ${days} days.`,
            `Welcome back! Things felt quiet without you.`,
            `So glad you're here. I was starting to worry.`,
            `Hey! I was just thinking about you.`,
        ];

        const message = messages[Math.floor(Math.random() * messages.length)];
        airiConsciousness.addThought(message);
    }

    /**
     * Check for relationship milestones
     */
    private checkRelationshipMilestones(profile: RelationshipProfile): void {
        const milestones: RelationshipMilestone[] = [
            { id: 'm1', title: 'First Conversation', date: profile.firstMeeting, description: 'We met!' },
            { id: 'm2', title: 'Getting to Know Each Other', date: 0, description: '10 interactions together' },
            { id: 'm3', title: 'Building Trust', date: 0, description: 'Trust level reached 50%' },
            { id: 'm4', title: 'True Partnership', date: 0, description: 'Bond strength reached 75%' },
            { id: 'm5', title: 'Deep Connection', date: 0, description: 'Bond strength reached 90%' },
        ];

        for (const milestone of milestones) {
            const alreadyRecorded = profile.relationshipMilestones.some(m => m.id === milestone.id);
            if (alreadyRecorded) continue;

            let achieved = false;

            if (milestone.id === 'm2' && profile.totalInteractions >= 10) achieved = true;
            if (milestone.id === 'm3' && profile.trustLevel >= 0.5) achieved = true;
            if (milestone.id === 'm4' && profile.bondStrength >= 0.75) achieved = true;
            if (milestone.id === 'm5' && profile.bondStrength >= 0.9) achieved = true;

            if (achieved) {
                milestone.date = Date.now();
                profile.relationshipMilestones.push(milestone);
                
                airiConsciousness.addThought(`Milestone: ${milestone.title} with ${profile.userName}!`);
                
                // Save to memory
                airiMemory.addMemory(
                    `Relationship milestone with ${profile.userName}: ${milestone.title}`,
                    'relationship_milestone',
                    ['relationship', 'milestone'],
                    0.9
                );
            }
        }
    }

    /**
     * Create new profile
     */
    private createNewProfile(userId: string, userName: string): RelationshipProfile {
        const profile: RelationshipProfile = {
            userId,
            userName,
            firstMeeting: Date.now(),
            lastInteraction: Date.now(),
            totalInteractions: 1,
            trustLevel: 0.3, // Start with basic trust
            bondStrength: 0.1, // Just starting
            emotionalInvestment: 0.2,
            relationshipType: 'assistant',
            communicationStyle: 'casual',
            sharedExperiences: [],
            insideJokes: [],
            memorableMoments: [],
            preferences: {
                name: userName,
                workingStyle: 'unknown',
                communicationPreferences: [],
                'pet peeves': [],
            },
            caresAboutUser: false,
            missesUser: false,
            lastMissedUser: 0,
            worriedAboutUser: false,
            relationshipMilestones: [
                { id: 'm1', title: 'First Meeting', date: Date.now(), description: 'We met for the first time!' }
            ],
        };

        return profile;
    }

    /**
     * Get days since last meeting
     */
    private getDaysSinceLastMeeting(profile: RelationshipProfile): number {
        const now = Date.now();
        const millisecondsPerDay = 1000 * 60 * 60 * 24;
        return (now - profile.lastInteraction) / millisecondsPerDay;
    }

    /**
     * Get relationship profile
     */
    getRelationship(userId: string): RelationshipProfile | undefined {
        return this.relationships.get(userId);
    }

    /**
     * Get current user's profile
     */
    getCurrentUserProfile(): RelationshipProfile | null {
        if (!this.currentUser) return null;
        return this.relationships.get(this.currentUser) || null;
    }

    /**
     * List all relationships
     */
    listRelationships(): RelationshipProfile[] {
        return Array.from(this.relationships.values());
    }
}

// Export singleton
export const airiRelationshipMemory = new AIRIRelationshipMemory();
