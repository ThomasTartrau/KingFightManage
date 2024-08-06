use std::time::Duration;

use actix_web::web::ReqData;
use biscuit_auth::Biscuit;
use chrono::{DateTime, Utc};
use log::{error, trace};
use paperclip::actix::web::{Data, Path, Json};
use paperclip::actix::{api_v2_operation, Apiv2Schema, CreatedJson, NoContent};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::auth;
use crate::auth::iam::{authorize, create_service_access_token, RootToken};
use crate::{auth::iam::Action, utils::{openapi::OaBiscuitUserAccess, problems::MyProblem}};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct CreateSanctionPost {
    type_: String,
    name: String,
    motif: String,
    duration: i64,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct UpdateSanctionPost {
    type_: Option<String>,
    name: Option<String>,
    motif: Option<String>,
    duration: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct DeleteSanctionPost {
    sanction_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct Sanction {
    sanction_id: Uuid,
    type_: String,
    name: String,
    motif: String,
    duration: i64,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GetSanctins {
    sanctions: Vec<Sanction>,
}

#[api_v2_operation(
    summary = "Create a sanction",
    description = "",
    operation_id = "sanctions.create",
    consumes = "application/json",
    produces = "application/json",
    tags("Sanctions")
)]
pub async fn create_sanction(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    body: Json<CreateSanctionPost>,
) -> Result<NoContent, MyProblem> {
    if authorize(&biscuit, Action::SanctionsCreate).is_err() {
        return Err(MyProblem::Forbidden);
    }

    query!(
        "INSERT INTO sanctions.sanction(type, name, motif, duration) VALUES ($1, $2, $3, $4)",
        body.type_,
        body.name,
        body.motif,
        body.duration
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to insert sanction: {:?}", e);
        MyProblem::InternalServerError
    })?;

    Ok(NoContent)
}

#[api_v2_operation(
    summary = "Update a sanction",
    description = "",
    operation_id = "sanctions.update",
    consumes = "application/json",
    produces = "application/json",
    tags("Sanctions")
)]
pub async fn update_sanction(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    body: Json<UpdateSanctionPost>,
    sanction_id: Path<Uuid>,
) -> Result<NoContent, MyProblem> {
    if authorize(&biscuit, Action::SanctionsUpdate).is_err() {
        return Err(MyProblem::Forbidden);
    }

    let sanction_id = sanction_id.into_inner();

    query!(
        "UPDATE sanctions.sanction SET type = $1, name = $2, motif = $3, duration = $4 WHERE sanction__id = $5",
        body.type_,
        body.name,
        body.motif,
        body.duration,
        sanction_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to update sanction: {:?}", e);
        MyProblem::InternalServerError
    })?;

    Ok(NoContent)
}

#[api_v2_operation(
    summary = "Delete a sanction",
    description = "",
    operation_id = "sanctions.delete",
    consumes = "application/json",
    produces = "application/json",
    tags("Sanctions")
)]
pub async fn delete_sanction(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    body: Json<DeleteSanctionPost>,
) -> Result<NoContent, MyProblem> {
    if authorize(&biscuit, Action::SanctionsDelete).is_err() {
        return Err(MyProblem::Forbidden);
    }

    query!(
        "DELETE FROM sanctions.sanction WHERE sanction__id = $1",
        body.sanction_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to delete sanction: {:?}", e);
        MyProblem::InternalServerError
    })?;

    Ok(NoContent)
}

#[api_v2_operation(
    summary = "Get sanctions",
    description = "",
    operation_id = "sanctions.gets",
    produces = "application/json",
    tags("Sanctions")
)]
pub async fn get_sanctions(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<Json<GetSanctins>, MyProblem> {
    if authorize(&biscuit, Action::SanctionsGets).is_err() {
        return Err(MyProblem::Forbidden);
    }

    let sanctions = query_as!(
        Sanction,
        "SELECT sanction__id as sanction_id, type as type_, name, motif, duration, created_at FROM sanctions.sanction"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to get sanctions: {:?}", e);
        MyProblem::InternalServerError
    })?;

    Ok(Json(GetSanctins { sanctions }))
}