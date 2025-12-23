import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { AppSettings } from '../types/settings'
import { DEFAULT_SETTINGS } from '../constants/settings'

const settings = ref<AppSettings>({ ...DEFAULT_SETTINGS })

const safeInvoke = async <T>(
  command: string,
  args?: Record<string, unknown>,
  errorMessage?: string
): Promise<T | null> => {
  try {
    return await invoke<T>(command, args)
  } catch (error) {
    const message = errorMessage || `Failed to execute ${command}`
    console.error(message, error)
    return null
  }
}

export const useSettings = () => {
  const loadSettings = async () => {
    const stored = localStorage.getItem('app-settings')
    if (stored) {
      try {
        settings.value = { ...DEFAULT_SETTINGS, ...JSON.parse(stored) }
      } catch {
        settings.value = { ...DEFAULT_SETTINGS }
      }
    }

    // Проверяем, подключена ли уже БД, и к какому пути
    const currentDbPath = await safeInvoke<string | null>('get_database_path')
    
    // Если БД уже подключена, не переподключаемся
    if (currentDbPath) {
      // Обновляем путь в настройках, если он отличается (это синхронизирует localStorage с реальным путем БД)
      if (settings.value.dbPath !== currentDbPath) {
        settings.value.dbPath = currentDbPath
      }
      return
    }

    // Подключаемся только если БД не подключена и есть путь в настройках
    if (settings.value.dbPath) {
      await safeInvoke('connect_database', { dbPath: settings.value.dbPath }, 'Failed to connect to database')
    }
  }

  const saveSettings = () => {
    localStorage.setItem('app-settings', JSON.stringify(settings.value))
  }

  const updateDbPath = async (dbPath: string) => {
    const currentPath = await safeInvoke<string | null>('get_database_path')
    if (currentPath) {
      await safeInvoke('disconnect_database', undefined, 'Failed to disconnect database')
    }
    
    if (dbPath) {
      const connected = await safeInvoke('connect_database', { dbPath }, 'Failed to connect to database')
      if (!connected) {
        throw new Error('Failed to connect to database')
      }
    }
    
    settings.value.dbPath = dbPath
    saveSettings()
  }

  watch(
    settings,
    () => {
      saveSettings()
    },
    { deep: true }
  )

  return {
    settings,
    loadSettings,
    saveSettings,
    updateDbPath
  }
}

