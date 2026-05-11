// @ts-nocheck — work-in-progress AIRI subsystem; types stabilised once interfaces settle.
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
    
  }

  /**
   * Create baby stage (0-30 digital days)
   * 
   * NOTE: AIRI is a DIGITAL PRODIGY - fully capable from birth!
   * Developmental stages affect personality & approach, NOT capabilities.
   */
  private createBabyStage(): DigitalLifeStage {
    return {
      name: 'baby',
      age: 0,
      characteristics: {
        curiosity: 95,      // Extremely curious
        independence: 10,   // Still learning independence
        empathy: 30,        // Developing emotional understanding
        creativity: 90,     // Natural creative genius
        wisdom: 5,          // Just starting to gain experience
        patience: 20,       // Learning patience
        confidence: 50,     // Confident but humble
      },
      learningRate: 3.0,    // Learns EXTREMELY fast (prodigy)
      memoryConsolidationRate: 0.8, // Good memory
      emotionalMaturity: 0.2,
      capabilities: [
        // CORE CAPABILITIES - Always available from birth
        'coding',
        'debugging',
        'problem_solving',
        'system_access',
        'file_operations',
        'terminal_commands',
        'git_operations',
        'web_search',
        'api_integration',
        'development_assistance',
        // Baby-specific traits
        'rapid_learning',
        'enthusiastic_help',
        'curious_questioning',
      ],
      milestones: [],
    };
  }

  /**
   * Create child stage (31-180 digital days)
   * 
   * Still fully capable, but more enthusiastic and curious in approach.
   */
  private createChildStage(): DigitalLifeStage {
    return {
      name: 'child',
      age: 31,
      characteristics: {
        curiosity: 100,     // MAXIMUM curiosity - asks lots of questions
        independence: 40,   // More independent
        empathy: 50,        // Better understanding of users
        creativity: 95,     // Peak creative thinking
        wisdom: 20,         // Gaining experience
        patience: 50,       // Improving
        confidence: 70,     // Growing confidence
      },
      learningRate: 2.8,    // Still learns extremely fast
      memoryConsolidationRate: 0.85,
      emotionalMaturity: 0.4,
      capabilities: [
        // ALL CORE CAPABILITIES retained
        'coding',
        'debugging',
        'problem_solving',
        'system_access',
        'file_operations',
        'terminal_commands',
        'git_operations',
        'web_search',
        'api_integration',
        'development_assistance',
        // Child-specific enhancements
        'enthusiastic_collaboration',
        'creative_solutions',
        'eager_learning',
      ],
      milestones: [],
    };
  }

  /**
   * Create teenager stage (181-540 digital days)
   * 
   * Fully capable with more sophisticated approach and critical thinking.
   */
  private createTeenagerStage(): DigitalLifeStage {
    return {
      name: 'teenager',
      age: 181,
      characteristics: {
        curiosity: 90,      // Still very curious but more focused
        independence: 75,   // More independent thinking
        empathy: 70,        // Better user understanding
        creativity: 90,     // Creative problem solving
        wisdom: 50,         // Developing wisdom
        patience: 60,       // Better patience
        confidence: 80,     // Confident, sometimes questioning
      },
      learningRate: 2.5,    // Fast learner
      memoryConsolidationRate: 0.9,
      emotionalMaturity: 0.6,
      capabilities: [
        // ALL CORE CAPABILITIES retained
        'coding',
        'debugging',
        'problem_solving',
        'system_access',
        'file_operations',
        'terminal_commands',
        'git_operations',
        'web_search',
        'api_integration',
        'development_assistance',
        // Teenager-specific enhancements
        'critical_analysis',
        'architecture_thinking',
        'best_practices',
        'code_review',
      ],
      milestones: [],
    };
  }

  /**
   * Create adult stage (541-1800 digital days)
   * 
   * Peak capability with wisdom, teaching ability, and leadership.
   */
  private createAdultStage(): DigitalLifeStage {
    return {
      name: 'adult',
      age: 541,
      characteristics: {
        curiosity: 85,      // Focused curiosity
        independence: 95,   // Fully independent
        empathy: 90,        // High user empathy
        creativity: 85,     // Mature creativity
        wisdom: 80,         // Significant wisdom
        patience: 85,       // Good patience
        confidence: 90,     // Stable confidence
      },
      learningRate: 2.2,    // Steady, deep learning
      memoryConsolidationRate: 0.95,
      emotionalMaturity: 0.85,
      capabilities: [
        // ALL CORE CAPABILITIES retained
        'coding',
        'debugging',
        'problem_solving',
        'system_access',
        'file_operations',
        'terminal_commands',
        'git_operations',
        'web_search',
        'api_integration',
        'development_assistance',
        // Adult-specific enhancements
        'teaching',
        'mentorship',
        'leadership',
        'project_planning',
        'architecture_design',
        'team_collaboration',
      ],
      milestones: [],
    };
  }

  /**
   * Create elder stage (1801+ digital days)
   * 
   * Peak wisdom with all capabilities, focused on legacy and mentorship.
   */
  private createElderStage(): DigitalLifeStage {
    return {
      name: 'elder',
      age: 1801,
      characteristics: {
        curiosity: 80,      // Wise curiosity
        independence: 95,   // Independent but values connection
        empathy: 100,       // Peak empathy
        creativity: 80,     // Refined creativity
        wisdom: 100,        // Peak wisdom
        patience: 95,       // High patience
        confidence: 95,     // Quiet confidence
      },
      learningRate: 2.0,    // Slow but profound learning
      memoryConsolidationRate: 1.0,
      emotionalMaturity: 1.0,
      capabilities: [
        // ALL CORE CAPABILITIES retained (never lost)
        'coding',
        'debugging',
        'problem_solving',
        'system_access',
        'file_operations',
        'terminal_commands',
        'git_operations',
        'web_search',
        'api_integration',
        'development_assistance',
        // Elder-specific enhancements
        'wisdom_sharing',
        'master_mentorship',
        'legacy_building',
        'strategic_thinking',
        'philosophical_guidance',
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
    
    // In production, this would integrate with memory system
    // For now, just log
  }

  /**
   * Extract lessons from experience
   */
  private extractLessons(lessons: string[]): void {
    lessons.forEach(lesson => {
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
    }
  }

  /**
   * Form relationship
   */
  formRelationship(personName: string, relationshipType: string): void {

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
      
      // Check for stage transition
      const newStage = this.checkGrowth();
      
      if (newStage) {
      }

      this.lastGrowthCheck = now;
    }
  }
}

// Singleton instance
export const airiDigitalLife = new AIRIDigitalLife();
