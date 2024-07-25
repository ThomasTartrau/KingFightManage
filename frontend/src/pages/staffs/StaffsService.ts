import type { AxiosError, AxiosResponse } from 'axios'
import type { Problem } from '@/http'
import http, { handleError } from '@/http'
import type { components } from '@/types'

type definitions = components['schemas']
type User = definitions['User']
type GetStaffsResponse = definitions['GetStaffsResponse']

export type Staffs = User[]

export async function getStaffs(): Promise<Staffs> {
  const res = await http.get('/users').then(
    (res: AxiosResponse<GetStaffsResponse>) => {
      return res.data
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err)),
  )

  return res.users
}
