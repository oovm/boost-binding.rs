import { I18n, I18nResources } from './types'

export class I18nImpl implements I18n {
  private resources: I18nResources
  private _currentLocale: string

  constructor(resources: I18nResources = {}, currentLocale: string = 'en') {
    this.resources = resources
    this._currentLocale = currentLocale
  }

  get currentLocale(): string {
    return this._currentLocale
  }

  set currentLocale(locale: string) {
    this._currentLocale = locale
  }

  t(key: string): string {
    const locale = this.currentLocale
    if (this.resources[locale] && this.resources[locale][key]) {
      return this.resources[locale][key]
    }

    // Fallback to English
    if (this.resources['en'] && this.resources['en'][key]) {
      return this.resources['en'][key]
    }

    return key
  }

  tWithArgs(key: string, args: Record<string, string>): string {
    let result = this.t(key)
    for (const [name, value] of Object.entries(args)) {
      result = result.replace(`{${name}}`, value)
    }
    return result
  }
}

// Default resources
export const defaultResources: I18nResources = {
  en: {
    'errors.expected_string_argument': 'Expected string argument',
    'errors.expected_integer_argument': 'Expected integer argument',
    'errors.invalid_integer': 'Invalid integer',
    'errors.expected_unsigned_integer_argument': 'Expected unsigned integer argument',
    'errors.invalid_unsigned_integer': 'Invalid unsigned integer',
    'errors.expected_boolean_argument': 'Expected boolean argument',
    'errors.invalid_boolean': 'Invalid boolean',
    'errors.expected_command': 'Expected command',
    'errors.unknown_command': 'Unknown command: {command}',
    'messages.cli_initialized': 'iTools CLI initialized',
    'messages.initializing_project': 'Initializing project: {name}',
    'messages.building_project': 'Building project',
    'messages.starting_dev_server': 'Starting development server',
    'messages.checking_project': 'Checking project'
  },
  'zh-CN': {
    'errors.expected_string_argument': '期望字符串参数',
    'errors.expected_integer_argument': '期望整数参数',
    'errors.invalid_integer': '无效的整数',
    'errors.expected_unsigned_integer_argument': '期望无符号整数参数',
    'errors.invalid_unsigned_integer': '无效的无符号整数',
    'errors.expected_boolean_argument': '期望布尔参数',
    'errors.invalid_boolean': '无效的布尔值',
    'errors.expected_command': '期望命令',
    'errors.unknown_command': '未知命令: {command}',
    'messages.cli_initialized': 'iTools CLI 已初始化',
    'messages.initializing_project': '正在初始化项目: {name}',
    'messages.building_project': '正在构建项目',
    'messages.starting_dev_server': '正在启动开发服务器',
    'messages.checking_project': '正在检查项目'
  }
}

// Global i18n instance
export const i18n = new I18nImpl(defaultResources, process.env.LANG || 'en')

export const t = (key: string): string => i18n.t(key)
export const tWithArgs = (key: string, args: Record<string, string>): string => i18n.tWithArgs(key, args)