export const formatDateKey = (date: Date): string => {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

export const isSameDay = (date1: Date, date2: Date): boolean => {
  return (
    date1.getFullYear() === date2.getFullYear() &&
    date1.getMonth() === date2.getMonth() &&
    date1.getDate() === date2.getDate()
  )
}

export const normalizeDate = (value: string | Date | null): Date => {
  if (!value) return new Date()
  if (typeof value === 'string') {
    const parsed = new Date(value)
    return isNaN(parsed.getTime()) ? new Date() : parsed
  }
  return value
}

export const getToday = (): Date => {
  const today = new Date()
  today.setHours(0, 0, 0, 0)
  return today
}

export const createMonthDate = (year: number, month: number): Date => {
  return new Date(year, month, 1)
}

export const getStartDayOfWeek = (date: Date): number => {
  const day = date.getDay()
  return day === 0 ? 6 : day - 1
}

