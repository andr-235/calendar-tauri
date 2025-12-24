<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { ControlCard } from '../types/calendar'
import type { User } from '../types/auth'
import { useControlCards } from '../composables/useControlCards'
import { EXECUTION_PERIOD_TYPES, DEPARTMENTS } from '../constants/calendar'

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

const { getNextCardNumber, getUsersForExecutorSelection, getUsersForControllerSelection } = useControlCards()

const cardNumber = ref<number>(1)
const year = ref<number>(new Date().getFullYear())
const executorUserId = ref<number | null>(null)
const users = ref<User[]>([])
const reporter = ref('')
const summary = ref('')
const documentReference = ref('')
const returnTo = ref('')
const executionDeadline = ref('')
const executionPeriodType = ref<'daily' | 'weekly' | 'monthly' | ''>('')
const extendedDeadline = ref('')
const resolution = ref('')
const department = ref('')
const controller = ref('')
const controllerUserId = ref<number | null>(null)
const controllerUsers = ref<User[]>([])
const customDepartment = ref(false)
const customController = ref(false)

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
    controllerUsers.value = await getUsersForControllerSelection()
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
    returnTo.value = props.card.returnTo ?? ''
    executionDeadline.value = props.card.executionDeadline ?? ''
    executionPeriodType.value = props.card.executionPeriodType ?? ''
    extendedDeadline.value = props.card.extendedDeadline ?? ''
    resolution.value = props.card.resolution ?? ''
    department.value = props.card.department ?? ''
    controller.value = props.card.controller ?? ''
    controllerUserId.value = props.card.controllerUserId ?? null
    customDepartment.value = props.card.department ? !DEPARTMENTS.includes(props.card.department as any) : false
    customController.value = props.card.controller ? !controllerUsers.value.some(u => u.username === props.card?.controller) : false
  } else {
    year.value = new Date().getFullYear()
    await loadNextCardNumber()
    executorUserId.value = null
    reporter.value = ''
    summary.value = ''
    documentReference.value = ''
    returnTo.value = ''
    executionDeadline.value = ''
    executionPeriodType.value = ''
    extendedDeadline.value = ''
    resolution.value = ''
    department.value = ''
    controller.value = ''
    controllerUserId.value = null
    customDepartment.value = false
    customController.value = false
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
  const finalController = customController.value ? controller.value.trim() : (controllerUsers.value.find(u => u.id === controllerUserId.value)?.username || '')
  const finalDepartment = customDepartment.value ? department.value.trim() : department.value.trim()
  
  emit('save', {
    cardNumber: cardNumber.value,
    year: year.value,
    executor: executorUser?.username || '',
    executorUserId: executorUserId.value,
    reporter: reporter.value.trim(),
    summary: summary.value.trim(),
    documentReference: documentReference.value.trim(),
    returnTo: returnTo.value.trim() || undefined,
    executionDeadline: executionDeadline.value.trim() || undefined,
    executionPeriodType: executionPeriodType.value || undefined,
    extendedDeadline: extendedDeadline.value.trim() || undefined,
    resolution: resolution.value.trim() || undefined,
    department: finalDepartment || undefined,
    controller: finalController || undefined,
    controllerUserId: controllerUserId.value ?? undefined,
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

        <div class="form-group">
          <label for="returnTo">По исполнению подлежит возврату в</label>
          <input
            id="returnTo"
            v-model="returnTo"
            type="text"
            placeholder="Введите, куда возвращать"
          />
        </div>

        <div class="form-row">
          <div class="form-group">
            <label for="executionDeadline">Срок исполнения</label>
            <input
              id="executionDeadline"
              v-model="executionDeadline"
              type="date"
            />
          </div>
          <div class="form-group">
            <label for="executionPeriodType">Тип периода</label>
            <select
              id="executionPeriodType"
              v-model="executionPeriodType"
            >
              <option value="">Не указан</option>
              <option
                v-for="type in EXECUTION_PERIOD_TYPES"
                :key="type.value"
                :value="type.value"
              >
                {{ type.label }}
              </option>
            </select>
          </div>
        </div>

        <div class="form-group">
          <label for="extendedDeadline">Срок продлен до</label>
          <input
            id="extendedDeadline"
            v-model="extendedDeadline"
            type="date"
          />
        </div>

        <div class="form-group">
          <label for="resolution">Резолюция</label>
          <textarea
            id="resolution"
            v-model="resolution"
            placeholder="Введите резолюцию"
            rows="3"
          />
        </div>

        <div class="form-group">
          <label for="department">Подразделение</label>
          <div style="display: flex; gap: 8px; align-items: center;">
            <select
              v-if="!customDepartment"
              id="department"
              v-model="department"
              @change="if (department === 'custom') { customDepartment = true; department = '' }"
            >
              <option value="">Выберите подразделение</option>
              <option
                v-for="dept in DEPARTMENTS"
                :key="dept"
                :value="dept"
              >
                {{ dept }}
              </option>
              <option value="custom">Другое...</option>
            </select>
            <input
              v-else
              id="department"
              v-model="department"
              type="text"
              placeholder="Введите подразделение"
            />
            <button
              v-if="customDepartment"
              type="button"
              @click="customDepartment = false; department = ''"
              style="padding: 4px 8px; font-size: 12px;"
            >
              Выбрать из списка
            </button>
          </div>
        </div>

        <div class="form-group">
          <label for="controller">Контроль осуществляет</label>
          <div style="display: flex; gap: 8px; align-items: center;">
            <select
              v-if="!customController"
              id="controller"
              v-model.number="controllerUserId"
              @change="if (controllerUserId === -1) { customController = true; controllerUserId = null; controller = '' }"
            >
              <option :value="null">Выберите контроллера</option>
              <option
                v-for="user in controllerUsers"
                :key="user.id"
                :value="user.id"
              >
                {{ user.username }}
              </option>
              <option :value="-1">Другое...</option>
            </select>
            <input
              v-else
              id="controller"
              v-model="controller"
              type="text"
              placeholder="Введите контроллера"
            />
            <button
              v-if="customController"
              type="button"
              @click="customController = false; controllerUserId = null; controller = ''"
              style="padding: 4px 8px; font-size: 12px;"
            >
              Выбрать из списка
            </button>
          </div>
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

