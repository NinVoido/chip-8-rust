mod gui;
mod utilities;
use gui::gui_base::Framework;
use pixels::{Error, Pixels, SurfaceTexture};
use utilities::{cpu::Cpu, draw_to_pixels::draw_to_pixels};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const FPS: std::time::Duration = std::time::Duration::from_millis(16);
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
    //Channel for transmitting input from winit to Cpu
    //let (winout, cpuin) = std::sync::mpsc::channel();
    //Channel for transmitting start info
    let (starts, startr) = std::sync::mpsc::channel();
    //Error transmitting from cpu to event loop
    let (errors, errorr) = std::sync::mpsc::channel();
    //Transmit frame from chip to pixels
    let (pixelsscr, pixelsscreen) = std::sync::mpsc::channel();
    //Check if redraw redraw_needed
    //let (redrawn, redrawr) = std::sync::mpsc::channel();
    let cpu_loop = std::thread::spawn(move || {
        //Check if loop should be s t a r t e d
        let mut error: Option<String> = None;
        loop {
            dbg!("Im here");
            let exec_started: (bool, String) = startr.recv().unwrap();
            dbg!("Now Im here");
            chip.reset();
            chip.load_rom(exec_started.1);
            loop {
                let cycle_started = std::time::Instant::now();
                for programs_executed in 1..=10 {
                    let err = chip.execute(None);
                    if let Some(terr) = err.err() {
                        error = Some(format_error(&chip, terr.to_string()));
                        break;
                    }
                }
                std::thread::sleep(FPS - cycle_started.elapsed());
                errors.send(error.clone()).unwrap();
                if error.is_some() {
                    break;
                }

                if chip.redraw_needed {
                    pixelsscr.send(Some(chip.screen)).unwrap();
                    chip.redraw_needed = false
                } else {
                    pixelsscr.send(None).unwrap();
                }
            }
            //Clean the error
            error = None;
        }
    });
    dbg!("Reached the event loop");
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
            starts.send((true, path)).unwrap();
            egui_things.unload_path();
            loop_started = true;
        }
        if loop_started {
            let err = errorr.recv().unwrap();
            if let Some(error) = err {
                egui_things.throw_error(error);
                loop_started = false;
            }
        
        if let Some(screen) = pixelsscreen.recv().unwrap() {
            draw_to_pixels(screen, pixels.get_frame_mut());
        }}
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

fn format_error(chip: &Cpu, error: String) -> String {
    let mut err = format!(
        "Fatal: {};\nInfo:\nIndex: {:X?}\nCommand: {:X?}{:X}\nPC:{}\n",
        error,
        chip.i,
        chip.ram[chip.pc as usize - 2],
        chip.ram[chip.pc as usize - 1],
        chip.pc
    );
    if error == "Popping from empty stack" || error == "Stack overflow" {
        err += format!("Stack info:\n{:?}\n{}", chip.stack.stack, chip.stack.sp).as_str();
    }
    err
}
