/**
 * Проверяет, доступны ли Tauri API
 */
export function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
}

/**
 * Безопасный вызов Tauri команды с обработкой ошибок
 */
export async function safeInvoke<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T | null> {
  if (!isTauri()) {
    console.warn(`Tauri API not available. Cannot invoke command: ${command}`)
    return null
  }

  try {
    const { invoke } = await import('@tauri-apps/api/core')
    return await invoke<T>(command, args)
  } catch (error) {
    console.error(`Failed to invoke ${command}:`, error)
    return null
  }
}

