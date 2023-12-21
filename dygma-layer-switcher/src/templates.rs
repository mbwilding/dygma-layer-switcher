use crate::verbiage;
use eframe::egui;
use eframe::egui::CollapsingHeader;

pub fn editable_label(
    ui: &mut egui::Ui,
    value: &mut String,
    editing: &mut bool,
    hint: Option<&str>,
) -> bool {
    if *editing {
        let widget = ui.text_edit_singleline(value);

        if widget.lost_focus() {
            *editing = false;
        }

        if widget.changed() {
            return true;
        }

        if !widget.has_focus() {
            widget.request_focus();
        }
    } else if ui
        .button(value.as_str())
        .on_hover_text(hint.unwrap_or_default())
        .clicked()
    {
        *editing = true;
    }

    false
}

pub fn editable_collapsing<F: FnMut(&mut egui::Ui)>(
    ui: &mut egui::Ui,
    value: &mut String,
    editing: &mut bool,
    mut content: F,
) {
    if *editing {
        let widget = ui.text_edit_singleline(value);

        if widget.lost_focus() {
            *editing = false;
        }

        if !widget.has_focus() {
            widget.request_focus();
        }
    } else {
        let header_response = CollapsingHeader::new(value.clone())
            .default_open(false)
            .show(ui, |ui| {
                content(ui);
            })
            .header_response;

        if header_response
            .on_hover_text(verbiage::RENAME_HINT)
            .secondary_clicked()
        {
            *editing = true;
        }
    }
}
