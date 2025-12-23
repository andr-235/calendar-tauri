<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { ControlCard } from '../types/calendar'
import type { User } from '../types/auth'
import { useControlCards } from '../composables/useControlCards'

interface Props {
  modelValue: boolean
  card?: ControlCard | null
}

const props = withDefaults(defineProps<Props>(), {
  card: null
})

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  'save': [card: Omit<ControlCard, 'id' | 'createdAt'> & { executorUserId: number }]
}>()

const { getNextCardNumber, getUsersForExecutorSelection } = useControlCards()

const cardNumber = ref<number>(1)
const year = ref<number>(new Date().getFullYear())
const executorUserId = ref<number | null>(null)
const users = ref<User[]>([])
const reporter = ref('')
const summary = ref('')
const documentReference = ref('')

const isEditMode = computed(() => !!props.card)

const loadNextCardNumber = async () => {
  const nextNumber = await getNextCardNumber(year.value)
  if (nextNumber !== null) {
    cardNumber.value = nextNumber
  }
}

const loadUsers = async () => {
  try {
    users.value = await getUsersForExecutorSelection()
  } catch (error) {
    console.error('Ошибка загрузки пользователей:', error)
  }
}

const resetForm = async () => {
  if (props.card) {
    cardNumber.value = props.card.cardNumber
    year.value = props.card.year
    executorUserId.value = props.card.executorUserId ?? null
    reporter.value = props.card.reporter
    summary.value = props.card.summary
    documentReference.value = props.card.documentReference
  } else {
    year.value = new Date().getFullYear()
    await loadNextCardNumber()
    executorUserId.value = null
    reporter.value = ''
    summary.value = ''
    documentReference.value = ''
  }
}

watch(() => props.modelValue, async (isOpen) => {
  if (isOpen) {
    await loadUsers()
    await resetForm()
  }
})

watch(() => props.card, async () => {
  if (props.modelValue) {
    await resetForm()
  }
}, { immediate: true })

watch(() => year.value, async () => {
  if (!isEditMode.value && props.modelValue) {
    await loadNextCardNumber()
  }
})

const handleSave = () => {
  if (!executorUserId.value || !reporter.value.trim() || !summary.value.trim() || !documentReference.value.trim()) {
    return
  }
  
  const executorUser = users.value.find(u => u.id === executorUserId.value)
  const now = new Date().toISOString()
  emit('save', {
    cardNumber: cardNumber.value,
    year: year.value,
    executor: executorUser?.username || '',
    executorUserId: executorUserId.value,
    reporter: reporter.value.trim(),
    summary: summary.value.trim(),
    documentReference: documentReference.value.trim(),
    startDate: now,
    endDate: now
  })
  emit('update:modelValue', false)
}

const handleCancel = () => {
  emit('update:modelValue', false)
}

const isValid = computed(() => {
  return executorUserId.value !== null && 
         reporter.value.trim() && 
         summary.value.trim() && 
         documentReference.value.trim()
})
</script>

<template>
  <div v-if="modelValue" class="event-dialog-overlay" @click.self="handleCancel">
    <div class="event-dialog">
      <div class="event-dialog-header">
        <h2>{{ isEditMode ? 'Редактировать контрольную карточку' : 'Новая контрольная карточка' }}</h2>
        <button class="close-button" @click="handleCancel" type="button">×</button>
      </div>
      
      <div class="event-dialog-body">
        <div class="form-row">
          <div class="form-group">
            <label for="cardNumber">Номер карточки *</label>
            <input
              id="cardNumber"
              v-model.number="cardNumber"
              type="number"
              min="1"
              required
            />
          </div>
          
          <div class="form-group">
            <label for="year">Год *</label>
            <input
              id="year"
              v-model.number="year"
              type="number"
              :min="2000"
              :max="2100"
              required
            />
          </div>
        </div>
        
        <div class="form-group">
          <label for="executor">Исполнитель *</label>
          <select
            id="executor"
            v-model.number="executorUserId"
            required
          >
            <option :value="null">Выберите исполнителя</option>
            <option
              v-for="user in users"
              :key="user.id"
              :value="user.id"
            >
              {{ user.username }}
            </option>
          </select>
        </div>
        
        <div class="form-group">
          <label for="reporter">Кому докладывать *</label>
          <input
            id="reporter"
            v-model="reporter"
            type="text"
            placeholder="Введите, кому докладывать"
            required
          />
        </div>
        
        <div class="form-group">
          <label for="summary">Краткое содержание *</label>
          <textarea
            id="summary"
            v-model="summary"
            placeholder="Введите краткое содержание"
            rows="3"
            required
          />
        </div>
        
        <div class="form-group">
          <label for="documentReference">Документ-основание *</label>
          <input
            id="documentReference"
            v-model="documentReference"
            type="text"
            placeholder="Введите ссылку на документ"
            required
          />
        </div>
      </div>
      
      <div class="event-dialog-footer">
        <button class="button button-secondary" @click="handleCancel" type="button">
          Отмена
        </button>
        <button
          class="button button-primary"
          @click="handleSave"
          type="button"
          :disabled="!isValid"
        >
          {{ isEditMode ? 'Сохранить' : 'Создать' }}
        </button>
      </div>
    </div>
  </div>
</template>

