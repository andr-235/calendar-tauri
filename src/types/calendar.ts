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

