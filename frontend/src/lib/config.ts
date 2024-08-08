interface Config {
  API_ENDPOINT: string;
  API_TIMEOUT: number;
  FRONTEND_DEV_MODE: boolean;
  SITE_NAME: string;
  BOUTIQUE_OBJECTIVE_MONTHLY: number;
}

export const config: Config = {
  API_ENDPOINT: "http://localhost:8080/api/v1",
  API_TIMEOUT: 3000, // 3 seconds
  FRONTEND_DEV_MODE: true,
  SITE_NAME: "KingFightManage",
  BOUTIQUE_OBJECTIVE_MONTHLY: 2000,
};
