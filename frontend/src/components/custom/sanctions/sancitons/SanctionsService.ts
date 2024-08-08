import { push } from "notivue";
import type { UUID } from "@/http";
import http, { displayError } from "@/http";
import type { components } from "@/types";

type definitions = components["schemas"];
type UpdateSanctionPost = definitions["UpdateSanctionPost"];
type CreateSanctionPost = definitions["CreateSanctionPost"];

export async function deleteSanction(sanction_id: UUID): Promise<void> {
  await http
    .delete(`/sanctions/${sanction_id}`)
    .then(() => {
      push.success({
        title: "Sanction supprimée",
        message: "L'opération a été effectuée avec succès",
      });
    })
    .catch(displayError);
}

export async function updateSanction(
  sanction_id: UUID,
  sanction: UpdateSanctionPost,
) {
  await http
    .put(`/sanctions/${sanction_id}`, sanction)
    .then(() => {
      push.success({
        title: "Sanction modifié",
        message: "L'opération a été effectuée avec succès",
      });
    })
    .catch(displayError);
}

export async function createSanction(sanction: CreateSanctionPost) {
  await http
    .post("/sanctions", sanction)
    .then(() => {
      push.success({
        title: "Sanction créée",
        message: "L'opération a été effectuée avec succès",
      });
    })
    .catch(displayError);
}
