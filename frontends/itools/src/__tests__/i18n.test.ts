import { describe, it, expect } from 'vitest'
import { I18nImpl, i18n, t, tWithArgs } from '../i18n'

describe('I18n', () => {
  it('should create i18n instance with default resources', () => {
    const i18nInstance = new I18nImpl()
    expect(i18nInstance.currentLocale).toBe('en')
  })

  it('should create i18n instance with custom resources and locale', () => {
    const resources = {
      en: {
        'test.key': 'Test value'
      }
    }
    const i18nInstance = new I18nImpl(resources, 'en')
    expect(i18nInstance.currentLocale).toBe('en')
    expect(i18nInstance.t('test.key')).toBe('Test value')
  })

  it('should get translation for existing key', () => {
    const i18nInstance = new I18nImpl({
      en: {
        'test.key': 'Test value'
      }
    })
    expect(i18nInstance.t('test.key')).toBe('Test value')
  })

  it('should fallback to English for non-existing key in current locale', () => {
    const i18nInstance = new I18nImpl({
      en: {
        'test.key': 'Test value'
      },
      'zh-CN': {}
    }, 'zh-CN')
    expect(i18nInstance.t('test.key')).toBe('Test value')
  })

  it('should return key itself for non-existing key', () => {
    const i18nInstance = new I18nImpl()
    expect(i18nInstance.t('non.existing.key')).toBe('non.existing.key')
  })

  it('should get translation with args', () => {
    const i18nInstance = new I18nImpl({
      en: {
        'test.key': 'Hello {name}'
      }
    })
    expect(i18nInstance.tWithArgs('test.key', { name: 'World' })).toBe('Hello World')
  })

  it('should change current locale', () => {
    const i18nInstance = new I18nImpl({
      en: {
        'test.key': 'Test value'
      },
      'zh-CN': {
        'test.key': '测试值'
      }
    })
    expect(i18nInstance.t('test.key')).toBe('Test value')
    i18nInstance.currentLocale = 'zh-CN'
    expect(i18nInstance.t('test.key')).toBe('测试值')
  })

  it('should use global i18n instance', () => {
    expect(t('messages.cli_initialized')).toBe('iTools CLI initialized')
    expect(tWithArgs('messages.initializing_project', { name: 'test' })).toBe('Initializing project: test')
  })
})
