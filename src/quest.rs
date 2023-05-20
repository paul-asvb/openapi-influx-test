use questdb::{
    ingress::{Buffer, CertificateAuthority, SenderBuilder, Tls},
    Result,
};
use uuid::Uuid;

use crate::measurement::MoistureMeasurement;

pub fn write(sensor_id: Uuid, measurement: MoistureMeasurement) -> Result<()> {
    let host: String = std::env::args()
        .nth(1)
        .unwrap_or("positive-orange-118-67209e79.ilp.c7at.questdb.com".to_string());
    let port: u16 = std::env::args()
        .nth(2)
        .unwrap_or("30104".to_string())
        .parse()
        .unwrap();
    let mut sender = SenderBuilder::new(host, port)
        .auth(
            "admin",                                       // kid
            "5j_o9Ea9M8n_1HRK7F6Fih8c_W-Dbb6qPC5VFyvIXtA", // d
            "lPLYyXVgGLPZfikoqh3LPIlqeYx-CFFEAxPeO2JC98g", // x
            "sYwijDcqBxZC78GZ_j8jrBRUVKHwMzYkLBYEpPxnc4Q",
        ) // y
        .tls(Tls::Enabled(CertificateAuthority::WebpkiRoots))
        .connect()?;
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
