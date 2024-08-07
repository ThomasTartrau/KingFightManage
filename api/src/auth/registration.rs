use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use biscuit_auth::Biscuit;
use lettre::message::Mailbox;
use lettre::Address;
use log::{debug, error, warn};
use paperclip::actix::web::{Data, Json};
use paperclip::actix::{api_v2_operation, Apiv2Schema, CreatedJson};
use serde::{Deserialize, Serialize};
use sqlx::query;
use std::str::FromStr;
use uuid::Uuid;
use validator::Validate;

use crate::auth::iam::create_email_verification_token;
use crate::utils::mailer::Mail;
use crate::utils::problems::MyProblem;

use super::iam::authorize_registration;

#[derive(Debug, Serialize, Apiv2Schema)]
pub struct Registration {
    user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema, Validate)]
pub struct RegistrationPost {
    #[validate(non_control_character, length(min = 1, max = 50))]
    username: String,
    #[validate(non_control_character, email, length(max = 100))]
    email: String,
    #[validate(non_control_character, length(min = 10, max = 100))]
    password: String,
    #[validate(non_control_character, length(min = 1, max = 1000))]
    registration_token: String,
}

#[api_v2_operation(
    summary = "Create a new user account",
    description = "",
    operation_id = "register",
    consumes = "application/json",
    produces = "application/json",
    tags("Organizations Management")
)]
pub async fn register(
    state: Data<crate::State>,
    body: Json<RegistrationPost>,
) -> Result<CreatedJson<Registration>, MyProblem> {
    if let Err(e) = body.validate() {
        return Err(MyProblem::Validation(e));
    }

    let body = body.into_inner();

    let token = Biscuit::from_base64(body.registration_token, state.biscuit_private_key.public())
        .map_err(|e| {
        debug!("{e}");
        MyProblem::Forbidden
    })?;

    authorize_registration(&token).map_err(|e| {
        debug!("{e}");
        MyProblem::Forbidden
    })?;

    let recipient_address = Address::from_str(&body.email).map_err(|e| {
        // Should not happen because we checked (using a validator) that body.email is a well structured email address
        error!("Error trying to parse email address: {e}");
        MyProblem::InternalServerError
    })?;

    if body.password.len() >= usize::from(state.password_minimum_length) {
        let mut tx = state.db.begin().await?;

        let user_id = Uuid::new_v4();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(body.password.as_bytes(), &salt)
            .map_err(|e| {
                error!("Error trying to hash user password: {e}");
                MyProblem::InternalServerError
            })?
            .serialize();
        query!(
            "
                INSERT INTO iam.user (user__id, email, password, username, role)
                VALUES ($1, $2, $3, $4, $5)
            ",
            &user_id,
            &body.email,
            password_hash.as_str(),
            &body.username,
            state.default_role
        )
        .execute(&mut *tx)
        .await?;

        let verification_token =
            create_email_verification_token(&state.biscuit_private_key, user_id).map_err(|e| {
                error!("Error trying to create email verification token: {e}");
                MyProblem::InternalServerError
            })?;
        let recipient = Mailbox::new(Some(format!("{}", body.username)), recipient_address);
        state
            .mailer
            .send_mail(
                Mail::VerifyUserEmail {
                    url: format!(
                        "{}verify-email?token={}",
                        state.app_url, &verification_token.serialized_biscuit
                    ),
                },
                recipient,
            )
            .await
            .map_err(|e| {
                warn!("Could not send verification email: {e}");
                e
            })?;

        tx.commit().await?;

        Ok(CreatedJson(Registration { user_id }))
    } else {
        Err(MyProblem::PasswordTooShort(state.password_minimum_length))
    }
}
