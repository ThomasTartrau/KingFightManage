import type { AxiosError, AxiosResponse } from 'axios'
import type { Problem } from '@/http'
import http, { handleError } from '@/http'
import type { components } from '@/types'

type definitions = components['schemas']
type Player = definitions['Player']
type GetOnlinePlayersResponse = definitions['GetOnlinePlayersResponse']

export async function getOnlinesPlayers(): Promise<Player[]> {
  return http.get('/players/onlines').then(
    (res: AxiosResponse<GetOnlinePlayersResponse>) => {
      const players = res.data.players
      const playerList: Player[] = []
      players.forEach((player) => {
        playerList.push({
          player_id: player.player_id,
          name: player.name,
          created_at: player.created_at,
        })
      })
      return Promise.resolve(playerList)
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err)),
  )
}
