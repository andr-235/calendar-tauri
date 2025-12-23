import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useAuthStore } from '../stores/auth'
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

export const useUsers = () => {
  const authStore = useAuthStore()
  const users = ref<User[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  const getAllUsers = async () => {
    if (!authStore.token) {
      throw new Error('Not authenticated')
    }

    isLoading.value = true
    error.value = null

    try {
      const usersData = await invoke<UserResponse[]>('get_all_users', {
        token: authStore.token
      })
      users.value = usersData.map(mapUserResponse)
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to fetch users'
      error.value = message
      throw new Error(message)
    } finally {
      isLoading.value = false
    }
  }

  const createUser = async (username: string, password: string, role: UserRole) => {
    if (!authStore.token) {
      throw new Error('Not authenticated')
    }

    error.value = null

    try {
      await invoke<number>('register_user', {
        username,
        password,
        role,
        token: authStore.token
      })
      await getAllUsers()
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to create user'
      error.value = message
      throw new Error(message)
    }
  }

  const updateUser = async (id: number, username: string, role: UserRole) => {
    if (!authStore.token) {
      throw new Error('Not authenticated')
    }

    error.value = null

    try {
      await invoke<number>('update_user', {
        id,
        username,
        role,
        token: authStore.token
      })
      await getAllUsers()
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to update user'
      error.value = message
      throw new Error(message)
    }
  }

  const deleteUser = async (id: number) => {
    if (!authStore.token) {
      throw new Error('Not authenticated')
    }

    error.value = null

    try {
      await invoke<number>('delete_user', {
        id,
        token: authStore.token
      })
      await getAllUsers()
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to delete user'
      error.value = message
      throw new Error(message)
    }
  }

  const changePassword = async (id: number, newPassword: string) => {
    if (!authStore.token) {
      throw new Error('Not authenticated')
    }

    error.value = null

    try {
      await invoke<number>('change_user_password', {
        id,
        newPassword,
        token: authStore.token
      })
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to change password'
      error.value = message
      throw new Error(message)
    }
  }

  return {
    users,
    isLoading,
    error,
    getAllUsers,
    createUser,
    updateUser,
    deleteUser,
    changePassword
  }
}

