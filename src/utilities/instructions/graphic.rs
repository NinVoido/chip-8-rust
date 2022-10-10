impl crate::utilities::cpu::Cpu {
    ///FX29 instruction implementation
    ///Sets index register to the address of hex character resembling value in register VX
    pub fn ld_index_font(&mut self, x: u8) {
        self.i = 0x50 + 5 * self.registers[x as usize] as u16
    }
    ///00E0 instruction implementation
    ///Clears the screen
    pub fn cls(&mut self) {
        for pixel in self.screen.iter_mut() {
            *pixel = [false; 64]
        }
        self.redraw_needed = true
    }
    ///DXYN instruction implementation
    ///Draws a n-byte sprite at VX, VY
    pub fn drw(&mut self, vx: u8, vy: u8, n: u8) {
        //Defaulting VF to 0
        self.registers[15] = 0;

        let sprite = &self.ram[self.i as usize..(self.i + n as u16) as usize];
        for row in 0..n {
            for i in (0..8).rev() {
                let cords = (
                    ((7 - i) + self.registers[vx as usize]) % 64,
                    (row + self.registers[vy as usize]) % 32,
                );
                let prev = self.screen[cords.1 as usize][cords.0 as usize];
                self.screen[cords.1 as usize][cords.0 as usize] =
                    self.screen[cords.1 as usize][cords.0 as usize] as u8
                        ^ (sprite[row as usize] >> i & 1)
                        == 1;
                if prev != self.screen[cords.1 as usize][cords.0 as usize] {
                    self.redraw_needed = true;
                    //If value changed from 1 to 0, pixel was erased, so we change the value of VF
                    //to 1
                    if prev == true {
                        self.registers[15] = 1;
                    }
                }
            }
        }
    }
}
