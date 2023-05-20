use utoipa::OpenApi;

use crate::{device, measurement};

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
        schemas(device::Device, device::DeviceError, device::DeviceRegister, measurement::MoistureMeasurement)
    ),
    tags(
        (name = "granit", description = "Device management API")
    )
)]
pub struct GranitApiDoc;
