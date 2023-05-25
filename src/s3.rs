use std::io::Cursor;

use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};

// Save a `Stream` to a file
pub async fn store_object() {
    let s3 = AmazonS3Builder::new()
        .with_region("eu-central-1")
        .with_bucket_name("bton")
        .build();

    // create an ObjectStore
    let object_store: Arc<dyn ObjectStore> = Arc::new(get_object_store());

    // Retrieve a specific file
    let path: Path = "data/file01.parquet".try_into().unwrap();

    // fetch the bytes from object store
    let stream = object_store.get(&path).await.unwrap().into_stream();

    // Count the '0's using `map` from `StreamExt` trait
    let num_zeros = stream
        .map(|bytes| {
            let bytes = bytes.unwrap();
            bytes.iter().filter(|b| **b == 0).count()
        })
        .collect::<Vec<usize>>()
        .await
        .into_iter()
        .sum::<usize>();

    println!("Num zeros in {} is {}", path, num_zeros);

    // create an ObjectStore
    let object_store: Arc<dyn ObjectStore> = Arc::new(get_object_store());

    // Retrieve a specific file
    let path: Path = "data/file01.parquet".try_into().unwrap();

    // fetch the bytes from object store
    let stream = object_store.get(&path).await.unwrap().into_stream();

    // Count the '0's using `map` from `StreamExt` trait
    let num_zeros = stream
        .map(|bytes| {
            let bytes = bytes.unwrap();
            bytes.iter().filter(|b| **b == 0).count()
        })
        .collect::<Vec<usize>>()
        .await
        .into_iter()
        .sum::<usize>();

    println!("Num zeros in {} is {}", path, num_zeros);
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
