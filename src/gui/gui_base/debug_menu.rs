impl crate::gui::gui_base::Gui {
    pub(super) fn debug_ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Debug menu")
            .open(&mut self.debug_open)
            .show(ctx, |ui| {
                ui.label("Current executing command:");
            });
    }
}
