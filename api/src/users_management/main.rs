use actix_web::web::Data;
use log::debug;
use paperclip::actix::{api_v2_operation, Apiv2Schema, CreatedJson};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use uuid::Uuid;

use crate::{auth::iam::create_registration_token, utils::problems::MyProblem};

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
) -> Result<CreatedJson<GenerateRegistrationTokenResponse>, MyProblem> {

    let token = create_registration_token(&state.biscuit_private_key).map_err(|e| {
        debug!("{e}");
        MyProblem::Forbidden
    })?;

    Ok(CreatedJson(GenerateRegistrationTokenResponse {
        registration_token: token.serialized_biscuit,
    }))
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
) -> Result<CreatedJson<GetUsersResponse>, MyProblem> {
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
}