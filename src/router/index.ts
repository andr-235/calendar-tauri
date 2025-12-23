import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import CalendarPage from '../pages/CalendarPage.vue'
import SettingsPage from '../pages/SettingsPage.vue'
import LoginView from '../pages/LoginView.vue'
import UsersManagementPage from '../pages/UsersManagementPage.vue'

const routes: RouteRecordRaw[] = [
  {
    path: '/login',
    name: 'login',
    component: LoginView,
    meta: { requiresAuth: false }
  },
  {
    path: '/',
    name: 'calendar',
    component: CalendarPage,
    meta: { requiresAuth: true }
  },
  {
    path: '/settings',
    name: 'settings',
    component: SettingsPage,
    meta: { requiresAuth: true, role: ['admin'] }
  },
  {
    path: '/users',
    name: 'users',
    component: UsersManagementPage,
    meta: { requiresAuth: true, role: ['admin'] }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

router.beforeEach(async (to, _from, next) => {
  const authStore = useAuthStore()
  
  if (to.meta.requiresAuth === false) {
    if (authStore.isAuthenticated) {
      next({ path: '/' })
    } else {
      next()
    }
    return
  }

  if (to.meta.requiresAuth) {
    if (!authStore.token) {
      next({ path: '/login', query: { redirect: to.fullPath } })
      return
    }

    if (!authStore.user) {
      try {
        await authStore.fetchCurrentUser()
      } catch {
        next({ path: '/login', query: { redirect: to.fullPath } })
        return
      }
    }

    if (to.meta.role && Array.isArray(to.meta.role)) {
      const userRole = authStore.user?.role
      if (!userRole || !to.meta.role.includes(userRole)) {
        next({ path: '/' })
        return
      }
    }
  }

  next()
})

export default router

