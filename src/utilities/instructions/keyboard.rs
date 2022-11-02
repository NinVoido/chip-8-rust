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

fn to_hex(chr: char) -> u8 {
    match chr.to_ascii_lowercase() {
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
