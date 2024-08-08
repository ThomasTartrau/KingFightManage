import type { AxiosError, AxiosResponse } from "axios";
import type { Problem } from "@/http";
import http, { handleError } from "@/http";
import type { components } from "@/types";

type definitions = components["schemas"];
type Error = definitions["Error"];

export async function getErrors(): Promise<Error[]> {
  return http.get("/errors").then(
    (res: AxiosResponse<Error[]>) => {
      return res.data;
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err))
  );
}
