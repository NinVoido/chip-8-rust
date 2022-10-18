impl crate::utilities::cpu::Cpu {
    ///EX9E instruction implementation
    ///Skips next instruction if certain key is pressed
    pub fn skp(&mut self, x: u8) {
        if self.keypad[x as usize] {
            self.pc += 2
        }
    }
    ///EXA1 instruction implementation
    ///Skips next instruction if certain key is NOT pressed
    pub fn sknp(&mut self, x: u8) {
        if !self.keypad[x as usize] {
            self.pc += 2
        }
    }
    ///FX0A instruction implementation
    ///Waits for a key to be pressed, than puts hex value of pressed key into register VX
    pub fn ld_keyboard(&mut self, x: u8) {
        self.scan_info = (true, x)
    }
}

fn to_keycode(keynum: u8) -> winit::event::VirtualKeyCode {
    match keynum {
        0 => winit::event::VirtualKeyCode::X,
        1 => winit::event::VirtualKeyCode::Key1,
        2 => winit::event::VirtualKeyCode::Key2,
        3 => winit::event::VirtualKeyCode::Key3,
        4 => winit::event::VirtualKeyCode::Q,
        5 => winit::event::VirtualKeyCode::W,
        6 => winit::event::VirtualKeyCode::E,
        7 => winit::event::VirtualKeyCode::A,
        8 => winit::event::VirtualKeyCode::S,
        9 => winit::event::VirtualKeyCode::D,
        10 => winit::event::VirtualKeyCode::Z,
        11 => winit::event::VirtualKeyCode::C,
        12 => winit::event::VirtualKeyCode::Key4,
        13 => winit::event::VirtualKeyCode::R,
        14 => winit::event::VirtualKeyCode::F,
        15 => winit::event::VirtualKeyCode::V,
        _ => winit::event::VirtualKeyCode::Unlabeled,
    }
}

//fn to_hex(
