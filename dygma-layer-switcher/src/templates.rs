use eframe::egui;

pub fn editable_label(ui: &mut egui::Ui, value: &mut String, editing: &mut bool) {
    if *editing {
        if ui.text_edit_singleline(value).lost_focus() {
            *editing = false;
        }
    } else if ui.button(value.as_str()).clicked() {
        *editing = true;
    }
}

pub fn editable_collapsing<F: FnMut(&mut egui::Ui)>(
    ui: &mut egui::Ui,
    value: &mut String,
    editing: &mut bool,
    mut content: F,
) {
    if *editing {
        if ui.text_edit_singleline(value).lost_focus() {
            *editing = false;
        }
    } else {
        let header_response = ui
            .collapsing(&value.clone(), |ui| {
                content(ui);
            })
            .header_response;

        if header_response.secondary_clicked() {
            *editing = true;
        }
    }
}
