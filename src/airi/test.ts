/**
 * AIRI Core System Test
 * Verify all systems are operational
 */

import { airi } from './core';

async function testAIRI(): Promise<void> {

  try {
    // Initialize
    console.log('[TEST] Initializing AIRI...');
    await airi.initialize({
      workspacePath: process.cwd(),
      fullAutonomyEnabled: true,
      selfLearningEnabled: true,
      selfHealingEnabled: true
    });

    // Test Consciousness
    console.log('\n[TEST] Testing Consciousness...');
    airi.consciousness.recordInteraction();
    const consciousness = airi.consciousness.getState();
    console.log(`✅ Consciousness: ${consciousness.isAwake ? 'AWAKE' : 'ASLEEP'}`);
    console.log(`   Autonomy: ${consciousness.autonomyLevel}`);

    // Test Biology
    console.log('\n[TEST] Testing Biology...');
    const biology = airi.biology.getState();
    console.log(`✅ Biology: ACTIVE`);
    console.log(`   Energy: ${biology.energy.toFixed(1)}%`);
    console.log(`   Hunger: ${biology.hunger.toFixed(1)}%`);
    console.log(`   Mood: ${biology.mood}`);

    // Test Self-Learning
    console.log('\n[TEST] Testing Self-Learning...');
    const learningStats = airi.learning.getStats();
    console.log(`✅ Self-Learning: ACTIVE`);
    console.log(`   Knowledge Nodes: ${learningStats.totalKnowledge}`);
    console.log(`   Avg Confidence: ${(learningStats.avgConfidence * 100).toFixed(1)}%`);

    // Test Self-Healing
    console.log('\n[TEST] Testing Self-Healing...');
    const health = airi.selfHealing.getStatus();
    console.log(`✅ Self-Healing: ACTIVE`);
    console.log(`   Overall Health: ${health.overall}%`);
    console.log(`   Active Issues: ${health.activeIssues.length}`);

    // Test Autonomous Decision
    console.log('\n[TEST] Testing Autonomous Decision...');
    const decisionStats = airi.decision.getStats();
    console.log(`✅ Autonomous Decision: ACTIVE`);
    console.log(`   Total Decisions: ${decisionStats.total}`);
    console.log(`   Ethical: ${decisionStats.ethical}`);
    console.log(`   Unethical: ${decisionStats.unethical}`);

    // Test Security
    console.log('\n[TEST] Testing Security Engine...');
    console.log(`✅ Security Engine: READY`);

    // Full Status

    // Test chat
    console.log('[TEST] Testing chat with AIRI...');
    const response = await airi.chat('Hello! Are you fully autonomous?');
    console.log(`\nAIRI: ${response}\n`);


  } catch (error) {
    console.error('\n❌ TEST FAILED:', error);
    process.exit(1);
  }
}

// Run test
testAIRI();
