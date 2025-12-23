import type { SelectOption, Theme, Language, FirstDayOfWeek } from '../types/settings'

export const THEMES: SelectOption<Theme>[] = [
  { value: 'auto', label: 'Автоматически' },
  { value: 'light', label: 'Светлая' },
  { value: 'dark', label: 'Тёмная' }
]

export const LANGUAGES: SelectOption<Language>[] = [
  { value: 'ru', label: 'Русский' },
  { value: 'en', label: 'English' }
]

export const WEEK_DAYS: SelectOption<FirstDayOfWeek>[] = [
  { value: 0, label: 'Воскресенье' },
  { value: 1, label: 'Понедельник' }
]

export const DEFAULT_SETTINGS = {
  theme: 'auto' as const,
  language: 'ru' as const,
  firstDayOfWeek: 1 as const,
  notifications: true,
  dbPath: ''
}

