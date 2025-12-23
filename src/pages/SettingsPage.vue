<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { save } from '@tauri-apps/plugin-dialog'
import { useSettings } from '../composables/useSettings'
import { THEMES, LANGUAGES, WEEK_DAYS } from '../constants/settings'
import '../styles/settings-page.scss'

const { settings, loadSettings, updateDbPath } = useSettings()
const isConnecting = ref(false)

onMounted(() => {
  loadSettings()
})

const selectDbPath = async () => {
  try {
    const selected = await save({
      defaultPath: settings.value.dbPath || 'calendar.db',
      filters: [{ name: 'Database', extensions: ['db', 'sqlite', 'sqlite3'] }]
    })
    
    if (selected) {
      isConnecting.value = true
      await updateDbPath(selected)
      isConnecting.value = false
    }
  } catch (error) {
    console.error('Failed to select database path:', error)
    isConnecting.value = false
  }
}
</script>

<template>
  <div class="settings-page page-container-narrow">
    <h1>Настройки</h1>
    
    <div class="settings-section section">
      <h2 class="section-title">Внешний вид</h2>
      
      <div class="setting-item">
        <label for="theme">Тема:</label>
        <select id="theme" v-model="settings.theme">
          <option v-for="theme in THEMES" :key="theme.value" :value="theme.value">
            {{ theme.label }}
          </option>
        </select>
      </div>
    </div>
    
    <div class="settings-section section">
      <h2 class="section-title">Язык и регион</h2>
      
      <div class="setting-item">
        <label for="language">Язык:</label>
        <select id="language" v-model="settings.language">
          <option v-for="lang in LANGUAGES" :key="lang.value" :value="lang.value">
            {{ lang.label }}
          </option>
        </select>
      </div>
      
      <div class="setting-item">
        <label for="firstDayOfWeek">Первый день недели:</label>
        <select id="firstDayOfWeek" v-model.number="settings.firstDayOfWeek">
          <option v-for="day in WEEK_DAYS" :key="day.value" :value="day.value">
            {{ day.label }}
          </option>
        </select>
      </div>
    </div>
    
    <div class="settings-section section">
      <h2 class="section-title">Уведомления</h2>
      
      <div class="setting-item">
        <label>
          <input type="checkbox" v-model="settings.notifications" />
          Включить уведомления
        </label>
      </div>
    </div>
    
    <div class="settings-section section">
      <h2 class="section-title">База данных</h2>
      
      <div class="setting-item">
        <label for="dbPath">Путь к файлу базы данных:</label>
        <div class="db-path-input">
          <input 
            id="dbPath" 
            type="text" 
            v-model="settings.dbPath" 
            placeholder="Выберите путь к файлу БД"
            readonly
          />
          <button 
            type="button" 
            @click="selectDbPath" 
            :disabled="isConnecting"
            class="btn-select-path"
          >
            {{ isConnecting ? 'Подключение...' : 'Выбрать' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

