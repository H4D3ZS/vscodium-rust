import type { Plugin } from 'vue'
import type { Router, RouteRecordRaw } from 'vue-router'

import Tres from '@tresjs/core'
import NProgress from 'nprogress'

import { autoAnimatePlugin } from '@formkit/auto-animate/vue'
import { isEnvTruthy } from '@proj-airi/stage-shared'
import { MotionPlugin } from '@vueuse/motion'
import { createPinia } from 'pinia'
import { setupLayouts } from 'virtual:generated-layouts'
import { createApp } from 'vue'
import { createRouter, createWebHashHistory, createWebHistory } from 'vue-router'
import { routes } from 'vue-router/auto-routes'
import { env } from '@huggingface/transformers'

// Configure transformers for browser/Tauri compatibility
env.allowLocalModels = false;
env.allowRemoteModels = true;
// Disable WASM proxy to avoid worker communication issues in some environments
if (env.backends.onnx.wasm) {
  env.backends.onnx.wasm.proxy = false;
}

// Global promise rejection handling to prevent infinite spam in dev mode
window.addEventListener('unhandledrejection', (event) => {
  // Silent certain spammy errors that we know are handled internally or harmless
  if (event.reason?.message?.includes('Unable to determine content-length')) return;
  if (event.reason?.message?.includes('requestAdapter')) return;
  
  console.error('[Global] Unhandled Promise Rejection:', event.reason);
  // Prevent Vite from showing the error overlay for these
  event.preventDefault();
});

import App from './App.vue'

import { i18n } from './modules/i18n'

import '@proj-airi/font-cjkfonts-allseto/index.css'
import '@proj-airi/font-xiaolai/index.css'
import '@unocss/reset/tailwind.css'
import 'splitpanes/dist/splitpanes.css'
import 'vue-sonner/style.css'
import './styles/main.css'
import 'uno.css'

const pinia = createPinia()

// TODO: vite-plugin-vue-layouts is long deprecated, replace with another layout solution
const routeRecords = setupLayouts(routes as RouteRecordRaw[])

let router: Router
if (isEnvTruthy(import.meta.env.VITE_APP_TARGET_HUGGINGFACE_SPACE))
  router = createRouter({ routes: routeRecords, history: createWebHashHistory() })
else
  router = createRouter({ routes: routeRecords, history: createWebHistory() })

router.beforeEach((to, from) => {
  if (to.path !== from.path)
    NProgress.start()
})

router.afterEach(() => {
  NProgress.done()
})

const app = createApp(App)

// Check URL parameters for model/character selection (for external integration)
const urlParams = new URLSearchParams(window.location.search)
const charParam = urlParams.get('char')
const modelUrlParam = urlParams.get('modelUrl')

if (charParam || modelUrlParam) {
  console.log('[Main] 🎭 URL model params detected:', { char: charParam, modelUrl: modelUrlParam })
  
  // Wait for pinia to be ready, then set the model
  setTimeout(() => {
    try {
      // Import store dynamically after pinia is initialized
      import('@proj-airi/stage-ui/stores/settings/stage-model').then(({ useSettingsStageModel }) => {
        const store = useSettingsStageModel(pinia)
        
        if (charParam) {
          // Map common character IDs to preset IDs
          const charMap: Record<string, string> = {
            'hiyori_pro': 'preset-live2d-1',
            'hiyori_free': 'preset-live2d-2',
            'avatar_a': 'preset-vrm-1',
            'avatar_b': 'preset-vrm-2',
            'airi': 'preset-live2d-1',
            'sage': 'preset-live2d-3',
            'nova': 'preset-live2d-4',
            'kawaii': 'preset-live2d-5',
            'sentinel': 'preset-live2d-6',
            'oracle': 'preset-live2d-7',
            'phantom': 'preset-live2d-8',
            'titan': 'preset-live2d-9',
          }
          
          const mappedId = charMap[charParam] || charParam
          store.stageModelSelected = mappedId
          console.log('[Main] ✅ Character set to:', mappedId)
        }
        
        if (modelUrlParam) {
          store.replaceStageModelUrl(modelUrlParam)
          console.log('[Main] ✅ Model URL set to:', modelUrlParam)
        }
      }).catch(err => {
        console.error('[Main] ❌ Failed to set model from URL:', err)
      })
    } catch (err) {
      console.error('[Main] ❌ Error applying URL model params:', err)
    }
  }, 500)
}

app
  .use(MotionPlugin)
  // TODO: Fix autoAnimatePlugin type error
  .use(autoAnimatePlugin as unknown as Plugin)
  .use(router)
  .use(pinia)
  .use(i18n)
  .use(Tres)
  .mount('#app')

if (import.meta.env.DEV && !import.meta.env.SSR) {
  function captureEvents(el: HTMLElement) {
    // Force `pointer-events: auto` as DismissableLayer in Reka UI adds
    // `pointer-events: none` to document body.
    el.style.pointerEvents = 'auto'

    // We need to capture events inside elements like devtools to prevent them
    // from leaking to other layers (like DismissableLayer in Reka UI).
    //
    // See: https://github.com/unovue/reka-ui/blob/14866201d179b8bae3c8b4346a1ca8eff1c5eaa4/packages/radix-vue/src/DismissableLayer/DismissableLayer.vue#L186-L188
    el.addEventListener('focus', e => e.stopPropagation(), { capture: true })
    el.addEventListener('blur', e => e.stopPropagation(), { capture: true })
    el.addEventListener('pointerdown', e => e.stopPropagation(), { capture: true })
  }

  const observer = new MutationObserver((mutationsList, observer) => {
    for (const mutation of mutationsList) {
      if (mutation.type === 'childList') {
        const devtoolsContainer = document.getElementById('__vue-devtools-container__')

        if (devtoolsContainer) {
          captureEvents(devtoolsContainer)
          observer.disconnect()
        }
      }
    }
  })

  observer.observe(document.body, { childList: true, subtree: true })

  // Disconnect on timeout in case the MutationObserver is left here forever.
  // `observer.disconnect()` is idempotent, so it's safe to call it multiple times.
  setTimeout(() => observer.disconnect(), 15 * 1000)
}
