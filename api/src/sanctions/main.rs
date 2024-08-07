use actix_web::web::ReqData;
use biscuit_auth::Biscuit;
use chrono::{DateTime, Utc};
use log::{error, trace};
use paperclip::actix::web::{Data, Path, Json};
use paperclip::actix::{api_v2_operation, Apiv2Schema, CreatedJson, NoContent};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::auth::iam::{authorize, create_service_access_token, RootToken};
use crate::{auth::iam::Action, utils::{openapi::OaBiscuitUserAccess, problems::MyProblem}};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct CreateSanctionPost {
    type_: String,
    name: String,
    duration: i64,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct UpdateSanctionPost {
    type_: String,
    name: String,
    duration: i64,
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
    duration: i64,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GetSanctions {
    sanctions: Vec<Sanction>,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GetPlayerSanctions {
    player_id: Uuid,
    player_name: String,
    staff_name: String,
    sanction_name: String,
    sanction_type: String,
    sanction_motif: String,
    sanction_duration: i64,
    sanction_created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GetSanctionsLogs {
    player_id: Uuid,
    player_name: String,
    staff_name: String,
    sanction_name: String,
    sanction_type: String,
    sanction_motif: String,
    sanction_duration: i64,
    sanction_created_at: DateTime<Utc>,
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
        "INSERT INTO sanctions.sanction(type, name, duration) VALUES ($1, $2, $3)",
        body.type_,
        body.name,
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
        "UPDATE sanctions.sanction SET type = $1, name = $2, duration = $3 WHERE sanction__id = $4",
        body.type_,
        body.name,
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
    sanction_id: Path<Uuid>,
) -> Result<NoContent, MyProblem> {
    if authorize(&biscuit, Action::SanctionsDelete).is_err() {
        return Err(MyProblem::Forbidden);
    }

    let sanction_id = sanction_id.into_inner();

    query!(
        "DELETE FROM sanctions.sanction WHERE sanction__id = $1",
        sanction_id
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
) -> Result<CreatedJson<GetSanctions>, MyProblem> {
    if authorize(&biscuit, Action::SanctionsGets).is_err() {
        return Err(MyProblem::Forbidden);
    }

    let sanctions = query_as!(
        Sanction,
        "SELECT sanction__id as sanction_id, type as type_, name, duration, created_at FROM sanctions.sanction"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to get sanctions: {:?}", e);
        MyProblem::InternalServerError
    })?;

    Ok(CreatedJson(GetSanctions { sanctions }))
}

#[api_v2_operation(
    summary = "Get player sanctions",
    description = "",
    operation_id = "sanctions.get-player-sanction",
    produces = "application/json",
    tags("Sanctions")
)]
pub async fn get_player_sanctions(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    player_id: Path<Uuid>,
) -> Result<CreatedJson<Vec<GetPlayerSanctions>>, MyProblem> {
    if authorize(&biscuit, Action::SanctionsGetPlayerSanction).is_err() {
        return Err(MyProblem::Forbidden);
    }

    let player_id = player_id.into_inner();

    let sanctions = query_as!(
        GetPlayerSanctions,
        r#"
        SELECT
            p.player__id as player_id,
            p.name AS player_name,
            ls.staff_name as staff_name,
            ls.sanction_motif as sanction_motif,
            s.name AS sanction_name,
            s.type AS sanction_type,
            s.duration AS sanction_duration,
            s.created_at AS sanction_created_at
        FROM
            logs.sanction ls
        JOIN
            sanctions.sanction s ON s.sanction__id = ls.sanction__id
        JOIN
            players.player p ON p.player__id = ls.player__id
        WHERE
            p.player__id = $1
        "#,
        player_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to get player sanctions: {:?}", e);
        MyProblem::InternalServerError
    })?;

    Ok(CreatedJson(sanctions))
}

#[api_v2_operation(
    summary = "Get sanctions logs",
    description = "",
    operation_id = "sanctions.get-logs",
    produces = "application/json",
    tags("Sanctions")
)]
pub async fn get_sanctions_logs(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<CreatedJson<Vec<GetSanctionsLogs>>, MyProblem> {
    if authorize(&biscuit, Action::SanctionsGetLogs).is_err() {
        return Err(MyProblem::Forbidden);
    }

    let sanctions = query_as!(
        GetSanctionsLogs,
        r#"
        SELECT
            p.player__id as player_id,
            p.name AS player_name,
            ls.staff_name as staff_name,
            ls.sanction_motif as sanction_motif,
            s.name AS sanction_name,
            s.type AS sanction_type,
            s.duration AS sanction_duration,
            s.created_at AS sanction_created_at
        FROM
            logs.sanction ls
        JOIN
            sanctions.sanction s ON s.sanction__id = ls.sanction__id
        JOIN
            players.player p ON p.player__id = ls.player__id
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to get sanctions logs: {:?}", e);
        MyProblem::InternalServerError
    })?;

    Ok(CreatedJson(sanctions))
}