use axum::{
    body::Full,
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use reqwest::StatusCode;
use std::net::SocketAddr;
use tracing::log::info;

use std::sync::Arc;

use axum::routing;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{api_docs::GranitApiDoc, config::Configuration, device::Store};

mod api_docs;
mod config;
mod device;
mod measurement;
mod r2;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config =
        Arc::new(envy::from_env::<Configuration>().expect("required config could not be parsed"));

    tracing_subscriber::fmt()
        .with_max_level(config.log_level())
        .init();

    // for (n, v) in env::vars() {
    //     println!("{}: {}", n, v);
    // }

    //info!("{:?}", &config);

    let store = Arc::new(Store::default());

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", GranitApiDoc::openapi()))
        .route("/", get(root))
        .route("/health", get(health))
       // .route("/bin-dump", post(dump))
        .route("/devices", routing::get(device::list))
        .route("/devices", routing::post(device::register))
        .route(
            "/devices/:id",
            routing::put(device::change_metadata).delete(device::delete),
        )
        .route("/devices/:id/write", routing::post(device::write_data))
        .with_state(config);

    let addr = SocketAddr::from((config.socket_addr(), config.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn root() -> axum::response::Html<&'static str> {
    "<a href='/docs'>docs</a>".into()
}
async fn health() -> StatusCode {
    StatusCode::OK
}

async fn dump(body: Full<axum::body::Bytes>) -> StatusCode {
    // let content = body.into();

    // r2::store_object("asdf".to_string(), content, config).await;

    StatusCode::OK
}
