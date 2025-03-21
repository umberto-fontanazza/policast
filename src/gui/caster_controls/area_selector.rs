use super::Gui;

impl Gui {
    pub fn area_selector(&mut self, ui: &mut egui::Ui) {
        let pointer = ui.input(|i| i.pointer.clone()); // Usa un closure per accedere al pointer
        let capturer = &mut self.capturer;
        if capturer.selecting_area {
            // Cattura il clic iniziale
            if pointer.any_pressed() && capturer.start_point.is_none() {
                if let Some(pos) = pointer.interact_pos() {
                    capturer.start_point = Some(pos);
                }
            }

            // Aggiorna il punto finale mentre si trascina
            if pointer.primary_down() {
                if let Some(pos) = pointer.interact_pos() {
                    capturer.end_point = Some(pos);
                }
            }

            // Rilascia il mouse per confermare la selezione
            if pointer.any_released()
                && capturer.start_point.is_some()
                && capturer.end_point.is_some()
            {
                if let (Some(start), Some(end)) = (capturer.start_point, capturer.end_point) {
                    // Calcola l'area selezionata
                    let x = start.x.min(end.x) as u32;
                    let y = start.y.min(end.y) as u32;
                    let width = (start.x - end.x).abs() as u32;
                    let height = (start.y - end.y).abs() as u32;

                    capturer.selected_area = Some((x, y, width, height));
                    capturer.selecting_area = false; // Disabilita la selezione
                }
            }

            // Disegna un rettangolo durante la selezione
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

        // Mostra l'area selezionata se esiste
        if let Some((x, y, width, height)) = capturer.selected_area {
            ui.label(format!(
                "Selected Area: Position ({}, {}), Size ({}, {})",
                x, y, width, height
            ));
        }
    }
}
