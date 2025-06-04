use std::time::Duration;

use crate::{data_structs::ConfigFile, files};
use egui_notify::Toasts;
use serde::{Deserialize, Serialize};

/// This is used for the error modal
#[derive(Default, Serialize, Deserialize)]
struct ErrorAnnounceState {
    shown: bool,
    text: String,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    answer: String,
    error_state: ErrorAnnounceState,
    config_data: ConfigFile,

    #[serde(skip)] // Don't preserve state on reboot
    toasts_handler: Toasts,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        // Init egui-notify

        // Load default values for struct
        Default::default()
    }

    /// Shows an error modal to the user and logs the error in the console
    fn show_error(&mut self, text: &str) {
        self.toasts_handler
            .info(text)
            .duration(Some(Duration::from_secs(5)));

        println!("Error: {}", text);
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Show Toasts
        self.toasts_handler.show(ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Open").clicked() {
                        // Prompt the user to select a JSON file
                        match files::pick_json() {
                            Ok(config_data) => self.config_data = config_data,
                            Err(e) => self.show_error(&e.to_string()),
                        }
                    }
                });
                ui.menu_button("Theme", |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                });
                ui.add_space(16.0);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("The question");
            ui.horizontal(|ui| {
                ui.label("Answer: ");
                ui.text_edit_singleline(&mut self.answer);
            });
            if ui.button("Check answer").clicked() {
                if self.answer.is_empty() {
                    self.show_error("No text was entered in the answer");
                } else {
                    println!("Answer is {}", self.answer);
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
