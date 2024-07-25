import http, { Problem, handleError } from "@/http";
import { components } from "@/types";
import { AxiosError, AxiosResponse } from "axios";

type definitions = components["schemas"];
type User = definitions["User"];
type GetStaffsResponse = definitions["GetStaffsResponse"];

export type Staffs = User[];

export async function getStaffs(): Promise<Staffs> {
  const res = await http.get("/users").then(
    (res: AxiosResponse<GetStaffsResponse>) => {
      return res.data;
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err))
  );

  return res.users;
}
