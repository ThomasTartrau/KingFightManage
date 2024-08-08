import type { UUID } from "@/http";

export interface components {
  schemas: {
    LoginPost: {
      email: string;
      password: string;
    };
    Problem: {
      detail: string;
      id: string;
      /** Format: int32 */
      status: number;
      title: string;
    };
    LoginResponse: {
      access_token: string;
      /** Format: date-time */
      access_token_expiration: string;
      user_id: UUID;
      email: string;
      username: string;
      role: string;
      refresh_token: string;
      /** Format: date-time */
      refresh_token_expiration: string;
    };
    GetStaffsResponse: {
      users: [
        {
          user_id: UUID;
          username: string;
          role: string;
          is_online: boolean;
        },
      ];
    };
    User: {
      user_id: UUID;
      username: string;
      role: string;
      is_online: boolean;
    };
    Log: {
      log_id: UUID;
      username: string;
      action: string;
      /** Format: date-time */
      created_at: string;
    };
    PbLog: {
      log_id: UUID;
      username: string;
      action: string;
      amount: number;
      /** Format: date-time */
      created_at: string;
    };
    GetStaffLogsResponse: {
      logs: [
        {
          log_id: UUID;
          username: string;
          action: string;
          /** Format: date-time */
          created_at: string;
        },
      ];
    };
    BoutiqueGetLogsResponse: {
      logs: [
        {
          log_id: UUID;
          username: string;
          action: string;
          /** Format: date-time */
          created_at: string;
        },
      ];
    };
    GetPbLogsResponse: {
      logs: [
        {
          log_id: UUID;
          username: string;
          action: string;
          amount: number;
          /** Format: date-time */
          created_at: string;
        },
      ];
    };
    ServiceAccess: {
      token_id: UUID;
      biscuit: string;
      /** Format: date-time */
      created_at: string;
    };
    GetServiceAccessResponse: {
      service_access: [
        {
          token_id: UUID;
          biscuit: string;
          /** Format: date-time */
          created_at: string;
        },
      ];
    };
    CreateSanctionPost: {
      type_: string;
      name: string;
      duration: number;
    };

    UpdateSanctionPost: {
      type_: string;
      name: string;
      duration: number;
    };

    DeleteSanctionPost: {
      sanction_id: UUID;
    };

    Sanction: {
      sanction_id: UUID;
      type_: string;
      name: string;
      duration: number;
      /** Format: date-time */
      created_at: string;
    };

    GetSanctions: {
      sanctions: [
        {
          sanction_id: UUID;
          type_: string;
          name: string;
          duration: number;
          /** Format: date-time */
          created_at: string;
        },
      ];
    };

    GetPlayerSanctions: {
      player_id: UUID;
      player_name: string;
      staff_name: string;
      sanction_name: string;
      sanction_type: string;
      sanction_motif: string;
      sanction_duration: number;
      /** Format: date-time */
      sanction_created_at: string;
    };

    GetSanctionsLogs: {
      player_id: UUID;
      player_name: string;
      staff_name: string;
      sanction_name: string;
      sanction_type: string;
      sanction_motif: string;
      sanction_duration: number;
      /** Format: date-time */
      sanction_created_at: string;
    };

    Player: {
      player_id: UUID;
      name: string;
      /** Format: date-time */
      created_at: string;
    };

    GetOnlinePlayersResponse: {
      players: [
        {
          player_id: UUID;
          name: string;
          /** Format: date-time */
          created_at: string;
        },
      ];
    };
  };
}
