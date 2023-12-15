use crate::structs::*;
use crate::templates;
use eframe::egui::{CentralPanel, Context, DragValue, TopBottomPanel};
use eframe::{egui, Frame, Storage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const MAX_LAYERS: u8 = 10;

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
                .map(|i| (i, Layer::new(i)))
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
            ui.add(DragValue::new(&mut self.base_layer));
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
                templates::editable_collapsing(ui, &mut layer.name, &mut layer.is_editing, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Add Window").clicked() {
                            layer.apps.push(App::new_window());
                        }
                        if ui.button("Add Process").clicked() {
                            layer.apps.push(App::new_process());
                        }
                        if ui.button("Add Parent").clicked() {
                            layer.apps.push(App::new_parent());
                        }
                    });

                    ui.collapsing("Windows", |ui| {
                        layer.apps.iter_mut().for_each(|app| {
                            if let Mode::Window(window) = &mut app.mode {
                                ui.horizontal(|ui| {
                                    ui.checkbox(&mut app.is_enabled, "");
                                    templates::editable_label(
                                        ui,
                                        &mut window.name,
                                        &mut window.is_editing,
                                    );
                                });
                            }
                        });
                    });

                    ui.collapsing("Processes", |ui| {
                        layer.apps.iter_mut().for_each(|app| {
                            if let Mode::Process(process) = &mut app.mode {
                                ui.horizontal(|ui| {
                                    ui.checkbox(&mut app.is_enabled, "");
                                    templates::editable_label(
                                        ui,
                                        &mut process.name,
                                        &mut process.is_editing,
                                    );
                                });
                            }
                        });
                    });

                    ui.collapsing("Parents", |ui| {
                        layer.apps.iter_mut().for_each(|app| {
                            if let Mode::Parent(parent) = &mut app.mode {
                                ui.horizontal(|ui| {
                                    ui.checkbox(&mut app.is_enabled, "");
                                    if ui.button("Add Exclude").clicked() {
                                        parent.excludes.push(Exclude::new());
                                    }
                                    templates::editable_label(
                                        ui,
                                        &mut parent.name,
                                        &mut parent.is_editing,
                                    );
                                });
                                ui.indent("Excludes", |ui| {
                                    parent.excludes.iter_mut().for_each(|exclude| {
                                        ui.horizontal(|ui| {
                                            ui.checkbox(&mut exclude.is_enabled, "");
                                            templates::editable_label(
                                                ui,
                                                &mut exclude.name,
                                                &mut exclude.is_editing,
                                            );
                                        });
                                    });
                                });
                            }
                        });
                    });
                });
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
