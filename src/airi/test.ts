/**
 * AIRI Core System Test
 * Verify all systems are operational
 */

import { airi } from './core';

async function testAIRI(): Promise<void> {

  try {
    // Initialize
    await airi.initialize({
      workspacePath: process.cwd(),
      fullAutonomyEnabled: true,
      selfLearningEnabled: true,
      selfHealingEnabled: true
    });

    // Test Consciousness
    airi.consciousness.recordInteraction();
    const consciousness = airi.consciousness.getState();

    // Test Biology
    const biology = airi.biology.getState();

    // Test Self-Learning
    const learningStats = airi.learning.getStats();

    // Test Self-Healing
    const health = airi.selfHealing.getStatus();

    // Test Autonomous Decision
    const decisionStats = airi.decision.getStats();

    // Test Security

    // Full Status

    // Test chat
    const response = await airi.chat('Hello! Are you fully autonomous?');


  } catch (error) {
 console.error('\n TEST FAILED:', error);
    process.exit(1);
  }
}

// Run test
testAIRI();
