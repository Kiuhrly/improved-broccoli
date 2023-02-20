use std::{fs, io};

use chip8::cpu::Chip8;
use egui::{Color32, DroppedFile, RichText};

use crate::{
    keyboard::get_key_state,
    screen_ui::draw_chip8_screen,
    settings::{load_settings, save_settings, LoadSettingsError, Settings},
};

#[derive(Default)]
pub struct App {
    chip8: Option<Chip8>,
    previous_keyboard_state: [bool; 16],
    delta_accumulator: f32,

    filename: String,

    settings: Settings,
    settings_error: Option<SettingsStorageError>,
    settings_window_open: bool,
    settings_error_window_open: bool,
}

#[derive(Clone, Copy)]
enum SettingsStorageError {
    NoneStorage,
    LoadSettingsError(LoadSettingsError),
}

const SETTINGS_KEY: &str = "settings.json";

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings: Result<Settings, SettingsStorageError> = match cc.storage {
            Some(storage) => match load_settings(storage, SETTINGS_KEY) {
                Ok(settings) => Ok(settings),
                Err(err) => Err(SettingsStorageError::LoadSettingsError(err)),
            },
            None => Err(SettingsStorageError::NoneStorage),
        };
        Self {
            settings_error: settings.clone().err(),
            settings: settings.unwrap_or_default(),
            ..Default::default()
        }
    }
}

impl App {
    fn top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let settings_button_text: RichText = if self.settings_error.is_some() {
                    RichText::new(t!("top_menu.settings.button_text_with_warning"))
                        .color(Color32::RED)
                } else {
                    t!("top_menu.settings.button_text").into()
                };
                ui.menu_button(settings_button_text, |ui| {
                    if self.settings_error.is_some() {
                        let button_text =
                            RichText::new(t!("top_menu.settings.open_error_info.button_text"))
                                .color(Color32::RED);
                        if ui.button(button_text).clicked() {
                            self.settings_error_window_open = true;
                            ui.close_menu();
                        }
                    }

                    if ui
                        .button(t!("top_menu.settings.open_settings.button_text"))
                        .clicked()
                    {
                        self.settings_window_open = true;
                        ui.close_menu();
                    }
                });
            });
        });
    }

    fn settings_window(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let was_open = self.settings_window_open;
        let settings_window = egui::Window::new(t!("settings_window.title"))
            .resizable(true)
            .collapsible(false)
            .open(&mut self.settings_window_open);
        settings_window.show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.settings.settings_menu(ui);
            });
        });
        // If the settings window was just closed, save the settings
        if was_open && !self.settings_window_open {
            if let Some(storage) = frame.storage_mut() {
                save_settings(storage, SETTINGS_KEY, &self.settings);
                storage.flush();
            } else {
                tracing::event!(
                    tracing::Level::ERROR,
                    "failed to get Storage from Frame when saving settings"
                );
                self.settings_error = Some(SettingsStorageError::NoneStorage);
            }
        }
    }

    fn error_info_window(&mut self, ctx: &egui::Context) {
        if let Some(err) = self.settings_error {
            let settings_error_window = egui::Window::new(t!("error_info_window.title"))
                .resizable(true)
                .collapsible(false)
                .open(&mut self.settings_error_window_open);
            settings_error_window.show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading(t!("error_info_window.heading"));
                    match err {
                        SettingsStorageError::NoneStorage => {
                            ui.label(t!("error_info_window.label_nonestorage"));

                            #[cfg(target_arch = "wasm32")]
                            ui.label(t!("error_info_window.label_local_storage_note"));
                        }
                        SettingsStorageError::LoadSettingsError(err) => match err {
                            LoadSettingsError::Deserialize => {
                                ui.label(t!("error_info_window.label_deserialize"));
                            }
                        },
                    }
                });
            });
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.top_bar(ctx);
        self.error_info_window(ctx);
        self.settings_window(ctx, frame);

        // Check for dropped files to load
        let dropped_file: Option<DroppedFile> = ctx.input(|i| i.raw.dropped_files.get(0).cloned());
        if let Some(dropped_file) = dropped_file {
            let program: Option<Vec<u8>> = if let Some(dropped_file_bytes) = dropped_file.bytes {
                // If the dropped file comes with the file bytes (e.g. web), just
                // use them
                Some(dropped_file_bytes.to_vec())
            } else if let Some(dropped_file_path) = dropped_file.path {
                // Otherwise, try to get the filename and load the file

                // Can't get local file on wasm
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let mut file =
                        fs::File::open(std::path::Path::new(&dropped_file_path)).unwrap();
                    let mut program: Vec<u8> = vec![];
                    io::Read::read_to_end(&mut file, &mut program).unwrap();
                    Some(program)
                }

                #[cfg(target_arch = "wasm32")]
                None
            } else {
                // Otherwise, nothing useful from the dropped file
                None
            };

            // If we got a program from the dropped file, load it
            if let Some(program) = program {
                self.chip8 = Some(Chip8::new(&program));
                self.delta_accumulator = 0.0;
                ctx.request_repaint();
            }
        }

        // Handle CHIP-8 simulation
        if let Some(chip8) = &mut self.chip8 {
            let delta_time = ctx.input(|i| i.unstable_dt);
            self.delta_accumulator += delta_time;
            let frametime = 1.0 / 60.0; // CHIP-8 runs at 60hz

            let mut keyboard_state: [bool; 16] = Default::default();
            ctx.input(|i| keyboard_state = get_key_state(i));

            while self.delta_accumulator > frametime {
                // TODO un-hardcode cycles per frame
                for _ in 0..30 {
                    chip8
                        .cycle(&keyboard_state, &self.previous_keyboard_state)
                        .unwrap();
                }
                chip8.update_timers();
                self.previous_keyboard_state = keyboard_state;
                self.delta_accumulator -= frametime;
            }

            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                #[cfg(target_arch = "wasm32")]
                ui.label("Drag a file on to this window to load");
                #[cfg(not(target_arch = "wasm32"))]
                {
                    ui.label("Drag a file on to this window or enter a path below to load");
                    ui.horizontal(|ui| {
                        ui.label("File:");
                        ui.text_edit_singleline(&mut self.filename);
                        if ui.button("Load").clicked() {
                            let mut file =
                                fs::File::open(std::path::Path::new(&self.filename)).unwrap();
                            let mut program: Vec<u8> = vec![];
                            io::Read::read_to_end(&mut file, &mut program).unwrap();

                            self.chip8 = Some(Chip8::new(&program));
                            self.delta_accumulator = 0.0;
                            ctx.request_repaint();
                        }
                    });
                }

                if let Some(chip8) = &self.chip8 {
                    draw_chip8_screen(
                        ui,
                        chip8.get_screen(),
                        10,
                        self.settings.foreground_color,
                        self.settings.background_color,
                    );
                }
            })
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        save_settings(storage, SETTINGS_KEY, &self.settings);
        storage.flush();
    }
}
