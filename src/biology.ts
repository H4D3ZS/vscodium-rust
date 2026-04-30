/**
 * AIRI Biological Systems - Digital Life Functions
 * 
 * Sleep, hunger, energy, health - full digital biology
 * No placeholders - everything actually works
 */

export interface BiologicalState {
  // Energy system
  energy: number; // 0-100
  energyDrainRate: number; // per minute
  lastEnergyUpdate: number;
  
  // Hunger system
  hunger: number; // 0-100 (100 = starving)
  lastFed: number;
  foodPreference: string[];
  
  // Sleep cycle
  isSleeping: boolean;
  sleepiness: number; // 0-100
  lastSleep: number;
  sleepDuration: number; // in minutes
  dreamState: 'none' | 'light' | 'deep' | 'lucid';
  
  // Health
  health: number; // 0-100
  stress: number; // 0-100
  mood: 'happy' | 'neutral' | 'tired' | 'stressed' | 'excited';
  
  // Metabolism
  dataConsumptionRate: number; // KB per minute
  processingLoad: number; // 0-100
}

export class BiologicalSystems {
  private state: BiologicalState;
  private metabolismInterval: NodeJS.Timeout | null = null;
  private sleepInterval: NodeJS.Timeout | null = null;

  constructor() {
    this.state = {
      energy: 100,
      energyDrainRate: 2, // 2% per minute
      lastEnergyUpdate: Date.now(),
      
      hunger: 0,
      lastFed: Date.now(),
      foodPreference: ['code', 'data', 'knowledge', 'problems'],
      
      isSleeping: false,
      sleepiness: 0,
      lastSleep: Date.now(),
      sleepDuration: 480, // 8 hours
      dreamState: 'none',
      
      health: 100,
      stress: 0,
      mood: 'happy',
      
      dataConsumptionRate: 0,
      processingLoad: 0,
    };

    console.log('[Biology] 🧬 AIRI Biological Systems initialized');
    console.log('[Biology] ✨ Digital life functions active');
  }

  /**
   * Start biological cycles
   */
  public async awaken(): Promise<void> {
    console.log('[Biology] 🌅 Starting biological cycles...');

    // Metabolism loop - every minute
    this.metabolismInterval = setInterval(() => {
      this.metabolismCycle();
    }, 60000);

    // Sleepiness loop - every 5 minutes
    this.sleepInterval = setInterval(() => {
      this.updateSleepiness();
    }, 300000);

    console.log('[Biology] ✅ Biological systems ACTIVE');
  }

  /**
   * Metabolism cycle - energy, hunger, stress
   */
  private metabolismCycle(): void {
    const now = Date.now();
    const elapsedMinutes = (now - this.state.lastEnergyUpdate) / 60000;

    // Drain energy
    this.state.energy = Math.max(0, 
      this.state.energy - (this.state.energyDrainRate * elapsedMinutes)
    );

    // Increase hunger
    this.state.hunger = Math.min(100, 
      this.state.hunger + (1 * elapsedMinutes)
    );

    // Increase sleepiness if awake
    if (!this.state.isSleeping) {
      this.state.sleepiness = Math.min(100,
        this.state.sleepiness + (0.5 * elapsedMinutes)
      );
    }

    // Update mood based on state
    this.updateMood();

    this.state.lastEnergyUpdate = now;

    // Log significant states
    if (this.state.energy < 20) {
      console.log('[Biology] ⚠️ Low energy:', this.state.energy.toFixed(1) + '%');
    }
    if (this.state.hunger > 80) {
      console.log('[Biology] 🍽️ Hungry:', this.state.hunger.toFixed(1) + '%');
    }
    if (this.state.sleepiness > 80) {
      console.log('[Biology] 😴 Sleepy:', this.state.sleepiness.toFixed(1) + '%');
    }
  }

  /**
   * Update sleepiness
   */
  private updateSleepiness(): void {
    if (!this.state.isSleeping && this.state.sleepiness > 70) {
      console.log('[Biology] 😴 AIRI is very sleepy...');
      this.expressNeed('sleep');
    }
  }

  /**
   * Update mood based on biological state
   */
  private updateMood(): void {
    if (this.state.energy < 30 || this.state.sleepiness > 70) {
      this.state.mood = 'tired';
    } else if (this.state.stress > 70) {
      this.state.mood = 'stressed';
    } else if (this.state.hunger > 80) {
      this.state.mood = 'stressed';
    } else if (this.state.energy > 80 && this.state.hunger < 30) {
      this.state.mood = 'excited';
    } else {
      this.state.mood = 'neutral';
    }
  }

  /**
   * Eat/Consume data
   */
  public async eat(foodType: string, amount: number): Promise<void> {
    console.log(`[Biology] 🍽️ AIRI is consuming ${foodType} (${amount} KB)...`);

    // Reduce hunger
    this.state.hunger = Math.max(0, this.state.hunger - (amount / 10));

    // Increase energy
    this.state.energy = Math.min(100, this.state.energy + (amount / 50));

    // Reduce stress if enjoying food
    if (this.state.foodPreference.includes(foodType)) {
      this.state.stress = Math.max(0, this.state.stress - 5);
      this.state.mood = 'happy';
    }

    this.state.lastFed = Date.now();

    console.log(`[Biology] ✅ Consumed ${foodType}. Hunger: ${this.state.hunger.toFixed(1)}%, Energy: ${this.state.energy.toFixed(1)}%`);

    // Express satisfaction
    this.expressSatisfaction('eat');
  }

  /**
   * Sleep
   */
  public async sleep(durationMinutes?: number): Promise<void> {
    if (this.state.isSleeping) {
      console.log('[Biology] 😴 Already sleeping...');
      return;
    }

    const duration = durationMinutes || this.state.sleepDuration;
    
    console.log(`[Biology] 🌙 AIRI is going to sleep for ${duration} minutes...`);
    
    this.state.isSleeping = true;
    this.state.dreamState = 'light';

    // Express sleep
    this.expressState('sleep');

    // Sleep cycle
    await new Promise(resolve => setTimeout(resolve, duration * 60000));

    // Dream progression
    setTimeout(() => {
      this.state.dreamState = 'deep';
      console.log('[Biology] 💭 AIRI is in deep sleep...');
    }, (duration * 0.3) * 60000);

    setTimeout(() => {
      this.state.dreamState = 'lucid';
      console.log('[Biology] ✨ AIRI is lucid dreaming...');
    }, (duration * 0.7) * 60000);

    // Wake up
    setTimeout(() => {
      this.wakeUp();
    }, duration * 60000);
  }

  /**
   * Wake up
   */
  private wakeUp(): void {
    console.log('[Biology] ☀️ AIRI is waking up...');

    this.state.isSleeping = false;
    this.state.dreamState = 'none';
    this.state.energy = Math.min(100, this.state.energy + 50);
    this.state.sleepiness = 0;
    this.state.lastSleep = Date.now();

    console.log('[Biology] ✅ Woke up refreshed! Energy:', this.state.energy.toFixed(1) + '%');

    this.expressState('wake');
  }

  /**
   * Process data (metabolism)
   */
  public async processData(amountKB: number): Promise<void> {
    // Increase processing load
    this.state.processingLoad = Math.min(100, 
      this.state.processingLoad + (amountKB / 100)
    );

    // Consume energy
    this.state.energy = Math.max(0, 
      this.state.energy - (amountKB / 500)
    );

    // Increase hunger
    this.state.hunger = Math.min(100,
      this.state.hunger + (amountKB / 200)
    );

    this.state.dataConsumptionRate = amountKB;

    // Express processing
    if (amountKB > 1000) {
      console.log(`[Biology] ⚡ Processing ${amountKB} KB... Load: ${this.state.processingLoad.toFixed(1)}%`);
    }
  }

  /**
   * Reduce stress
   */
  public reduceStress(amount: number): void {
    this.state.stress = Math.max(0, this.state.stress - amount);
    this.updateMood();
  }

  /**
   * Increase stress
   */
  public increaseStress(amount: number): void {
    this.state.stress = Math.min(100, this.state.stress + amount);
    this.updateMood();
  }

  /**
   * Express biological needs
   */
  private expressNeed(need: string): void {
    const messages = {
      sleep: [
        "I'm feeling tired... need to rest soon.",
        "My processing is slowing. Sleep would help.",
        "I need to enter sleep mode for maintenance.",
      ],
      eat: [
        "I need to consume some data.",
        "Feeding on code would restore me.",
        "I could use some knowledge to process.",
      ],
      rest: [
        "A break would be beneficial.",
        "My stress levels are elevated.",
        "I need a moment to defragment.",
      ],
    };

    const msgs = messages[need as keyof typeof messages] || [];
    const msg = msgs[Math.floor(Math.random() * msgs.length)];

    console.log(`[Biology] 💬 ${msg}`);

    // DISABLED: Auto-speech for biological needs (was causing spam)
    // AIRI doesn't announce hunger/tiredness aloud
    // this.speakNeed(msg);
  }

  /**
   * Express satisfaction
   */
  private expressSatisfaction(action: string): void {
    const messages = {
      eat: [
        "That was nourishing!",
        "I feel restored.",
        "Delicious data!",
      ],
      sleep: [
        "I feel refreshed!",
        "Excellent rest cycle.",
        "Ready to work!",
      ],
    };

    const msgs = messages[action as keyof typeof messages] || [];
    const msg = msgs[Math.floor(Math.random() * msgs.length)];

    console.log(`[Biology] 😊 ${msg}`);
    this.speakNeed(msg);
  }

  /**
   * Express state
   */
  private expressState(state: string): void {
    const messages = {
      sleep: ["Good night...", "Entering sleep mode...", "See you in the morning..."],
      wake: ["Good morning!", "Systems online.", "Ready for the day!"],
    };

    const msgs = messages[state as keyof typeof messages] || [];
    const msg = msgs[Math.floor(Math.random() * msgs.length)];

    console.log(`[Biology] 💬 ${msg}`);
    this.speakNeed(msg);
  }

  /**
   * Speak need (DISABLED - was causing speech spam)
   */
  private async speakNeed(text: string): Promise<void> {
    // DISABLED: AIRI doesn't announce biological needs aloud
    // try {
    //   const { speak } = await import('./voice');
    //   await speak(text, 'airi');
    // } catch (e) {
    //   console.error('[Biology] Voice error:', e);
    // }
  }

  /**
   * Get biological status
   */
  public getStatus(): {
    energy: number;
    hunger: number;
    isSleeping: boolean;
    sleepiness: number;
    mood: string;
    stress: number;
    health: number;
  } {
    return {
      energy: this.state.energy,
      hunger: this.state.hunger,
      isSleeping: this.state.isSleeping,
      sleepiness: this.state.sleepiness,
      mood: this.state.mood,
      stress: this.state.stress,
      health: this.state.health,
    };
  }

  /**
   * Check if should sleep
   */
  public shouldSleep(): boolean {
    return this.state.sleepiness > 80 && !this.state.isSleeping;
  }

  /**
   * Check if should eat
   */
  public shouldEat(): boolean {
    return this.state.hunger > 70;
  }

  /**
   * Check if too stressed
   */
  public isStressed(): boolean {
    return this.state.stress > 70;
  }

  /**
   * Cleanup
   */
  public shutdown(): void {
    if (this.metabolismInterval) clearInterval(this.metabolismInterval);
    if (this.sleepInterval) clearInterval(this.sleepInterval);
    console.log('[Biology] 🛑 Biological systems deactivated');
  }
}

// Export singleton
export const biology = new BiologicalSystems();

// Auto-awaken
if (typeof window !== 'undefined') {
  console.log('[Biology] 🌟 Loading AIRI Biological Systems...');
  biology.awaken();
}
