<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useNavigation } from '../composables/useNavigation'
import { useAuthStore } from '../stores/auth'
import '../styles/navbar.scss'
import '../styles/navbar-component.scss'

const router = useRouter()
const { navigateTo, isActiveRoute } = useNavigation()
const authStore = useAuthStore()

const routes = computed(() => {
  const allRoutes = [
    { name: 'calendar', path: '/', label: 'Создание контрольной карточки', roles: ['admin', 'user', 'controller'] as const },
    { name: 'users', path: '/users', label: 'Управление пользователями', roles: ['admin'] as const },
    { name: 'settings', path: '/settings', label: 'Настройки', roles: ['admin'] as const }
  ]
  
  if (!authStore.user) return []
  
  return allRoutes.filter(route => route.roles.includes(authStore.user!.role as any))
})

const handleLogout = () => {
  authStore.logout()
  router.push('/login')
}
</script>

<template>
  <nav class="navbar">
    <div class="navbar-routes">
      <button
        v-for="route in routes"
        :key="route.name"
        class="navbar-button"
        :class="{ active: isActiveRoute(route.name) }"
        @click="navigateTo(route.path)"
      >
        {{ route.label }}
      </button>
    </div>
    <div v-if="authStore.user" class="navbar-user">
      <span class="navbar-username">{{ authStore.user.username }}</span>
      <button class="navbar-button navbar-button-logout" @click="handleLogout">
        Выход
      </button>
    </div>
  </nav>
</template>


