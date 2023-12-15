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
