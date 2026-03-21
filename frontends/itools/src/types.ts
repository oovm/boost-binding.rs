export interface Command {
  init: (name: string) => Promise<void>
  build: () => Promise<void>
  dev: () => Promise<void>
  check: () => Promise<void>
}

export interface Config {
  title: string
  description: string
  lang: string
  themeConfig?: {
    nav?: Array<{
      text: string
      link: string
    }>
  }
}

export interface I18nResources {
  [locale: string]: {
    [key: string]: string
  }
}

export interface I18n {
  t: (key: string) => string
  tWithArgs: (key: string, args: Record<string, string>) => string
  currentLocale: string
}