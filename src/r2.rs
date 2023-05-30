use std::sync::Arc;

use axum::response::IntoResponse;
use axum::Json;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::{Bucket, Region};
use serde::{Deserialize, Serialize};
use tracing::debug;

use axum::{body::Bytes, extract::State};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use utoipa::ToSchema;

use crate::config::Configuration;
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct KeyList {
    number_of_keys: usize,
    keys: Vec<String>,
}

/// Dump bytes
#[utoipa::path(
    post,
    path = "/r2/dump",
    request_body = String,
    responses(
        (status = 200, description = "Will only return 200 after the object storage has returned successful storage"),
        (status = 500, description = "If any any error occurs 500 will be thrown")
    )
)]
pub async fn dump(State(config): State<Arc<Configuration>>, body: Bytes) -> StatusCode {
    let now: DateTime<Utc> = Utc::now();
    if store_object(now.timestamp_nanos().to_string(), body.as_ref(), config)
        .await
        .is_err()
    {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

/// list dumps
#[utoipa::path(
    get,
    path = "/r2/list",
    responses(
        (status = 200, body=KeyList, description = "Will only return list of string with all keys, might return [] on error"),
    )
)]
pub async fn list(State(config): State<Arc<Configuration>>) -> impl IntoResponse {
    if let Ok(list) = list_bucket_content(config).await {
        return list;
    } else {
        return Json(KeyList {
            number_of_keys: 0,
            keys: vec![],
        });
    }
}

pub async fn store_object(
    s3_path: String,
    content: &[u8],
    config: Arc<Configuration>,
) -> Result<(), S3Error> {
    let credentials = Credentials::new(
        Some(&config.cf_access_key_id),
        Some(&config.cf_secret_access_key),
        None,
        None,
        None,
    )
    .unwrap();

    let bucket = Bucket::new(
        &config.cf_bucket_name,
        Region::R2 {
            account_id: config.cf_account_id.clone(),
        },
        credentials,
    )
    .unwrap()
    .with_path_style();

    let response_data = bucket.put_object(s3_path, content).await?;

    debug!("{}", response_data);

    Ok(())
}

pub async fn list_bucket_content(config: Arc<Configuration>) -> Result<Json<KeyList>, S3Error> {
    let credentials = Credentials::new(
        Some(&config.cf_access_key_id),
        Some(&config.cf_secret_access_key),
        None,
        None,
        None,
    )
    .unwrap();

    let bucket = Bucket::new(
        &config.cf_bucket_name,
        Region::R2 {
            account_id: config.cf_account_id.clone(),
        },
        credentials,
    )
    .unwrap()
    .with_path_style();

    let objects = bucket.list("".to_string(), Some("".to_string())).await?;

    let mut counter: usize = 0;

    for o in &objects {
        counter = counter + o.contents.len()
    }

    let list: Vec<String> = objects
        .iter()
        .map(|bu| {
            bu.contents
                .iter()
                .map(|contents| contents.key.clone())
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect();

    let keylist = KeyList {
        number_of_keys: counter,
        keys: list,
    };

    Ok(Json(keylist))
}

#[cfg(test)]
mod tests {

    use crate::config::Configuration;
    use crate::r2::list_bucket_content;

    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_list() {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
        let config =
            envy::from_env::<Configuration>().expect("required config could not be parsed");

        let _bla = list_bucket_content(config.into()).await;
    }
}
