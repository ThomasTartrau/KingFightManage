use actix_web::web::ReqData;
use biscuit_auth::Biscuit;
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
pub struct Error {
    error_id: Uuid,
    content: String,
    plugin_name: String,
    priority: i16,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct ErrorPost {
    content: String,
    plugin_name: String,
    priority: i16,
}

#[api_v2_operation(
    summary = "Get errors",
    description = "",
    operation_id = "errors.get",
    consumes = "application/json",
    produces = "application/json",
    tags("Errors")
)]
pub async fn get_errors(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<Json<Vec<Error>>, MyProblem> {
    if authorize(&biscuit, Action::ErrorsGet).is_err() {
        return Err(MyProblem::Forbidden);
    }

    let errors = query_as!(
        Error,
        "SELECT error__id as error_id, content, plugin_name, priority FROM logs.errors"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to fetch errors: {:?}", e);
        MyProblem::InternalServerError
    })?;

    Ok(Json(errors))
}

#[api_v2_operation(
    summary = "Post error",
    description = "",
    operation_id = "errors.post",
    consumes = "application/json",
    produces = "application/json",
    tags("Errors")
)]
pub async fn post_error(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    body: Json<ErrorPost>,
) -> Result<NoContent, MyProblem> {
    if authorize(&biscuit, Action::ErrorsPost).is_err() {
        return Err(MyProblem::Forbidden);
    }

    query!(
        "INSERT INTO logs.errors (content, plugin_name, priority) VALUES ($1, $2, $3)",
        body.content,
        body.plugin_name,
        body.priority
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to insert error: {:?}", e);
        MyProblem::InternalServerError
    })?;

    Ok(NoContent)
}