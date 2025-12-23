export type UserRole = 'admin' | 'user' | 'controller'

export interface User {
  id: number
  username: string
  role: UserRole
  createdAt: string
}

