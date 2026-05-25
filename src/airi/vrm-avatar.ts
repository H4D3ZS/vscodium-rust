// @ts-nocheck — work-in-progress AIRI subsystem; types stabilised once interfaces settle.
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
  private camera: THREE.PerspectiveCamera;
  private renderer: THREE.WebGLRenderer | null = null;
  private vrm: VRM | null = null;
  private mixer: THREE.AnimationMixer | null = null;
  private clock: THREE.Clock;
  private state: AvatarState;
  private isInitialized: boolean = false;
  private animationFrameId: number | null = null;
  private container: HTMLDivElement | null = null;

  constructor() {
    this.scene = new THREE.Scene();
    this.camera = new THREE.PerspectiveCamera(30, 1, 0.1, 20);
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
  }

  /**
   * Initialize the VRM avatar natively inside a WebGL Canvas
   */
  async initialize(container?: HTMLDivElement, vrmUrl?: string): Promise<boolean> {
    this.dispose();

    // Guard: container must be a mounted DOM node with layout
    if (!container || !container.isConnected) {
      console.warn('[VRM] initialize() called with unmounted container; skipping.');
      return false;
    }

    try {
      this.container = container;
      const width = container.clientWidth || 300;
      const height = container.clientHeight || 300;

      this.renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
      this.renderer.setSize(width, height);
      this.renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
      this.renderer.outputColorSpace = THREE.SRGBColorSpace;

      // Clear container and append canvas
      container.innerHTML = '';
      container.appendChild(this.renderer.domElement);

      // Setup camera — tighter framing: look at upper chest so head fills frame
      this.camera.aspect = width / height;
      this.camera.updateProjectionMatrix();
      this.camera.position.set(0, 1.55, 0.65);
      this.camera.lookAt(0, 1.45, 0);

      // Setup lights (clear scene first to prevent duplicate lights)
      while (this.scene.children.length > 0) {
        const obj = this.scene.children[0];
        this.scene.remove(obj);
      }

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
      const isLikelyVrm = vrmUrlOrDefault.toLowerCase().endsWith('.vrm');
      if (!isLikelyVrm) {
        console.warn('[VRM] Invalid model URL (expected .vrm):', vrmUrlOrDefault);
        this._cleanupRenderer(container);
        return false;
      }

      const gltf = await new Promise<any>((resolve, reject) => {
        loader.load(
          vrmUrlOrDefault,
          resolve,
          (_progress) => { },
          reject
        );
      });

      this.vrm = gltf.userData.vrm;
      if (!this.vrm) {
        console.warn('[VRM] Loaded GLTF but no VRM data found; skipping.');
        this._cleanupRenderer(container);
        return false;
      }
      this.scene.add(this.vrm.scene);

      // Setup animation mixer
      this.mixer = new THREE.AnimationMixer(this.vrm.scene);

      // Look at camera
      if (this.vrm.meta?.metaVersion === '0' || this.vrm.meta?.metaVersion === '1') {
        const lookAtTarget = new THREE.Object3D();
        lookAtTarget.position.set(0, 1.55, 0);
        this.scene.add(lookAtTarget);

        if (this.vrm.lookAt) {
          this.vrm.lookAt.target = lookAtTarget;
        }
      }

      this.isInitialized = true;
      this.animate();

      return true;

    } catch (error: any) {
      const msg = String(error?.message || error || '');
      if (msg.includes('Unrecognized token') || msg.includes('<')) {
        console.warn('[VRM] Model URL returned HTML/non-VRM payload; avatar disabled.');
      } else {
        console.error('[VRM] ❌ Failed to load VRM:', error);
      }
      // Always clean up the renderer canvas on failure so the DOM doesn't
      // show an empty/transparent Three.js canvas (which causes the visual gap).
      this._cleanupRenderer(this.container);
      return false;
    }
  }

  /** Remove the WebGL canvas from the container and free GPU resources. */
  private _cleanupRenderer(container: HTMLDivElement | null): void {
    if (this.renderer) {
      try {
        if (container && this.renderer.domElement.parentElement === container) {
          container.removeChild(this.renderer.domElement);
        }
        this.renderer.dispose();
      } catch { /* ignore */ }
      this.renderer = null;
    }
  }

  private lastRenderTime: number = 0;
  private targetFps: number = 24; // Throttle to 24fps to drastically save CPU

  /**
   * Animation loop
   */
  private animate = (): void => {
    if (!this.isInitialized) return;
    this.animationFrameId = requestAnimationFrame(this.animate);

    const now = performance.now();
    const elapsed = now - this.lastRenderTime;
    const fpsInterval = 1000 / this.targetFps;

    // Throttle rendering to save CPU
    if (elapsed < fpsInterval && this.lastRenderTime !== 0) {
      return;
    }

    // Update last render time accounting for missed frames
    if (this.lastRenderTime === 0) {
      this.lastRenderTime = now;
    } else {
      this.lastRenderTime = now - (elapsed % fpsInterval);
    }

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

      // Speaking mouth animation
      this.updateSpeaking(delta);

      // Ambient animation based on state
      this.updateAmbient(delta);
    }

    // Render
    if (this.renderer) {
      this.renderer.render(this.scene, this.camera);
    }
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
   * Update speaking animation with smooth, natural lip sync
   */
  private updateSpeaking(delta: number): void {
    if (!this.vrm || !this.vrm.expressionManager) return;

    if (this.state.isSpeaking) {
      // Dynamic mouth movement using time-based sine wave
      const time = Date.now() * 0.012; // Adjust speed for a natural talking pace
      const mouthOpen = 0.15 + (Math.sin(time) + 1.0) * 0.25; // values between 0.15 and 0.65
      const mouthWidth = 0.1 + (Math.cos(time * 0.8) + 1.0) * 0.1;

      this.vrm.expressionManager.setValue('aa', mouthOpen);
      this.vrm.expressionManager.setValue('ih', mouthWidth);
    } else {
      // Ensure mouth is fully closed when not speaking
      this.vrm.expressionManager.setValue('aa', 0);
      this.vrm.expressionManager.setValue('ih', 0);
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
        this.vrm.expressionManager.setValue('joy', 0.8);
        this.vrm.expressionManager.setValue('eyeHappy', 0.6);
        break;

      case 'excited':
        this.vrm.expressionManager.setValue('joy', 1.0);
        this.vrm.expressionManager.setValue('fun', 0.5);
        this.vrm.expressionManager.setValue('eyeSurprised', 0.3);
        break;

      case 'thinking':
        this.vrm.expressionManager.setValue('lookup', 0.5);
        this.vrm.expressionManager.setValue('mouthFunnel', 0.3);
        break;

      case 'concerned':
        this.vrm.expressionManager.setValue('sorrow', 0.5);
        this.vrm.expressionManager.setValue('mouthTight', 0.4);
        break;

      case 'tired':
        this.vrm.expressionManager.setValue('sorrow', 0.4);
        this.vrm.expressionManager.setValue('blinkLeft', 0.3);
        this.vrm.expressionManager.setValue('blinkRight', 0.3);
        break;

      case 'focused':
        this.vrm.expressionManager.setValue('lookup', 0.3);
        break;

      case 'surprised':
        this.vrm.expressionManager.setValue('eyeSurprised', 0.8);
        this.vrm.expressionManager.setValue('aa', 0.3);
        break;

      default:
        // Neutral - no additional expressions
        break;
    }
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
    this.state.isSpeaking = isSpeaking;
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
      if (this.vrm?.humanoid.getNormalizedBoneNode('head')) {
        this.vrm.humanoid.getNormalizedBoneNode('head')!.rotation.x = 0;
      }
    }
  }

  /**
   * Set thinking state
   */
  setThinking(isThinking: boolean): void {
    this.state.isThinking = isThinking;

    if (isThinking) {
      this.setEmotion('thinking');
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

    if (lower.includes('happy') || lower.includes('great') || lower.includes('awesome') || lower.includes('perfect')) {
      this.setEmotion('happy');
    } else if (lower.includes('sad') || lower.includes('frustrated') || lower.includes('stuck') || lower.includes('error')) {
      this.setEmotion('concerned');
    } else if (lower.includes('excited') || lower.includes('amazing') || lower.includes('wow')) {
      this.setEmotion('excited');
    } else if (lower.includes('think') || lower.includes('wonder') || lower.includes('question') || lower.includes('why')) {
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
   * Resize handler adapting to container boundaries
   */
  onResize(): void {
    if (!this.isInitialized || !this.container || !this.renderer) return;

    const width = this.container.clientWidth || 400;
    const height = this.container.clientHeight || 400;
    this.camera.aspect = width / height;
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(width, height);
  }

  /**
   * Cleanup resources to prevent frame loops and memory leaks
   */
  dispose(): void {
    this.isInitialized = false;

    if (this.animationFrameId !== null) {
      cancelAnimationFrame(this.animationFrameId);
      this.animationFrameId = null;
    }

    if (this.vrm) {
      this.scene.remove(this.vrm.scene);
      this.vrm.scene.traverse((obj) => {
        if (obj.isMesh) {
          obj.geometry.dispose();
          if (obj.material) {
            if (Array.isArray(obj.material)) {
              obj.material.forEach((m) => m.dispose());
            } else {
              obj.material.dispose();
            }
          }
        }
      });
      this.vrm = null;
    }

    if (this.renderer) {
      this.renderer.dispose();
      this.renderer.domElement.remove();
      this.renderer = null;
    }

    this.container = null;
  }
}

// Export singleton
export const airiVRMAvatar = new AIRIVRMAvatar();
