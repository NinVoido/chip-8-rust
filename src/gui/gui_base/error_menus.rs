//!Module for creating error messages
//!Error handling itself is done in main
impl crate::gui::gui_base::Gui {
    ///Creates an error message window
    pub(super) fn error_menus(&mut self, ctx: &egui::Context) {
        egui::Window::new("⚠Error menu⚠")
            .open(&mut self.error_occured)
            .show(ctx, |ui| {
                ui.label("Error occured");
                if let Some(err) = &self.error {
                    ui.label(format!("Error info:\n{}", err))
                } else {
                    ui.label("Uknown error occured")
                };
                if !self.debug_open {
                    if ui.button("Open debug for maitenance").clicked() {
                        self.debug_open = true
                    }
                }
            });
    }
}
