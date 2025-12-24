<script setup lang="ts">
import type { ControlCard } from '../types/calendar'
import { EXECUTION_PERIOD_TYPES } from '../constants/calendar'
import '../styles/print-card.scss'

interface Props {
  card: ControlCard
}

defineProps<Props>()

const formatDate = (dateString?: string): string => {
  if (!dateString) return ''
  try {
    const date = new Date(dateString)
    return date.toLocaleDateString('ru-RU', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit'
    })
  } catch {
    return dateString
  }
}

const formatDateFull = (dateString?: string): string => {
  if (!dateString) return ''
  try {
    const date = new Date(dateString)
    return date.toLocaleDateString('ru-RU', {
      year: 'numeric',
      month: 'long',
      day: 'numeric'
    })
  } catch {
    return dateString
  }
}

const getPeriodTypeLabel = (type?: string): string => {
  if (!type) return ''
  const period = EXECUTION_PERIOD_TYPES.find(p => p.value === type)
  return period ? period.label.toLowerCase() : ''
}
</script>

<template>
  <article class="print-card">
    <header class="print-card-header">
      <div class="print-header-left">
        <div class="print-header-label">По исполнению подлежит возврату в</div>
        <div class="print-header-value">{{ card.returnTo || '________________' }}</div>
      </div>
      <div class="print-header-center">
        <h1 class="print-card-title">КОНТРОЛЬНАЯ КАРТОЧКА</h1>
      </div>
      <div class="print-header-right">
        <div class="print-header-deadline">
          <div class="print-header-label">Срок исполнения:</div>
          <div class="print-header-value">
            {{ card.executionDeadline ? formatDate(card.executionDeadline) : '________________' }}
            <span v-if="card.executionPeriodType">({{ getPeriodTypeLabel(card.executionPeriodType) }})</span>
          </div>
        </div>
        <div class="print-header-deadline">
          <div class="print-header-label">Срок продлен до:</div>
          <div class="print-header-value">{{ card.extendedDeadline ? formatDate(card.extendedDeadline) : '________________' }}</div>
        </div>
      </div>
    </header>

    <div class="print-card-body">
      <div class="print-card-left">
        <div class="print-field">
          <div class="print-field-value">№ {{ card.year }}/{{ card.cardNumber }}</div>
        </div>
        <div class="print-field">
          <div class="print-field-label">Исполнитель:</div>
          <div class="print-field-value">{{ card.executor || '________________' }}</div>
        </div>
        <div class="print-field">
          <div class="print-field-label">Документ основание:</div>
          <div class="print-field-value">{{ card.documentReference || '________________' }}</div>
        </div>
        <div class="print-field">
          <div class="print-field-label">Краткое содержание:</div>
          <div class="print-field-value">{{ card.summary || '________________' }}</div>
        </div>
        <div class="print-field">
          <div class="print-field-label">Резолюция:</div>
          <div class="print-field-value">{{ card.resolution || '________________' }}</div>
        </div>
      </div>
      <div class="print-card-right">
        <div class="print-field">
          <div class="print-field-label">Подразделение:</div>
          <div class="print-field-value">{{ card.department || '________________' }}</div>
        </div>
        <div class="print-field">
          <div class="print-field-label">Дата</div>
          <div class="print-field-value">{{ card.createdAt ? formatDateFull(card.createdAt) : '________________' }}</div>
        </div>
      </div>
    </div>

    <footer class="print-card-footer">
      <div class="print-footer-label">Контроль осуществляет:</div>
      <div class="print-footer-value">{{ card.controller || '________________' }}</div>
    </footer>
  </article>
</template>


