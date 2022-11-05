use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

const APP_ID: &str = "com.github.amikha1lov.rustExp";

fn main() {

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {

    let button = Button::builder()
        .label("Test Btn")
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    button.connect_clicked(move |button| {
        button.set_label("signal test");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("First GTK app in Rust")
        .child(&button)
        .build();

    window.present();
}