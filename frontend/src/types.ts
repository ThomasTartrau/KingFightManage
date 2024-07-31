import type { UUID } from '@/http'

export interface components {
  schemas: {
    LoginPost: {
      email: string
      password: string
    }
    Problem: {
      detail: string
      id: string
      /** Format: int32 */
      status: number
      title: string
    }
    LoginResponse: {
      access_token: string
      /** Format: date-time */
      access_token_expiration: string
      user_id: UUID
      email: string
      username: string
      role: string
      refresh_token: string
      /** Format: date-time */
      refresh_token_expiration: string
    }
    GetStaffsResponse: {
      users: [
        {
          user_id: UUID
          username: string
          role: string
          is_online: boolean
        },
      ]
    }
    User: {
      user_id: UUID
      username: string
      role: string
      is_online: boolean
    }
    Log: {
      log_id: UUID
      username: string
      action: string
      /** Format: date-time */
      created_at: string
    }
    GetStaffLogsResponse: {
      logs: [
        {
          log_id: UUID
          username: string
          action: string
          /** Format: date-time */
          created_at: string
        },
      ]
    }
    GetBoutiqueLogsResponse: {
      logs: [
        {
          log_id: UUID
          username: string
          action: string
          /** Format: date-time */
          created_at: string
        },
      ]
    }
  }
}
