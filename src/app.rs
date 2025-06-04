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
    error_state: ErrorAnnounceState,
    config_data: ConfigFile,
    question_id: usize,

    #[serde(skip)] // Don't preserve state on reboot
    answer: String,
    #[serde(skip)]
    toasts_handler: Toasts,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        // Load default values for struct
        Default::default()
    }

    /// Shows an error modal to the user and logs the error in the console
    fn show_error(&mut self, text: &str) {
        self.toasts_handler
            .error(text)
            .duration(Some(Duration::from_secs(5)));

        println!("Error: {}", text);
    }

    fn reset_data(&mut self) {
        *self = TemplateApp::default();
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
                    if ui.button("Open").clicked() {
                        // Prompt the user to select a JSON file
                        match files::pick_json() {
                            Ok(config_data) => {
                                self.reset_data();
                                self.config_data = config_data
                            }
                            Err(e) => self.show_error(&e.to_string()),
                        }
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Debug", |ui| {
                    if ui.button("Reset data").clicked() {
                        self.reset_data();
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
            let current_question = match self.config_data.questions.get(self.question_id) {
                Some(question) => question,
                None => {
                    // Probably not loaded questions
                    if self.config_data.questions.is_empty() {
                        ui.heading( "No question pack is loaded");
                        ui.label("Try loading a question pack with File > Open.");
                    }
                    // Finished questions
                    else if self.question_id + 1 > self.config_data.questions.len() {
                        ui.heading("You finished 🎉");
                        if ui.button("Restart").clicked() {
                            self.question_id = 0;
                        }
                        ui.label("Or load another question pack from File > Open");
                    }
                    else {
                        ui.label("Uhh, the app is in a weird state. There seems to be questions but there are no questions at the same time...");
                    }
                    return;
                }
            };

            ui.heading(&current_question.question);
            ui.horizontal(|ui| {
                ui.label("Answer: ");
                ui.text_edit_singleline(&mut self.answer);
            });
            if ui.button("Check answer").clicked() {
                if self.answer.is_empty() {
                    self.show_error("No text was entered in the answer");
                } else if current_question
                    .answers
                    .contains(&self.answer.to_lowercase())
                {
                    self.toasts_handler
                        .success(format!("Question {} correct!", self.question_id))
                        .duration(Some(Duration::from_secs(5)));
                    self.question_id += 1;
                    self.answer = "".to_string();
                } else {
                    self.toasts_handler
                        .info(format!("Question {} Incorrect :(", self.question_id))
                        .duration(Some(Duration::from_secs(5)));
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                ui.label(format!("Set: {}", self.config_data.set_name))
            });
        });
    }
}
