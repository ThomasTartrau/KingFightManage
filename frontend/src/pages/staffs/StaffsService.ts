import type { AxiosError, AxiosResponse } from 'axios'
import type { Problem } from '@/http'
import http, { handleError } from '@/http'
import type { components } from '@/types'

type definitions = components['schemas']
type User = definitions['User']
type Log = definitions['Log']
type GetStaffsResponse = definitions['GetStaffsResponse']
type GetLogsResponse = definitions['GetLogsResponse']

export type Staffs = User[]
export type Logs = Log[]

export async function getStaffs(): Promise<Staffs> {
  const res = await http.get('/staffs').then(
    (res: AxiosResponse<GetStaffsResponse>) => {
      return res.data
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err)),
  )

  return res.users
}

export async function getLogs(): Promise<Logs> {
  const res = await http.get('/staffs/logs').then(
    (res: AxiosResponse<GetLogsResponse>) => {
      return res.data
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err)),
  )

  return res.logs
}
