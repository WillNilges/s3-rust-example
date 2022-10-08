mod devcade_s3;
use devcade_s3::S3;

mod application_row;
use crate::application_row::ApplicationRow;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, gio};
const APP_ID: &str = "edu.csh.rit.devcade.onboard";

// GUI Functions

fn build_ui(app: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::builder()
        .default_width(600)
        .default_height(600)
        .application(app)
        .title("ListView: Applications Launcher")
        .build();

    let model = gio::ListStore::new(gio::AppInfo::static_type());
    gio::AppInfo::all().iter().for_each(|app_info| {
        model.append(app_info);
    });

    let factory = gtk4::SignalListItemFactory::new();
    // the "setup" stage is used for creating the widgets
    factory.connect_setup(move |_factory, item| {
        // In gtk4 < 4.8, you don't need the following line
        // as gtk used to pass GtkListItem directly. In order to make that API
        // generic for potentially future new APIs, it was switched to taking a GObject in 4.8
        let item = item.downcast_ref::<gtk4::ListItem>().unwrap();
        let row = ApplicationRow::new();
        item.set_child(Some(&row));
    });

    // the bind stage is used for "binding" the data to the created widgets on the "setup" stage
    factory.connect_bind(move |_factory, item| {
        // In gtk4 < 4.8, you don't need the following line
        // as gtk used to pass GtkListItem directly. In order to make that API
        // generic for potentially future new APIs, it was switched to taking a GObject in 4.8
        let item = item.downcast_ref::<gtk4::ListItem>().unwrap();
        let app_info = item.item().unwrap().downcast::<gio::AppInfo>().unwrap();

        let child = item.child().unwrap().downcast::<ApplicationRow>().unwrap();
        child.set_app_info(&app_info);
    });

    // A sorter used to sort AppInfo in the model by their name
    let sorter = gtk4::CustomSorter::new(move |obj1, obj2| {
        let app_info1 = obj1.downcast_ref::<gio::AppInfo>().unwrap();
        let app_info2 = obj2.downcast_ref::<gio::AppInfo>().unwrap();

        app_info1
            .name()
            .to_lowercase()
            .cmp(&app_info2.name().to_lowercase())
            .into()
    });
    let sorted_model = gtk4::SortListModel::new(Some(&model), Some(&sorter));
    let selection_model = gtk4::SingleSelection::new(Some(&sorted_model));

    let list_view = gtk4::ListView::new(Some(&selection_model), Some(&factory));
    // Launch the application when an item of the list is activated
    list_view.connect_activate(move |list_view, position| {
        let model = list_view.model().unwrap();
        let app_info = model
            .item(position)
            .unwrap()
            .downcast::<gio::AppInfo>()
            .unwrap();

        let context = list_view.display().app_launch_context();
        if let Err(err) = app_info.launch(&[], Some(&context)) {
            let parent_window = list_view.root().unwrap().downcast::<gtk4::Window>().unwrap();

            gtk4::MessageDialog::builder()
                .text(&format!("Failed to start {}", app_info.name()))
                .secondary_text(&err.to_string())
                .message_type(gtk4::MessageType::Error)
                .modal(true)
                .transient_for(&parent_window)
                .build()
                .show();
        }
    });

    let scrolled_window = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_view)
        .build();

    window.set_child(Some(&scrolled_window));
    window.show();
}
/*
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
}*/

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
//    std::fs::create_dir(devcade_games_dir)?;
//    S3::download_game(game_uri, format!("{}{}", devcade_games_dir, object).to_string()).await?;

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run();

    Ok(())
}

