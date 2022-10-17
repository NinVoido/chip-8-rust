mod debug_menu;
mod error_menus;
use egui::{ClippedPrimitive, Context, TexturesDelta};
use egui_wgpu::renderer::{RenderPass, ScreenDescriptor};
use pixels::{wgpu, PixelsContext};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

use crate::utilities::cpu::Cpu;
///Main struct representing EGUI
pub struct Framework {
    ///Egui context
    egui_ctx: Context,
    ///Egui state
    egui_state: egui_winit::State,
    ///Screen descriptor
    screen_descriptor: ScreenDescriptor,
    ///Render pass
    rpass: RenderPass,
    ///Paint jobs
    paint_jobs: Vec<ClippedPrimitive>,
    ///Textures
    textures: TexturesDelta,
    ///State of GUI itself
    gui: Gui,
}
///Struct representing UI state
struct Gui {
    ///Show debug screen or not
    debug_open: bool,
    ///Load screen
    start_open: bool,
    ///Picked path to the ROM
    picked_path: Option<String>,
    ///State of RUN ROM button
    rom_choosed: bool,
    ///Error occured or not
    error_occured: bool,
    ///Error itself
    error: Option<String>,
    ///Information about program
    debug_info: Option<DebugInfo>,
    ///Register viewer
    regs_open: bool,
    ///Stack viewer
    stack_open: bool,
    ///Register changed flag
    regs_changed: bool,
    ///Timer changed flag
    timer_changed: bool,
    ///If program is started in debugging mode
    debug_start: bool,
    ///If timer debug window is open
    timers_open: bool,
}

struct DebugInfo {
    registers: [u8; 16],
    pc: u16,
    stack: crate::utilities::cpu::Stack,
    cmd: (u8, u8),
    last_cmd: (u8, u8),
    st: u8,
    dt: u8,
}

impl Framework {
    ///Create gui
    pub fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        width: u32,
        height: u32,
        scale_factor: f32,
        pixels: &pixels::Pixels,
    ) -> Self {
        let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        let mut egui_state = egui_winit::State::new(event_loop);
        egui_state.set_max_texture_side(max_texture_size);
        egui_state.set_pixels_per_point(scale_factor);
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: scale_factor,
        };
        let rpass = RenderPass::new(pixels.device(), pixels.render_texture_format(), 1);
        let textures = TexturesDelta::default();
        let gui = Gui::new();

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            rpass,
            paint_jobs: Vec::new(),
            textures,
            gui,
        }
    }
    ///Passes events from winit to egui
    pub fn handle_event(&mut self, event: &winit::event::WindowEvent) {
        self.egui_state.on_event(&self.egui_ctx, event);
    }
    ///Manages resizing
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.size_in_pixels = [width, height];
        }
    }
    ///Manages scaling
    pub fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.pixels_per_point = scale_factor as f32;
    }
    ///Transports rom path from egui to Main
    ///Some if path was chosen and run button was pressed
    ///None if else
    pub fn rom_started(&self) -> Option<(bool, String)> {
        if let Some(path) = &self.gui.picked_path {
            if self.gui.debug_start {
                return Some((true, path.clone()));
            } else if self.gui.rom_choosed {
                return Some((false, path.clone()));
            }
        }
        None
    }
    ///Unloads path from gui after ROM was launched
    pub fn unload_path(&mut self) {
        self.gui.picked_path = None;
        self.gui.rom_choosed = false;
        self.gui.debug_start = false;
    }
    ///Get new registers from GUI
    pub fn get_info(&mut self) -> (Option<[u8; 16]>, Option<(u8, u8)>) {
        let mut result: (Option<[u8; 16]>, Option<(u8, u8)>) = (None, None);
        if let Some(dbginfo) = &self.gui.debug_info {
            if self.gui.regs_changed {
                result.0 = Some(dbginfo.registers);
                self.gui.regs_changed = false
            }

            if self.gui.timer_changed {
                result.1 = Some((dbginfo.dt, dbginfo.st));
                self.gui.timer_changed = false
            }
        }
        return result;
    }
    ///Transports error from main to error gui
    pub fn throw_error(&mut self, error: String) {
        self.gui.error_occured = true;
        self.gui.error = Some(error)
    }
    ///Gives debug info to the gui
    pub fn debug_send(&mut self, chip: &Cpu) {
        self.gui.debug_info = Some(DebugInfo {
            registers: chip.registers,
            stack: chip.stack,
            pc: chip.pc,
            last_cmd: if let Some(dbg) = &self.gui.debug_info {
                dbg.cmd
            } else {
                (0, 0)
            },
            cmd: (chip.ram[chip.pc as usize], chip.ram[chip.pc as usize + 1]),
            st: chip.st,
            dt: chip.dt,
        });
        //        println!("{:X?}|{:X?}", chip.ram[chip.pc as usize], chip.ram[chip.pc as usize + 1])
    }
    ///Tells egui to open debug menu
    pub fn open_debug(&mut self) {
        self.gui.debug_open = true
    }
    ///Manages rendering egui
    pub fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) {
        for (id, image_delta) in &self.textures.set {
            self.rpass
                .update_texture(&context.device, &context.queue, *id, image_delta);
        }
        self.rpass.update_buffers(
            &context.device,
            &context.queue,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        self.rpass.execute(
            encoder,
            render_target,
            &self.paint_jobs,
            &self.screen_descriptor,
            None,
        );

        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.rpass.free_texture(id);
        }
    }
    ///Prepares egui for rendering
    pub fn prepare(&mut self, window: &Window) {
        let raw_input = self.egui_state.take_egui_input(window);
        let output = self.egui_ctx.run(raw_input, |egui_ctx| {
            self.gui.ui(egui_ctx);
            self.gui.debug_ui(egui_ctx);
            self.gui.error_menus(egui_ctx);
        });

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(window, &self.egui_ctx, output.platform_output);
        self.paint_jobs = self.egui_ctx.tessellate(output.shapes);
    }
}

impl Gui {
    ///Creates a new gui with default states
    fn new() -> Self {
        Self {
            debug_open: false,
            start_open: true,
            picked_path: None,
            rom_choosed: false,
            error_occured: false,
            error: None,
            debug_info: None,
            regs_open: false,
            regs_changed: false,
            stack_open: false,
            debug_start: false,
            timers_open: false,
            timer_changed: false,
        }
    }
    ///Creates main UI
    fn ui(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load file").clicked() {
                        self.start_open = true;
                        ui.close_menu();
                    }
                });

                ui.menu_button("Debug", |ui| {
                    if ui.button("Open menu").clicked() {
                        self.debug_open = true;
                        ui.close_menu();
                    }
                })
            });
        });

        egui::Window::new("CHIP-8 emu, WIP")
            .open(&mut self.start_open)
            .show(ctx, |ui| {
                ui.label("This is a CHIP-8 emulator.");

                if ui.button("Open ROM").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.picked_path = Some(path.display().to_string());
                    }
                };

                if let Some(picked_path) = &self.picked_path {
                    ui.horizontal(|ui| {
                        ui.label("Picked file:");
                        ui.monospace(picked_path);

                        ui.separator();

                        if ui.button("Run ROM").clicked() {
                            self.rom_choosed = true;
                        }

                        ui.separator();

                        if ui.button("Debug ROM").clicked() {
                            self.debug_start = true
                        }
                    });
                }
            });
    }
}
