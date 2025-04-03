use egui::Rect;

use super::Gui;

impl Gui {
    pub fn area_selector(&mut self, ui: &mut egui::Ui, preview_rect: &Rect) {
        let pointer = ui.input(|i| i.pointer.clone());
        let capturer = &mut self.capturer;
        if !capturer.selecting_area {
            return;
        }
        // Cattura il clic iniziale
        if pointer.any_pressed() && capturer.start_point.is_none() {
            if let Some(mut pos) = pointer.interact_pos() {
                pos = preview_rect.clamp(pos);
                capturer.start_point = Some(pos);
            }
        }

        // Aggiorna il punto finale mentre si trascina
        if pointer.primary_down() {
            if let Some(mut pos) = pointer.interact_pos() {
                pos = preview_rect.clamp(pos);
                capturer.end_point = Some(pos);
            }
        }

        // Rilascia il mouse per confermare la selezione
        if pointer.any_released() && capturer.start_point.is_some() && capturer.end_point.is_some()
        {
            if let (Some(start), Some(end)) = (capturer.start_point, capturer.end_point) {
                capturer.selected_area = Some(Rect::from_two_pos(start, end));
                capturer.selecting_area = false;
            }
        }

        // Display area being selected
        if let (Some(start), Some(end)) = (capturer.start_point, capturer.end_point) {
            let rect = egui::Rect::from_two_pos(start, end);
            ui.painter().rect(
                rect,
                0.0,
                egui::Color32::from_rgba_premultiplied(150, 150, 200, 100),
                egui::Stroke::new(1.0, egui::Color32::GRAY),
            );
        }
    }
}
