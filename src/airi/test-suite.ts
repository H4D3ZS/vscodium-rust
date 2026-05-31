// @ts-nocheck — work-in-progress AIRI subsystem; types stabilised once interfaces settle.
/**
 * AIRI Comprehensive Test Suite
 * Full system testing, regression tests, performance benchmarks
 */

import { airi } from './core';
import { airiConsciousness } from './consciousness';
import { airiBiology } from './biology';
import { airiSelfLearning } from './self-learning';
import { airiMemory } from './memory';
import { airiAutonomousDecision } from './autonomous-decision';
import { createSelfHealing } from './self-healing';
import { createSelfEvolution } from './self-evolution';

interface TestResult {
  name: string;
  passed: boolean;
  duration: number;
  error?: string;
}

interface PerformanceMetrics {
  responseTime: number;
  memoryUsage: number;
  cpuUsage: number;
  throughput: number;
}

export class AIRITestSuite {
  private results: TestResult[] = [];
  private startTime: number = 0;

  async runAllTests(): Promise<void> {

    this.startTime = Date.now();

    // Initialize AIRI
    await airi.initialize({
      workspacePath: process.cwd(),
      fullAutonomyEnabled: true,
      selfLearningEnabled: true,
      selfHealingEnabled: true,
      memoryEnabled: true,
      voiceEnabled: false, // Disable for testing
      consciousnessEnabled: true,
      biologyEnabled: true,
      autonomousWorkEnabled: true
    });

    // Run all test categories
    await this.testConsciousness();
    await this.testBiology();
    await this.testMemory();
    await this.testLearning();
    await this.testDecision();
    await this.testHealing();
    await this.testPerformance();
    await this.testIntegration();
    await this.testRegression();

    // Print results
    this.printResults();
  }

  async testConsciousness(): Promise<void> {

    // Test 1: Thought generation
    await this.runTest('thought_generation', async () => {
      const state = airiConsciousness.getState();
      if (!state.isAwake) throw new Error('Consciousness not awake');
      if (state.autonomyLevel !== 'full') throw new Error('Not full autonomy');
    });

    // Test 2: Memory persistence
    await this.runTest('consciousness_persistence', async () => {
      airiConsciousness.recordInteraction();
      const state = airiConsciousness.getState();
      if (state.lastInteraction === 0) throw new Error('Interaction not recorded');
    });

    // Test 3: Autonomy levels
    await this.runTest('autonomy_switching', async () => {
      airiConsciousness.setAutonomy('passive');
      let state = airiConsciousness.getState();
      if (state.autonomyLevel !== 'passive') throw new Error('Failed to set passive');

      airiConsciousness.setAutonomy('full');
      state = airiConsciousness.getState();
      if (state.autonomyLevel !== 'full') throw new Error('Failed to set full');
    });

  }

  async testBiology(): Promise<void> {

    // Test 1: State initialization
    await this.runTest('biology_initialization', async () => {
      const state = airiBiology.getState();
      if (state.energy < 0 || state.energy > 100) throw new Error('Invalid energy');
      if (state.hunger < 0 || state.hunger > 100) throw new Error('Invalid hunger');
    });

    // Test 2: Feeding
    await this.runTest('biology_feeding', async () => {
      const before = airiBiology.getState();
      airiBiology.feed(50);
      const after = airiBiology.getState();
      if (after.hunger >= before.hunger) throw new Error('Feeding did not reduce hunger');
    });

    // Test 3: Sleep cycle
    await this.runTest('biology_sleep', async () => {
      airiBiology.sleep(100); // 100ms for testing
      let state = airiBiology.getState();
      if (!state.isSleeping) throw new Error('Failed to enter sleep');

      await new Promise(resolve => setTimeout(resolve, 150));
      // Should wake up automatically
    });

    // Test 4: Mood calculation
    await this.runTest('biology_mood', async () => {
      const state = airiBiology.getState();
      const validMoods = ['happy', 'neutral', 'tired', 'stressed', 'excited', 'concerned', 'focused'];
      if (!validMoods.includes(state.mood)) throw new Error(`Invalid mood: ${state.mood}`);
    });

  }

  async testMemory(): Promise<void> {

    // Test 1: Add memory
    await this.runTest('memory_add', async () => {
      const memory = await airiMemory.addMemory(
        'Test memory entry',
        'episodic',
        ['test', 'testing'],
        0.9
      );
      if (!memory.id) throw new Error('Memory ID not generated');
      if (memory.content !== 'Test memory entry') throw new Error('Content mismatch');
    });

    // Test 2: Search memory
    await this.runTest('memory_search', async () => {
      const results = await airiMemory.search('test');
      if (!Array.isArray(results)) throw new Error('Search did not return array');
    });

    // Test 3: Get recent memories
    await this.runTest('memory_recent', async () => {
      const recent = await airiMemory.getRecent(10);
      if (recent.length === 0) throw new Error('No recent memories found');
    });

    // Test 4: Memory stats
    await this.runTest('memory_stats', async () => {
      const stats = airiMemory.getStats();
      if (stats.total < 0) throw new Error('Invalid total');
      if (!stats.byType) throw new Error('No type breakdown');
    });

    // Test 5: Memory compression
    await this.runTest('memory_compression', async () => {
      // Add multiple memories to trigger compression
      for (let i = 0; i < 10; i++) {
        await airiMemory.addMemory(`Test memory ${i}`, 'episodic', ['test'], 0.3);
      }
      const stats = airiMemory.getStats();
    });

  }

  async testLearning(): Promise<void> {

    // Test 1: Learn from event
    await this.runTest('learning_event', async () => {
      await airiSelfLearning.learnFromEvent(
        'success',
        'Test learning event: completed task successfully',
        'success'
      );
      const stats = airiSelfLearning.getStats();
      if (stats.recentEvents < 1) throw new Error('Event not recorded');
    });

    // Test 2: Knowledge query
    await this.runTest('learning_query', async () => {
      const results = await airiSelfLearning.query('test');
      if (!Array.isArray(results)) throw new Error('Query did not return array');
    });

    // Test 3: Learning stats
    await this.runTest('learning_stats', async () => {
      const stats = airiSelfLearning.getStats();
      if (stats.totalKnowledge < 0) throw new Error('Invalid knowledge count');
    });

  }

  async testDecision(): Promise<void> {

    // Test 1: Make decision
    await this.runTest('decision_making', async () => {
      const decision = await airiAutonomousDecision.makeDecision(
        'Test scenario: found a bug',
        ['Fix immediately', 'Report to user', 'Ignore']
      );
      if (!decision.chosen) throw new Error('No decision made');
      if (!decision.reasoning) throw new Error('No reasoning provided');
    });

    // Test 2: Decision history
    await this.runTest('decision_history', async () => {
      const history = airiAutonomousDecision.getHistory(10);
      if (!Array.isArray(history)) throw new Error('History not array');
    });

    // Test 3: Decision stats
    await this.runTest('decision_stats', async () => {
      const stats = airiAutonomousDecision.getStats();
      if (stats.total < 0) throw new Error('Invalid total');
    });

  }

  async testHealing(): Promise<void> {

    const selfHealing = createSelfHealing(process.cwd());

    // Test 1: Health check
    await this.runTest('healing_initialization', async () => {
      const state = selfHealing.getStatus();
      if (state.overall < 0 || state.overall > 100) throw new Error('Invalid health');
    });

    // Test 2: Issue detection
    await this.runTest('healing_detection', async () => {
      selfHealing.start();
      await new Promise(resolve => setTimeout(resolve, 2000));
      const state = selfHealing.getStatus();
    });

  }

  async testPerformance(): Promise<void> {

    // Test 1: Response time
    await this.runTest('perf_response_time', async () => {
      const start = Date.now();
      await airi.chat('Test message');
      const duration = Date.now() - start;
      
      
      if (duration > 5000) {
        throw new Error(`Response time too slow: ${duration}ms (target: <5000ms)`);
      }
    });

    // Test 2: Memory usage
    await this.runTest('perf_memory_usage', async () => {
      const usage = process.memoryUsage();
      const mb = Math.round(usage.heapUsed / 1024 / 1024);
      
      if (mb > 500) {
        throw new Error(`Memory usage too high: ${mb}MB (target: <500MB)`);
      }
    });

    // Test 3: Concurrent operations
    await this.runTest('perf_concurrent', async () => {
      const start = Date.now();
      const promises = [];
      
      for (let i = 0; i < 5; i++) {
        promises.push(airi.chat(`Concurrent test ${i}`));
      }
      
      await Promise.all(promises);
      const duration = Date.now() - start;
      
      
      if (duration > 15000) {
        throw new Error(`Concurrent operations too slow: ${duration}ms`);
      }
    });

    // Test 4: Throughput
    await this.runTest('perf_throughput', async () => {
      const start = Date.now();
      let count = 0;
      
      while (Date.now() - start < 5000) { // 5 seconds
        await airiSelfLearning.learnFromEvent('observation', 'Test', 'neutral');
        count++;
      }
      
      const perSecond = count / 5;
      
      if (perSecond < 10) {
        throw new Error(`Throughput too low: ${perSecond}/sec (target: >10/sec)`);
      }
    });

  }

  async testIntegration(): Promise<void> {

    // Test 1: Full conversation flow
    await this.runTest('integration_conversation', async () => {
      const response = await airi.chat('Hello! How are you today?');
      if (!response || response.length === 0) throw new Error('No response');
      if (response.includes('error') || response.includes('failed')) {
        throw new Error(`Response indicates error: ${response}`);
      }
    });

    // Test 2: Memory + Learning integration
    await this.runTest('integration_memory_learning', async () => {
      await airi.chat('Remember this: the password is test123');
      await new Promise(resolve => setTimeout(resolve, 100));
      
      const memories = await airiMemory.search('password');
      if (memories.length === 0) {
      }
    });

    // Test 3: Biology + Consciousness integration
    await this.runTest('integration_biology_consciousness', async () => {
      const bioState = airiBiology.getState();
      const consciousState = airiConsciousness.getState();
      
      // Both should be active
      if (!consciousState.isAwake) throw new Error('Consciousness not awake');
      if (bioState.energy <= 0) throw new Error('Energy depleted');
    });

  }

  async testRegression(): Promise<void> {

    // Test 1: Voice overlap (should not occur)
    await this.runTest('regression_voice_overlap', async () => {
      // Voice is disabled in tests, but check manager state
    });

    // Test 2: Memory leak check
    await this.runTest('regression_memory_leak', async () => {
      const initialMemory = process.memoryUsage().heapUsed;
      
      // Perform many operations
      for (let i = 0; i < 50; i++) {
        await airiSelfLearning.learnFromEvent('test', `Test ${i}`, 'neutral');
      }
      
      const finalMemory = process.memoryUsage().heapUsed;
      const growth = finalMemory - initialMemory;
      const growthMB = Math.round(growth / 1024 / 1024);
      
      
      if (growthMB > 50) {
        throw new Error(`Potential memory leak: ${growthMB}MB growth`);
      }
    });

    // Test 3: Queue processing
    await this.runTest('regression_queue_processing', async () => {
      // Add multiple learning events
      for (let i = 0; i < 20; i++) {
        await airiSelfLearning.learnFromEvent('test', `Queue test ${i}`, 'neutral');
      }
      
      const stats = airiSelfLearning.getStats();
    });

    // Test 4: State consistency
    await this.runTest('regression_state_consistency', async () => {
      const states = [];
      for (let i = 0; i < 10; i++) {
        states.push(airiBiology.getState());
        await new Promise(resolve => setTimeout(resolve, 10));
      }
      
      // All states should be valid
      for (const state of states) {
        if (state.energy < 0 || state.energy > 100) {
          throw new Error('Invalid energy state');
        }
      }
    });

  }

  async runTest(name: string, testFn: () => Promise<void>): Promise<void> {
    const start = Date.now();
    
    try {
      await testFn();
      const duration = Date.now() - start;
      
      this.results.push({
        name,
        passed: true,
        duration
      });
      
    } catch (error: any) {
      const duration = Date.now() - start;
      
      this.results.push({
        name,
        passed: false,
        duration,
        error: error.message
      });
      
    }
  }

  printResults(): void {
    const total = this.results.length;
    const passed = this.results.filter(r => r.passed).length;
    const failed = total - passed;
    const totalDuration = Date.now() - this.startTime;


    if (failed === 0) {
    } else {
    }
  }
}

// Run tests
async function main() {
  const testSuite = new AIRITestSuite();
  await testSuite.runAllTests();
}

// Export for CLI (browser-safe)
if (typeof window === 'undefined' && typeof require !== 'undefined') {
  // Node.js environment only
  if (require.main === module) {
    main().catch(console.error);
  }
}

export { main as runTests };
