use std::{env, fs::File, io::Read};

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3 as s3;
use tracing::{debug, error};

// Save a `Stream` to a file
pub async fn store_object() {
    debug!("store_object");
    // let vars = env::vars();

    // // Iterate over the variables and print them
    // for (key, value) in vars {
    //     debug!("{}: {}", key, value);
    // }

    let region_provider = RegionProviderChain::default_provider().or_else("eu-central-1");
    let config = aws_config::from_env().region(region_provider).load().await;

    debug!("{:#?}", &config);
    let client = s3::Client::new(&config);

    let mut f = File::open("./fly.toml").unwrap();

    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).unwrap();

    let put = client
        .put_object()
        .body(buffer.into())
        .bucket("bton")
        .key("input");
    let put_object_request = put.send().await;

    match put_object_request {
        Ok(_) => debug!("File uploaded successfully"),
        Err(err) => error!("{}", err),
    }

    // dbg!(bla);
}

#[cfg(test)]
mod tests {

    use super::store_object;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_store_object() {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();

        store_object().await;
    }
}
