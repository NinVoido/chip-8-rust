//!Module for creating debug windows
impl crate::gui::gui_base::Gui {
    ///Creates debug GUI
    pub(super) fn debug_ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Debug menu")
            .open(&mut self.debug_open)
            .show(ctx, |ui| {
                if let Some(dbginfo) = &self.debug_info {
                    ui.label(format!(
                        "Stack: {:?}\nStack pointer: {}\n",
                        dbginfo.stack.stack, dbginfo.stack.sp
                    ));
                    
                    ui.label(format!("Last instruction: {:X?}{:X?}", dbginfo.cmd.0, dbginfo.cmd.1));

                    if ui.button("Open registers").clicked() {
                        self.regs_open = true;
                    }
                } else {
                    ui.label("No debug information available");
                }
            });

        egui::Window::new("Registers")
            .open(&mut self.regs_open)
            .show(ctx, |ui| {
                if let Some(dbginfo) = &mut self.debug_info {

                    let table = egui_extras::TableBuilder::new(ui)
                        .striped(true)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                        .column(egui_extras::Size::initial(60.0).at_least(40.0))
                        .column(egui_extras::Size::initial(60.0).at_least(40.0))
                        .resizable(true);

                    table
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading("Index");
                            });
                            header.col(|ui| {
                                ui.heading("Value");
                            });
                        })
                        .body(|mut body| {
                            for row_index in 0..16 {
                                body.row(20.0, |mut row| {
                                    row.col(|ui| {
                                        ui.centered_and_justified(|ui| {
                                            ui.label(format!("{:X?}", row_index));
                                        });
                                    });
                                    row.col(|ui| {
                                        ui.centered_and_justified(|ui| {
                                            if ui.add(egui::DragValue::new(
                                                &mut dbginfo.registers[row_index as usize],
                                            ).clamp_range(0..=255)).changed() {
                                                self.regs_changed = true;
                                            }
                                        });
                                    });
                                });
                            }
                        });
                }
            });
    }
}
