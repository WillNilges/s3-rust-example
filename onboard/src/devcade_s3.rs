use aws_sdk_s3 as s3;
use aws_sdk_s3::{Client, Endpoint, Error};
use aws_sdk_s3::presigning::config::PresigningConfig;
use std::time::Duration;
use std::io::Cursor;

#[derive(Clone)]
pub struct S3 {
    pub client: Client,
}

impl S3 {
    // Shows your buckets in the endpoint.
    pub async fn show_buckets(&self) -> Result<(), Error> {
        let resp = self.client.list_buckets().send().await?;
        let buckets = resp.buckets().unwrap_or_default();
        let num_buckets = buckets.len();

        for bucket in buckets {
            println!("{}", bucket.name().unwrap_or_default());
        }

        println!();
        println!("Found {} buckets.", num_buckets);

        Ok(())
    }

    pub async fn show_objects(&self, bucket: &str) -> Result<(), Error> {
        let resp = self.client.list_objects_v2().bucket(bucket).send().await?;

        for object in resp.contents().unwrap_or_default() {
            println!("{}", object.key().unwrap_or_default());
        }

        Ok(())
    }

    // Builds a valid URI from which you can literally just download the file
    pub async fn get_object(
        &self,
        bucket: &str,
        object: &str,
        expires_in: u64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let expires_in = Duration::from_secs(expires_in);
        let presigned_request = self.client
            .get_object()
            .bucket(bucket)
            .key(object)
            .presigned(PresigningConfig::expires_in(expires_in)?)
            .await?;

        println!("Object URI: {}", presigned_request.uri());

        Ok(presigned_request.uri().to_string())
    }

    pub async fn get_game(&self, bucket: &str, object: &str, directory: &str) -> Result<(), Box<dyn std::error::Error>> {
        // TODO (willnilges): if this doesn't work, I reduced the expiry time. Try increasing it.
        println!("Oh hey that wasn't supposed to work. {}, {}", bucket, object); 
        let game_uri = self.get_object(&bucket, &object, 90).await?; 
        // Download game, create directory if we need to.
        std::fs::create_dir(&directory)?;
        S3::download_game(game_uri, format!("{}{}", directory, object).to_string()).await
    }

    pub async fn connect_s3() -> Result<S3, Box<dyn std::error::Error>> {
        // Load configuration and credentials from the environment.
        let shared_config = aws_config::load_from_env().await;

        // Create an S3 config from the shared config and override the endpoint resolver.
        let s3_config = s3::config::Builder::from(&shared_config)
            .endpoint_resolver(Endpoint::immutable("https://s3.csh.rit.edu".parse().expect("valid URI")))
            .build();

        // Create an S3 client to send requests to S3 Object Lambda.
        Ok(S3{ client: s3::Client::from_conf(s3_config) })
    }

    // Downloads a game from a generated URI
    pub async fn download_game(uri: String, destination: String) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::get(uri).await?;
        let mut file = std::fs::File::create(destination)?;
        let mut content = Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
        Ok(())
    }
}
