use crate::structs::*;
use crate::templates;
use eframe::egui::{CentralPanel, Context, Slider, TopBottomPanel};
use eframe::{egui, Frame, Storage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const MAX_LAYERS: u8 = 10;
const EDIT_TEXT: &str = "Placeholder";

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct DygmaLayerSwitcher {
    pub logging: bool,
    pub port: String,
    pub base_layer: u8,
    pub mappings: BTreeMap<u8, Layer>,

    #[serde(skip)]
    pub editing_port: bool,
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

            editing_port: false,
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

    fn logging_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Logging");
            ui.checkbox(&mut self.logging, "");
        });
    }

    fn port_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Port");
            templates::editable_label(ui, &mut self.port, &mut self.editing_port);
        });
    }

    fn base_layer_control(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Base Layer");
            ui.add(Slider::new(&mut self.base_layer, 1..=MAX_LAYERS - 1).trailing_fill(true));
        });
    }

    fn top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.collapsing("Settings", |ui| {
                self.logging_control(ui);
                self.port_control(ui);
                self.base_layer_control(ui);
            });
        });
    }

    fn central_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
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
                        mode: Mode::Window(EDIT_TEXT.to_string()),
                        is_enabled: true,
                    });
                }

                if add_process {
                    layer.apps.push(App {
                        mode: Mode::Process(EDIT_TEXT.to_string()),
                        is_enabled: true,
                    });
                }

                if add_parent {
                    layer.apps.push(App {
                        mode: Mode::Parent(Parent {
                            process: EDIT_TEXT.to_string(),
                            excludes: vec![],
                        }),
                        is_enabled: true,
                    });
                }
            }
        });
    }
}

impl eframe::App for DygmaLayerSwitcher {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.top_panel(ctx);
        self.central_panel(ctx);
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
