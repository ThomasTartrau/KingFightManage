import http, { displayError, UUID } from "@/http";
import { push } from "notivue";

export async function setRole(user_id: UUID, role: String) {
  await http
    .post("/users/set-role", {
      user_id,
      role: role.toLowerCase(),
    })
    .then(() => {
      push.success({
        title: "Role mis à jour",
        message: "Le rôle de l'utilisateur a été mis à jour",
      });
    })
    .catch(displayError);
}

export async function deleteUser(user_id: UUID) {
  await http
    .delete(`/users/${user_id}`)
    .then(() => {
      push.success({
        title: "Utilisateur supprimé",
        message: "L'utilisateur a été supprimé",
      });
    })
    .catch(displayError);
}

export async function sendMessage(
  user_id: UUID,
  username: string,
  message: string
) {
  await http
    .post("/users/send-message", { user_id, username, message })
    .then(() => {
      push.success({
        title: "Message envoyé",
        message: "Le message a été envoyé",
      });
    })
    .catch(displayError);
}
