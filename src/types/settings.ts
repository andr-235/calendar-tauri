export type Theme = 'auto' | 'light' | 'dark'

export type Language = 'ru' | 'en'

export type FirstDayOfWeek = 0 | 1

export interface AppSettings {
  theme: Theme
  language: Language
  firstDayOfWeek: FirstDayOfWeek
  notifications: boolean
  dbPath: string
}

export interface SelectOption<T = string> {
  value: T
  label: string
}

