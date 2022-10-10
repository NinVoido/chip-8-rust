impl crate::gui::gui_base::Gui {

    pub(super) fn error_menus(&mut self, ctx: &egui::Context,){
        egui::Window::new("⚠Error menu⚠")
            .open(&mut self.error_occured)
            .show(ctx, |ui| {
                ui.label("Error occured");
                if let Some(err) = &self.error{
                    ui.label(format!("Backtrace:\n{}", err))
                } else {
                    ui.label("Uknown error occured")
                }
            });
    }
}
