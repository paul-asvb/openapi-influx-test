use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::{device, measurement};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RandomMetadata {
    #[schema(example = "This is a String")]
    pub mymetadata: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        device::list,
        device::register,
        device::change_metadata,
        device::write_data,
        device::delete,

    ),
    components(
        schemas(device::Device, device::DeviceError, device::DeviceRegister, measurement::MoistureMeasurement,RandomMetadata)
    ),
    tags(
        (name = "granit", description = "Device management API")
    )
)]
pub struct GranitApiDoc;
