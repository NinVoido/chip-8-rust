mod debug_menu;
mod error_menus;
use egui::{ClippedPrimitive, Context, TexturesDelta};
use egui_wgpu::renderer::{RenderPass, ScreenDescriptor};
use pixels::{wgpu, PixelsContext};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;
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
    pub fn rom_started(&self) -> Option<String> {
        if self.gui.rom_choosed {
            return self.gui.picked_path.clone();
        }
        None
    }
    ///Unloads path from gui after ROM was launched
    pub fn unload_path(&mut self) {
        self.gui.picked_path = None;
        self.gui.rom_choosed = false;
    }
    ///Transports error from main to error gui
    pub fn throw_error(&mut self, error: String) {
        self.gui.error_occured = true;
        self.gui.error = Some(error)
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
                    });
                }
            });
    }
}
