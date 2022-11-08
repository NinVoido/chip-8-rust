//!SCHIP-48 instructions are implemented here
impl crate::utilities::cpu::Cpu {
    ///00FD\
    ///Stops the executing
    pub fn exit(&mut self) {
        self.reset();
        self.stopped = true
    }

    ///FX75\
    ///Store V0-Vx in user flags
    pub fn ld_r_vx(&mut self, x: u8) {
        for i in 0..=x as usize {
            self.fl_regs[i] = self.registers[i]
        }
    }

    ///FX85\
    ///Restore V0-Vx from user flags
    pub fn ld_vx_r(&mut self, x: u8) {
        for i in 0..=x as usize {
            self.registers[i] = self.fl_regs[i]
        }
    }
    ///8XY6\
    ///If least-significant bit of Vx is 1, VF is set to 1, otherwise 0. Then Vx>>=1
    pub fn shr_schip(&mut self, x: u8) {
        self.registers[15] = self.registers[x as usize] & 0b1;
        self.registers[x as usize] = self.registers[x as usize] >> 1;
    }
    ///8XYE\
    ///If most-significant bit of Vx is 1, VF is set to 1, otherwise 0. Then Vx<<=1
    pub fn shl_schip(&mut self, x: u8) {
        self.registers[15] = self.registers[x as usize] >> 7;
        self.registers[x as usize] = self.registers[x as usize] << 1;
    }
    //TODO - Sadly, IDK how to properly scale CHIP's screen, so I guess I will do nothing (yet)
    ///00FF\
    ///Turns on high res mode
    pub fn high(&mut self) {
        self.highres = true;
        self.screen = vec![vec![false; 128]; 64]
    }
    ///00FE\
    ///Turns off high res mode
    pub fn low(&mut self) {
        self.highres = false;
        self.screen = vec![vec![false; 64]; 32]
    }
    ///FX30\
    ///Points index reg to bighex font
    pub fn ld_index_bigfont(&mut self, x: u8) {
        self.i = 0x50 + 80 + 10 * self.registers[x as usize] as u16
    }
    ///00CN\
    ///Scrolls display down by N lines
    pub fn scd(&mut self, n: u8) {
        for i in (n as usize..self.screen.len()).rev() {
            self.screen[i] = self.screen[i - n as usize].clone()
        }
        for i in 0..n as usize {
            self.screen[i as usize] = vec![false; self.screen[0].len()]
        }

        self.redraw_needed = true;
    }
    ///00FB\
    ///Scrolls display right by 4
    pub fn scr(&mut self) {
        for row in 0..self.screen.len() {
            for col in 0..4 {
                self.screen[row][col] = false
            }
            for col in 4..self.screen[0].len() {
                self.screen[row][col] = self.screen[row][col - 4]
            }
        }
        self.redraw_needed = true
    }
    ///00FC\
    ///Scrolls display left by 4
    pub fn scl(&mut self) {
        for row in 0..self.screen.len() {
            for col in 124..self.screen[0].len() {
                self.screen[row][col] = false
            }
            for col in 0..self.screen[0].len() - 4 {
                self.screen[row][col] = self.screen[row][col + 4]
            }
        }
        self.redraw_needed = true
    }
    ///DXYN\
    ///This is updated version of DRW command, as it works differently on SCHIP-48
    pub fn drw_schip(&mut self, x: u8, y: u8, n: u8) {
        self.registers[15] = 0;
        let sprite =
            &self.ram[self.i as usize..(self.i + if n == 0 { 32 } else { n as u16 }) as usize];
        if n != 0 {
            for row in 0..n {
                let mut alr_changed = false;
                for i in (0..8).rev() {
                    let cords = (
                        ((7 - i) + self.registers[x as usize] as u16) as u8
                            % self.screen[0].len() as u8,
                        ((row as usize + self.registers[y as usize] as usize) % self.screen.len())
                            as u8,
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
                        if prev == true && !alr_changed {
                            self.registers[15] += 1;
                            alr_changed = true
                        }
                    }
                }
            }
        } else {
            for row in 0..16 {
                let mut alr_changed = false;
                let sprite_strip = ((sprite[row * 2] as u16) << 8) + sprite[row * 2 + 1] as u16;
                for i in (0..16).rev() {
                    let cords = (
                        (((15 - i) + self.registers[x as usize] as usize) % self.screen[0].len())
                            as u8,
                        ((row + self.registers[y as usize] as usize) % self.screen.len()) as u8,
                    );
                    let prev = self.screen[cords.1 as usize][cords.0 as usize];
                    self.screen[cords.1 as usize][cords.0 as usize] =
                        self.screen[cords.1 as usize][cords.0 as usize] as u8
                            ^ ((sprite_strip as usize >> i) as u8 & 1)
                            == 1;
                    if prev != self.screen[cords.1 as usize][cords.0 as usize] {
                        self.redraw_needed = true;
                        //If value changed from 1 to 0, pixel was erased, so we change the value of VF
                        //to 1
                        if prev == true && !alr_changed {
                            self.registers[15] += 1;
                            alr_changed = true
                        }
                    }
                }
            }
        }
    }
}
