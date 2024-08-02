import { push } from "notivue";
import type { UUID } from "@/http";
import http, { displayError } from "@/http";

export async function setRole(user_id: UUID, role: string) {
  await http
    .post("/staffs/set-role", {
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
    .delete(`/staffs/${user_id}`)
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
    .post("/events/send-message", { user_id, username, message })
    .then(() => {
      push.success({
        title: "Message envoyé",
        message: "Le message a été envoyé",
      });
    })
    .catch(displayError);
}

export async function generateRegistrationToken() {
  await http
    .get("/staffs/generate-registration-token")
    .then((res) => {
      try {
        navigator.clipboard.writeText(res.data.registration_token);
        push.success({
          title: "Copié",
          message: "Le token a été copié dans le presse-papiers.",
        });
      } catch {
        push.error({
          title: "Erreur",
          message: "Une erreur est survenue lors de la copie du token.",
        });
      }
    })
    .catch(displayError);
}

export async function deleteServiceAccessToken(tokenId: UUID) {
  await http
    .delete(`/service-access/${tokenId}`)
    .then(() => {
      push.success({
        title: "Token supprimé",
        message: "Le token a été supprimé",
      });
    })
    .catch(displayError);
}
