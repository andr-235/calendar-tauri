import { computed, ref, type Ref, type ComputedRef } from 'vue'
import type { DayCell } from '../types/calendar'
import {
  formatDateKey,
  isSameDay,
  getToday,
  createMonthDate,
  getStartDayOfWeek
} from '../utils/dateUtils'
import { MONTH_NAMES, WEEK_DAYS, CALENDAR_CELLS_COUNT } from '../constants/calendar'

export const useCalendar = <T extends { id: string; startDate: string; endDate: string }>(
  initialDate: ComputedRef<Date> | Ref<Date>,
  events: ComputedRef<T[]> | Ref<T[]>
) => {
  const currentMonth = ref(createMonthDate(initialDate.value.getFullYear(), initialDate.value.getMonth()))
  const today = getToday()

  const createEventsMap = (eventsList: T[]): Map<string, T[]> => {
    const map = new Map<string, T[]>()
    eventsList.forEach(event => {
      const startDate = new Date(event.startDate)
      const endDate = new Date(event.endDate)
      
      const currentDate = new Date(startDate)
      while (currentDate <= endDate) {
        const dateKey = formatDateKey(currentDate)
        const existing = map.get(dateKey) || []
        map.set(dateKey, [...existing, event])
        
        currentDate.setDate(currentDate.getDate() + 1)
      }
    })
    return map
  }

  const eventsMap = computed(() => createEventsMap(events.value))

  const calendarDays = computed((): DayCell<T>[] => {
    const year = currentMonth.value.getFullYear()
    const month = currentMonth.value.getMonth()

    const firstDay = createMonthDate(year, month)
    const lastDay = new Date(year, month + 1, 0)

    const startDayOfWeek = getStartDayOfWeek(firstDay)
    const daysInMonth = lastDay.getDate()

    const days: DayCell<T>[] = []

    for (let i = 0; i < startDayOfWeek; i++) {
      days.push({ date: null, isCurrentMonth: false, isToday: false, events: [] })
    }

    for (let day = 1; day <= daysInMonth; day++) {
      const date = new Date(year, month, day)
      const dateKey = formatDateKey(date)
      const dayEvents = eventsMap.value.get(dateKey) || []
      const isToday = isSameDay(date, today)

      days.push({
        date,
        isCurrentMonth: true,
        isToday,
        events: dayEvents
      })
    }

    const remainingCells = CALENDAR_CELLS_COUNT - days.length
    for (let i = 0; i < remainingCells; i++) {
      days.push({ date: null, isCurrentMonth: false, isToday: false, events: [] })
    }

    return days
  })

  const monthYearLabel = computed(() => {
    return `${MONTH_NAMES[currentMonth.value.getMonth()]} ${currentMonth.value.getFullYear()}`
  })

  const navigateMonth = (direction: -1 | 1) => {
    currentMonth.value = createMonthDate(
      currentMonth.value.getFullYear(),
      currentMonth.value.getMonth() + direction
    )
  }

  const prevMonth = () => navigateMonth(-1)
  const nextMonth = () => navigateMonth(1)

  return {
    calendarDays,
    monthYearLabel,
    weekDays: WEEK_DAYS,
    prevMonth,
    nextMonth
  }
}

