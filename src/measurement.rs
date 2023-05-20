use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, Default)]
pub struct MoistureMeasurement {
    pub datetime: DateTime<Utc>,
    pub temperature: f32,
    pub dryness_index: f32,
}
