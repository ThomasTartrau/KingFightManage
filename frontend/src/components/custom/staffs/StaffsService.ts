import { push } from 'notivue'
import type { UUID } from '@/http'
import http, { displayError } from '@/http'

export async function setRole(user_id: UUID, role: string) {
  await http
    .post('/staffs/set-role', {
      user_id,
      role: role.toLowerCase(),
    })
    .then(() => {
      push.success({
        title: 'Role mis à jour',
        message: 'Le rôle de l\'utilisateur a été mis à jour',
      })
    })
    .catch(displayError)
}

export async function deleteUser(user_id: UUID) {
  await http
    .delete(`/staffs/${user_id}`)
    .then(() => {
      push.success({
        title: 'Utilisateur supprimé',
        message: 'L\'utilisateur a été supprimé',
      })
    })
    .catch(displayError)
}

export async function sendMessage(
  user_id: UUID,
  username: string,
  message: string,
) {
  await http
    .post('/events/send-message', { user_id, username, message })
    .then(() => {
      push.success({
        title: 'Message envoyé',
        message: 'Le message a été envoyé',
      })
    })
    .catch(displayError)
}
