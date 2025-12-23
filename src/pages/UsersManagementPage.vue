<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { useUsers } from '../composables/useUsers'
import type { User, UserRole } from '../types/auth'
import '../styles/users-management.scss'

const { users, isLoading, error, getAllUsers, createUser, updateUser, deleteUser, changePassword } = useUsers()

const showCreateDialog = ref(false)
const showEditDialog = ref(false)
const showPasswordDialog = ref(false)
const selectedUser = ref<User | null>(null)
const deleteConfirmId = ref<number | null>(null)

const newUsername = ref('')
const newPassword = ref('')
const newRole = ref<UserRole>('user')

const editUsername = ref('')
const editRole = ref<UserRole>('user')

const passwordUserId = ref<number | null>(null)
const newPasswordValue = ref('')
const confirmPassword = ref('')

const roleLabels: Record<UserRole, string> = {
  admin: 'Администратор',
  user: 'Пользователь',
  controller: 'Контроллер'
}

onMounted(async () => {
  await getAllUsers()
})

const openCreateDialog = () => {
  newUsername.value = ''
  newPassword.value = ''
  newRole.value = 'user'
  showCreateDialog.value = true
}

const openEditDialog = (user: User) => {
  selectedUser.value = user
  editUsername.value = user.username
  editRole.value = user.role
  showEditDialog.value = true
}

const openPasswordDialog = (user: User) => {
  passwordUserId.value = user.id
  newPasswordValue.value = ''
  confirmPassword.value = ''
  showPasswordDialog.value = true
}

const handleCreate = async () => {
  if (!newUsername.value.trim() || !newPassword.value.trim()) {
    return
  }

  if (newPassword.value.length < 6) {
    alert('Пароль должен содержать минимум 6 символов')
    return
  }

  try {
    await createUser(newUsername.value.trim(), newPassword.value, newRole.value)
    showCreateDialog.value = false
    newUsername.value = ''
    newPassword.value = ''
    newRole.value = 'user'
  } catch (err) {
    alert(err instanceof Error ? err.message : 'Ошибка при создании пользователя')
  }
}

const handleUpdate = async () => {
  if (!selectedUser.value || !editUsername.value.trim()) {
    return
  }

  try {
    await updateUser(selectedUser.value.id, editUsername.value.trim(), editRole.value)
    showEditDialog.value = false
    selectedUser.value = null
  } catch (err) {
    alert(err instanceof Error ? err.message : 'Ошибка при обновлении пользователя')
  }
}

const handlePasswordChange = async () => {
  if (!passwordUserId.value || !newPasswordValue.value.trim()) {
    return
  }

  if (newPasswordValue.value.length < 6) {
    alert('Пароль должен содержать минимум 6 символов')
    return
  }

  if (newPasswordValue.value !== confirmPassword.value) {
    alert('Пароли не совпадают')
    return
  }

  try {
    await changePassword(passwordUserId.value, newPasswordValue.value)
    showPasswordDialog.value = false
    passwordUserId.value = null
    newPasswordValue.value = ''
    confirmPassword.value = ''
  } catch (err) {
    alert(err instanceof Error ? err.message : 'Ошибка при смене пароля')
  }
}

const handleDelete = async (id: number) => {
  if (!confirm('Вы уверены, что хотите удалить этого пользователя?')) {
    return
  }

  try {
    await deleteUser(id)
    deleteConfirmId.value = null
  } catch (err) {
    alert(err instanceof Error ? err.message : 'Ошибка при удалении пользователя')
  }
}

const isValidCreate = computed(() => {
  return newUsername.value.trim().length > 0 && newPassword.value.length >= 6
})

const isValidEdit = computed(() => {
  return editUsername.value.trim().length > 0
})

const isValidPassword = computed(() => {
  return newPasswordValue.value.length >= 6 && newPasswordValue.value === confirmPassword.value
})
</script>

<template>
  <div class="users-management-page page-container">
    <div class="page-header">
      <h1>Управление пользователями</h1>
      <button class="button button-primary" @click="openCreateDialog">
        Добавить пользователя
      </button>
    </div>

    <div v-if="error" class="error-message">
      {{ error }}
    </div>

    <div v-if="isLoading" class="loading">
      Загрузка...
    </div>

    <div v-else-if="users.length === 0" class="empty-state">
      <p>Пользователи не найдены</p>
    </div>

    <table v-else class="users-table">
      <thead>
        <tr>
          <th>ID</th>
          <th>Имя пользователя</th>
          <th>Роль</th>
          <th>Дата создания</th>
          <th>Действия</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="user in users" :key="user.id">
          <td>{{ user.id }}</td>
          <td>{{ user.username }}</td>
          <td>{{ roleLabels[user.role] }}</td>
          <td>{{ new Date(user.createdAt).toLocaleDateString('ru-RU') }}</td>
          <td class="actions">
            <button class="button button-secondary" @click="openEditDialog(user)">
              Редактировать
            </button>
            <button class="button button-secondary" @click="openPasswordDialog(user)">
              Сменить пароль
            </button>
            <button class="button button-danger" @click="handleDelete(user.id)">
              Удалить
            </button>
          </td>
        </tr>
      </tbody>
    </table>

    <!-- Create Dialog -->
    <div v-if="showCreateDialog" class="event-dialog-overlay" @click.self="showCreateDialog = false">
      <div class="event-dialog">
        <div class="event-dialog-header">
          <h2>Добавить пользователя</h2>
          <button class="close-button" @click="showCreateDialog = false" type="button">×</button>
        </div>
        
        <div class="event-dialog-body">
          <div class="form-group">
            <label for="newUsername">Имя пользователя *</label>
            <input
              id="newUsername"
              v-model="newUsername"
              type="text"
              placeholder="Введите имя пользователя"
              required
            />
          </div>
          
          <div class="form-group">
            <label for="newPassword">Пароль *</label>
            <input
              id="newPassword"
              v-model="newPassword"
              type="password"
              placeholder="Минимум 6 символов"
              required
            />
          </div>
          
          <div class="form-group">
            <label for="newRole">Роль *</label>
            <select id="newRole" v-model="newRole">
              <option value="admin">Администратор</option>
              <option value="user">Пользователь</option>
              <option value="controller">Контроллер</option>
            </select>
          </div>
        </div>
        
        <div class="event-dialog-footer">
          <button class="button button-secondary" @click="showCreateDialog = false" type="button">
            Отмена
          </button>
          <button
            class="button button-primary"
            @click="handleCreate"
            type="button"
            :disabled="!isValidCreate"
          >
            Создать
          </button>
        </div>
      </div>
    </div>

    <!-- Edit Dialog -->
    <div v-if="showEditDialog" class="event-dialog-overlay" @click.self="showEditDialog = false">
      <div class="event-dialog">
        <div class="event-dialog-header">
          <h2>Редактировать пользователя</h2>
          <button class="close-button" @click="showEditDialog = false" type="button">×</button>
        </div>
        
        <div class="event-dialog-body">
          <div class="form-group">
            <label for="editUsername">Имя пользователя *</label>
            <input
              id="editUsername"
              v-model="editUsername"
              type="text"
              placeholder="Введите имя пользователя"
              required
            />
          </div>
          
          <div class="form-group">
            <label for="editRole">Роль *</label>
            <select id="editRole" v-model="editRole">
              <option value="admin">Администратор</option>
              <option value="user">Пользователь</option>
              <option value="controller">Контроллер</option>
            </select>
          </div>
        </div>
        
        <div class="event-dialog-footer">
          <button class="button button-secondary" @click="showEditDialog = false" type="button">
            Отмена
          </button>
          <button
            class="button button-primary"
            @click="handleUpdate"
            type="button"
            :disabled="!isValidEdit"
          >
            Сохранить
          </button>
        </div>
      </div>
    </div>

    <!-- Password Dialog -->
    <div v-if="showPasswordDialog" class="event-dialog-overlay" @click.self="showPasswordDialog = false">
      <div class="event-dialog">
        <div class="event-dialog-header">
          <h2>Сменить пароль</h2>
          <button class="close-button" @click="showPasswordDialog = false" type="button">×</button>
        </div>
        
        <div class="event-dialog-body">
          <div class="form-group">
            <label for="newPasswordValue">Новый пароль *</label>
            <input
              id="newPasswordValue"
              v-model="newPasswordValue"
              type="password"
              placeholder="Минимум 6 символов"
              required
            />
          </div>
          
          <div class="form-group">
            <label for="confirmPassword">Подтвердите пароль *</label>
            <input
              id="confirmPassword"
              v-model="confirmPassword"
              type="password"
              placeholder="Повторите пароль"
              required
            />
          </div>
        </div>
        
        <div class="event-dialog-footer">
          <button class="button button-secondary" @click="showPasswordDialog = false" type="button">
            Отмена
          </button>
          <button
            class="button button-primary"
            @click="handlePasswordChange"
            type="button"
            :disabled="!isValidPassword"
          >
            Изменить
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

