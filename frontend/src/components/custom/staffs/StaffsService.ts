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
        title: "Role updated",
        message: "The role of the user has been updated",
      });
    })
    .catch(displayError);
}

export async function deleteUser(user_id: UUID) {
  await http
    .delete(`/users/${user_id}`)
    .then(() => {
      push.success({
        title: "User deleted",
        message: "The user has been deleted",
      });
    })
    .catch(displayError);
}
