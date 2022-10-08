use aws_sdk_s3 as s3;
use aws_sdk_s3::Endpoint;

use aws_sdk_s3::{Client, Error};

// Shows your buckets in the endpoint.
async fn show_buckets(client: &Client) -> Result<(), Error> {
    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets().unwrap_or_default();
    let num_buckets = buckets.len();

    for bucket in buckets {
        println!("{}", bucket.name().unwrap_or_default());
    }

    println!();
    println!("Found {} buckets.", num_buckets);

    Ok(())
}

async fn show_objects(client: &Client, bucket: &str) -> Result<(), Error> {
    let resp = client.list_objects_v2().bucket(bucket).send().await?;

    for object in resp.contents().unwrap_or_default() {
        println!("{}", object.key().unwrap_or_default());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    // Load configuration and credentials from the environment.
    let shared_config = aws_config::load_from_env().await;

    // Create an S3 config from the shared config and override the endpoint resolver.
    let s3_config = s3::config::Builder::from(&shared_config)
        .endpoint_resolver(Endpoint::immutable("https://s3.csh.rit.edu".parse().expect("valid URI")))
        .build();

    // Create an S3 client to send requests to S3 Object Lambda.
    let client = s3::Client::from_conf(s3_config);

    show_buckets(&client).await;
    show_objects(&client, "devcade-games").await
}

