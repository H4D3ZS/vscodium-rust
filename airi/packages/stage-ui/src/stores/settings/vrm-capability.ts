export type StageVrmCapabilityStatus = 'pending' | 'supported' | 'unsupported'

export interface StageVrmHardwareDetails {
  webgl2Supported: boolean
  maxTextureSize: number | null
  hardwareConcurrency: number | null
  deviceMemoryGB: number | null
  renderer: string
}

export interface StageVrmCapability {
  status: StageVrmCapabilityStatus
  supported: boolean
  reasons: string[]
  details: StageVrmHardwareDetails
  checkedAt: number
}

interface NavigatorWithDeviceMemory extends Navigator {
  deviceMemory?: number
}

interface WebGLDebugRendererInfo {
  UNMASKED_RENDERER_WEBGL: number
}

const MIN_HARDWARE_CONCURRENCY = 4
const MIN_DEVICE_MEMORY_GB = 4
const MIN_MAX_TEXTURE_SIZE = 4096
const SOFTWARE_RENDERER_PATTERN = /\b(swiftshader|llvmpipe|software|basic render|mesa offscreen)\b/i

export function createPendingStageVrmCapability(): StageVrmCapability {
  return {
    status: 'pending',
    supported: false,
    reasons: [],
    details: {
      webgl2Supported: false,
      maxTextureSize: null,
      hardwareConcurrency: null,
      deviceMemoryGB: null,
      renderer: '',
    },
    checkedAt: 0,
  }
}

/**
 * Evaluates whether the current renderer is suitable for VRM.
 *
 * Use when:
 * - Deciding whether a VRM stage can be mounted
 * - Explaining why a machine cannot enable the VRM renderer
 *
 * Expects:
 * - Missing optional browser hardware signals are represented as null
 *
 * Returns:
 * - A stable capability object with user-facing failure reasons
 */
export function evaluateStageVrmCapability(details: StageVrmHardwareDetails): StageVrmCapability {
  const reasons: string[] = []

  if (!details.webgl2Supported)
    reasons.push('WebGL2 is required for the VRM renderer.')

  if (details.maxTextureSize !== null && details.maxTextureSize < MIN_MAX_TEXTURE_SIZE) {
    reasons.push(`GPU max texture size must be at least ${MIN_MAX_TEXTURE_SIZE}px.`)
  }

  if (details.hardwareConcurrency !== null && details.hardwareConcurrency < MIN_HARDWARE_CONCURRENCY) {
    reasons.push(`At least ${MIN_HARDWARE_CONCURRENCY} CPU threads are required.`)
  }

  if (details.deviceMemoryGB !== null && details.deviceMemoryGB < MIN_DEVICE_MEMORY_GB) {
    reasons.push(`At least ${MIN_DEVICE_MEMORY_GB} GB system memory is required.`)
  }

  if (SOFTWARE_RENDERER_PATTERN.test(details.renderer))
    reasons.push('Hardware GPU acceleration is required; software rendering is not supported.')

  const supported = reasons.length === 0

  return {
    status: supported ? 'supported' : 'unsupported',
    supported,
    reasons,
    details,
    checkedAt: Date.now(),
  }
}

function toFiniteNumber(value: unknown): number | null {
  return typeof value === 'number' && Number.isFinite(value) ? value : null
}

function getRendererName(gl: WebGL2RenderingContext): string {
  const debugInfo = gl.getExtension('WEBGL_debug_renderer_info') as WebGLDebugRendererInfo | null
  if (!debugInfo)
    return ''

  const renderer = gl.getParameter(debugInfo.UNMASKED_RENDERER_WEBGL)
  return typeof renderer === 'string' ? renderer : ''
}

/**
 * Detects VRM renderer hardware capability in the browser.
 *
 * Use when:
 * - Initializing stage model settings
 * - Refreshing the VRM enablement gate after startup
 *
 * Expects:
 * - A browser-like renderer process with document and navigator available
 *
 * Returns:
 * - Unsupported capability when WebGL2 context creation fails
 */
export function detectStageVrmCapability(): StageVrmCapability {
  const navigatorLike = globalThis.navigator as NavigatorWithDeviceMemory | undefined
  const hardwareConcurrency = toFiniteNumber(navigatorLike?.hardwareConcurrency)
  const deviceMemoryGB = toFiniteNumber(navigatorLike?.deviceMemory)

  if (!globalThis.document) {
    return evaluateStageVrmCapability({
      webgl2Supported: false,
      maxTextureSize: null,
      hardwareConcurrency,
      deviceMemoryGB,
      renderer: '',
    })
  }

  const canvas = document.createElement('canvas')
  const gl = canvas.getContext('webgl2', { failIfMajorPerformanceCaveat: true })

  if (!gl) {
    return evaluateStageVrmCapability({
      webgl2Supported: false,
      maxTextureSize: null,
      hardwareConcurrency,
      deviceMemoryGB,
      renderer: '',
    })
  }

  const maxTextureSize = toFiniteNumber(gl.getParameter(gl.MAX_TEXTURE_SIZE))
  const renderer = getRendererName(gl)

  return evaluateStageVrmCapability({
    webgl2Supported: true,
    maxTextureSize,
    hardwareConcurrency,
    deviceMemoryGB,
    renderer,
  })
}
