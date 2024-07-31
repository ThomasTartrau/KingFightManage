import type { AxiosError, AxiosResponse } from "axios";
import type { Problem } from "@/http";
import http, { handleError } from "@/http";
import type { components } from "@/types";

type definitions = components["schemas"];
type Log = definitions["Log"];
type GetBoutiqueLogsResponse = definitions["GetBoutiqueLogsResponse"];

export type Logs = Log[];

export async function getLogs(): Promise<Logs> {
  const res = await http.get("/logs/boutique").then(
    (res: AxiosResponse<GetBoutiqueLogsResponse>) => {
      return res.data;
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err))
  );

  return res.logs;
}

export async function getPbLogs(): Promise<Logs> {
  const res = await http.get("/logs/pb").then(
    (res: AxiosResponse<GetBoutiqueLogsResponse>) => {
      return res.data;
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err))
  );

  return res.logs;
}
