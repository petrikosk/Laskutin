// Utility function to format date for display in Finnish format (DD.MM.YYYY)
export const formatDate = (dateInput: string | Date | null | undefined): string => {
  if (!dateInput) return ''

  // If it's already a YYYY-MM-DD string, parse it directly to avoid timezone issues
  if (typeof dateInput === 'string') {
    const parts = dateInput.split('-')
    if (parts.length === 3 && parts[0].length === 4) {
      return `${parts[2]}.${parts[1]}.${parts[0]}`
    }
  }

  const date = dateInput instanceof Date ? dateInput : new Date(dateInput)
  const day = String(date.getDate()).padStart(2, '0')
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const year = date.getFullYear()
  return `${day}.${month}.${year}`
}

// Convert Date to YYYY-MM-DD string without timezone issues
export const dateToYYYYMMDD = (date: Date): string => {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

// Get today's date as YYYY-MM-DD string
export const getTodayYYYYMMDD = (): string => {
  return dateToYYYYMMDD(new Date())
}

// Get a date N days in the future as YYYY-MM-DD string
export const getDateInFutureYYYYMMDD = (days: number): string => {
  const date = new Date()
  date.setDate(date.getDate() + days)
  return dateToYYYYMMDD(date)
}