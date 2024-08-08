export enum Roles {
  SUPPORT,
  MODERATEUR,
  RESPONSABLE,
  DEVELOPPEUR,
  ADMINISTRATEUR,
}

export enum Actions {
  StaffsSetRank,
  StaffsDeleteUser,
  StaffsSendMessage,
  StaffsGetLogs,
  StaffsGenerateRegistrationToken,

  ServiceAccessGenerate,
  ServiceAccessDelete,
  ServiceAccessCopy,

  SanctionsCreate,
  SanctionsUpdate,
  SanctionsDelete,

  PlayersMute,
  PlayersKick,
  PlayersBan,
  PlayersSendMessage,
  PlayersGetSanctionLogs,

  SidebarStaffs,
  SidebarLogsStaffs,
  SidebarBoutiqueLogs,
  SidebarBoutiquePbLogs,
  SidebarBoutiqueGraphs,
  SidebarServiceAccess,
  SidebarSanctions,
  SidebarSanctionsLogs,
  SidebarSanctionPlayersLogs,
  SidebarPlayers,

  SidebarCategoryAdmin,
  SidebarCategoryBoutique,
  SidebarCategorySanction,
  SidebarCategoryPlayers,
}

const HeadRoles = [Roles.ADMINISTRATEUR, Roles.DEVELOPPEUR, Roles.RESPONSABLE];
const AllRoles = [...HeadRoles, Roles.MODERATEUR, Roles.SUPPORT];

const allowed = {
  [Actions.StaffsSetRank]: HeadRoles,
  [Actions.StaffsDeleteUser]: HeadRoles,
  [Actions.StaffsSendMessage]: AllRoles,
  [Actions.StaffsGetLogs]: AllRoles,
  [Actions.StaffsGenerateRegistrationToken]: HeadRoles,

  [Actions.ServiceAccessGenerate]: [Roles.ADMINISTRATEUR],
  [Actions.ServiceAccessDelete]: [Roles.ADMINISTRATEUR],
  [Actions.ServiceAccessCopy]: [Roles.ADMINISTRATEUR],

  [Actions.SanctionsCreate]: HeadRoles,
  [Actions.SanctionsUpdate]: HeadRoles,
  [Actions.SanctionsDelete]: HeadRoles,

  [Actions.PlayersMute]: AllRoles,
  [Actions.PlayersKick]: [
    Roles.ADMINISTRATEUR,
    Roles.DEVELOPPEUR,
    Roles.RESPONSABLE,
    Roles.MODERATEUR,
  ],
  [Actions.PlayersBan]: [
    Roles.ADMINISTRATEUR,
    Roles.DEVELOPPEUR,
    Roles.RESPONSABLE,
    Roles.MODERATEUR,
  ],
  [Actions.PlayersSendMessage]: AllRoles,
  [Actions.PlayersGetSanctionLogs]: AllRoles,

  [Actions.SidebarStaffs]: AllRoles,
  [Actions.SidebarLogsStaffs]: AllRoles,
  [Actions.SidebarBoutiqueLogs]: [Roles.ADMINISTRATEUR],
  [Actions.SidebarBoutiquePbLogs]: [Roles.ADMINISTRATEUR],
  [Actions.SidebarBoutiqueGraphs]: [Roles.ADMINISTRATEUR],
  [Actions.SidebarServiceAccess]: [Roles.ADMINISTRATEUR],
  [Actions.SidebarSanctions]: AllRoles,
  [Actions.SidebarSanctionsLogs]: AllRoles,
  [Actions.SidebarSanctionPlayersLogs]: AllRoles,
  [Actions.SidebarPlayers]: AllRoles,

  [Actions.SidebarCategoryAdmin]: AllRoles,
  [Actions.SidebarCategoryBoutique]: [Roles.ADMINISTRATEUR],
  [Actions.SidebarCategorySanction]: AllRoles,
  [Actions.SidebarCategoryPlayers]: AllRoles,
};

function getRole(roleName: string): Roles {
  return Roles[roleName.toUpperCase() as keyof typeof Roles];
}

function hasPermission(role: Roles | null, action: Actions) {
  if (role === null || role === undefined) return false;
  return allowed[action].includes(role);
}

function getStringRoles() {
  return Object.keys(Roles).filter((key) => Number.isNaN(Number(key)));
}

export default {
  getRole,
  hasPermission,
  getStringRoles,
};
