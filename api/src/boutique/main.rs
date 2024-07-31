use actix_web::web::ReqData;
use biscuit_auth::Biscuit;
use chrono::{DateTime, Utc};
use log::{debug, trace};
use paperclip::actix::web::Data;
use paperclip::actix::{api_v2_operation, Apiv2Schema, CreatedJson};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use uuid::Uuid;

use crate::{auth::iam::{authorize_only_user, Action}, utils::{openapi::OaBiscuitUserAccess, problems::MyProblem}};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct Log {
    log_id: Uuid,
    username: String,
    action: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct PbLog {
    log_id: Uuid,
    username: String,
    action: String,
    amount: i16,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GetLogsResponse {
    logs: Vec<Log>,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GetPbLogsResponse {
    logs: Vec<PbLog>,
}

#[api_v2_operation(
    summary = "Get boutique logs",
    description = "",
    operation_id = "boutique.get-logs",
    consumes = "application/json",
    produces = "application/json",
    tags("Boutique")
)]
pub async fn get_logs(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<CreatedJson<GetLogsResponse>, MyProblem> {
    if let Ok(_token) = authorize_only_user(&biscuit, Action::GetBoutiqueLogs) {
        let logs = query_as!(
            Log,
            "SELECT boutique_log__id as log_id, username, action, created_at FROM logs.boutique",
        )
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            debug!("{e}");
            MyProblem::InternalServerError
        })?;

        trace!("logs: {logs:?}");

        Ok(CreatedJson(GetLogsResponse { logs }))
    } else {
        Err(MyProblem::Forbidden)
    }
}

#[api_v2_operation(
    summary = "Get pb logs",
    description = "",
    operation_id = "boutique.get-boutique-pb-logs",
    consumes = "application/json",
    produces = "application/json",
    tags("Boutique")
)]
pub async fn get_pb_logs(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<CreatedJson<GetPbLogsResponse>, MyProblem> {
    if let Ok(_token) = authorize_only_user(&biscuit, Action::GetBoutiquePbLogs) {
        let logs = query_as!(
            PbLog,
            "SELECT pb_log__id as log_id, username, action, amount, created_at FROM logs.pb",
        )
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            debug!("{e}");
            MyProblem::InternalServerError
        })?;

        Ok(CreatedJson(GetPbLogsResponse { logs }))
    } else {
        Err(MyProblem::Forbidden)
    }
}