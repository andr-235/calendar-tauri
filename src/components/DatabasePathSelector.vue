<script setup lang="ts">
import { ref } from 'vue'
import { isTauri } from '../utils/tauri'
import '../styles/database-selector.scss'

const props = defineProps<{
  isFirstRun: boolean
}>()

const emit = defineEmits<{
  (e: 'path-selected', path: string): void
  (e: 'cancel'): void
}>()

const loading = ref(false)
const error = ref<string | null>(null)

const selectDatabasePath = async () => {
  error.value = null
  loading.value = true

  if (!isTauri()) {
    error.value = 'Диалоги выбора файлов доступны только в нативном приложении'
    loading.value = false
    return
  }

  try {
    let selected: string | null = null

    if (props.isFirstRun) {
      // При первом запуске - создаем новый файл БД
      const { save } = await import('@tauri-apps/plugin-dialog')
      selected = await save({
        defaultPath: 'calendar.db',
        filters: [{ name: 'Database', extensions: ['db', 'sqlite', 'sqlite3'] }],
        title: 'Выберите место для создания базы данных'
      })
    } else {
      // При последующих запусках - открываем существующий файл БД
      const { open } = await import('@tauri-apps/plugin-dialog')
      selected = await open({
        filters: [{ name: 'Database', extensions: ['db', 'sqlite', 'sqlite3'] }],
        title: 'Выберите файл базы данных',
        multiple: false
      }) as string | null
    }

    if (selected) {
      // Подключаемся к БД
      try {
        const { invoke } = await import('@tauri-apps/api/core')
        await invoke('connect_database', { dbPath: selected })
        emit('path-selected', selected)
      } catch (connectErr) {
        error.value = `Не удалось подключиться к базе данных: ${connectErr instanceof Error ? connectErr.message : String(connectErr)}`
        loading.value = false
        return
      }
    } else {
      // Пользователь отменил выбор
      loading.value = false
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Ошибка выбора пути к базе данных'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="db-selector-overlay">
    <div class="db-selector-modal">
      <h2 v-if="isFirstRun">Создание базы данных</h2>
      <h2 v-else>Выбор базы данных</h2>
      
      <p v-if="isFirstRun" class="info-text">
        Выберите место, где будет создан файл базы данных. После создания базы данных вы сможете создать первого администратора.
      </p>
      <p v-else class="info-text">
        Выберите файл базы данных для подключения. После подключения вы сможете войти в систему.
      </p>
      
      <div v-if="error" class="error-message">{{ error }}</div>
      
      <div class="button-group">
        <button 
          type="button" 
          @click="selectDatabasePath" 
          :disabled="loading"
          class="button button-primary"
        >
          {{ loading ? 'Подключение...' : (isFirstRun ? 'Создать базу данных' : 'Выбрать базу данных') }}
        </button>
        <button 
          type="button" 
          @click="$emit('cancel')" 
          :disabled="loading"
          class="button button-secondary"
        >
          Отмена
        </button>
      </div>
    </div>
  </div>
</template>


