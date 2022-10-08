mod devcade_s3;
use devcade_s3::S3;


use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};
const APP_ID: &str = "edu.csh.rit.devcade.onboard";

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

    let s3_conn = S3::connect_s3().await?;

    let bucket = "devcade-games";
    let object = "bankshot.zip";
    let devcade_games_dir = "/tmp/devcade_games/";

    s3_conn.show_buckets().await?;
    s3_conn.show_objects("devcade-games").await?;

    let game_uri = s3_conn.get_object(&bucket, &object, 900).await?;

    // Download game, create directory if we need to.
    std::fs::create_dir(devcade_games_dir)?;
    S3::download_game(game_uri, format!("{}{}", devcade_games_dir, object).to_string()).await?;

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run();

    Ok(())
}

