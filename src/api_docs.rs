use utoipa::OpenApi;

use crate::device;

#[derive(OpenApi)]
#[openapi(
    paths(
        device::list,
        device::register,
        device::change_metadata,
        device::delete,
    ),
    components(
        schemas(device::Device, device::DeviceError, device::DeviceRegister)
    ),
    tags(
        (name = "granit", description = "Device management API")
    )
)]
pub struct GranitApiDoc;
