use gtk4::gio::ApplicationFlags;
use gtk4::prelude::*;
use gtk4::{Application, Settings};

mod backend;
mod frontend;
mod globals;
mod types;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    gtk4::init().expect("Could not initialize gtk4");

    Settings::default()
        .unwrap()
        .set_gtk_icon_theme_name(Some("Adwaita"));

    // Airgorah-RT: load the saved Appearance preference (System/Light/Dark)
    // and apply it before the window is built so there's no flash of the
    // wrong theme on launch.
    backend::settings::load_settings();
    backend::apply_theme(backend::get_settings().theme_mode());

    let application = Application::builder()
        .application_id(globals::APP_ID)
        .flags(ApplicationFlags::NON_UNIQUE)
        .build();

    application.connect_activate(frontend::build_ui);
    application.run();
}
