impl crate::utilities::cpu::Cpu {
    ///EX9E instruction implementation
    ///Skips next instruction if certain key is pressed
    pub fn skp(&mut self, x: u8) {
        if self.registers[x as usize] <= 0xf {
            if self.keypad[self.registers[x as usize] as usize] {
                self.pc += 2
            }
        }
    }
    ///EXA1 instruction implementation
    ///Skips next instruction if certain key is NOT pressed
    pub fn sknp(&mut self, x: u8) {
        if self.registers[x as usize] <= 0xf {
            if !self.keypad[self.registers[x as usize] as usize] {
                self.pc += 2
            }
        }
    }
    ///FX0A instruction implementation
    ///Waits for a key to be pressed, than puts hex value of pressed key into register VX
    pub fn ld_keyboard(&mut self, x: u8) {
        self.scan_info = (true, x)
    }
    ///Puts scanned value into register
    pub fn put_scanned(&mut self, chr: char) {
        self.registers[self.scan_info.1 as usize] = to_hex(chr);
        self.scan_info.0 = false
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

fn to_hex(chr: char) -> u8 {
    match chr.to_ascii_lowercase(){
        'x' => 0, 
        '1' => 1,
        '2' => 2,
        '3' => 3,
        'q' => 4,
        'w' => 5,
        'e' => 6,
        'a' => 7,
        's' => 8,
        'd' => 9,
        'z' => 10,
        'c' => 11,
        '4' => 12,
        'r' => 13,
        'f' => 14,
        'v' => 15,
        _ => 0, 
    }
}
