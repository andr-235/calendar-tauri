export interface ControlCard {
  id: string
  cardNumber: number
  year: number
  executor: string
  reporter: string
  summary: string
  documentReference: string
  executorUserId?: number
  createdAt?: string
  startDate: string
  endDate: string
  returnTo?: string
  executionDeadline?: string
  executionPeriodType?: 'daily' | 'weekly' | 'monthly'
  extendedDeadline?: string
  resolution?: string
  department?: string
  controller?: string
  controllerUserId?: number
}

export interface CalendarEvent {
  id: string
  startDate: string
  endDate: string
  title?: string
}

export interface DayCell<T = CalendarEvent> {
  date: Date | null
  isCurrentMonth: boolean
  isToday: boolean
  events: T[]
}

