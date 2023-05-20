use std::sync::Arc;

use axum::{
    extract::{self, Path, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::debug;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    measurement::{self, MoistureMeasurement},
    quest,
};

/// In-memory store
pub(super) type Store = Mutex<Vec<Device>>;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub(super) struct Device {
    id: Uuid,
    #[schema(example = "json with any metadata")]
    metadata: Option<serde_json::Value>,
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

/// List all registered devices
#[utoipa::path(
    get,
    path = "/devices",
    responses(
        (status = 200, description = "List all devices successfully", body = [Device])
    )
)]
pub(super) async fn list(State(store): State<Arc<Store>>) -> Json<Vec<Device>> {
    let devices = store.lock().await.clone();
    Json(devices)
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct DeviceRegister {
    id: Uuid,
}

/// Register a Device
#[utoipa::path(
    post,
    path = "/device",
    request_body = DeviceRegister,
    responses(
        (status = 201, description = "Device registered successfully", body = Device),
        (status = 409, description = "Device already exists", body = DeviceError)
    )
)]
pub(super) async fn register(
    State(store): State<Arc<Store>>,
    Json(payload): extract::Json<DeviceRegister>,
) -> impl IntoResponse {
    let mut devices = store.lock().await;

    let id = payload.id;

    devices
        .iter_mut()
        .find(|existing_device| existing_device.id == id)
        .map(|found| {
            (
                StatusCode::CONFLICT,
                Json(DeviceError::Conflict(format!(
                    "device already exists: {}",
                    found.id
                ))),
            )
                .into_response()
        })
        .unwrap_or_else(|| {
            let device = Device { id, metadata: None };
            devices.push(device.clone());

            (StatusCode::CREATED, Json(device)).into_response()
        })
}

/// Update Device metadata
#[utoipa::path(
    put,
    path = "/devices/{id}",
    responses(
        (status = 200, description = "Device metadata updated successfully"),
        (status = 404, description = "Device not found")
    ),
    request_body = serde_json::Value,
    params(
        ("id" = Uuid, Path, description = "Device id"),
    ),
)]
pub(super) async fn change_metadata(
    Path(id): Path<Uuid>,
    State(store): State<Arc<Store>>,
    Json(payload): extract::Json<String>,
) -> StatusCode {
    debug!("---->ohkjh");

    let metadata: serde_json::Value = serde_json::from_str(&payload).unwrap();

    let mut devices = store.lock().await;

    debug!("{}", &payload);

    devices
        .iter_mut()
        .find(|d| d.id == id)
        .map(|d| {
            d.metadata = Some(metadata);
            StatusCode::OK
        })
        .unwrap_or(StatusCode::NOT_FOUND)
}

/// Update Device metadata
#[utoipa::path(
    put,
    path = "/devices/{id}/write",
    responses(
        (status = 200, description = "Device data written successfully"),
        (status = 404, description = "Device not found")
    ),
    request_body = serde_json::Value,
    params(
        ("id" = Uuid, Path, description = "Device id"),
    ),
)]
pub(super) async fn write_data(
    Path(id): Path<Uuid>,
    State(store): State<Arc<Store>>,
    Json(measurement): extract::Json<MoistureMeasurement>,
) -> StatusCode {
    let mut devices = store.lock().await;

    if let Some(device) = devices.iter_mut().find(|d| d.id == id) {
        match quest::write(device.id, measurement) {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    } else {
        return StatusCode::NOT_FOUND;
    }
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
        ("id" = Uuid, Path, description = "Device database id")
    ),
)]
pub(super) async fn delete(
    Path(id): Path<Uuid>,
    State(store): State<Arc<Store>>,
    //headers: HeaderMap,
) -> impl IntoResponse {
    let mut devices = store.lock().await;

    let len = devices.len();

    devices.retain(|d| d.id != id);

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
