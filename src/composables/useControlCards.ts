import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useAuthStore } from '../stores/auth'
import type { ControlCard } from '../types/calendar'
import type { User } from '../types/auth'

interface ControlCardResponse {
  id: number
  card_number: number
  year: number
  executor: string
  reporter: string
  summary: string
  document_reference: string
  executor_user_id?: number | null
  created_at: string
}

function mapControlCardResponse(card: ControlCardResponse): ControlCard {
  return {
    id: String(card.id),
    cardNumber: card.card_number,
    year: card.year,
    executor: card.executor,
    reporter: card.reporter,
    summary: card.summary,
    documentReference: card.document_reference,
    executorUserId: card.executor_user_id ?? undefined,
    createdAt: card.created_at,
    startDate: card.created_at,
    endDate: card.created_at
  }
}

export function useControlCards() {
  const authStore = useAuthStore()
  const cards = ref<ControlCard[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const getToken = (): string => {
    if (!authStore.token) {
      throw new Error('Not authenticated')
    }
    return authStore.token
  }

  const withLoading = async <T>(
    operation: () => Promise<T>,
    errorMessage: string
  ): Promise<T | null> => {
    try {
      loading.value = true
      error.value = null
      return await operation()
    } catch (err) {
      error.value = err instanceof Error ? err.message : errorMessage
      console.error(errorMessage, err)
      return null
    } finally {
      loading.value = false
    }
  }

  const loadCards = async () => {
    const token = getToken()
    const response = await withLoading(
      async () => invoke<ControlCardResponse[]>('get_all_control_cards', { token }),
      'Ошибка загрузки контрольных карточек'
    )
    if (response) {
      cards.value = response.map(mapControlCardResponse)
    }
  }

  const getNextCardNumber = async (year: number): Promise<number | null> => {
    return await withLoading(
      async () => invoke<number>('get_next_card_number', { year }),
      'Ошибка получения следующего номера карточки'
    )
  }

  const createCard = async (
    cardNumber: number,
    year: number,
    executorUserId: number,
    reporter: string,
    summary: string,
    documentReference: string
  ): Promise<ControlCard | null> => {
    const token = getToken()
    const id = await withLoading(
      async () => invoke<number>('create_control_card', {
        cardNumber,
        year,
        executorUserId,
        reporter,
        summary,
        documentReference,
        token
      }),
      'Ошибка создания контрольной карточки'
    )
    if (!id) return null

    // Перезагружаем карточки чтобы получить полную информацию включая executor
    await loadCards()
    const card = cards.value.find(c => c.id === String(id))
    return card || null
  }

  const updateCard = async (
    id: string,
    cardNumber: number,
    year: number,
    executorUserId: number,
    reporter: string,
    summary: string,
    documentReference: string
  ): Promise<boolean> => {
    const token = getToken()
    const result = await withLoading(
      async () => invoke('update_control_card', {
        id: Number(id),
        cardNumber,
        year,
        executorUserId,
        reporter,
        summary,
        documentReference,
        token
      }),
      'Ошибка обновления контрольной карточки'
    )
    if (!result) return false

    // Перезагружаем карточки чтобы получить обновленную информацию
    await loadCards()
    return true
  }

  const deleteCard = async (id: string): Promise<boolean> => {
    const token = getToken()
    const result = await withLoading(
      async () => invoke('delete_control_card', { id: Number(id), token }),
      'Ошибка удаления контрольной карточки'
    )
    if (!result) return false

    cards.value = cards.value.filter(c => c.id !== id)
    return true
  }

  const getUsersForExecutorSelection = async (): Promise<User[]> => {
    const token = getToken()
    const users = await withLoading(
      async () => invoke<User[]>('get_users_for_executor_selection', { token }),
      'Ошибка загрузки списка пользователей'
    )
    return users || []
  }

  return {
    cards,
    loading,
    error,
    loadCards,
    getNextCardNumber,
    createCard,
    updateCard,
    deleteCard,
    getUsersForExecutorSelection
  }
}

