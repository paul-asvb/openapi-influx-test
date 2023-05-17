use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use utoipa::ToSchema;

/// In-memory store
pub(super) type Store = Mutex<Vec<Device>>;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub(super) struct Device {
    id: i32,
    #[schema(example = "Some random String ")]
    value: String,
    done: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub(super) enum DeviceError {
    /// Device already exists conflict.
    #[schema(example = "Device already exists")]
    Conflict(String),
    /// Device not found by id.
    #[schema(example = "id = 1")]
    NotFound(String),
    /// Device operation unauthorized
    #[schema(example = "missing api key")]
    Unauthorized(String),
}

#[utoipa::path(
    get,
    path = "/devices",
    responses(
        (status = 200, description = "List all devices successfully", body = [Device])
    )
)]
pub(super) async fn list(State(store): State<Arc<Store>>) -> Json<Vec<Device>> {
    let todos = store.lock().await.clone();
    Json(todos)
}

#[utoipa::path(
    post,
    path = "/devices",
    request_body = i32,
    responses(
        (status = 201, description = "Device registered successfully", body = Device),
        (status = 409, description = "Device already exists", body = DeviceError)
    )
)]
pub(super) async fn register(
    State(store): State<Arc<Store>>,
    Json(todo): Json<Device>,
) -> impl IntoResponse {
    let mut devices = store.lock().await;

    devices
        .iter_mut()
        .find(|existing_todo| existing_todo.id == todo.id)
        .map(|found| {
            (
                StatusCode::CONFLICT,
                Json(DeviceError::Conflict(format!(
                    "todo already exists: {}",
                    found.id
                ))),
            )
                .into_response()
        })
        .unwrap_or_else(|| {
            devices.push(todo.clone());

            (StatusCode::CREATED, Json(todo)).into_response()
        })
}

/// Mark Device item done by id
///
/// Mark Device item done by given id. Return only status 200 on success or 404 if Device is not found.
#[utoipa::path(
    put,
    path = "/devices/{id}",
    responses(
        (status = 200, description = "Device marked done successfully"),
        (status = 404, description = "Device not found")
    ),
    params(
        ("id" = i32, Path, description = "Device id")
        //("metadata" = String, Path, description = "Device Metadata")
    ),
)]
pub(super) async fn change_metadata(
    Path(id): Path<i32>,
    //Parameter(metadata): Parameter<String>,
    State(store): State<Arc<Store>>,
    //headers: HeaderMap,
) -> StatusCode {
    let mut todos = store.lock().await;

    todos
        .iter_mut()
        .find(|todo| todo.id == id)
        .map(|todo| {
            todo.done = true;
            StatusCode::OK
        })
        .unwrap_or(StatusCode::NOT_FOUND)
}

/// Delete Device item by id
///
/// Delete Device item from in-memory storage by id. Returns either 200 success of 404 with DeviceError if Device is not found.
#[utoipa::path(
    delete,
    path = "/devices/{id}",
    responses(
        (status = 200, description = "Device marked done successfully"),
        (status = 401, description = "Unauthorized to delete Device", body = DeviceError, example = json!(DeviceError::Unauthorized(String::from("missing api key")))),
        (status = 404, description = "Device not found", body = DeviceError, example = json!(DeviceError::NotFound(String::from("id = 1"))))
    ),
    params(
        ("id" = i32, Path, description = "Device database id")
    ),
    security(
        ("api_key" = [])
    )
)]
pub(super) async fn delete(
    Path(id): Path<i32>,
    State(store): State<Arc<Store>>,
    //headers: HeaderMap,
) -> impl IntoResponse {
    let mut devices = store.lock().await;

    let len = devices.len();

    devices.retain(|todo| todo.id != id);

    if devices.len() != len {
        StatusCode::OK.into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(DeviceError::NotFound(format!("id = {id}"))),
        )
            .into_response()
    }
}
