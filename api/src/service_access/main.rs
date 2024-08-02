use actix_web::web::ReqData;
use biscuit_auth::Biscuit;
use chrono::{DateTime, Utc};
use log::error;
use paperclip::actix::web::{Data, Path};
use paperclip::actix::{api_v2_operation, Apiv2Schema, CreatedJson, NoContent};
use serde::Serialize;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::auth::iam::{authorize, create_service_access_token, RootToken};
use crate::{auth::iam::Action, utils::{openapi::OaBiscuitUserAccess, problems::MyProblem}};

#[derive(Debug, Serialize, Apiv2Schema)]
pub struct ServiceToken {
    token_id: Uuid,
    biscuit: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Apiv2Schema)]
pub struct ServiceTokenResponse {
    service_access: Vec<ServiceToken>,
}

#[api_v2_operation(
    summary = "Create service access",
    description = "",
    operation_id = "service-access.create-service-access",
    consumes = "application/json",
    produces = "application/json",
    tags("Service Access")
)]
pub async fn create_service_access(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<CreatedJson<ServiceToken>, MyProblem> {
    if authorize(&biscuit, Action::ServiceAccessCreateServiceAccess).is_err() {
        return Err(MyProblem::Forbidden);
    }

    let token_id = Uuid::new_v4();
    match create_service_access_token(&state.biscuit_private_key, token_id) {
            Ok(RootToken {
                serialized_biscuit,
                revocation_id,
                ..
            }) => {
                let service_token = query_as!(
                    ServiceToken,
                    r#"
                        INSERT INTO iam.token (type, revocation_id, biscuit)
                        VALUES ('service_access', $1, $2)
                        RETURNING token__id AS token_id, biscuit AS "biscuit!", created_at
                    "#,
                    revocation_id,
                    Some(serialized_biscuit),
                )
                .fetch_one(&state.db)
                .await
                .map_err(MyProblem::from)?;

                Ok(CreatedJson(service_token))
            }
            Err(e) => {
                error!("Could not create a Biscuit (service access token): {e}");
                Err(MyProblem::InternalServerError)
        }
    }
}

#[api_v2_operation(
    summary = "Delete service access",
    description = "",
    operation_id = "service-access.delete-service-access",
    consumes = "application/json",
    produces = "application/json",
    tags("Service Access")
)]
pub async fn delete_service_access(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    token_id: Path<Uuid>,
) -> Result<NoContent, MyProblem> {
    let token_id = token_id.into_inner();

    if authorize(
        &biscuit,
        Action::ServiceAccessDeleteServiceAccess
    )
    .is_err()
    {
        return Err(MyProblem::Forbidden);
    }

    let service_token = query_as!(
        ServiceToken,
        r#"
            SELECT token__id AS token_id, biscuit AS "biscuit!", created_at
            FROM iam.token
            WHERE token__id = $1
                AND type = 'service_access'
                AND (expired_at IS NULL OR expired_at > statement_timestamp())
        "#,
        &token_id,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(MyProblem::from)?;

    match service_token {
        Some(_st) => {
            query!(
                "
                    UPDATE iam.token
                    SET expired_at = statement_timestamp()
                    WHERE token__id = $1
                        AND type = 'service_access'
                ",
                &token_id,
            )
            .execute(&state.db)
            .await
            .map_err(MyProblem::from)?;

            Ok(NoContent)
        }
        None => Err(MyProblem::NotFound),
    }
}

#[api_v2_operation(
    summary = "Get service access",
    description = "",
    operation_id = "service-access.get-service-access",
    consumes = "application/json",
    produces = "application/json",
    tags("Service Access")
)]
pub async fn get_service_access(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<CreatedJson<ServiceTokenResponse>, MyProblem> {
    if authorize(
        &biscuit,
        Action::ServiceAccessGetServiceAccess
    )
    .is_err()
    {
        return Err(MyProblem::Forbidden);
    }

    let service_access = query_as!(
        ServiceToken,
        r#"
            SELECT token__id AS token_id, biscuit AS "biscuit!", created_at
            FROM iam.token
            WHERE type = 'service_access'
                AND (expired_at IS NULL OR expired_at > statement_timestamp())
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(MyProblem::from)?;

    Ok(CreatedJson(ServiceTokenResponse {
        service_access,
    }))
}