use actix_web::web::ReqData;
use biscuit_auth::Biscuit;
use chrono::{DateTime, Utc};
use log::debug;
use paperclip::actix::web::{Data, Json};
use paperclip::actix::{api_v2_operation, Apiv2Schema, CreatedJson, NoContent};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::auth::iam::{authorize_only_user, Action};
use crate::utils::{openapi::OaBiscuitUserAccess, problems::MyProblem};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct Event {
    event_id: Uuid,
    event_type: String,
    event_data: Value,
    created_at: DateTime<Utc>,
    dispatched_at: Option<DateTime<Utc>>,
    status: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct IngestEventPost {
    event_type: String,
    data: Value,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct GetEventsResponse {
    events: Vec<Event>,
}

#[api_v2_operation(
    summary = "Ingest an event",
    description = "Ingest an event",
    operation_id = "events.ingest",
    consumes = "application/json",
    produces = "application/json",
    tags("Events")
)]
pub async fn ingest_event(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    body: Json<IngestEventPost>,
) -> Result<NoContent, MyProblem> {
    let body = body.into_inner();
    
    if let Ok(_token) = authorize_only_user(&biscuit, Action::EventsIngest) {
        let is_ingested = query!(
            "INSERT INTO events.event (event_type, event_data) VALUES ($1, $2) RETURNING event__id",
            body.event_type,
            body.data
        )
        .fetch_optional(&state.db)
        .await?
        .is_some();

        if is_ingested {
            Ok(NoContent)
        } else {
            Err(MyProblem::NotFound)
        }
    } else {
        Err(MyProblem::Forbidden)
    }

}

#[api_v2_operation(
    summary = "Get all events",
    description = "",
    operation_id = "events.get_all",
    produces = "application/json",
    tags("Events")
)]
pub async fn get_events(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<CreatedJson<GetEventsResponse>, MyProblem> {
    if let Ok(_token) = authorize_only_user(&biscuit, Action::EventsGetAll) {
       
        let events = query_as!(Event, "SELECT event__id AS event_id, event_type, event_data, created_at, dispatched_at, status FROM events.event WHERE status = 'pending' AND dispatched_at IS NULL ORDER BY created_at ASC")
            .fetch_all(&state.db)
            .await
            .map_err(|e| {
                debug!("{e}");
                MyProblem::InternalServerError
            })?;

        for event in &events {
            let event_id = event.event_id;
            query!("UPDATE events.event SET status = 'received', dispatched_at = NOW() WHERE event__id = $1", event_id)
                .execute(&state.db)
                .await
                .map_err(|e| {
                    debug!("{e}");
                    MyProblem::InternalServerError
                })?;
        }

        Ok(CreatedJson(GetEventsResponse { events }))
    } else {
        Err(MyProblem::Forbidden)
    }
}