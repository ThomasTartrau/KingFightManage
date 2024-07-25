use actix_web::web::ReqData;
use biscuit_auth::Biscuit;
use log::{debug};
use paperclip::actix::web::{Data, Json, Path};
use paperclip::actix::{api_v2_operation, Apiv2Schema, CreatedJson, NoContent};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query, query_as};
use uuid::Uuid;
use validator::Validate;

use crate::{auth::auth::UserLookup, auth::iam::{authorize_only_user, create_registration_token, Action, Role}, utils::{openapi::OaBiscuitUserAccess, problems::MyProblem}};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GenerateRegistrationTokenResponse {
    registration_token: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
struct User {
    user_id: Uuid,
    username: String,
    role: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GetUsersResponse {
    users: Vec<User>,
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

#[derive(Debug, Serialize, Deserialize, Apiv2Schema, Validate)]
pub struct SendMessagePost {
    user_id: Uuid,
    #[validate(non_control_character, length(min = 1, max = 1000))]
    message: String,
    #[validate(non_control_character, length(min = 1, max = 100))]
    username: String,
}

#[api_v2_operation(
    summary = "Generate a new registration token",
    description = "",
    operation_id = "users-management.generate-registration-token",
    consumes = "application/json",
    produces = "application/json",
    tags("Users Management")
)]
pub async fn generate_registration_token(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<CreatedJson<GenerateRegistrationTokenResponse>, MyProblem> {

    if let Ok(_token) = authorize_only_user(&biscuit, Action::UsersGenerateRegistrationToken) {
        let token = create_registration_token(&state.biscuit_private_key).map_err(|e| {
            debug!("{e}");
            MyProblem::Forbidden
        })?;
    
        Ok(CreatedJson(GenerateRegistrationTokenResponse {
            registration_token: token.serialized_biscuit,
        }))
    } else {
        Err(MyProblem::Forbidden)
    }
}

#[api_v2_operation(
    summary = "Get users",
    description = "",
    operation_id = "users-management.get-users",
    consumes = "application/json",
    produces = "application/json",
    tags("Users Management")
)]
pub async fn get_users(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<CreatedJson<GetUsersResponse>, MyProblem> {

    if let Ok(_token) = authorize_only_user(&biscuit, Action::UsersGetUsers) {
        let users = query_as!(
            User,
            "SELECT user__id as user_id, username, role FROM iam.user"
        )
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            debug!("{e}");
            MyProblem::InternalServerError
        })?;

        Ok(CreatedJson(GetUsersResponse { users }))
    } else {
        Err(MyProblem::Forbidden)
    }
}

#[api_v2_operation(
    summary = "Set role",
    description = "",
    operation_id = "users-management.set-role",
    consumes = "application/json",
    produces = "application/json",
    tags("Users Management")
)]
pub async fn set_role(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    body: Json<SetRolePost>,
) -> Result<NoContent, MyProblem> {

    let body = body.into_inner();
    
    if let Ok(token) = authorize_only_user(&biscuit, Action::UsersSetRank) {

        let roles = Role::values();
        if !roles.contains(&body.role) {
            return Err(MyProblem::BadRequest);
        }

        if Role::to_role(&body.role).get_order() >= Role::to_role(&token.role).get_order() {
            return Err(MyProblem::Forbidden);
        }
    

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
        .fetch_optional(&state.db)
        .await?
        .is_some();

        if set_role {
            Ok(NoContent)
        } else {
            Err(MyProblem::InternalServerError)
        }
    } else {
        Err(MyProblem::Forbidden)
    }
}

#[api_v2_operation(
    summary = "Delete user",
    description = "",
    operation_id = "users-management.delete-user",
    consumes = "application/json",
    produces = "application/json",
    tags("Users Management")
)]
pub async fn delete_user(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    user_id: Path<Uuid>,
) -> Result<NoContent, MyProblem> {
    let user_id = user_id.into_inner();

    if let Ok(token) = authorize_only_user(&biscuit, Action::UsersDeleteUser) {
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
        .map_err(MyProblem::from)?;

        if let Some(user) = user_lookup {
            if Role::to_role(&user.role).get_order() >= Role::to_role(&token.role).get_order() {
                return Err(MyProblem::Forbidden);
            }

            let delete_user = query!(
                "DELETE FROM iam.user WHERE user__id = $1",
                &user_id,
            )
            .execute(&state.db)
            .await
            .map_err(MyProblem::from)?;

            if delete_user.rows_affected() > 0 {
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
    summary = "Send message",
    description = "",
    operation_id = "users-management.send-message",
    consumes = "application/json",
    produces = "application/json",
    tags("Users Management")
)]
pub async fn send_message(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    body: Json<SendMessagePost>,
) -> Result<NoContent, MyProblem> {
    if let Err(e) = body.validate() {
        return Err(MyProblem::Validation(e));
    }

    let body = body.into_inner();

    if let Ok(token) = authorize_only_user(&biscuit, Action::UsersSendMessage) {
        let event_type = "send_message";
            let event_data = json!({
                "sender": &token.username,
                "username": body.username,
                "message": body.message,
            });

            let result = query!("INSERT INTO events.event (event_type, event_data) VALUES ($1, $2)", event_type, event_data)
                .execute(&state.db)
                .await
                .map_err(MyProblem::from)?;

            if result.rows_affected() > 0 {
                Ok(NoContent)
            } else {
                Err(MyProblem::InternalServerError)
            }
    } else {
        Err(MyProblem::Forbidden)
    }
}
