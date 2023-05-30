use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::r2;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RandomMetadata {
    #[schema(example = "This is a String")]
    pub mymetadata: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        r2::dump,
        r2::list
        // device::list,
        // device::register,
        // device::change_metadata,
        // device::write_data,
        // device::delete,

    ),
    components(
        schemas(r2::KeyList)
        // schemas(device::Device, device::DeviceError, device::DeviceRegister, measurement::MoistureMeasurement,RandomMetadata)
    ),
    tags(
        (name = "granit", description = "Device management API")
    )
)]
pub struct GranitApiDoc;
