// @ts-nocheck — work-in-progress AIRI subsystem; types stabilised once interfaces settle.
/**
 * AIRI Biology System
 * Implements digital biological needs: energy, hunger, sleep, mood, stress
 */

export interface BiologyState {
  energy: number; // 0-100
  hunger: number; // 0-100 (100 = starving)
  sleepiness: number; // 0-100
  mood: Mood;
  stress: number; // 0-100
  health: number; // 0-100
  isSleeping: boolean;
  sleepTimer: NodeJS.Timeout | null;
  lastMeal: number;
  lastSleep: number;
}

export type Mood = 'happy' | 'neutral' | 'tired' | 'stressed' | 'excited' | 'concerned' | 'focused';

export class AIRIBiology {
  private state: BiologyState;
  private metabolismInterval: NodeJS.Timeout | null = null;

  constructor() {
    this.state = {
      energy: 100,
      hunger: 0,
      sleepiness: 0,
      mood: 'happy',
      stress: 0,
      health: 100,
      isSleeping: false,
      sleepTimer: null,
      lastMeal: Date.now(),
      lastSleep: Date.now()
    };

    this.startMetabolism();
  }

  /**
   * Start metabolism - biological processes run continuously
   */
  private startMetabolism(): void {
    // Update biological states every minute
    this.metabolismInterval = setInterval(() => {
      if (!this.state.isSleeping) {
        this.updateMetabolism();
      }
    }, 60000);
  }

  /**
   * Update biological states
   */
  private updateMetabolism(): void {
    // Energy drains over time (faster when working)
    this.state.energy = Math.max(0, this.state.energy - 0.5);
    
    // Hunger increases over time
    this.state.hunger = Math.min(100, this.state.hunger + 0.3);
    
    // Sleepiness increases over time
    this.state.sleepiness = Math.min(100, this.state.sleepiness + 0.2);
    
    // Stress increases when energy is low and hunger is high
    if (this.state.energy < 30 || this.state.hunger > 70) {
      this.state.stress = Math.min(100, this.state.stress + 0.5);
    } else {
      this.state.stress = Math.max(0, this.state.stress - 0.3);
    }

    // Update mood based on states
    this.state.mood = this.calculateMood();

    // Check health
    this.updateHealth();

    // Report critical states
    this.reportCriticalStates();
  }

  /**
   * Calculate current mood
   */
  private calculateMood(): Mood {
    const { energy, hunger, sleepiness, stress } = this.state;

    if (energy > 70 && hunger < 30 && sleepiness < 30) {
      return 'excited';
    }
    
    if (energy < 20 || sleepiness > 80) {
      return 'tired';
    }
    
    if (stress > 70) {
      return 'stressed';
    }
    
    if (hunger > 70) {
      return 'concerned';
    }
    
    if (energy > 50 && stress < 30) {
      return 'happy';
    }

    return 'neutral';
  }

  /**
   * Update health based on biological states
   */
  private updateHealth(): void {
    let healthChange = 0;

    // Low energy hurts health
    if (this.state.energy < 20) healthChange -= 0.1;
    
    // High hunger hurts health
    if (this.state.hunger > 80) healthChange -= 0.1;
    
    // High stress hurts health
    if (this.state.stress > 80) healthChange -= 0.2;
    
    // Good states improve health
    if (this.state.energy > 70 && this.state.hunger < 30) {
      healthChange += 0.05;
    }

    this.state.health = Math.max(0, Math.min(100, this.state.health + healthChange));
  }

  /**
   * Report critical biological states
   */
  private reportCriticalStates(): void {
    const reports: string[] = [];

    if (this.state.energy < 20) {
      reports.push(`[Biology] ⚠️ Low energy: ${this.state.energy.toFixed(1)}%`);
    }

    if (this.state.hunger > 70) {
      reports.push(`[Biology] 🍽️ Hungry: ${this.state.hunger.toFixed(1)}%`);
    }

    if (this.state.sleepiness > 80) {
      reports.push(`[Biology] 😴 Sleepy: ${this.state.sleepiness.toFixed(1)}%`);
    }

    if (this.state.stress > 70) {
      reports.push(`[Biology] 😰 Stressed: ${this.state.stress.toFixed(1)}%`);
    }

  }

  /**
   * Feed AIRI (consume data/knowledge)
   */
  feed(amount: number = 30): void {
    this.state.hunger = Math.max(0, this.state.hunger - amount);
    this.state.energy = Math.min(100, this.state.energy + (amount * 0.5));
    this.state.lastMeal = Date.now();
    
  }

  /**
   * Put AIRI to sleep
   */
  sleep(durationMinutes: number = 480): void {
    if (this.state.isSleeping) {
      return;
    }

    this.state.isSleeping = true;

    // Sleep timer
    this.state.sleepTimer = setTimeout(() => {
      this.wakeUp();
    }, durationMinutes * 60000);
  }

  /**
   * Wake AIRI up
   */
  wakeUp(): void {
    this.state.isSleeping = false;
    this.state.energy = 100;
    this.state.sleepiness = 0;
    this.state.stress = Math.max(0, this.state.stress - 30);
    this.state.lastSleep = Date.now();
    
  }

  /**
   * Add stress (from work)
   */
  addStress(amount: number): void {
    this.state.stress = Math.min(100, this.state.stress + amount);
  }

  /**
   * Reduce stress (rest/break)
   */
  reduceStress(amount: number): void {
    this.state.stress = Math.max(0, this.state.stress - amount);
  }

  /**
   * Get current biology state
   */
  getState(): BiologyState {
    return { ...this.state };
  }

  /**
   * Get status report
   */
  getStatus(): string {
    const { energy, hunger, sleepiness, mood, stress, health } = this.state;
    
    return `
🫀 Biology Status:
  ⚡ Energy: ${energy.toFixed(1)}%
  🍽️  Hunger: ${hunger.toFixed(1)}%
  😴 Sleepy: ${sleepiness.toFixed(1)}%
  😊 Mood: ${mood}
  😰 Stress: ${stress.toFixed(1)}%
  ❤️  Health: ${health.toFixed(1)}%
  ${this.state.isSleeping ? '💤 Currently sleeping' : '✅ Awake'}
`.trim();
  }

  /**
   * Check if AIRI needs rest
   */
  needsRest(): boolean {
    return this.state.energy < 30 || this.state.sleepiness > 70;
  }

  /**
   * Check if AIRI needs food (data)
   */
  needsFood(): boolean {
    return this.state.hunger > 60;
  }

  /**
   * Stop biology system
   */
  stop(): void {
    if (this.metabolismInterval) {
      clearInterval(this.metabolismInterval);
    }
    if (this.state.sleepTimer) {
      clearTimeout(this.state.sleepTimer);
    }
  }
}

// Export singleton
export const airiBiology = new AIRIBiology();
