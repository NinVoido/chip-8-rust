mod gui;
mod utilities;

use gui::gui_base::Framework;
use pixels::{Error, Pixels, SurfaceTexture};
use utilities::cpu::Cpu;
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let mut chip = Cpu::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("CHIP-8 emu")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let (mut pixels, mut egui_things) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
        let egui_things = Framework::new(
            &event_loop,
            window_size.width,
            window_size.height,
            scale_factor,
            &pixels,
        );

        (pixels, egui_things)
    };
    let mut loop_started = false;
    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(winit::event::VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(scale_factor) = input.scale_factor() {
                egui_things.scale_factor(scale_factor);
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
                egui_things.resize(size.width, size.height);
            }
            window.request_redraw();
        }
        if let Some(path) = egui_things.rom_started() {
            chip.reset();
            chip.load_rom(path).unwrap();
            egui_things.unload_path();
            loop_started = true;
        }
        if loop_started {
            if let Some(error) = chip.execute(&input).err() {
                loop_started = false;
                let mut err = format!(
                    "Fatal: {};\nBacktrace:\nIndex: {:X?}\nCommand: {:X?}{:X}\n",
                    error,
                    chip.i,
                    chip.ram[chip.pc as usize - 2],
                    chip.ram[chip.pc as usize - 1]
                );
                if error == "Popping from empty stack" || error == "Stack overflow" {
                    err += format!("Stack backtrace:\n{:?}\n{}", chip.stack.stack, chip.stack.sp).as_str();
                }
                egui_things.throw_error(err.to_string());
            }
        }
        if chip.redraw_needed {
            chip.draw_to_pixels(pixels.get_frame_mut());
            window.request_redraw();
        }
        match event {
            Event::WindowEvent { event, .. } => {
                egui_things.handle_event(&event);
            }
            Event::RedrawRequested(_) => {
                egui_things.prepare(&window);

                let render_result = pixels.render_with(|encoder, render_target, context| {
                    context.scaling_renderer.render(encoder, render_target);

                    egui_things.render(encoder, render_target, context);

                    Ok(())
                });

                if render_result.is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}
