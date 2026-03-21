import { describe, it, expect, vi } from 'vitest'
import { ConfigManager, defaultConfig, configManager } from '../config'

describe('ConfigManager', () => {
  it('should create config manager with default config', () => {
    const manager = new ConfigManager()
    expect(manager.getConfig()).toEqual(defaultConfig)
  })

  it('should create config manager with custom config', () => {
    const customConfig = {
      title: 'Custom Title',
      description: 'Custom Description',
      lang: 'zh-CN'
    }
    const manager = new ConfigManager(customConfig)
    expect(manager.getConfig()).toEqual(customConfig)
  })

  it('should update config', () => {
    const manager = new ConfigManager()
    const update = {
      title: 'Updated Title'
    }
    manager.updateConfig(update)
    expect(manager.getConfig()).toEqual({
      ...defaultConfig,
      ...update
    })
  })

  it('should create config from object', () => {
    const obj = {
      title: 'Object Title',
      description: 'Object Description',
      lang: 'zh-CN'
    }
    const manager = ConfigManager.fromObject(obj)
    expect(manager.getConfig()).toEqual(obj)
  })

  it('should create config from JSON string', () => {
    const json = JSON.stringify({
      title: 'JSON Title',
      description: 'JSON Description',
      lang: 'zh-CN'
    })
    const manager = ConfigManager.fromJSON(json)
    expect(manager.getConfig()).toEqual({
      title: 'JSON Title',
      description: 'JSON Description',
      lang: 'zh-CN'
    })
  })

  it('should use default values for missing fields', () => {
    const obj = {
      title: 'Only Title'
    }
    const manager = ConfigManager.fromObject(obj)
    expect(manager.getConfig()).toEqual({
      title: 'Only Title',
      description: defaultConfig.description,
      lang: defaultConfig.lang
    })
  })

  it('should use global config manager', () => {
    expect(configManager.getConfig()).toEqual(defaultConfig)
  })

  it('should throw error when loading from file in browser environment', async () => {
    // Mock browser environment
    const originalWindow = globalThis.window
    ;(globalThis as any).window = {} as Window

    try {
      await ConfigManager.fromFile('config.json')
      expect.fail('Should have thrown an error')
    } catch (error) {
      expect((error as Error).message).toBe('Cannot load config from file in browser environment')
    } finally {
      // Restore original environment
      ;(globalThis as any).window = originalWindow
    }
  })

  it('should load config from file in Node.js environment', async () => {
    // Mock Node.js environment
    const originalWindow = globalThis.window
    ;(globalThis as any).window = undefined

    // Mock fs module
    const mockReadFile = vi.fn().mockResolvedValue(JSON.stringify({
      title: 'File Title',
      description: 'File Description',
      lang: 'zh-CN'
    }))

    vi.doMock('fs/promises', () => ({
      readFile: mockReadFile
    }))

    try {
      const manager = await ConfigManager.fromFile('config.json')
      expect(manager.getConfig()).toEqual({
        title: 'File Title',
        description: 'File Description',
        lang: 'zh-CN'
      })
      expect(mockReadFile).toHaveBeenCalledWith('config.json', 'utf-8')
    } finally {
      // Restore original environment
      ;(globalThis as any).window = originalWindow
      vi.resetModules()
    }
  })
})
