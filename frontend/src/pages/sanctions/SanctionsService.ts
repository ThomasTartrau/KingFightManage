import type { AxiosError, AxiosResponse } from 'axios'
import type { Problem, UUID } from '@/http'
import http, { handleError } from '@/http'
import type { components } from '@/types'

type definitions = components['schemas']

export type Sanction = definitions['Sanction']

type GetSanctions = definitions['GetSanctions']

type GetPlayerSanctions = definitions['GetPlayerSanctions']
type GetSanctionsLogs = definitions['GetSanctionsLogs']

export function getSanctions(): Promise<Sanction[]> {
  return http.get('/sanctions').then(
    (res: AxiosResponse<GetSanctions>) => {
      return res.data.sanctions
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err)),
  )
}

export function getPlayerSanctions(
  player_id: UUID,
): Promise<GetPlayerSanctions[]> {
  return http.get(`/logs/sanction/${player_id}`).then(
    (res: AxiosResponse<GetPlayerSanctions[]>) => {
      return res.data
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err)),
  )
}

export function getSanctionsLogs(): Promise<GetSanctionsLogs[]> {
  return http.get('/logs/sanction').then(
    (res: AxiosResponse<GetSanctionsLogs[]>) => {
      return res.data
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err)),
  )
}
