/**
 * Resource Lifecycle Manager
 *
 * Central registry for all intervals, listeners, and timers. Every resource
 * must be registered here and will be automatically cleaned up on demand.
 *
 * Root cause of memory leaks: intervals and listeners are created ad-hoc
 * with no central tracking. This module provides a single place to manage
 * them all.
 *
 * Usage:
 *   import { registry } from './resourceRegistry';
 *
 *   // Register an interval
 *   const id = registry.addInterval(() => doWork(), 5000, 'polling');
 *
 *   // Register an event listener
 *   const id = registry.addListener(window, 'click', handler, 'ui');
 *
 *   // Clean up all resources in a category
 *   registry.cleanup('polling');
 *
 *   // Clean up everything
 *   registry.cleanupAll();
 */

type ResourceId = string;

interface Resource {
  id: ResourceId;
  category: string;
  type: 'interval' | 'timeout' | 'listener' | 'subscription';
  cleanup: () => void;
  created: number;
  description?: string;
}

class ResourceRegistry {
  private resources: Map<ResourceId, Resource> = new Map();
  private nextId = 1;

  /**
   * Register a setInterval. Returns an ID for cleanup.
   */
  addInterval(
    fn: () => void,
    ms: number,
    category: string = 'default',
    description?: string
  ): ResourceId {
    const id = `res_${this.nextId++}`;
    const handle = setInterval(fn, ms);
    this.resources.set(id, {
      id,
      category,
      type: 'interval',
      cleanup: () => clearInterval(handle),
      created: Date.now(),
      description,
    });
    return id;
  }

  /**
   * Register a setTimeout. Returns an ID for cleanup.
   */
  addTimeout(
    fn: () => void,
    ms: number,
    category: string = 'default',
    description?: string
  ): ResourceId {
    const id = `res_${this.nextId++}`;
    const handle = setTimeout(() => {
      fn();
      this.resources.delete(id); // auto-remove after execution
    }, ms);
    this.resources.set(id, {
      id,
      category,
      type: 'timeout',
      cleanup: () => clearTimeout(handle),
      created: Date.now(),
      description,
    });
    return id;
  }

  /**
   * Register an event listener. Returns an ID for cleanup.
   */
  addListener<K extends keyof WindowEventMap>(
    target: Window | Document | HTMLElement,
    event: string,
    handler: EventListener,
    category: string = 'default',
    description?: string
  ): ResourceId {
    const id = `res_${this.nextId++}`;
    target.addEventListener(event, handler);
    this.resources.set(id, {
      id,
      category,
      type: 'listener',
      cleanup: () => target.removeEventListener(event, handler),
      created: Date.now(),
      description,
    });
    return id;
  }

  /**
   * Register a Tauri listen subscription. Returns an ID for cleanup.
   */
  addSubscription(
    unlisten: () => void,
    category: string = 'default',
    description?: string
  ): ResourceId {
    const id = `res_${this.nextId++}`;
    this.resources.set(id, {
      id,
      category,
      type: 'subscription',
      cleanup: unlisten,
      created: Date.now(),
      description,
    });
    return id;
  }

  /**
   * Register a custom cleanup function.
   */
  addCustom(
    cleanup: () => void,
    category: string = 'default',
    description?: string
  ): ResourceId {
    const id = `res_${this.nextId++}`;
    this.resources.set(id, {
      id,
      category,
      type: 'listener', // generic type
      cleanup,
      created: Date.now(),
      description,
    });
    return id;
  }

  /**
   * Remove a specific resource by ID.
   */
  remove(id: ResourceId): boolean {
    const resource = this.resources.get(id);
    if (resource) {
      try { resource.cleanup(); } catch { /* non-fatal */ }
      this.resources.delete(id);
      return true;
    }
    return false;
  }

  /**
   * Clean up all resources in a category.
   */
  cleanup(category: string): number {
    let count = 0;
    for (const [id, resource] of this.resources) {
      if (resource.category === category) {
        try { resource.cleanup(); } catch { /* non-fatal */ }
        this.resources.delete(id);
        count++;
      }
    }
    return count;
  }

  /**
   * Clean up ALL resources. Call this on component unmount or app shutdown.
   */
  cleanupAll(): number {
    const count = this.resources.size;
    for (const [id, resource] of this.resources) {
      try { resource.cleanup(); } catch { /* non-fatal */ }
    }
    this.resources.clear();
    return count;
  }

  /**
   * Get stats about current resources.
   */
  stats(): {
    total: number;
    byCategory: Record<string, number>;
    byType: Record<string, number>;
    oldestAge: number;
  } {
    const byCategory: Record<string, number> = {};
    const byType: Record<string, number> = {};
    let oldest = Date.now();

    for (const resource of this.resources.values()) {
      byCategory[resource.category] = (byCategory[resource.category] || 0) + 1;
      byType[resource.type] = (byType[resource.type] || 0) + 1;
      if (resource.created < oldest) oldest = resource.created;
    }

    return {
      total: this.resources.size,
      byCategory,
      byType,
      oldestAge: Date.now() - oldest,
    };
  }

  /**
   * List all resources (for debugging).
   */
  list(): Resource[] {
    return Array.from(this.resources.values());
  }
}

/** Singleton resource registry — one per app lifetime. */
export const registry = new ResourceRegistry();

/**
 * React hook for automatic cleanup on unmount.
 * Usage: useResourceCleanup('component-name');
 * This cleans up all resources in the given category when the component unmounts.
 */
export function createCleanupEffect(category: string): () => void {
  return () => {
    const cleaned = registry.cleanup(category);
    if (cleaned > 0) {
      console.log(`[ResourceRegistry] Cleaned up ${cleaned} resources in category '${category}'`);
    }
  };
}
