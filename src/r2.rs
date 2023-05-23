use std::io::Cursor;

use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};

// Save a `Stream` to a file
pub async fn store_object() {
    let region = Region::Custom {
        name: "auto".to_owned(),
        endpoint: "https://7f2b1009fa60bde9f961c3853cc9ea0c.r2.cloudflarestorage.com/".to_owned(),
    };

    let client = S3Client::new_with(
        rusoto_core::HttpClient::new().expect("failed to create request dispatcher"),
        rusoto_core::credential::StaticProvider::new_minimal(
            "1e647413fe4ffb6ca36d8c81f74e67c3".to_owned(),
            "e37bc88585918d46c2318f3c3bcfa3992b911b861aec20bc78093618e569c49c".to_owned(),
        ),
        region,
    );

    let mut put = PutObjectRequest::default();
    put.bucket = "bton".to_string();

    let name = "Jake".to_string();

    let mut buffer: Vec<u8> = name.into_bytes();
    let mut cursor = Cursor::new(buffer);

    put.body.

    client.put_object(put);
}

#[cfg(test)]
mod tests {

    use super::store_object;

    #[tokio::test]
    async fn test_store_object() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
        store_object().await;
    }
}
