import { describe, expect, it } from 'vitest'

import { evaluateStageVrmCapability } from './vrm-capability'

describe('evaluateStageVrmCapability', () => {
  it('supports VRM when the renderer meets the required baseline', () => {
    const result = evaluateStageVrmCapability({
      webgl2Supported: true,
      maxTextureSize: 8192,
      hardwareConcurrency: 8,
      deviceMemoryGB: 16,
      renderer: 'NVIDIA GeForce RTX 4060',
    })

    expect(result.status).toBe('supported')
    expect(result.supported).toBe(true)
    expect(result.reasons).toEqual([])
  })

  it('disables VRM when required hardware signals are below baseline', () => {
    const result = evaluateStageVrmCapability({
      webgl2Supported: false,
      maxTextureSize: 2048,
      hardwareConcurrency: 2,
      deviceMemoryGB: 2,
      renderer: 'Google SwiftShader',
    })

    expect(result.status).toBe('unsupported')
    expect(result.supported).toBe(false)
    expect(result.reasons).toEqual([
      'WebGL2 is required for the VRM renderer.',
      'GPU max texture size must be at least 4096px.',
      'At least 4 CPU threads are required.',
      'At least 4 GB system memory is required.',
      'Hardware GPU acceleration is required; software rendering is not supported.',
    ])
  })

  it('does not fail optional hardware signals that browsers hide', () => {
    const result = evaluateStageVrmCapability({
      webgl2Supported: true,
      maxTextureSize: null,
      hardwareConcurrency: null,
      deviceMemoryGB: null,
      renderer: '',
    })

    expect(result.status).toBe('supported')
    expect(result.supported).toBe(true)
  })
})
