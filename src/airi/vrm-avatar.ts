/**
 * AIRI 3D VRM Avatar System
 * Complete 3D avatar integration with interactive expressions, lip-sync, and emotions
 * Connects to voice, conversation, and biological states
 */

import * as THREE from 'three';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader';
import { VRMLoaderPlugin, VRM } from '@pixiv/three-vrm';

export interface AvatarState {
  emotion: AvatarEmotion;
  isSpeaking: boolean;
  isListening: boolean;
  isThinking: boolean;
  energy: number;
  blinkTimer: number;
  lastBlink: number;
}

export type AvatarEmotion =
  | 'neutral'
  | 'happy'
  | 'excited'
  | 'thinking'
  | 'concerned'
  | 'tired'
  | 'focused'
  | 'surprised';

export class AIRIVRMAvatar {
  private scene: THREE.Scene;
  private camera: THREE.Camera;
  private renderer: THREE.WebGLRenderer;
  private vrm: VRM | null;
  private mixer: THREE.AnimationMixer | null;
  private clock: THREE.Clock;
  private state: AvatarState;
  private isInitialized: boolean = false;

  constructor() {
    this.scene = new THREE.Scene();
    this.camera = new THREE.PerspectiveCamera(
      30,
      window.innerWidth / window.innerHeight,
      0.1,
      20
    );
    this.renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
    this.vrm = null;
    this.mixer = null;
    this.clock = new THREE.Clock();
    
    this.state = {
      emotion: 'neutral',
      isSpeaking: false,
      isListening: false,
      isThinking: false,
      energy: 100,
      blinkTimer: 0,
      lastBlink: 0
    };

    console.log('[VRM] ✨ Ready for interactive expressions and lip-sync');
  }

  /**
   * Initialize the VRM avatar
   */
  async initialize(vrmUrl?: string): Promise<boolean> {
    console.log('[VRM] 🚀 Loading VRM model...');

    try {
      // Setup renderer
      this.renderer.setSize(window.innerWidth, window.innerHeight);
      this.renderer.setPixelRatio(window.devicePixelRatio);
      this.renderer.outputEncoding = THREE.sRGBEncoding;
      document.body.appendChild(this.renderer.domElement);

      // Setup camera
      this.camera.position.set(0, 1.4, 0.7);
      this.camera.lookAt(0, 1.2, 0);

      // Setup lights
      const light = new THREE.DirectionalLight(0xffffff, 0.6);
      light.position.set(1, 1, 1);
      this.scene.add(light);

      const ambientLight = new THREE.AmbientLight(0xffffff, 0.4);
      this.scene.add(ambientLight);

      // Load VRM model
      const loader = new GLTFLoader();
      loader.crossOrigin = 'anonymous';

      loader.register((parser) => {
        return new VRMLoaderPlugin(parser, {
          autoUpdateHumanBones: true
        });
      });

      const vrmUrlOrDefault = vrmUrl || '/models/airi.vrm';
      
      const gltf = await new Promise<any>((resolve, reject) => {
        loader.load(
          vrmUrlOrDefault,
          resolve,
          (progress) => {
            console.log(`[VRM] Loading: ${(progress.loaded / progress.total * 100).toFixed(0)}%`);
          },
          reject
        );
      });

      this.vrm = gltf.userData.vrm;
      this.scene.add(this.vrm.scene);

      // Setup animation mixer
      this.mixer = new THREE.AnimationMixer(this.vrm.scene);

      // Look at camera
      if (this.vrm.meta?.metaVersion === '0' || this.vrm.meta?.metaVersion === '1') {
        const lookAtTarget = new THREE.Object3D();
        lookAtTarget.position.set(0, 1.4, 0);
        this.scene.add(lookAtTarget);
        
        if (this.vrm.lookAt) {
          this.vrm.lookAt.target = lookAtTarget;
        }
      }

      this.isInitialized = true;
      this.animate();

      console.log('[VRM] ✅ VRM avatar loaded successfully');
      console.log(`[VRM] 👤 Model: ${this.vrm.meta?.author}`);
      
      return true;

    } catch (error) {
      console.error('[VRM] ❌ Failed to load VRM:', error);
      console.log('[VRM] ⚠️ Using fallback 2D mode');
      return false;
    }
  }

  /**
   * Animation loop
   */
  private animate = (): void => {
    requestAnimationFrame(this.animate);

    const delta = this.clock.getDelta();

    // Update mixer
    if (this.mixer) {
      this.mixer.update(delta);
    }

    // Update VRM
    if (this.vrm) {
      this.vrm.update(delta);

      // Blink animation
      this.updateBlink(delta);

      // Ambient animation based on state
      this.updateAmbient(delta);
    }

    // Render
    this.renderer.render(this.scene, this.camera);
  };

  /**
   * Update blink animation
   */
  private updateBlink(delta: number): void {
    if (!this.vrm || !this.vrm.expressionManager) return;

    this.state.blinkTimer += delta;

    // Blink every 3-5 seconds
    if (this.state.blinkTimer > 3 + Math.random() * 2) {
      this.state.blinkTimer = 0;
      this.state.lastBlink = Date.now();

      // Play blink animation
      this.vrm.expressionManager.setValue('blinkLeft', 1);
      this.vrm.expressionManager.setValue('blinkRight', 1);

      setTimeout(() => {
        if (this.vrm?.expressionManager) {
          this.vrm.expressionManager.setValue('blinkLeft', 0);
          this.vrm.expressionManager.setValue('blinkRight', 0);
        }
      }, 150);
    }
  }

  /**
   * Update ambient animation based on state
   */
  private updateAmbient(delta: number): void {
    if (!this.vrm) return;

    // Subtle breathing animation
    const time = Date.now() * 0.001;
    const breathing = Math.sin(time) * 0.005;

    if (this.vrm.humanoid.getNormalizedBoneNode('hips')) {
      this.vrm.humanoid.getNormalizedBoneNode('hips')!.position.y += breathing;
    }

    // Energy-based animation
    if (this.state.energy < 30) {
      // Tired - slower movement
      this.vrm.scene.rotation.z = Math.sin(time * 0.5) * 0.02;
    } else if (this.state.energy > 80) {
      // Energetic - subtle bounce
      this.vrm.scene.position.y = Math.sin(time * 2) * 0.01;
    }
  }

  /**
   * Set avatar emotion/expression
   */
  setEmotion(emotion: AvatarEmotion): void {
    if (!this.vrm || !this.vrm.expressionManager) return;

    this.state.emotion = emotion;

    // Reset all expressions
    this.resetExpressions();

    // Set emotion-specific expressions
    switch (emotion) {
      case 'happy':
        this.vrm.expressionManager.setValue('aa', 0.5);
        this.vrm.expressionManager.setValue('blinkLeft', 0.2);
        this.vrm.expressionManager.setValue('blinkRight', 0.2);
        break;

      case 'excited':
        this.vrm.expressionManager.setValue('aa', 0.8);
        this.vrm.expressionManager.setValue('eyeSurprised', 0.6);
        break;

      case 'thinking':
        this.vrm.expressionManager.setValue('lookup', 0.5);
        this.vrm.expressionManager.setValue('mouthFunnel', 0.3);
        break;

      case 'concerned':
        this.vrm.expressionManager.setValue('angry', 0.3);
        this.vrm.expressionManager.setValue('mouthTight', 0.4);
        break;

      case 'tired':
        this.vrm.expressionManager.setValue('sorrow', 0.4);
        this.vrm.expressionManager.setValue('blinkLeft', 0.5);
        this.vrm.expressionManager.setValue('blinkRight', 0.5);
        break;

      case 'focused':
        this.vrm.expressionManager.setValue('lookup', 0.3);
        break;

      case 'surprised':
        this.vrm.expressionManager.setValue('eyeSurprised', 0.8);
        this.vrm.expressionManager.setValue('aa', 0.6);
        break;

      default:
        // Neutral - no additional expressions
        break;
    }

    console.log(`[VRM] 😊 Emotion set: ${emotion}`);
  }

  /**
   * Reset all expressions
   */
  private resetExpressions(): void {
    if (!this.vrm?.expressionManager) return;

    const expressions = [
      'aa', 'ih', 'ou', 'ee', 'oh',
      'blinkLeft', 'blinkRight',
      'joy', 'angry', 'sorrow', 'fun',
      'lookup', 'lookdown', 'lookleft', 'lookright',
      'eyeSurprised', 'eyeHappy', 'mouthFunnel', 'mouthTight'
    ];

    expressions.forEach(expr => {
      this.vrm!.expressionManager!.setValue(expr, 0);
    });
  }

  /**
   * Lip sync with voice
   */
  setSpeaking(isSpeaking: boolean, audioData?: Float32Array): void {
    if (!this.vrm) return;

    this.state.isSpeaking = isSpeaking;

    if (isSpeaking && audioData) {
      // Analyze audio data for lip sync
      const average = audioData.reduce((a, b) => a + Math.abs(b), 0) / audioData.length;
      const mouthOpen = Math.min(1, average * 2);

      this.vrm.expressionManager?.setValue('aa', mouthOpen);
      this.vrm.expressionManager?.setValue('ih', mouthOpen * 0.5);
    } else if (isSpeaking) {
      // Simple talking animation without audio data
      const time = Date.now() * 0.01;
      const mouthOpen = (Math.sin(time) + 1) * 0.3;
      this.vrm.expressionManager?.setValue('aa', mouthOpen);
    } else {
      // Close mouth
      this.vrm.expressionManager?.setValue('aa', 0);
      this.vrm.expressionManager?.setValue('ih', 0);
    }
  }

  /**
   * Set listening state
   */
  setListening(isListening: boolean): void {
    this.state.isListening = isListening;

    if (isListening) {
      this.setEmotion('focused');
      // Tilt head slightly
      if (this.vrm?.humanoid.getNormalizedBoneNode('head')) {
        this.vrm.humanoid.getNormalizedBoneNode('head')!.rotation.x = 0.1;
      }
    } else {
      this.setEmotion('neutral');
    }
  }

  /**
   * Set thinking state
   */
  setThinking(isThinking: boolean): void {
    this.state.isThinking = isThinking;

    if (isThinking) {
      this.setEmotion('thinking');
      // Hand to chin gesture (if available)
      console.log('[VRM] 🤔 Thinking pose');
    } else {
      this.setEmotion('neutral');
    }
  }

  /**
   * Update energy level
   */
  setEnergy(energy: number): void {
    this.state.energy = energy;

    // Visual feedback for low energy
    if (energy < 30) {
      this.setEmotion('tired');
    } else if (energy > 80) {
      this.setEmotion('excited');
    }
  }

  /**
   * React to conversation
   */
  reactToConversation(text: string): void {
    // Analyze text for emotional content
    const lower = text.toLowerCase();

    if (lower.includes('happy') || lower.includes('great') || lower.includes('awesome')) {
      this.setEmotion('happy');
    } else if (lower.includes('sad') || lower.includes('frustrated') || lower.includes('stuck')) {
      this.setEmotion('concerned');
    } else if (lower.includes('excited') || lower.includes('amazing')) {
      this.setEmotion('excited');
    } else if (lower.includes('think') || lower.includes('wonder') || lower.includes('question')) {
      this.setEmotion('thinking');
    }
  }

  /**
   * Get current avatar state
   */
  getState(): AvatarState {
    return { ...this.state };
  }

  /**
   * Resize handler
   */
  onResize(): void {
    if (!this.isInitialized) return;

    this.camera.aspect = window.innerWidth / window.innerHeight;
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(window.innerWidth, window.innerHeight);
  }

  /**
   * Cleanup
   */
  dispose(): void {
    if (this.vrm) {
      this.vrm.scene.traverse((obj) => {
        if (obj.isMesh) {
          obj.geometry.dispose();
          if (obj.material) {
            obj.material.dispose();
          }
        }
      });
    }
    
    this.renderer.dispose();
    this.renderer.domElement.remove();
    this.isInitialized = false;
  }
}

// Export singleton
export const airiVRMAvatar = new AIRIVRMAvatar();
