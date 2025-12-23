<script setup lang="ts">
import type { ControlCard } from '../types/calendar'

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
      month: 'long',
      day: 'numeric'
    })
  } catch {
    return dateString
  }
}
</script>

<template>
  <article class="print-card">
    <header class="print-card-header">
      <div class="print-header-content">
        <h2 class="print-card-number">№{{ card.cardNumber }}/{{ card.year }}</h2>
      </div>
    </header>
    <div class="print-card-content">
      <dl class="print-fields">
        <div class="print-field">
          <dt class="print-field-label">Исполнитель</dt>
          <dd class="print-field-value">{{ card.executor }}</dd>
        </div>
        <div class="print-field">
          <dt class="print-field-label">Кому докладывать</dt>
          <dd class="print-field-value">{{ card.reporter }}</dd>
        </div>
        <div class="print-field">
          <dt class="print-field-label">Краткое содержание</dt>
          <dd class="print-field-value">{{ card.summary }}</dd>
        </div>
        <div class="print-field">
          <dt class="print-field-label">Документ-основание</dt>
          <dd class="print-field-value">{{ card.documentReference }}</dd>
        </div>
        <div v-if="card.createdAt" class="print-field">
          <dt class="print-field-label">Дата создания</dt>
          <dd class="print-field-value">{{ formatDate(card.createdAt) }}</dd>
        </div>
      </dl>
    </div>
  </article>
</template>

<style scoped lang="scss">
.print-card {
  max-width: 210mm;
  margin: 0 auto;
  padding: 25mm;
  background: white;
  color: #000;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  line-height: 1.6;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.print-card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 0 0 20px 0;
  margin-bottom: 20px;
  border-bottom: 1px solid #d1d5db;
}

.print-header-content {
  flex: 1;
  min-width: 0;
}

.print-card-number {
  margin: 0;
  font-size: 28px;
  font-weight: 600;
  color: #000;
  letter-spacing: -0.02em;
  line-height: 1.3;
}

.print-card-content {
  padding: 0;
}

.print-fields {
  display: flex;
  flex-direction: column;
  gap: 24px;
  margin: 0;
}

.print-field {
  display: grid;
  grid-template-columns: 200px 1fr;
  gap: 20px;
  align-items: flex-start;
  padding-bottom: 20px;
  border-bottom: 1px solid #f3f4f6;

  &:last-child {
    border-bottom: none;
    padding-bottom: 0;
  }
}

.print-field-label {
  font-weight: 500;
  color: #6b7280;
  font-size: 16px;
  line-height: 1.5;
  margin: 0;
  letter-spacing: 0.01em;
}

.print-field-value {
  color: #000;
  font-size: 16px;
  line-height: 1.6;
  margin: 0;
  word-break: break-word;
}

@media print {
  .print-card {
    margin: 0;
    padding: 20mm;
    box-shadow: none;
  }

  .print-card-header {
    border-bottom: 1px solid #d1d5db;
  }

  .print-field {
    page-break-inside: avoid;
  }
}
</style>

