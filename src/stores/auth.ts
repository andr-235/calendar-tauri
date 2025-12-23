import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { User, UserRole } from '../types/auth'

interface UserResponse {
  id: number
  username: string
  role: string
  created_at: string
}

function mapUserResponse(user: UserResponse): User {
  return {
    id: user.id,
    username: user.username,
    role: user.role as UserRole,
    createdAt: user.created_at
  }
}

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(localStorage.getItem('auth_token'))
  const user = ref<User | null>(null)

  const isAuthenticated = computed(() => !!token.value && !!user.value)
  const isAdmin = computed(() => user.value?.role === 'admin')
  const isUser = computed(() => user.value?.role === 'user')
  const isController = computed(() => user.value?.role === 'controller')

  async function login(username: string, password: string): Promise<void> {
    try {
      const jwt = await invoke<string>('login', { username, password })
      token.value = jwt
      localStorage.setItem('auth_token', jwt)
      await fetchCurrentUser()
    } catch (error) {
      throw new Error(error instanceof Error ? error.message : 'Login failed')
    }
  }

  function logout(): void {
    token.value = null
    user.value = null
    localStorage.removeItem('auth_token')
  }

  async function fetchCurrentUser(): Promise<void> {
    if (!token.value) {
      user.value = null
      return
    }

    try {
      const userData = await invoke<UserResponse>('get_current_user', {
        token: token.value
      })
      user.value = mapUserResponse(userData)
    } catch (error) {
      logout()
      throw error
    }
  }

  return {
    token,
    user,
    isAuthenticated,
    isAdmin,
    isUser,
    isController,
    login,
    logout,
    fetchCurrentUser
  }
})

