use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, Default)]
pub struct MoistureMeasurement {
    #[serde(with = "chrono::serde::ts_seconds")]
    #[schema(example = 1684599720)]
    pub datetime: DateTime<Utc>,
    #[schema(example = 0.25)]
    pub temperature: f32,
    #[schema(example = 1.31)]
    pub dryness_index: f32,
}

#[cfg(test)]
mod tests {
    use serde_json::{self, json};

    use crate::measurement::MoistureMeasurement;

    #[test]
    fn test_measure_deserialization() {
        let json_value = json!({ "datetime": 1684599720,"temperature":1.2,"dryness_index":1.24 });
        let deserialized: Result<MoistureMeasurement, serde_json::Error> =
            serde_json::from_value(json_value);
        assert!(deserialized.is_ok(), "Deserialization failed");
    }
}
