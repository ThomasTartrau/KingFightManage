use actix_multipart::form::json::Json;
use actix_web::web::Data;
use log::debug;
use paperclip::actix::{api_v2_operation, Apiv2Schema, CreatedJson};
use serde::{Deserialize, Serialize};

use crate::{auth::iam::create_registration_token, utils::problems::MyProblem};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GenerateRegistrationTokenResponse {
    registration_token: String,
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