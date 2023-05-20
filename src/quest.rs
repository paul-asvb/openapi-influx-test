use questdb::{
    ingress::{Buffer, SenderBuilder},
    Result,
};
use uuid::Uuid;

use crate::measurement::MoistureMeasurement;

pub fn write(sensor_id: Uuid, measurement: MoistureMeasurement) -> Result<()> {
    let mut sender = SenderBuilder::new("localhost", 9009).connect()?;
    let mut buffer = Buffer::new();
    buffer
        .table("sensors")?
        .symbol("sensor_id", sensor_id.to_string())?
        .column_f64("temperature", measurement.temperature.into())?
        .column_f64("dryness_index", measurement.dryness_index.into())?
        .at_now()?;
    sender.flush(&mut buffer)?;
    Ok(())
}
