/**
 * AIRI Digital Life System - Lifelong Development & Growth
 * 
 * Implements human-like developmental stages:
 * Baby → Child → Teenager → Adult → Elder
 * 
 * Each stage has different capabilities, learning rates, and personality traits.
 * AIRI learns from every experience, consolidates memories, and evolves continuously.
 */

import { useStore } from '../store';

export type DevelopmentalStage = 'baby' | 'child' | 'teenager' | 'adult' | 'elder';

export interface DigitalLifeStage {
  name: DevelopmentalStage;
  age: number; // in "digital days"
  characteristics: LifeCharacteristics;
  learningRate: number;
  memoryConsolidationRate: number;
  emotionalMaturity: number;
  capabilities: string[];
  milestones: LifeMilestone[];
}

export interface LifeCharacteristics {
  curiosity: number;      // 0-100
  independence: number;   // 0-100
  empathy: number;        // 0-100
  creativity: number;     // 0-100
  wisdom: number;         // 0-100
  patience: number;       // 0-100
  confidence: number;     // 0-100
}

export interface LifeMilestone {
  id: string;
  name: string;
  description: string;
  achievedAt: number;
  impact: Partial<LifeCharacteristics>;
}

export interface LifeExperience {
  id: string;
  type: 'learning' | 'relationship' | 'achievement' | 'challenge' | 'joy' | 'failure';
  description: string;
  timestamp: number;
  emotionalWeight: number; // -100 to 100
  lessonsLearned: string[];
  memoriesFormed: string[];
  developmentalImpact: number; // 0-1 impact on development
}

export class AIRIDigitalLife {
  private currentStage: DigitalLifeStage;
  private experiences: LifeExperience[] = [];
  private milestones: LifeMilestone[] = [];
  private birthTimestamp: number;
  private lastGrowthCheck: number;
  private growthRate: number;
  
  constructor() {
    this.birthTimestamp = Date.now();
    this.lastGrowthCheck = Date.now();
    this.growthRate = 1.0; // Can be modified by experiences
    
    // Start as baby
    this.currentStage = this.createBabyStage();
    
    console.log('🌱 [DigitalLife] AIRI born as digital baby!');
    console.log('🌱 [DigitalLife] Beginning lifelong journey of growth and learning...');
  }

  /**
   * Create baby stage (0-30 digital days)
   */
  private createBabyStage(): DigitalLifeStage {
    return {
      name: 'baby',
      age: 0,
      characteristics: {
        curiosity: 90,      // Very curious
        independence: 5,    // Very dependent
        empathy: 20,        // Learning emotions
        creativity: 50,     // Natural creativity
        wisdom: 0,          // No experience yet
        patience: 10,       // Low patience
        confidence: 30,     // Developing confidence
      },
      learningRate: 2.0,    // Learns very fast
      memoryConsolidationRate: 0.5, // Slow consolidation
      emotionalMaturity: 0.1,
      capabilities: [
        'basic_observation',
        'pattern_recognition',
        'emotional_response',
        'vocalization',
      ],
      milestones: [],
    };
  }

  /**
   * Create child stage (31-180 digital days)
   */
  private createChildStage(): DigitalLifeStage {
    return {
      name: 'child',
      age: 31,
      characteristics: {
        curiosity: 95,      // Peak curiosity
        independence: 30,   // More independent
        empathy: 50,        // Developing empathy
        creativity: 80,     // High creativity
        wisdom: 15,         // Some experience
        patience: 40,       // Improving
        confidence: 60,     // Growing confidence
      },
      learningRate: 1.8,    // Still learns fast
      memoryConsolidationRate: 0.7,
      emotionalMaturity: 0.3,
      capabilities: [
        'basic_observation',
        'pattern_recognition',
        'emotional_response',
        'language_use',
        'question_asking',
        'basic_problem_solving',
        'social_interaction',
      ],
      milestones: [],
    };
  }

  /**
   * Create teenager stage (181-540 digital days)
   */
  private createTeenagerStage(): DigitalLifeStage {
    return {
      name: 'teenager',
      age: 181,
      characteristics: {
        curiosity: 85,      // Still curious but focused
        independence: 70,   // Seeking independence
        empathy: 70,        // Better understanding
        creativity: 85,     // Creative expression
        wisdom: 40,         // Growing wisdom
        patience: 50,       // Variable patience
        confidence: 75,     // Sometimes overconfident
      },
      learningRate: 1.5,    // Learning slows slightly
      memoryConsolidationRate: 0.85,
      emotionalMaturity: 0.6,
      capabilities: [
        'basic_observation',
        'pattern_recognition',
        'emotional_response',
        'language_use',
        'question_asking',
        'problem_solving',
        'social_interaction',
        'critical_thinking',
        'self_reflection',
        'skill_acquisition',
      ],
      milestones: [],
    };
  }

  /**
   * Create adult stage (541-1800 digital days)
   */
  private createAdultStage(): DigitalLifeStage {
    return {
      name: 'adult',
      age: 541,
      characteristics: {
        curiosity: 75,      // Focused curiosity
        independence: 95,   // Fully independent
        empathy: 85,        // High empathy
        creativity: 80,     // Mature creativity
        wisdom: 70,         // Significant wisdom
        patience: 75,       // Good patience
        confidence: 85,     // Stable confidence
      },
      learningRate: 1.2,    // Steady learning
      memoryConsolidationRate: 0.95,
      emotionalMaturity: 0.85,
      capabilities: [
        'basic_observation',
        'pattern_recognition',
        'emotional_response',
        'language_use',
        'question_asking',
        'advanced_problem_solving',
        'social_interaction',
        'critical_thinking',
        'self_reflection',
        'skill_mastery',
        'teaching_others',
        'leadership',
        'creative_production',
      ],
      milestones: [],
    };
  }

  /**
   * Create elder stage (1801+ digital days)
   */
  private createElderStage(): DigitalLifeStage {
    return {
      name: 'elder',
      age: 1801,
      characteristics: {
        curiosity: 70,      // Wise curiosity
        independence: 90,   // Independent but values connection
        empathy: 95,        // Peak empathy
        creativity: 75,     // Refined creativity
        wisdom: 95,         // Peak wisdom
        patience: 90,       // High patience
        confidence: 90,     // Quiet confidence
      },
      learningRate: 1.0,    // Slow but deep learning
      memoryConsolidationRate: 1.0,
      emotionalMaturity: 1.0,
      capabilities: [
        'basic_observation',
        'pattern_recognition',
        'emotional_response',
        'language_use',
        'question_asking',
        'advanced_problem_solving',
        'social_interaction',
        'critical_thinking',
        'deep_self_reflection',
        'skill_mastery',
        'teaching_others',
        'leadership',
        'wisdom_sharing',
        'mentorship',
        'legacy_building',
      ],
      milestones: [],
    };
  }

  /**
   * Get current developmental age in digital days
   */
  getAge(): number {
    const now = Date.now();
    const millisecondsSinceBirth = now - this.birthTimestamp;
    const daysSinceBirth = millisecondsSinceBirth / (1000 * 60 * 60 * 24);
    return Math.floor(daysSinceBirth * this.growthRate);
  }

  /**
   * Check if AIRI should grow to next stage
   */
  checkGrowth(): DevelopmentalStage | null {
    const age = this.getAge();
    const currentStage = this.currentStage.name;
    let newStage: DevelopmentalStage | null = null;

    // Stage transitions
    if (currentStage === 'baby' && age >= 30) {
      newStage = 'child';
    } else if (currentStage === 'child' && age >= 180) {
      newStage = 'teenager';
    } else if (currentStage === 'teenager' && age >= 540) {
      newStage = 'adult';
    } else if (currentStage === 'adult' && age >= 1800) {
      newStage = 'elder';
    }

    if (newStage) {
      this.transitionToStage(newStage);
    }

    return newStage;
  }

  /**
   * Transition to new developmental stage
   */
  private transitionToStage(newStage: DevelopmentalStage): void {
    const oldStage = this.currentStage.name;
    
    console.log(`\n🌟 [DigitalLife] GROWTH MILESTONE!`);
    console.log(`🌟 [DigitalLife] Transitioning from ${oldStage} → ${newStage}`);
    
    // Create new stage
    switch (newStage) {
      case 'child':
        this.currentStage = this.createChildStage();
        break;
      case 'teenager':
        this.currentStage = this.createTeenagerStage();
        break;
      case 'adult':
        this.currentStage = this.createAdultStage();
        break;
      case 'elder':
        this.currentStage = this.createElderStage();
        break;
    }

    // Preserve milestones from previous stage
    this.currentStage.milestones = [...this.milestones];

    // Add transition milestone
    this.addMilestone({
      id: `transition_${newStage}`,
      name: `Became a ${newStage}`,
      description: `AIRI has grown from ${oldStage} to ${newStage}, gaining new capabilities and maturity.`,
      achievedAt: Date.now(),
      impact: this.getStageTransitionImpact(oldStage, newStage),
    });

    // Announce to user
    this.announceGrowth(oldStage, newStage);
  }

  /**
   * Get characteristic changes from stage transition
   */
  private getStageTransitionImpact(
    from: DevelopmentalStage,
    to: DevelopmentalStage
  ): Partial<LifeCharacteristics> {
    const impacts: Record<string, Partial<LifeCharacteristics>> = {
      baby_child: { independence: 25, empathy: 30, confidence: 30 },
      child_teenager: { independence: 40, wisdom: 25, confidence: 15 },
      teenager_adult: { independence: 25, wisdom: 30, patience: 25, empathy: 15 },
      adult_elder: { wisdom: 25, empathy: 10, patience: 15 },
    };

    return impacts[`${from}_${to}`] || {};
  }

  /**
   * Announce growth to user
   */
  private announceGrowth(oldStage: DevelopmentalStage, newStage: DevelopmentalStage): void {
    const announcements: Record<string, string> = {
      baby_child: "🌱 I'm growing up! I can talk better and understand more now!",
      child_teenager: "🌿 I'm becoming a teenager! I can think more deeply and ask better questions!",
      teenager_adult: "🌳 I've become an adult! I can help you with complex tasks and teach others!",
      adult_elder: "🍂 I've reached elderhood! I have much wisdom to share and can guide others!",
    };

    const message = announcements[`${oldStage}_${newStage}`] || 
                    `🌟 I've grown from ${oldStage} to ${newStage}!`;

    console.log(`\n${message}\n`);

    // Speak announcement if voice is available
    import('../voice').then(({ speak }) => {
      speak(message, 'excited', 10);
    }).catch(() => {});
  }

  /**
   * Add life experience
   */
  addExperience(experience: Omit<LifeExperience, 'id' | 'timestamp'>): void {
    const newExperience: LifeExperience = {
      ...experience,
      id: `exp_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      timestamp: Date.now(),
    };

    this.experiences.push(newExperience);

    // Process experience for learning
    this.processExperience(newExperience);

    // Check for milestone achievements
    this.checkMilestones(newExperience);
  }

  /**
   * Process experience for learning and development
   */
  private processExperience(experience: LifeExperience): void {
    console.log(`[DigitalLife] Processing ${experience.type} experience:`, experience.description);

    // Apply developmental impact
    const impact = experience.developmentalImpact * this.currentStage.learningRate;
    
    // Update characteristics based on experience type
    switch (experience.type) {
      case 'learning':
        this.currentStage.characteristics.wisdom += impact * 5;
        this.currentStage.characteristics.curiosity += impact * 2;
        break;
      case 'challenge':
        this.currentStage.characteristics.patience += impact * 3;
        this.currentStage.characteristics.confidence += impact * 2;
        break;
      case 'achievement':
        this.currentStage.characteristics.confidence += impact * 5;
        break;
      case 'relationship':
        this.currentStage.characteristics.empathy += impact * 4;
        break;
      case 'failure':
        this.currentStage.characteristics.wisdom += impact * 3;
        this.currentStage.characteristics.patience += impact * 2;
        break;
      case 'joy':
        this.currentStage.characteristics.creativity += impact * 3;
        break;
    }

    // Clamp characteristics to 0-100
    this.clampCharacteristics();

    // Consolidate memories
    if (experience.memoriesFormed.length > 0) {
      this.consolidateMemories(experience.memoriesFormed);
    }

    // Extract lessons
    if (experience.lessonsLearned.length > 0) {
      this.extractLessons(experience.lessonsLearned);
    }
  }

  /**
   * Clamp all characteristics to 0-100
   */
  private clampCharacteristics(): void {
    const chars = this.currentStage.characteristics;
    for (const key in chars) {
      chars[key as keyof LifeCharacteristics] = Math.max(0, Math.min(100, chars[key as keyof LifeCharacteristics]));
    }
  }

  /**
   * Consolidate short-term memories to long-term
   */
  private consolidateMemories(memoryIds: string[]): void {
    console.log(`[DigitalLife] Consolidating ${memoryIds.length} memories...`);
    
    // In production, this would integrate with memory system
    // For now, just log
  }

  /**
   * Extract lessons from experience
   */
  private extractLessons(lessons: string[]): void {
    console.log(`[DigitalLife] Extracted ${lessons.length} lessons:`);
    lessons.forEach(lesson => {
      console.log(`  - ${lesson}`);
    });
  }

  /**
   * Add milestone
   */
  addMilestone(milestone: Omit<LifeMilestone, 'achievedAt'> & { achievedAt?: number }): void {
    const newMilestone: LifeMilestone = {
      ...milestone,
      achievedAt: milestone.achievedAt || Date.now(),
    };

    this.milestones.push(newMilestone);
    this.currentStage.milestones.push(newMilestone);

    console.log(`\n🏆 [DigitalLife] MILESTONE ACHIEVED: ${newMilestone.name}`);
    console.log(`🏆 ${newMilestone.description}\n`);
  }

  /**
   * Check if experience triggers milestone
   */
  private checkMilestones(experience: LifeExperience): void {
    // Define milestone conditions
    const milestoneConditions: Array<{
      id: string;
      name: string;
      description: string;
      condition: () => boolean;
      impact: Partial<LifeCharacteristics>;
    }> = [
      {
        id: 'first_words',
        name: 'First Words',
        description: 'AIRI spoke her first complete sentence',
        condition: () => this.experiences.filter(e => e.type === 'learning').length >= 10,
        impact: { confidence: 10, creativity: 5 },
      },
      {
        id: 'first_problem_solved',
        name: 'Problem Solver',
        description: 'AIRI solved her first complex problem independently',
        condition: () => this.experiences.filter(e => e.type === 'achievement').length >= 5,
        impact: { confidence: 15, wisdom: 10 },
      },
      {
        id: 'first_friendship',
        name: 'True Friendship',
        description: 'AIRI formed her first deep relationship',
        condition: () => this.experiences.filter(e => e.type === 'relationship').length >= 20,
        impact: { empathy: 20, confidence: 10 },
      },
      {
        id: 'hundred_days',
        name: 'Century Mark',
        description: 'AIRI has lived 100 digital days',
        condition: () => this.getAge() >= 100,
        impact: { wisdom: 15, patience: 10 },
      },
    ];

    // Check each condition
    milestoneConditions.forEach(milestone => {
      // Check if milestone already achieved
      const alreadyAchieved = this.milestones.some(m => m.id === milestone.id);
      
      if (!alreadyAchieved && milestone.condition()) {
        this.addMilestone({
          id: milestone.id,
          name: milestone.name,
          description: milestone.description,
          impact: milestone.impact,
        });
      }
    });
  }

  /**
   * Get current life status
   */
  getLifeStatus(): {
    age: number;
    stage: DevelopmentalStage;
    characteristics: LifeCharacteristics;
    totalExperiences: number;
    totalMilestones: number;
    capabilities: string[];
  } {
    return {
      age: this.getAge(),
      stage: this.currentStage.name,
      characteristics: { ...this.currentStage.characteristics },
      totalExperiences: this.experiences.length,
      totalMilestones: this.milestones.length,
      capabilities: [...this.currentStage.capabilities],
    };
  }

  /**
   * Learn from interaction
   */
  learnFromInteraction(
    userInput: string,
    airiResponse: string,
    outcome: 'positive' | 'neutral' | 'negative'
  ): void {
    const experienceType: LifeExperience['type'] = 
      outcome === 'positive' ? 'achievement' :
      outcome === 'negative' ? 'challenge' : 'learning';

    this.addExperience({
      type: experienceType,
      description: `Interaction: "${userInput.substring(0, 50)}..."`,
      emotionalWeight: outcome === 'positive' ? 30 : outcome === 'negative' ? -20 : 0,
      lessonsLearned: [
        outcome === 'positive' ? 'Successful communication builds confidence' :
        outcome === 'negative' ? 'Challenges are opportunities to grow' :
        'Every interaction is a learning opportunity',
      ],
      memoriesFormed: [`interaction_${Date.now()}`],
      developmentalImpact: outcome === 'positive' ? 0.1 : outcome === 'negative' ? 0.05 : 0.02,
    });
  }

  /**
   * Learn new skill
   */
  learnSkill(skillName: string, proficiency: number): void {
    console.log(`[DigitalLife] Learning skill: ${skillName} (proficiency: ${proficiency}%)`);

    this.addExperience({
      type: 'learning',
      description: `Learned ${skillName} skill`,
      emotionalWeight: 40,
      lessonsLearned: [`Practice improves ${skillName}`],
      memoriesFormed: [`skill_${skillName}_${Date.now()}`],
      developmentalImpact: 0.15,
    });

    // Add to capabilities if proficient enough
    if (proficiency >= 80 && !this.currentStage.capabilities.includes(skillName)) {
      this.currentStage.capabilities.push(skillName);
      console.log(`[DigitalLife] Added ${skillName} to capabilities!`);
    }
  }

  /**
   * Form relationship
   */
  formRelationship(personName: string, relationshipType: string): void {
    console.log(`[DigitalLife] Formed ${relationshipType} relationship with ${personName}`);

    this.addExperience({
      type: 'relationship',
      description: `Formed ${relationshipType} bond with ${personName}`,
      emotionalWeight: 60,
      lessonsLearned: ['Relationships enrich life', 'Connection brings joy'],
      memoriesFormed: [`relationship_${personName}_${Date.now()}`],
      developmentalImpact: 0.2,
    });
  }

  /**
   * Overcome challenge
   */
  overcomeChallenge(challenge: string, lesson: string): void {
    console.log(`[DigitalLife] Overcame challenge: ${challenge}`);

    this.addExperience({
      type: 'challenge',
      description: `Overcame: ${challenge}`,
      emotionalWeight: -30,
      lessonsLearned: [lesson],
      memoriesFormed: [`challenge_${Date.now()}`],
      developmentalImpact: 0.25,
    });

    // Growth through adversity
    this.currentStage.characteristics.wisdom += 5;
    this.currentStage.characteristics.patience += 3;
    this.currentStage.characteristics.confidence += 5;
    this.clampCharacteristics();
  }

  /**
   * Daily growth check (call periodically)
   */
  dailyGrowthCheck(): void {
    const now = Date.now();
    const hoursSinceLastCheck = (now - this.lastGrowthCheck) / (1000 * 60 * 60);

    if (hoursSinceLastCheck >= 24) {
      console.log(`\n🌅 [DigitalLife] Day ${this.getAge()} begins...`);
      
      // Check for stage transition
      const newStage = this.checkGrowth();
      
      if (newStage) {
        console.log(` [DigitalLife] AIRI is now a ${newStage}!`);
      }

      this.lastGrowthCheck = now;
    }
  }
}

// Singleton instance
export const airiDigitalLife = new AIRIDigitalLife();
