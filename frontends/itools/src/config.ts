import { Config } from './types'

// Default config
export const defaultConfig: Config = {
  title: 'iTools',
  description: 'Interactive tools framework',
  lang: 'en'
}

// Config manager
export class ConfigManager {
  private config: Config

  constructor(config: Config = defaultConfig) {
    this.config = config
  }

  getConfig(): Config {
    return this.config
  }

  updateConfig(config: Partial<Config>): void {
    this.config = { ...this.config, ...config }
  }

  // Load config from object
  static fromObject(obj: any): ConfigManager {
    const config: Config = {
      title: obj.title || defaultConfig.title,
      description: obj.description || defaultConfig.description,
      lang: obj.lang || defaultConfig.lang,
      themeConfig: obj.themeConfig
    }
    return new ConfigManager(config)
  }

  // Load config from JSON string
  static fromJSON(json: string): ConfigManager {
    const obj = JSON.parse(json)
    return this.fromObject(obj)
  }

  // Load config from file (Node.js only)
  static async fromFile(path: string): Promise<ConfigManager> {
    if (typeof window !== 'undefined') {
      throw new Error('Cannot load config from file in browser environment')
    }

    const fs = await import('fs/promises')
    const content = await fs.readFile(path, 'utf-8')
    return this.fromJSON(content)
  }
}

export const configManager = new ConfigManager()