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
      first_name: string;
      last_name: string;
      refresh_token: string;
      /** Format: date-time */
      refresh_token_expiration: string;
    };
    GetStaffsResponse: {
      users: [
        {
          user_id: UUID;
          first_name: string;
          last_name: string;
          role: string;
        },
      ];
    };
    User: {
      user_id: UUID;
      first_name: string;
      last_name: string;
      role: string;
    };
  };
}
