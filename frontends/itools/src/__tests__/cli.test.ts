import { describe, it, expect, vi } from 'vitest'
import { Cli, runCli } from '../cli'

describe('Cli', () => {
  it('should parse init command with name', () => {
    const args = ['init', 'test-project']
    const cli = Cli.parse(args)
    expect(cli.getCommand()).toBe('init')
    expect(cli.getOptions()).toEqual({ name: 'test-project' })
  })

  it('should parse build command', () => {
    const args = ['build']
    const cli = Cli.parse(args)
    expect(cli.getCommand()).toBe('build')
    expect(cli.getOptions()).toEqual({})
  })

  it('should parse dev command', () => {
    const args = ['dev']
    const cli = Cli.parse(args)
    expect(cli.getCommand()).toBe('dev')
    expect(cli.getOptions()).toEqual({})
  })

  it('should parse check command', () => {
    const args = ['check']
    const cli = Cli.parse(args)
    expect(cli.getCommand()).toBe('check')
    expect(cli.getOptions()).toEqual({})
  })

  it('should throw error for missing command', () => {
    const args: string[] = []
    expect(() => Cli.parse(args)).toThrow('Expected command')
  })

  it('should throw error for unknown command', () => {
    const args = ['unknown']
    expect(() => Cli.parse(args)).toThrow('Unknown command: unknown')
  })

  it('should throw error for init command without name', () => {
    const args = ['init']
    expect(() => Cli.parse(args)).toThrow('Expected string argument')
  })

  it('should execute init command', async () => {
    const consoleLogSpy = vi.spyOn(console, 'log').mockImplementation()
    const cli = new Cli('init', { name: 'test-project' })
    await cli.execute()
    expect(consoleLogSpy).toHaveBeenCalledWith('Initializing project: test-project')
    consoleLogSpy.mockRestore()
  })

  it('should execute build command', async () => {
    const consoleLogSpy = vi.spyOn(console, 'log').mockImplementation()
    const cli = new Cli('build')
    await cli.execute()
    expect(consoleLogSpy).toHaveBeenCalledWith('Building project')
    consoleLogSpy.mockRestore()
  })

  it('should execute dev command', async () => {
    const consoleLogSpy = vi.spyOn(console, 'log').mockImplementation()
    const cli = new Cli('dev')
    await cli.execute()
    expect(consoleLogSpy).toHaveBeenCalledWith('Starting development server')
    consoleLogSpy.mockRestore()
  })

  it('should execute check command', async () => {
    const consoleLogSpy = vi.spyOn(console, 'log').mockImplementation()
    const cli = new Cli('check')
    await cli.execute()
    expect(consoleLogSpy).toHaveBeenCalledWith('Checking project')
    consoleLogSpy.mockRestore()
  })

  it('should run cli with args', async () => {
    const consoleLogSpy = vi.spyOn(console, 'log').mockImplementation()
    await runCli(['init', 'test-project'])
    expect(consoleLogSpy).toHaveBeenCalledWith('Initializing project: test-project')
    consoleLogSpy.mockRestore()
  })

  it('should handle errors', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation()
    await runCli([])
    expect(consoleErrorSpy).toHaveBeenCalled()
    consoleErrorSpy.mockRestore()
  })
})