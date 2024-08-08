import HomePage from "@/pages/HomePage.vue";
import LoginPage from "@/pages/users/LoginPage.vue";
import RegisterPage from "@/pages/users/RegisterPage.vue";
import VerifyEmail from "@/pages/users/VerifyEmail.vue";
import UserSettings from "@/pages/users/UserSettings.vue";
import Error404 from "@/pages/others/Error404.vue";
import BeginResetpassword from "@/pages/users/BeginResetpassword.vue";
import ResetPassword from "@/pages/users/ResetPassword.vue";
import GetStaffs from "@/pages/staffs/GetStaffs.vue";
import LogsStaffs from "@/pages/staffs/LogsStaffs.vue";
import LogsBoutique from "@/pages/boutique/LogsBoutique.vue";
import LogsBoutiquePb from "@/pages/boutique/LogsBoutiquePb.vue";
import BoutiqueGraphs from "@/pages/boutique/BoutiqueGraphs.vue";
import ServiceAccess from "@/pages/staffs/ServiceAccess.vue";
import GetSanctions from "@/pages/sanctions/GetSanctions.vue";
import LogsSanctions from "@/pages/sanctions/LogsSanctions.vue";
import LogsPlayerSanction from "@/pages/sanctions/LogsPlayerSanction.vue";
import GetPlayers from "@/pages/players/GetPlayers.vue";

export type TemplateRoutes = string;

export const routes: Record<TemplateRoutes, string> = {
  Home: "Home",

  Login: "Login",
  Register: "Register",
  VerifyEmail: "VerifyEmail",
  BeginResetpassword: "BeginResetpassword",
  ResetPassword: "ResetPassword",

  Settings: "Settings",
  SecuritySettings: "SecuritySettings",
  DeleteAccountSettings: "DeleteAccountSettings",

  BoutiqueLogs: "BoutiqueLogs",
  BoutiqueLogsPb: "BoutiqueLogsPb",
  BoutiqueGraphs: "BoutiqueGraphs",

  Staffs: "Staffs",
  LogsStaffs: "LogsStaffs",
  ServiceAccess: "ServiceAccess",

  Sanctions: "Sanctions",
  SanctionsLogs: "SanctionsLogs",
  SanctionsPlayersLogs: "SanctionsPlayersLogs",

  PlayersOnline: "PlayersOnline",

  Error404: "Error404",
};

export default [
  {
    name: routes.Home,
    path: "/",
    component: HomePage,
  },

  {
    name: routes.Login,
    path: "/login",
    component: LoginPage,
    meta: {
      requiresAuth: false,
    },
  },
  {
    name: routes.Register,
    path: "/register",
    component: RegisterPage,
    meta: { requiresAuth: false },
  },
  {
    name: routes.VerifyEmail,
    path: "/verify-email",
    component: VerifyEmail,
    meta: { requiresAuth: false },
  },
  {
    name: routes.BeginResetpassword,
    path: "/begin-reset-password",
    component: BeginResetpassword,
    meta: {
      requiresAuth: false,
      redirectIfLoggedIn: false,
    },
  },
  {
    name: routes.ResetPassword,
    path: "/reset-password",
    component: ResetPassword,
    meta: {
      requiresAuth: false,
      redirectIfLoggedIn: false,
    },
  },

  {
    name: routes.Settings,
    path: "/settings",
    component: UserSettings,
  },
  {
    name: routes.SecuritySettings,
    path: "/settings/security",
    component: UserSettings,
  },
  {
    name: routes.DeleteAccountSettings,
    path: "/settings/delete-account",
    component: UserSettings,
  },

  {
    name: routes.BoutiqueLogs,
    path: "/boutique/logs",
    component: LogsBoutique,
  },
  {
    name: routes.BoutiqueLogsPb,
    path: "/boutique/logs/pb",
    component: LogsBoutiquePb,
  },

  {
    name: routes.Staffs,
    path: "/staffs",
    component: GetStaffs,
  },
  {
    name: routes.LogsStaffs,
    path: "/staffs/logs",
    component: LogsStaffs,
  },
  {
    name: routes.ServiceAccess,
    path: "/service-access",
    component: ServiceAccess,
  },
  {
    name: routes.BoutiqueGraphs,
    path: "/boutique/graphs",
    component: BoutiqueGraphs,
  },

  {
    name: routes.Sanctions,
    path: "/sanctions",
    component: GetSanctions,
  },
  {
    name: routes.SanctionsLogs,
    path: "/sanctions/logs",
    component: LogsSanctions,
  },
  {
    name: routes.SanctionsPlayersLogs,
    path: "/sanctions/logs/:player_id",
    component: LogsPlayerSanction,
  },

  {
    name: routes.PlayersOnline,
    path: "/players",
    component: GetPlayers,
  },

  {
    name: routes.Error404,
    path: "/:pathMatch(.*)*",
    component: Error404,
  },
];

export function useRoute(routeName: TemplateRoutes): string {
  return routes[routeName];
}
