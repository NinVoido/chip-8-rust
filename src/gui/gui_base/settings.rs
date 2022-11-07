impl super::Gui {
    pub(super) fn settings(&mut self, ctx: &egui::Context) {
        egui::Window::new("Settings")
            .open(&mut self.sett_open)
            .show(ctx, |ui| {
                if ui
                    .add(
                        egui::Slider::new(&mut self.clocks_per_fr, 0..=1000)
                            .step_by(10.0)
                            .text("Clocks per frame"),
                    )
                    .changed()
                {
                    self.clock_changed = true
                }
            });
    }
}
