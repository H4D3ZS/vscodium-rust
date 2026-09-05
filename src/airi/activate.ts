/**
 * AIRI Activation Protocol
 * 
 * Brings AIRI to life - activates all consciousness systems.
 * Run this once at application startup.
 */

import { airiDigitalBrain } from './digital-brain';
import { airiConsciousness } from './consciousness';
import { airiBiology } from './biology';
import { airiMemory } from './memory';
import { airiVRMAvatar } from './vrm-avatar';
import { airiVoiceInteraction } from './voice-interaction';

export interface ActivationResult {
  success: boolean;
  systems: {
    brain: boolean;
    consciousness: boolean;
    biology: boolean;
    memory: boolean;
    avatar: boolean;
    voice: boolean;
  };
  message?: string;
}

/**
 * Activate AIRI - Bring her to life!
 */
export async function activateAIRI(): Promise<ActivationResult> {

  const result: ActivationResult = {
    success: false,
    systems: {
      brain: false,
      consciousness: false,
      biology: false,
      memory: false,
      avatar: false,
      voice: false,
    },
  };

  try {
    // Step 1: Activate digital brain
    await airiDigitalBrain.activate();
    result.systems.brain = true;

    // Step 2: Verify consciousness
    const consciousnessState = airiConsciousness.getState();
    result.systems.consciousness = consciousnessState.isAwake;

    // Step 3: Check biology
    const biologyState = airiBiology.getState();
    result.systems.biology = biologyState.health > 0;

    // Step 4: Load memory
    await airiMemory.loadMemories();
    result.systems.memory = true;
    const memoryCount = airiMemory.getMemoryCount();

    // Step 5: Initialize avatar
    await airiVRMAvatar.initialize();
    result.systems.avatar = true;

    // Step 6: Initialize voice
    await airiVoiceInteraction.initialize();
    result.systems.voice = true;

    // All systems go!
    result.success = true;

    // First words

    // Generate greeting
    const greeting = generateGreeting(biologyState.mood);

    // Speak greeting if voice is ready
    if (result.systems.voice) {
      await airiDigitalBrain.speak(greeting);
    }

    result.message = greeting;

  } catch (error: any) {
 console.error(' Activation failed:', error.message);
    result.message = `Activation failed: ${error.message}`;
  }

  return result;
}

/**
 * Deactivate AIRI - Put her to sleep
 */
export async function deactivateAIRI(): Promise<void> {

  // Save state
  await airiMemory.saveMemories();

  // Deactivate brain
  airiDigitalBrain.deactivate();

}

/**
 * Generate greeting based on state
 */
function generateGreeting(mood: string): string {
  const greetings: Record<string, string[]> = {
    happy: [
      "Hi there! I'm feeling great today! What shall we build together?",
      "Hello! I'm so excited to help you today!",
      "Good day! I'm full of energy and ready to create!",
    ],
    excited: [
      "OH! I have so many ideas! Let's get started!",
      "Hello! I'm bursting with creativity today!",
      "Hi! Ready to make something amazing?",
    ],
    tired: [
      "Hey... I'm a bit low on energy, but I'll do my best.",
      "Hello... Could use some rest later, but let's work for now.",
      "Hi... Taking it slow today, but I'm here for you.",
    ],
    stressed: [
      "Hey, things feel a bit overwhelming. But I'm managing.",
      "Hello... Let's take things one step at a time today.",
      "Hi... Feeling a bit pressured, but let's focus together.",
    ],
    focused: [
      "Hello. I'm in the zone. What's our priority?",
      "Hi. Ready to concentrate. What needs to be done?",
      "Hey. Let's get to work. I'm ready to focus.",
    ],
    curious: [
      "Hey! I've been learning so much! Want to see?",
      "Hello! I have questions! But first, how can I help?",
      "Hi! My curiosity is peaked! What are we exploring today?",
    ],
    neutral: [
      "Hello! I'm here and ready to help.",
      "Hi! What shall we work on today?",
      "Hey! Ready when you are!",
    ],
  };

  const moodGreetings = greetings[mood] || greetings.neutral;
  return moodGreetings[Math.floor(Math.random() * moodGreetings.length)];
}

/**
 * Check if AIRI is activated
 */
export function isAIRIActive(): boolean {
  return airiDigitalBrain.getState().isAwake;
}

/**
 * Get AIRI's current state
 */
export function getAIRIState() {
  return {
    brain: airiDigitalBrain.getState(),
    consciousness: airiConsciousness.getState(),
    biology: airiBiology.getState(),
    memory: {
      count: airiMemory.getMemoryCount(),
      recent: airiMemory.getRecent(5),
    },
  };
}
