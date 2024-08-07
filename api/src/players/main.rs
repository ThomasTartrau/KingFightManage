use actix_web::web::ReqData;
use biscuit_auth::Biscuit;
use chrono::{DateTime, Utc};
use log::error;
use paperclip::actix::web::{Data, Json};
use paperclip::actix::{api_v2_operation, Apiv2Schema, NoContent};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::auth::iam::authorize;
use crate::{
    auth::iam::Action,
    utils::{openapi::OaBiscuitUserAccess, problems::MyProblem},
};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct PlayerJoinPost {
    player_id: Uuid,
    username: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct PlayerLeavePost {
    player_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct Player {
    pub player_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GetOnlinePlayersResponse {
    players: Vec<Player>,
}

#[api_v2_operation(
    summary = "Player join",
    description = "",
    operation_id = "players.join",
    consumes = "application/json",
    produces = "application/json",
    tags("Players")
)]
pub async fn player_join(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    body: Json<PlayerJoinPost>,
) -> Result<NoContent, MyProblem> {
    if authorize(&biscuit, Action::PlayersJoin).is_err() {
        return Err(MyProblem::Forbidden);
    }

    query!(
        "INSERT INTO players.player (player__id, name) VALUES ($1, $2)
         ON CONFLICT DO NOTHING",
        body.player_id,
        body.username
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to insert player: {:?}", e);
        MyProblem::InternalServerError
    })?;

    let player = query_as!(
        Player,
        "SELECT player__id as player_id, name, created_at FROM players.player WHERE player__id = $1 LIMIT 1",
        body.player_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to fetch player: {:?}", e);
        MyProblem::InternalServerError
    })?;

    if let Some(player) = player {
        query!(
            "INSERT INTO players.logged_in(player_id) VALUES ($1)",
            player.player_id
        )
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Failed to insert logged_in: {:?}", e);
            MyProblem::InternalServerError
        })?;

        Ok(NoContent)
    } else {
        Err(MyProblem::NotFound)
    }
}

#[api_v2_operation(
    summary = "Player leave",
    description = "",
    operation_id = "players.leave",
    consumes = "application/json",
    produces = "application/json",
    tags("Players")
)]
pub async fn player_leave(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    body: Json<PlayerLeavePost>,
) -> Result<NoContent, MyProblem> {
    if authorize(&biscuit, Action::PlayersLeave).is_err() {
        return Err(MyProblem::Forbidden);
    }

    query!(
        "INSERT INTO players.logged_out(player_id) VALUES ($1)",
        body.player_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to insert logged_out: {:?}", e);
        MyProblem::InternalServerError
    })?;

    Ok(NoContent)
}

pub async fn get_online_players(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<Json<GetOnlinePlayersResponse>, MyProblem> {
    if authorize(&biscuit, Action::PlayersGetOnline).is_err() {
        return Err(MyProblem::Forbidden);
    }

    let rows = query!(
        "SELECT player__id as player_id, name, created_at FROM players.get_online_players()"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to fetch online players: {:?}", e);
        MyProblem::InternalServerError
    })?;

    let mut players = Vec::new();
    for row in rows {
        players.push(Player {
            player_id: row.player_id.unwrap_or_default(),
            name: row.name.unwrap_or_default(),
            created_at: row.created_at.unwrap_or_default(),
        });
    }

    Ok(Json(GetOnlinePlayersResponse { players }))
}
