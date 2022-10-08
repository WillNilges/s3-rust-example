mod devcade_s3;
use devcade_s3::S3;

mod data_set;
use std::str::FromStr;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, gio, glib, gdk};
const APP_ID: &str = "edu.csh.rit.devcade.onboard";

// GUI Functions

fn build_ui(app: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::builder()
        .default_width(600)
        .default_height(600)
        .application(app)
        .title("FlowBox")
        .build();

    let flow_box = gtk4::FlowBox::builder()
        .valign(gtk4::Align::Start)
        .max_children_per_line(30)
        .min_children_per_line(4)
        .selection_mode(gtk4::SelectionMode::None)
        .build();

    data_set::COLORS.iter().for_each(|color| {
        let color_widget = create_color_button(color);
        flow_box.insert(&color_widget, -1);
    });

    let scrolled_window = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&flow_box)
        .build();

    window.set_child(Some(&scrolled_window));
    window.show();
}

fn create_color_button(color: &'static str) -> gtk4::Button {
    let button = gtk4::Button::new();
    let drawing_area = gtk4::DrawingArea::builder()
        .content_height(24)
        .content_width(24)
        .build();

    let rgba = gdk::RGBA::from_str(color).unwrap();
    drawing_area.set_draw_func(move |_, cr, _width, _height| {
        GdkCairoContextExt::set_source_rgba(cr, &rgba);
        cr.paint().expect("Invalid cairo surface state");
    });
    button.set_child(Some(&drawing_area));

    button
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let s3_conn = S3::connect_s3().await?;

    let bucket = "devcade-games";
    let object = "bankshot.zip";
    let devcade_games_dir = "/tmp/devcade_games/";

    s3_conn.show_buckets().await?;
    s3_conn.show_objects("devcade-games").await?;

    let game_uri = s3_conn.get_object(&bucket, &object, 90).await?; // TODO (willnilges): if this
                                                                    // doesn't work, I reduced the
                                                                    // expiry time. Try increasing it.

    // Download game, create directory if we need to.
//    std::fs::create_dir(devcade_games_dir)?;
//    S3::download_game(game_uri, format!("{}{}", devcade_games_dir, object).to_string()).await?;

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run();

    Ok(())
}

