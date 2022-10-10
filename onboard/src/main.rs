mod devcade_s3;
use devcade_s3::S3;

mod data_set;

use gtk4::prelude::*;
use gtk4::{Application};
const APP_ID: &str = "edu.csh.rit.devcade.onboard";

// GUI Functions
fn build_ui(app: &gtk4::Application, s3_conn: &S3) {
    let window = gtk4::ApplicationWindow::builder()
        .default_width(600)
        .default_height(600)
        .application(app)
        .title("Devcade")
        .build();
/*
    let welcome = gtk4::Text::builder()
        .placeholder_text("Welcome to Devcade")
        .build();
*/

    let my_label = gtk4::Label::builder()
          .margin_top(8)
          .margin_bottom(8)
          .label("Welcome to Devcade")
          .build();

    let flow_box = gtk4::ListBox::builder()
        .valign(gtk4::Align::Start)
        .selection_mode(gtk4::SelectionMode::None)
        .build();

    data_set::GAMES.iter().for_each(|game| {
        let game_widget = create_game_entry(game, s3_conn);
        flow_box.insert(&game_widget, -1);
    });

    let scrolled_window = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&flow_box)
        .build();

    window.set_child(Some(&my_label));
    window.set_child(Some(&scrolled_window));
    window.show();
}

fn create_game_entry(game: &'static str, s3_conn: &S3) -> gtk4::Button {
    let button = gtk4::Button::new();
    let connection = s3_conn.clone();
    let game_title = game.clone();

    button.set_label(game);
    button.connect_clicked(move |_button| {
        println!("Chom {}", game_title);
        let conn = connection.clone();
        tokio::spawn(async move {
            match conn.get_game("devcade-games", game_title, "/tmp/devcade_games/").await {
                Ok(()) => {println!("Chom!! :)");},
                _ => {println!("Shit.");}
            }
        });
    });
    button
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s3_conn = S3::connect_s3().await?;

    /*
    let bucket = "devcade-games";
    let object = "bankshot.zip";
    let devcade_games_dir = "/tmp/devcade_games/";

    s3_conn.show_buckets().await?;
    s3_conn.show_objects("devcade-games").await?;

    let game_uri = s3_conn.get_object(&bucket, &object, 90).await?; // TODO (willnilges): if this
                                                                    // doesn't work, I reduced the
                                                                    // expiry time. Try increasing it.

    // Download game, create directory if we need to.
    std::fs::create_dir(devcade_games_dir)?;
    S3::download_game(game_uri, format!("{}{}", devcade_games_dir, object).to_string()).await?;
    */

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    //app.connect_activate(build_ui);
    // Connect to "activate" signal of `app`
    app.connect_activate(move |app: &Application| {
        build_ui(app, &s3_conn);
    });

    // Run the application
    app.run();

    Ok(())
}

