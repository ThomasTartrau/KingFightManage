import http, { Problem, handleError } from "@/http";
import { components } from "@/types";
import { AxiosError, AxiosResponse } from "axios";

type definitions = components["schemas"];
type User = definitions["User"];

export function getStaffs(): Promise<User[]> {
  return http.get("/users").then(
    (res: AxiosResponse<User[]>) => res.data,
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err))
  );
}
