<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import DatabasePathSelector from '../components/DatabasePathSelector.vue'
import { isTauri, safeInvoke } from '../utils/tauri'
import '../styles/forms.scss'
import '../styles/login.scss'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()

const username = ref('')
const password = ref('')
const error = ref<string | null>(null)
const loading = ref(false)
const needsInit = ref(false)
const showDbSelector = ref(true)
const isFirstRun = ref(false)

onMounted(async () => {
  // Проверяем, подключена ли уже БД
  try {
    if (!isTauri()) {
      // В веб-версии Tauri API недоступны, показываем сообщение
      error.value = 'Это приложение работает только в нативной версии Tauri'
      showDbSelector.value = false
      return
    }

    const isConnected = await safeInvoke<boolean>('is_database_connected')
    
    if (isConnected === true) {
      // БД уже подключена, проверяем наличие пользователей
      await checkUsers()
      showDbSelector.value = false
    } else if (isConnected === false) {
      // БД не подключена, показываем селектор пути
      // Сначала проверяем, есть ли сохраненный путь в localStorage
      const stored = localStorage.getItem('app-settings')
      if (stored) {
        try {
          const settings = JSON.parse(stored)
          if (settings.dbPath) {
            // Пытаемся подключиться к сохраненному пути
            const connectResult = await safeInvoke('connect_database', { dbPath: settings.dbPath })
            if (connectResult !== null) {
              await checkUsers()
              showDbSelector.value = false
            } else {
              // Не удалось подключиться, показываем селектор (не первый запуск)
              isFirstRun.value = false
              showDbSelector.value = true
            }
          } else {
            // Нет сохраненного пути - первый запуск
            isFirstRun.value = true
            showDbSelector.value = true
          }
        } catch {
          // Ошибка парсинга - первый запуск
          isFirstRun.value = true
          showDbSelector.value = true
        }
      } else {
        // Нет сохраненных настроек - первый запуск
        isFirstRun.value = true
        showDbSelector.value = true
      }
    }
  } catch (err) {
    // При ошибке считаем, что это первый запуск
    isFirstRun.value = true
    showDbSelector.value = true
  }
})

const checkUsers = async () => {
  try {
    const hasUsers = await safeInvoke<boolean>('has_any_users')
    needsInit.value = !hasUsers
    isFirstRun.value = !hasUsers
    
    if (needsInit.value) {
      const windowsUsername = await safeInvoke<string>('get_windows_username')
      if (windowsUsername) {
        username.value = windowsUsername
      }
    }
  } catch (err) {
    needsInit.value = true
    isFirstRun.value = true
    const windowsUsername = await safeInvoke<string>('get_windows_username')
    if (windowsUsername) {
      username.value = windowsUsername
    }
  }
}

const handleDbPathSelected = async (dbPath: string) => {
  // Сохраняем путь в localStorage
  const stored = localStorage.getItem('app-settings')
  let settings = stored ? JSON.parse(stored) : {}
  settings.dbPath = dbPath
  localStorage.setItem('app-settings', JSON.stringify(settings))
  
  showDbSelector.value = false
  await checkUsers()
}

const handleInitAdmin = async () => {
  if (!username.value || !password.value) {
    error.value = 'Заполните все поля'
    return
  }

  if (password.value.length < 6) {
    error.value = 'Пароль должен быть не менее 6 символов'
    return
  }

  error.value = null
  loading.value = true

  try {
    const userId = await safeInvoke<number>('init_admin', {
      username: username.value,
      password: password.value
    })
    
    if (userId === null) {
      throw new Error('Не удалось создать администратора')
    }
    
    await authStore.login(username.value, password.value)
    router.push('/')
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Ошибка создания администратора'
  } finally {
    loading.value = false
  }
}

const handleLogin = async () => {
  if (!username.value || !password.value) {
    error.value = 'Заполните все поля'
    return
  }

  error.value = null
  loading.value = true

  try {
    await authStore.login(username.value, password.value)
    const redirect = route.query.redirect as string
    router.push(redirect || '/')
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Ошибка входа'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="login-container">
    <DatabasePathSelector
      v-if="showDbSelector"
      :is-first-run="isFirstRun"
      @path-selected="handleDbPathSelected"
      @cancel="showDbSelector = false"
    />
    
    <div v-else class="login-form">
      <h1 v-if="needsInit">Создание первого администратора</h1>
      <h1 v-else>Вход в систему</h1>
      
      <form @submit.prevent="needsInit ? handleInitAdmin() : handleLogin()">
        <div class="form-group">
          <label for="username">Имя пользователя</label>
          <input
            id="username"
            v-model="username"
            type="text"
            required
            :disabled="loading"
            autocomplete="username"
          />
        </div>
        <div class="form-group">
          <label for="password">Пароль</label>
          <input
            id="password"
            v-model="password"
            type="password"
            required
            :disabled="loading"
            :autocomplete="needsInit ? 'new-password' : 'current-password'"
          />
        </div>
        <div v-if="needsInit" class="info-message">
          Это первый запуск системы. Создайте учетную запись администратора.
        </div>
        <div v-if="error" class="error-message">{{ error }}</div>
        <button type="submit" class="button button-primary" :disabled="loading">
          <span v-if="needsInit">{{ loading ? 'Создание...' : 'Создать администратора' }}</span>
          <span v-else>{{ loading ? 'Вход...' : 'Войти' }}</span>
        </button>
      </form>
    </div>
  </div>
</template>


