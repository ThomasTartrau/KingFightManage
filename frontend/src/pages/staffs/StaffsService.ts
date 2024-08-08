import type { AxiosError, AxiosResponse } from "axios";
import type { Problem } from "@/http";
import http, { handleError } from "@/http";
import type { components } from "@/types";

type definitions = components["schemas"];
type User = definitions["User"];
type Log = definitions["Log"];
type ServiceAccess = definitions["ServiceAccess"];

type GetStaffsResponse = definitions["GetStaffsResponse"];
type GetStaffLogsResponse = definitions["GetStaffLogsResponse"];
type GetServiceAccessResponse = definitions["GetServiceAccessResponse"];

export type Staffs = User[];
export type Logs = Log[];
export type ServicesAccess = ServiceAccess[];

export async function getStaffs(): Promise<Staffs> {
  const res = await http.get("/staffs").then(
    (res: AxiosResponse<GetStaffsResponse>) => {
      return res.data;
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err)),
  );

  return res.users;
}

export async function getLogs(): Promise<Logs> {
  const res = await http.get("/staffs/logs").then(
    (res: AxiosResponse<GetStaffLogsResponse>) => {
      return res.data;
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err)),
  );

  return res.logs;
}

export async function createServiceAccess(): Promise<void> {
  await http.post("/service-access").then(
    (res: AxiosResponse<void>) => {
      return res.data;
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err)),
  );
}

export async function getServiceAccess(): Promise<ServicesAccess> {
  const res = await http.get("/service-access").then(
    (res: AxiosResponse<GetServiceAccessResponse>) => {
      return res.data;
    },
    (err: AxiosError<AxiosResponse<Problem>>) =>
      Promise.reject(handleError(err)),
  );

  return res.service_access;
}
