use crate::backend;

use gtk4::prelude::*;
use gtk4::*;

pub struct SettingsGui {
    pub window: Window,
    pub random_mac: CheckButton,
    pub default_mac: CheckButton,
    pub specific_mac: CheckButton,
    pub mac_entry: Entry,
    pub display_hidden_ap: CheckButton,
    pub kill_network_manager: CheckButton,
    // Airgorah-RT: Appearance / theme controls.
    pub theme_system: CheckButton,
    pub theme_light: CheckButton,
    pub theme_dark: CheckButton,
    pub save_but: Button,
}

impl SettingsGui {
    pub fn new(parent: &impl IsA<Window>) -> Self {
        let window = Window::builder()
            .title("Settings")
            .hide_on_close(true)
            .default_width(250)
            .default_height(260)
            .resizable(false)
            .transient_for(parent)
            .modal(true)
            .build();

        let random_mac = CheckButton::with_label("Random MAC address");
        let default_mac = CheckButton::with_label("Default MAC address");
        let specific_mac = CheckButton::with_label("Specific MAC address");
        let mac_entry = Entry::builder()
            .placeholder_text("ex: 00:00:01:02:03:04")
            .hexpand(true)
            .editable(true)
            .sensitive(false)
            .build();

        random_mac.set_active(true);
        default_mac.set_group(Some(&random_mac));
        specific_mac.set_group(Some(&random_mac));

        let mac_box = Box::new(Orientation::Vertical, 4);
        mac_box.set_margin_start(4);
        mac_box.set_margin_end(4);
        mac_box.set_margin_bottom(4);
        mac_box.append(&random_mac);
        mac_box.append(&default_mac);
        mac_box.append(&specific_mac);
        mac_box.append(&mac_entry);

        let mac_frame = Frame::new(Some("MAC"));
        mac_frame.set_child(Some(&mac_box));

        let display_hidden_ap = CheckButton::with_label("Display hidden APs");
        display_hidden_ap.set_active(true);

        let display_frame = Frame::new(Some("Display"));
        display_frame.set_child(Some(&display_hidden_ap));

        let kill_network_manager = CheckButton::with_label("Kill network managers");
        kill_network_manager.set_active(true);

        let process_frame = Frame::new(Some("Process"));
        process_frame.set_child(Some(&kill_network_manager));

        // Airgorah-RT: Appearance frame (System / Light / Dark theme).
        let theme_system = CheckButton::with_label("System");
        let theme_light = CheckButton::with_label("Light");
        let theme_dark = CheckButton::with_label("Dark");

        theme_system.set_active(true);
        theme_light.set_group(Some(&theme_system));
        theme_dark.set_group(Some(&theme_system));

        let theme_box = Box::new(Orientation::Horizontal, 8);
        theme_box.set_margin_start(4);
        theme_box.set_margin_end(4);
        theme_box.set_margin_bottom(4);
        theme_box.append(&theme_system);
        theme_box.append(&theme_light);
        theme_box.append(&theme_dark);

        let appearance_frame = Frame::new(Some("Appearance"));
        appearance_frame.set_child(Some(&theme_box));

        let save_but = Button::with_label("Save");

        let vbox = Box::new(Orientation::Vertical, 10);
        vbox.set_margin_top(10);
        vbox.set_margin_end(10);
        vbox.set_margin_start(10);
        vbox.set_margin_bottom(10);

        vbox.append(&mac_frame);
        vbox.append(&display_frame);
        vbox.append(&process_frame);
        vbox.append(&appearance_frame);
        vbox.append(&save_but);

        window.set_child(Some(&vbox));

        Self {
            window,
            random_mac,
            default_mac,
            specific_mac,
            mac_entry,
            display_hidden_ap,
            kill_network_manager,
            theme_system,
            theme_light,
            theme_dark,
            save_but,
        }
    }

    pub fn show(&self) {
        let settings = backend::get_settings();

        self.mac_entry.set_text("");
        self.save_but.set_sensitive(true);

        match settings.mac_address.as_str() {
            "random" => self.random_mac.set_active(true),
            "default" => self.default_mac.set_active(true),
            mac => {
                self.specific_mac.set_active(true);
                self.mac_entry.set_text(mac);
            }
        };

        self.display_hidden_ap
            .set_active(settings.display_hidden_ap);
        self.kill_network_manager
            .set_active(settings.kill_network_manager);

        // Airgorah-RT: reflect the saved theme preference in the radios.
        match settings.theme.as_str() {
            "light" => self.theme_light.set_active(true),
            "dark" => self.theme_dark.set_active(true),
            _ => self.theme_system.set_active(true),
        }

        self.window.show();
    }
}
