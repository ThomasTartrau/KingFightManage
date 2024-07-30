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
}

const allowed = {
  [Roles.SUPPORT]: [Actions.StaffsSendMessage, Actions.StaffsGetLogs],
  [Roles.MODERATEUR]: [Actions.StaffsSendMessage, Actions.StaffsGetLogs],
  [Roles.RESPONSABLE]: [Actions.StaffsSendMessage, Actions.StaffsGetLogs],
  [Roles.DEVELOPPEUR]: [
    Actions.StaffsSendMessage,
    Actions.StaffsGetLogs,
    Actions.StaffsSetRank,
  ],
  [Roles.ADMINISTRATEUR]: [
    Actions.StaffsSetRank,
    Actions.StaffsDeleteUser,
    Actions.StaffsSendMessage,
    Actions.StaffsGetLogs,
  ],
}

function getRole(roleName: string): Roles {
  return Roles[roleName.toUpperCase() as keyof typeof Roles]
}

function hasPermission(role: Roles | null, action: Actions) {
  if (role === null || role === undefined)
    return false
  return allowed[role].includes(action)
}

function getStringRoles() {
  return Object.keys(Roles).filter(key => Number.isNaN(Number(key)))
}

export default {
  getRole,
  hasPermission,
  getStringRoles,
}
