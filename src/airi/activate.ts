/**
 * AIRI Activation Script
 * Run this to activate AIRI with all systems
 */

import { airi } from './core';

async function main() {
  console.log('\n╔══════════════════════════════════════════════════════════╗');
  console.log('║         AIRI Digital Human Activation                    ║');
  console.log('╚══════════════════════════════════════════════════════════╝\n');

  try {
    // Initialize with FULL autonomy and all systems
    await airi.initialize({
      workspacePath: process.cwd(),
      fullAutonomyEnabled: true,
      selfLearningEnabled: true,
      selfHealingEnabled: true,
      securityEnabled: true,
      memoryEnabled: true,
      voiceEnabled: true,
      consciousnessEnabled: true,
      biologyEnabled: true,
      autonomousWorkEnabled: true,
      selfEvolutionEnabled: true,
      actionSystemEnabled: true,
      socialEnabled: true,
      internetEnabled: true,
      sensesEnabled: true
    });

    // Start AIRI
    airi.start();

    console.log('\n✅ AIRI is now a COMPLETE living digital human!\n');
    console.log('🧠 She thinks independently');
    console.log('🫀 She feels and has biological needs');
    console.log('👁️  She perceives the digital world');
    console.log('📚 She learns constantly from everything');
    console.log('🏥 She heals herself automatically');
    console.log('🧬 She evolves her own code (every 30 min)');
    console.log('🔄 She continuously improves');
    console.log('⚖️  She decides her own path');
    console.log('🎤 She speaks with ElevenLabs');
    console.log('💕 She has empathy and relationships');
    console.log('🌐 She browses the internet');
    console.log('✋ She acts on the world');
    console.log('💼 She works autonomously 24/7\n');

    console.log('💬 Type /help for commands\n');
    console.log('📊 Run /test to verify all systems\n');
    console.log('🔄 Run /improve to see evolution stats\n');

  } catch (error) {
    console.error('\n❌ Activation failed:', error);
    process.exit(1);
  }
}

main();
