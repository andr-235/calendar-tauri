<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import ControlCardDialog from '../components/ControlCardDialog.vue'
import PrintCardDialog from '../components/PrintCardDialog.vue'
import type { ControlCard } from '../types/calendar'
import { useControlCards } from '../composables/useControlCards'
import { useSettings } from '../composables/useSettings'
import { useAuthStore } from '../stores/auth'

const { cards, loadCards, createCard, updateCard, deleteCard } = useControlCards()
const { loadSettings, settings } = useSettings()
const authStore = useAuthStore()

const canCreateCard = authStore.isAdmin || authStore.isController
const canEditCard = authStore.isAdmin || authStore.isController

const selectedCard = ref<ControlCard | null>(null)
const showDialog = ref(false)
const printCard = ref<ControlCard | null>(null)
const showPrintDialog = ref(false)

onMounted(async () => {
  await loadSettings()
  if (settings.value.dbPath) {
    await loadCards()
  }
})

watch(() => settings.value.dbPath, async (newPath) => {
  if (newPath) {
    await loadCards()
  }
})

const handleCreateCard = () => {
  selectedCard.value = null
  showDialog.value = true
}

const handleEditCard = (card: ControlCard) => {
  selectedCard.value = card
  showDialog.value = true
}

const handleDeleteCard = async (card: ControlCard) => {
  if (confirm(`Удалить контрольную карточку №${card.cardNumber}/${card.year}?`)) {
    await deleteCard(card.id)
  }
}

const handlePrintCard = (card: ControlCard) => {
  printCard.value = card
  showPrintDialog.value = true
}

const handleSaveCard = async (cardData: Omit<ControlCard, 'id' | 'createdAt'> & { executorUserId: number }) => {
  if (selectedCard.value) {
    await updateCard(
      selectedCard.value.id,
      cardData.cardNumber,
      cardData.year,
      cardData.executorUserId,
      cardData.reporter,
      cardData.summary,
      cardData.documentReference,
      cardData.returnTo,
      cardData.executionDeadline,
      cardData.executionPeriodType,
      cardData.extendedDeadline,
      cardData.resolution,
      cardData.department,
      cardData.controller,
      cardData.controllerUserId
    )
  } else {
    await createCard(
      cardData.cardNumber,
      cardData.year,
      cardData.executorUserId,
      cardData.reporter,
      cardData.summary,
      cardData.documentReference,
      cardData.returnTo,
      cardData.executionDeadline,
      cardData.executionPeriodType,
      cardData.extendedDeadline,
      cardData.resolution,
      cardData.department,
      cardData.controller,
      cardData.controllerUserId
    )
  }
  showDialog.value = false
  selectedCard.value = null
  await loadCards()
}
</script>

<template>
  <div class="page-container">
    <div class="calendar-header-section">
      <h1>Контрольные карточки</h1>
      <button
        v-if="canCreateCard"
        class="button button-primary"
        @click="handleCreateCard"
        type="button"
      >
        + Создать карточку
      </button>
    </div>
    
    <div v-if="cards.length === 0" class="no-cards">
      <p>Нет созданных контрольных карточек</p>
    </div>
    
    <div v-else class="cards-list">
      <article
        v-for="card in cards"
        :key="card.id"
        class="card-item"
      >
        <header class="card-item-header">
          <div class="card-header-content">
            <h2 class="card-number">№{{ card.cardNumber }}/{{ card.year }}</h2>
          </div>
          <div class="card-actions">
            <button
              class="button button-secondary"
              @click="handlePrintCard(card)"
              type="button"
            >
              Печать/Экспорт
            </button>
            <template v-if="canEditCard">
              <button
                class="button button-secondary"
                @click="handleEditCard(card)"
                type="button"
              >
                Редактировать
              </button>
              <button
                class="button button-danger"
                @click="handleDeleteCard(card)"
                type="button"
              >
                Удалить
              </button>
            </template>
          </div>
        </header>
        <div class="card-item-content">
          <dl class="card-fields">
            <div class="card-field">
              <dt class="card-field-label">Исполнитель</dt>
              <dd class="card-field-value">{{ card.executor }}</dd>
            </div>
            <div class="card-field">
              <dt class="card-field-label">Кому докладывать</dt>
              <dd class="card-field-value">{{ card.reporter }}</dd>
            </div>
            <div class="card-field">
              <dt class="card-field-label">Краткое содержание</dt>
              <dd class="card-field-value">{{ card.summary }}</dd>
            </div>
            <div class="card-field">
              <dt class="card-field-label">Документ-основание</dt>
              <dd class="card-field-value">{{ card.documentReference }}</dd>
            </div>
          </dl>
        </div>
      </article>
    </div>
    
    <ControlCardDialog
      v-model="showDialog"
      :card="selectedCard"
      @save="handleSaveCard"
    />
    
    <PrintCardDialog
      v-model="showPrintDialog"
      :card="printCard"
    />
  </div>
</template>

