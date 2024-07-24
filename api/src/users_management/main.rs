use actix_web::web::{Data, ReqData};
use biscuit_auth::Biscuit;
use log::debug;
use paperclip::actix::{api_v2_operation, Apiv2Schema, CreatedJson};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use uuid::Uuid;

use crate::{auth::iam::{authorize_only_user, create_registration_token, Action}, utils::{openapi::OaBiscuitUserAccess, problems::MyProblem}};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GenerateRegistrationTokenResponse {
    registration_token: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
struct User {
    user_id: Uuid,
    first_name: String,
    last_name: String,
    role: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GetUsersResponse {
    users: Vec<User>,
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
            "SELECT user__id as user_id, first_name, last_name, role FROM iam.user"
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