import { computed, type Ref, type ComputedRef } from 'vue'
import type { DayCell } from '../types/calendar'
import { normalizeDate } from '../utils/dateUtils'
import { useCalendar } from './useCalendar'

export const useCalendarComponent = <T extends { id: string; startDate: string; endDate: string }>(
  modelValue: ComputedRef<string | Date | null> | Ref<string | Date | null>,
  events: ComputedRef<T[]> | Ref<T[]>,
  emit: {
    (e: 'update:modelValue', value: Date): void
    (e: 'day-click', date: Date, events: T[]): void
  }
) => {
  const currentDate = computed(() => normalizeDate(modelValue.value))

  const { calendarDays, monthYearLabel, weekDays, prevMonth, nextMonth } = useCalendar(
    currentDate,
    events
  )

  const handleDayClick = (cell: DayCell<T>) => {
    if (!cell.date || !cell.isCurrentMonth) return

    emit('update:modelValue', cell.date)
    emit('day-click', cell.date, cell.events)
  }

  return {
    calendarDays,
    monthYearLabel,
    weekDays,
    prevMonth,
    nextMonth,
    handleDayClick
  }
}

