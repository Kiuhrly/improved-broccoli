use egui::{Align, Color32, Layout, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Settings {
    pub foreground_color: Color32,
    pub background_color: Color32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            foreground_color: Color32::from_rgb(255, 255, 255),
            background_color: Color32::from_rgb(0, 0, 0),
        }
    }
}

impl Settings {
    pub fn settings_menu(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading(t!("settings_window.display_heading"));
            color_picker_setting(
                ui,
                t!("settings_window.foreground_color"),
                &mut self.foreground_color,
            );
            color_picker_setting(
                ui,
                t!("settings_window.background_color"),
                &mut self.background_color,
            );
            ui.horizontal(|ui| {
                if ui
                    .button(t!("settings_window.color_reset_button_text"))
                    .clicked()
                {
                    self.foreground_color = Settings::default().foreground_color;
                    self.background_color = Settings::default().background_color;
                }
                if ui
                    .button(t!("settings_window.color_swap_button_text"))
                    .clicked()
                {
                    std::mem::swap(&mut self.foreground_color, &mut self.background_color);
                }
            });
        });
    }
}

fn right_aligned_setting(
    ui: &mut Ui,
    text: impl Into<WidgetText>,
    add_contents: impl FnOnce(&mut Ui),
) {
    ui.horizontal(|ui| {
        ui.label(text);
        ui.with_layout(Layout::right_to_left(Align::Center), add_contents);
    });
}

fn color_picker_setting(ui: &mut Ui, text: impl Into<WidgetText>, color: &mut Color32) {
    right_aligned_setting(ui, text, |ui| {
        let mut srgb: [u8; 3] = [color.r(), color.g(), color.b()];
        ui.color_edit_button_srgb(&mut srgb);
        *color = Color32::from_rgb(srgb[0], srgb[1], srgb[2])
    });
}

/// Error type for `load_settings`
#[derive(Clone, Copy)]
pub enum LoadSettingsError {
    /// There was an error deserializing the settings file
    Deserialize,
}

/// Load the settings from a local file (native) or LocalStorage (web).
///
/// This may also save settings if there was no existing settings file.
pub fn load_settings(
    storage: &dyn eframe::Storage,
    key: &str,
) -> Result<Settings, LoadSettingsError> {
    if let Some(settings_str) = storage.get_string(key) {
        match serde_json::from_str(&settings_str) {
            Ok(settings) => Ok(settings),
            Err(_) => Err(LoadSettingsError::Deserialize),
        }
    } else {
        // No settings - load default.
        tracing::event!(tracing::Level::INFO, "no Settings found, loading default");
        let settings = Settings::default();
        Ok(settings)
    }
}

pub fn save_settings(storage: &mut dyn eframe::Storage, key: &str, settings: &Settings) {
    let settings_str =
        serde_json::to_string_pretty(settings).expect("Settings should serialize to json");
    storage.set_string(key, settings_str);
}
