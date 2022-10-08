use aws_sdk_s3 as s3;
use aws_sdk_s3::{Client, Endpoint, Error};
use aws_sdk_s3::presigning::config::PresigningConfig;
use std::time::Duration;
use std::io::Cursor;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};

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

// Builds a valid URI from which you can literally just download the file
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

// Downloads a game from a generated URI
async fn download_game(uri: String, destination: String) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(uri).await?;
    let mut file = std::fs::File::create(destination)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

// GUI Functions
fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
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
    let devcade_games_dir = "/tmp/devcade_games/";

    show_buckets(&client).await?;
    show_objects(&client, "devcade-games").await?;

    let game_uri = get_object(&client, &bucket, &object, 900).await?;

    // Download game, create directory if we need to.
    std::fs::create_dir(devcade_games_dir)?;
    download_game(game_uri, format!("{}{}", devcade_games_dir, object).to_string()).await?;

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run();

    Ok(())
}

