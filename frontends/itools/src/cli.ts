import { t, tWithArgs } from './i18n'

// Command types
export type CommandType = 'init' | 'build' | 'dev' | 'check'

export interface CommandOptions {
  name?: string
}

// CLI class
export class Cli {
  private command: CommandType
  private options: CommandOptions

  constructor(command: CommandType, options: CommandOptions = {}) {
    this.command = command
    this.options = options
  }

  getCommand(): CommandType {
    return this.command
  }

  getOptions(): CommandOptions {
    return this.options
  }

  // Parse CLI arguments
  static parse(args: string[]): Cli {
    if (args.length === 0) {
      throw new Error(t('errors.expected_command'))
    }

    const command = args[0] as CommandType
    const options: CommandOptions = {}

    switch (command) {
      case 'init':
        if (args.length < 2) {
          throw new Error(t('errors.expected_string_argument'))
        }
        options.name = args[1]
        break
      case 'build':
      case 'dev':
      case 'check':
        break
      default:
        throw new Error(tWithArgs('errors.unknown_command', { command }))
    }

    return new Cli(command, options)
  }

  // Execute command
  async execute(): Promise<void> {
    switch (this.command) {
      case 'init':
        await this.init()
        break
      case 'build':
        await this.build()
        break
      case 'dev':
        await this.dev()
        break
      case 'check':
        await this.check()
        break
    }
  }

  private async init(): Promise<void> {
    const name = this.options.name
    if (!name) {
      throw new Error(t('errors.expected_string_argument'))
    }
    console.log(tWithArgs('messages.initializing_project', { name }))
    // Implementation here
  }

  private async build(): Promise<void> {
    console.log(t('messages.building_project'))
    // Implementation here
  }

  private async dev(): Promise<void> {
    console.log(t('messages.starting_dev_server'))
    // Implementation here
  }

  private async check(): Promise<void> {
    console.log(t('messages.checking_project'))
    // Implementation here
  }
}

// Run CLI
export async function runCli(args: string[]): Promise<void> {
  try {
    const cli = Cli.parse(args)
    await cli.execute()
  } catch (error) {
    console.error(error)
  }
}

// If running as main module
if (import.meta.url === `file://${process.argv[1]}`) {
  runCli(process.argv.slice(2))
}