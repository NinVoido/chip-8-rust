//!Module for creating debug windows
impl crate::gui::gui_base::Gui {
    ///Creates debug GUI
    pub(super) fn debug_ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Debug menu")
            .open(&mut self.debug_open)
            .show(ctx, |ui| {
                if let Some(dbginfo) = &self.debug_info {
                    ui.label(format!("Last executing address: {:X?}", dbginfo.pc));

                    ui.label(format!(
                        "Next instruction: {:X?}|{:X?}",
                        dbginfo.cmd.0, dbginfo.cmd.1
                    ));

                    ui.label(format!(
                        "Last instruction: {:X?}|{:X}",
                        dbginfo.last_cmd.0, dbginfo.last_cmd.1
                    ));

                    if ui.button("Open registers").clicked() {
                        self.regs_open = true;
                    }

                    if ui.button("Open stack info").clicked() {
                        self.stack_open = true;
                    }

                    if ui.button("Open timers").clicked() {
                        self.timers_open = true
                    }

                    if ui.button("Open keypad").clicked() {
                        self.keypad_open = true
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
                        .columns(egui_extras::Size::initial(20.0).at_least(20.0), 16);

                    table
                        .body(|mut body| {
                                body.row(20.0, |mut row| {
                                    for col_ind in 0..16 {
                                    row.col(|ui| {
                                        ui.centered_and_justified(|ui| {
                                            ui.label(format!("{:X?}", col_ind));
                                        });
                                    });
                                    }
                                                                   });
                                body.row(20.0, |mut row| {
                                    for col_ind in 0..16 {
                                     row.col(|ui| {
                                        ui.centered_and_justified(|ui| {
                                            if ui
                                                .add(
                                                    egui::DragValue::new(
                                                        &mut dbginfo.registers[col_ind as usize],
                                                    )
                                                    .clamp_range(0..=255),
                                                )
                                                .changed()
                                            {
                                                self.regs_changed = true;
                                            }
                                        });
                                    });
                                    }

                                });
                        });
                }
            });

        egui::Window::new("Stack")
            .open(&mut self.stack_open)
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
                                ui.heading("Address");
                            });
                        })
                        .body(|mut body| {
                            let mut height;
                            for row_index in (0..dbginfo.stack.sp).rev() {
                                if dbginfo.stack.sp == row_index {
                                    height = 30.0
                                } else {
                                    height = 20.0
                                }
                                body.row(height, |mut row| {
                                    row.col(|ui| {
                                        ui.heading(row_index.to_string());
                                    });

                                    row.col(|ui| {
                                        ui.heading(
                                            dbginfo.stack.stack[row_index as usize].to_string(),
                                        );
                                    });
                                });
                            }
                        });
                }
            });

        egui::Window::new("Timers")
            .open(&mut self.timers_open)
            .show(ctx, |ui| {
                if let Some(dbginfo) = &mut self.debug_info {
                    if ui
                        .add(egui::Slider::new(&mut dbginfo.dt, 0..=255).text("Delay timer"))
                        .changed()
                        || ui
                            .add(egui::Slider::new(&mut dbginfo.st, 0..=255).text("Sound timer"))
                            .changed()
                    {
                        self.timer_changed = true
                    }
                }
            });

        egui::Window::new("Keypad")
            .open(&mut self.keypad_open)
            .show(ctx, |ui| {
                if let Some(dbginfo) = &mut self.debug_info {
                    let table = egui_extras::TableBuilder::new(ui)
                        .columns(egui_extras::Size::initial(40.0), 4);
                    table.body(|mut body| {
                        for row_index in 0..4 {
                            body.row(40.0, |mut row| {
                                row.col(|ui| {
                                    if ui
                                        .checkbox(
                                            &mut dbginfo.keypad[KEYPAD[row_index * 4]],
                                            format!("{:X?}", KEYPAD[row_index * 4]),
                                        )
                                        .changed()
                                    {
                                        self.keypad_changed = true
                                    };
                                });
                                row.col(|ui| {
                                    if ui
                                        .checkbox(
                                            &mut dbginfo.keypad[KEYPAD[row_index * 4 + 1]],
                                            format!("{:X?}", KEYPAD[row_index * 4 + 1]),
                                        )
                                        .changed()
                                    {
                                        self.keypad_changed = true
                                    };
                                });
                                row.col(|ui| {
                                    if ui
                                        .checkbox(
                                            &mut dbginfo.keypad[KEYPAD[row_index * 4 + 2]],
                                            format!("{:X?}", KEYPAD[row_index * 4 + 2]),
                                        )
                                        .changed()
                                    {
                                        self.keypad_changed = true
                                    };
                                });
                                row.col(|ui| {
                                    if ui
                                        .checkbox(
                                            &mut dbginfo.keypad[KEYPAD[row_index * 4 + 3]],
                                            format!("{:X?}", KEYPAD[row_index * 4 + 3]),
                                        )
                                        .changed()
                                    {
                                        self.keypad_changed = true
                                    };
                                });
                            });
                        }
                    });
                }
            });
    }
}

const KEYPAD: [usize; 16] = [1, 2, 3, 12, 4, 5, 6, 13, 7, 8, 9, 14, 10, 0, 11, 15];
