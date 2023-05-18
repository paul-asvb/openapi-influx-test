use axum::{routing::get, Router};
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
mod influx;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = envy::from_env::<Configuration>().expect("required config could not be parsed");

    tracing_subscriber::fmt()
        .with_max_level(config.log_level())
        .init();

    info!("app starting");

    let store = Arc::new(Store::default());

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", GranitApiDoc::openapi()))
        .route("/", get(root))
        .route("/health", get(health))
        .route("/devices", routing::get(device::list))
        .route("/device", routing::post(device::register))
        .route(
            "/device/:id",
            routing::put(device::change_metadata).delete(device::delete),
        )
        .with_state(store);

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
