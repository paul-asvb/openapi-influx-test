use s3::creds::Credentials;
use s3::error::S3Error;
use s3::{Bucket, Region};

use crate::config::Configuration;

pub async fn store_object(
    s3_path: String,
    content: &[u8],
    config: Configuration,
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
            account_id: config.cf_account_id,
        },
        credentials,
    )
    .unwrap()
    .with_path_style();

    let response_data = bucket.put_object(s3_path, content).await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::config::Configuration;

    use super::store_object;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_store_object() {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
        let config =
            envy::from_env::<Configuration>().expect("required config could not be parsed");

        store_object("sdfg".to_owned(), b"sdfg", config).await;
    }
}
