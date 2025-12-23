<script setup lang="ts">
import { ref } from 'vue'
import { isTauri } from '../utils/tauri'

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
      } catch (connectErr) {
        throw new Error(`Не удалось подключиться к базе данных: ${connectErr instanceof Error ? connectErr.message : String(connectErr)}`)
      }
      
      emit('path-selected', selected)
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

<style lang="scss" scoped>
.db-selector-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.db-selector-modal {
  background: var(--bg-secondary, #fff);
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
  max-width: 500px;
  width: 90%;

  h2 {
    margin-top: 0;
    margin-bottom: 1rem;
    text-align: center;
  }

  .info-text {
    margin-bottom: 1.5rem;
    color: var(--text-secondary, #666);
    line-height: 1.5;
  }

  .error-message {
    color: #dc3545;
    margin-bottom: 1rem;
    padding: 0.5rem;
    background: #f8d7da;
    border-radius: 4px;
    font-size: 0.9rem;
  }

  .button-group {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;

    .button {
      padding: 0.75rem 1.5rem;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      font-size: 1rem;
      transition: opacity 0.2s;

      &:disabled {
        opacity: 0.6;
        cursor: not-allowed;
      }

      &.button-primary {
        background: #007bff;
        color: white;

        &:hover:not(:disabled) {
          background: #0056b3;
        }
      }

      &.button-secondary {
        background: #6c757d;
        color: white;

        &:hover:not(:disabled) {
          background: #545b62;
        }
      }
    }
  }
}
</style>

