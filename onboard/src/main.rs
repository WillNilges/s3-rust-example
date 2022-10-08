use aws_sdk_s3 as s3;
use aws_sdk_s3::{Client, Endpoint, Error};
use aws_sdk_s3::presigning::config::PresigningConfig;
use std::time::Duration;
use std::io::Cursor;
use http::Uri;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

const APP_ID: &str = "edu.csh.rit.devcade.onboard";

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

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Present window
    window.present();
}

async fn get_object(
    client: &Client,
    bucket: &str,
    object: &str,
    expires_in: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    let expires_in = Duration::from_secs(expires_in);
    let presigned_request = client
        .get_object()
        .bucket(bucket)
        .key(object)
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;

    println!("Object URI: {}", presigned_request.uri());

    Ok(presigned_request.uri().to_string())
}

async fn download_game(uri: String, destination: String) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(uri).await?;
    let mut file = std::fs::File::create(destination)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Load configuration and credentials from the environment.
    let shared_config = aws_config::load_from_env().await;

    // Create an S3 config from the shared config and override the endpoint resolver.
    let s3_config = s3::config::Builder::from(&shared_config)
        .endpoint_resolver(Endpoint::immutable("https://s3.csh.rit.edu".parse().expect("valid URI")))
        .build();

    // Create an S3 client to send requests to S3 Object Lambda.
    let client = s3::Client::from_conf(s3_config);

    let bucket = "devcade-games";
    let object = "bankshot.zip";

    show_buckets(&client).await;
    show_objects(&client, "devcade-games").await;

    let gameUri = get_object(&client, &bucket, &object, 900).await?;

    download_game(gameUri, "bankshot.zip".to_string()).await;

    Ok(())

    /*
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run();*/

}

