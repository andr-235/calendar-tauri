<script setup lang="ts">
import { computed } from 'vue'
import type { ControlCard } from '../types/calendar'
import { formatDateKey } from '../utils/dateUtils'
import { useCalendarComponent } from '../composables/useCalendarComponent'
import '../styles/calendar.scss'

const props = withDefaults(
  defineProps<{
    modelValue?: string | Date | null
    events?: ControlCard[]
  }>(),
  {
    modelValue: null,
    events: () => []
  }
)

const emit = defineEmits<{
  'update:modelValue': [value: Date]
  'day-click': [date: Date, events: ControlCard[]]
}>()

const modelValueRef = computed(() => props.modelValue)
const eventsRef = computed(() => props.events)

const {
  calendarDays,
  monthYearLabel,
  weekDays,
  prevMonth,
  nextMonth,
  handleDayClick
} = useCalendarComponent(
  modelValueRef,
  eventsRef,
  emit
)
</script>

<template>
  <div class="calendar">
    <div class="calendar-header">
      <button class="nav-button" @click="prevMonth" type="button">‹</button>
      <h2 class="month-year">{{ monthYearLabel }}</h2>
      <button class="nav-button" @click="nextMonth" type="button">›</button>
    </div>
    
    <div class="calendar-grid">
      <div class="week-day" v-for="day in weekDays" :key="day">
        {{ day }}
      </div>
      
      <div
        v-for="(cell, index) in calendarDays"
        :key="cell.date ? formatDateKey(cell.date) : `empty-${index}`"
        class="day-cell"
        :class="{
          'other-month': !cell.isCurrentMonth,
          'today': cell.isToday,
          'has-events': cell.events.length > 0
        }"
        @click="handleDayClick(cell)"
      >
        <span class="day-number" v-if="cell.date">
          {{ cell.date.getDate() }}
        </span>
        
        <div class="events-indicator" v-if="cell.events.length > 0">
          <span
            v-for="event in cell.events.slice(0, 3)"
            :key="event.id"
            class="event-dot"
            :title="`${event.cardNumber}/${event.year}`"
          ></span>
          <span v-if="cell.events.length > 3" class="event-more">
            +{{ cell.events.length - 3 }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
