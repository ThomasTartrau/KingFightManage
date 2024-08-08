import { push } from "notivue";
import type { UUID } from "@/http";
import http, { displayError } from "@/http";

export async function mutePlayer(
  player_id: UUID,
  staff_name: string,
  sanction_id: UUID,
  motif: string,
  generate_event: boolean,
) {
  await http
    .post("/sanctions/mute", {
      player_id,
      staff_name,
      sanction_id,
      motif,
      generate_event,
    })
    .then(() => {
      push.success({
        title: "Succès",
        message: "La sanction a bien été appliquée",
      });
    })
    .catch(displayError);
}

export async function kickPlayer(
  player_id: UUID,
  staff_name: string,
  sanction_id: UUID,
  motif: string,
  generate_event: boolean,
) {
  await http
    .post("/sanctions/kick", {
      player_id,
      staff_name,
      sanction_id,
      motif,
      generate_event,
    })
    .then(() => {
      push.success({
        title: "Succès",
        message: "La sanction a bien été appliquée",
      });
    })
    .catch(displayError);
}

export async function banPlayer(
  player_id: UUID,
  staff_name: string,
  sanction_id: UUID,
  motif: string,
  generate_event: boolean,
) {
  await http
    .post("/sanctions/ban", {
      player_id,
      staff_name,
      sanction_id,
      motif,
      generate_event,
    })
    .then(() => {
      push.success({
        title: "Succès",
        message: "La sanction a bien été appliquée",
      });
    })
    .catch(displayError);
}
