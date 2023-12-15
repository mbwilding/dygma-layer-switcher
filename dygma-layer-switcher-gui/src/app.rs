use crate::structs::{App, Layer, Mode, Parent};
use eframe::egui;
use eframe::egui::{CentralPanel, Context, Slider};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const MAX_LAYERS: u8 = 10;
const DEFAULT_TEXT: &str = "Right click to edit";

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct DygmaLayerSwitcher {
    pub logging: bool,
    pub port: String,
    pub base_layer: u8,
    pub mappings: BTreeMap<u8, Layer>,
}

impl Default for DygmaLayerSwitcher {
    fn default() -> Self {
        Self {
            logging: false,
            port: String::new(),
            base_layer: 1,
            mappings: (0..MAX_LAYERS)
                .map(|i| {
                    (
                        i,
                        Layer {
                            name: format!("Layer {}", i + 1),
                            apps: vec![],
                            is_being_renamed: false,
                        },
                    )
                })
                .collect::<BTreeMap<u8, Layer>>(),
        }
    }
}

impl DygmaLayerSwitcher {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for DygmaLayerSwitcher {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dygma Layer Switcher");

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Logging");
                ui.checkbox(&mut self.logging, "");
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Port");
                ui.text_edit_singleline(&mut self.port);
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Base Layer");
                ui.add(Slider::new(&mut self.base_layer, 1..=MAX_LAYERS - 1).trailing_fill(true));
            });

            ui.separator();

            for (_index, layer) in self.mappings.iter_mut() {
                let mut rename_layer = false;
                let mut add_window = false;
                let mut add_process = false;
                let mut add_parent = false;

                ui.horizontal(|ui| {
                    if layer.is_being_renamed {
                        let focus_lost = ui.text_edit_singleline(&mut layer.name).lost_focus();
                        let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
                        if focus_lost && enter_pressed {
                            layer.is_being_renamed = false;
                        }
                    } else {
                        let collapsing_response = ui
                            .collapsing(&layer.name, |ui| {
                                ui.horizontal(|ui| {
                                    if ui.button("Add Window").clicked() {
                                        add_window = true;
                                    }
                                    if ui.button("Add Process").clicked() {
                                        add_process = true;
                                    }
                                    if ui.button("Add Parent").clicked() {
                                        add_parent = true;
                                    }
                                });

                                layer.apps.iter_mut().for_each(|app| match &mut app.mode {
                                    Mode::Window(window) => {
                                        ui.horizontal(|ui| {
                                            ui.checkbox(&mut app.is_enabled, "");
                                            ui.label(format!("Window: {:?}", window));
                                        });
                                    }
                                    Mode::Process(process) => {
                                        ui.label(format!("Process: {}", process));
                                    }
                                    Mode::Parent(parent) => {
                                        ui.label(format!("Parent: {}", parent.process));
                                        parent.excludes.iter().for_each(|exclude| {
                                            ui.label(format!("Exclude: {}", exclude));
                                        });
                                    }
                                });
                            })
                            .header_response;

                        rename_layer = collapsing_response.secondary_clicked();
                    }
                });

                if rename_layer {
                    layer.is_being_renamed = true;
                }

                if add_window {
                    layer.apps.push(App {
                        mode: Mode::Window(DEFAULT_TEXT.to_string()),
                        is_enabled: true,
                    });
                }

                if add_process {
                    layer.apps.push(App {
                        mode: Mode::Process(DEFAULT_TEXT.to_string()),
                        is_enabled: true,
                    });
                }

                if add_parent {
                    layer.apps.push(App {
                        mode: Mode::Parent(Parent {
                            process: DEFAULT_TEXT.to_string(),
                            excludes: vec![],
                        }),
                        is_enabled: true,
                    });
                }
            }
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
