use actix_web::web::ReqData;
use biscuit_auth::Biscuit;
use chrono::{DateTime, Utc};
use log::debug;
use paperclip::actix::web::{Data, Json, Path};
use paperclip::actix::{api_v2_operation, Apiv2Schema, CreatedJson, NoContent};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::auth::iam::{authorize, AuthorizedToken};
use crate::{
    auth::auth::UserLookup,
    auth::iam::{create_registration_token, Action, Role},
    utils::{openapi::OaBiscuitUserAccess, problems::MyProblem},
};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GenerateRegistrationTokenResponse {
    registration_token: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
struct User {
    user_id: Uuid,
    username: String,
    role: String,
    is_online: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GetUsersResponse {
    users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct Log {
    log_id: Uuid,
    username: String,
    action: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GetLogsResponse {
    logs: Vec<Log>,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct SetRolePost {
    user_id: Uuid,
    role: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct DeleteUserPost {
    user_id: Uuid,
}

#[api_v2_operation(
    summary = "Generate a new registration token",
    description = "",
    operation_id = "staffs.generate-registration-token",
    consumes = "application/json",
    produces = "application/json",
    tags("Staffs")
)]
pub async fn generate_registration_token(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<CreatedJson<GenerateRegistrationTokenResponse>, MyProblem> {
    if authorize(&biscuit, Action::StaffsGenerateRegistrationToken).is_err() {
        Err(MyProblem::Forbidden)
    } else {
        let token = create_registration_token(&state.biscuit_private_key).map_err(|e| {
            debug!("{e}");
            MyProblem::Forbidden
        })?;

        Ok(CreatedJson(GenerateRegistrationTokenResponse {
            registration_token: token.serialized_biscuit,
        }))
    }
}

#[api_v2_operation(
    summary = "Get staffs",
    description = "",
    operation_id = "staffs.get-users",
    consumes = "application/json",
    produces = "application/json",
    tags("Staffs")
)]
pub async fn get_staffs(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<CreatedJson<GetUsersResponse>, MyProblem> {
    if authorize(&biscuit, Action::StaffsGetUsers).is_err() {
        Err(MyProblem::Forbidden)
    } else {
        let users = query_as!(
            User,
            "SELECT players.player.player__id as user_id, username, role, players.is_logged_in(players.player.player__id) as is_online
            FROM iam.user
            JOIN players.player ON iam.user.username = players.player.name;"
        )
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            debug!("{e}");
            MyProblem::InternalServerError
        })?;

        Ok(CreatedJson(GetUsersResponse { users }))
    }
}

#[api_v2_operation(
    summary = "Set role",
    description = "",
    operation_id = "staffs.set-role",
    consumes = "application/json",
    produces = "application/json",
    tags("Staffs")
)]
pub async fn set_role(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    body: Json<SetRolePost>,
) -> Result<NoContent, MyProblem> {
    let body = body.into_inner();

    if let Ok(token) = authorize(&biscuit, Action::StaffsSetRank) {
        if let AuthorizedToken::User(user) = token {
            let roles = Role::values();
            if !roles.contains(&body.role) {
                return Err(MyProblem::BadRequest);
            }

            if Role::to_role(&body.role).get_order() >= Role::to_role(&user.role).get_order() {
                return Err(MyProblem::Forbidden);
            }
        }

        let mut tx = state.db.begin().await?;

        let set_role = query!(
            "
                UPDATE iam.user
                SET role = $1
                WHERE user__id = $2
                RETURNING user__id
            ",
            &body.role,
            &body.user_id,
        )
        .fetch_optional(&mut *tx)
        .await?
        .is_some();

        let tokens_revoked = query!(
            "
                UPDATE iam.token
                SET revoked_at = statement_timestamp()
                WHERE user__id = $1 
                AND revoked_at IS NULL
                AND (expired_at IS NULL OR expired_at > statement_timestamp())
            ",
            &body.user_id,
        )
        .execute(&mut *tx)
        .await?
        .rows_affected()
            > 0;

        if set_role && tokens_revoked {
            tx.commit().await?;
            Ok(NoContent)
        } else {
            tx.rollback().await?;
            Err(MyProblem::InternalServerError)
        }
    } else {
        Err(MyProblem::Forbidden)
    }
}

#[api_v2_operation(
    summary = "Delete user",
    description = "",
    operation_id = "staffs.delete-user",
    consumes = "application/json",
    produces = "application/json",
    tags("Staffs")
)]
pub async fn delete_user(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    user_id: Path<Uuid>,
) -> Result<NoContent, MyProblem> {
    let user_id = user_id.into_inner();

    if let Ok(token) = authorize(&biscuit, Action::StaffsDeleteUser) {
        let user_lookup = query_as!(
            UserLookup,
            "
                SELECT user__id AS user_id, email, username, role, email_verified_at, password AS password_hash
                FROM iam.user
                WHERE user__id = $1
            ",
            &user_id,
        )
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            debug!("{e}");
            MyProblem::InternalServerError
        })?;

        if let Some(user) = user_lookup {
            if let AuthorizedToken::User(token_user) = token {
                if Role::to_role(&user.role).get_order()
                    >= Role::to_role(&token_user.role).get_order()
                {
                    return Err(MyProblem::Forbidden);
                }
            }

            let delete_user = query!("DELETE FROM iam.user WHERE user__id = $1", &user_id,)
                .execute(&state.db)
                .await
                .map_err(|e| {
                    debug!("{e}");
                    MyProblem::InternalServerError
                })?
                .rows_affected()
                > 0;

            let revoke_tokens = query!(
                "
                    UPDATE iam.token
                    SET revoked_at = statement_timestamp()
                    WHERE user__id = $1
                    AND revoked_at IS NULL
                    AND (expired_at IS NULL OR expired_at > statement_timestamp())
                ",
                &user_id,
            )
            .execute(&state.db)
            .await?
            .rows_affected()
                > 0;

            if delete_user && revoke_tokens {
                Ok(NoContent)
            } else {
                Err(MyProblem::NotFound)
            }
        } else {
            return Err(MyProblem::NotFound);
        }
    } else {
        Err(MyProblem::Forbidden)
    }
}

#[api_v2_operation(
    summary = "Get logs",
    description = "",
    operation_id = "staffs.get-logs",
    consumes = "application/json",
    produces = "application/json",
    tags("Staffs")
)]
pub async fn get_logs(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<CreatedJson<GetLogsResponse>, MyProblem> {
    if authorize(&biscuit, Action::StaffsGetLogs).is_err() {
        Err(MyProblem::Forbidden)
    } else {
        let logs = query_as!(
            Log,
            "SELECT staff_log__id as log_id, username, action, created_at FROM logs.staffs",
        )
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            debug!("{e}");
            MyProblem::InternalServerError
        })?;

        Ok(CreatedJson(GetLogsResponse { logs }))
    }
}
