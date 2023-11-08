#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use common::config::Config;
use eframe::egui;
use eframe::egui::ScrollArea;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 480.0)),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Dygma Layer Switcher",
        options,
        Box::new(|_cc| Box::<App>::default()),
    )
}

struct App {
    layers: u8,
    config: Config,
}

impl Default for App {
    fn default() -> Self {
        Self {
            layers: 50, // TODO: Adjust to max layer count
            config: Config::load(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let config = &mut self.config;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("➕").clicked() {
                    config.add();
                }
                if ui.button("💾").clicked() {
                    config.save();
                }
                ui.separator();
                ui.label("Base Layer:");
                ui.add(egui::DragValue::new(&mut config.base_layer).clamp_range(1..=self.layers));
                ui.label("COM Port:");
                ui.text_edit_singleline(&mut config.comm_port);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for (index, app) in config.mappings.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        if let Some(exe_name) = &mut app.exe_name {
                            ui.horizontal(|ui| {
                                ui.label("Executable Name:");
                                ui.text_edit_singleline(exe_name);
                            });
                        } else {
                            // If there's no exe name, we can add a button to create one
                            if ui.button("Add Executable Name").clicked() {
                                app.exe_name = Some("".to_string());
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        if let Some(window_title) = &mut app.window_title {
                            ui.horizontal(|ui| {
                                ui.label("Window Title:");
                                ui.text_edit_singleline(window_title);
                            });
                        } else {
                            // If there's no window title, we can add a button to create one
                            if ui.button("Add Window Title").clicked() {
                                app.window_title = Some("".to_string());
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Layer:");
                        ui.add(egui::DragValue::new(&mut app.layer).clamp_range(1..=self.layers));
                    });
                    if ui.button("✖").clicked() {
                        config.remove(index);
                        // Stop processing here to prevent use-after-free
                        return;
                    }
                    ui.separator();
                }
            });
        });
    }
}
