/*use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, gio, glib, gdk};

pub struct Interface {
    pub client: Client,
}

impl Interface {
    pub fn build_ui(app: &gtk4::Application) {
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

        data_set::GAMES.iter().for_each(|game| {
            let color_widget = create_game_entry(game);
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

    fn create_game_entry(game: &'static str) -> gtk4::Button {
        let button = gtk4::Button::new();
        let drawing_area = gtk4::DrawingArea::builder()
            .content_height(24)
            .content_width(24)
            .build();

        button.set_label(game);
        button.connect_clicked( move |game| {
            println!("Chom: {}", game);
        });
        button
    }
}*/
