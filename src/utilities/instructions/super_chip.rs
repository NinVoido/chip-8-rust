//!SCHIP-48 instructions are implemented here
impl crate::utilities::cpu::Cpu {
    ///00FD instruction impl - stops the executing
    pub fn exit(&mut self) {
        self.reset();
        self.stopped = true
    }

    ///FX75 instruction implementation - store V0-Vx in user flags
    pub fn ld_r_vx(&mut self, x: u8) {
        for i in 0..=x as usize {
            self.fl_regs[i] = self.registers[i]
        }
    }

    ///FX85 instruction implementation - restore V0-Vx from user flags
    pub fn ld_vx_r(&mut self, x: u8) {
        for i in 0..=x as usize {
            self.registers[i] = self.fl_regs[i]
        }
    }
    ///Shr&Shl are done COSMAC-VIP way
    ///8XY6
    ///If least-significant bit of Vx is 1, VF is set to 1, otherwise 0. Then Vx>>=1
    pub fn shr_schip(&mut self, x: u8) {
        self.registers[15] = self.registers[x as usize] & 0b1;
        self.registers[x as usize] = self.registers[x as usize] >> 1;
    }
    ///8XYE
    ///If most-significant bit of Vx is 1, VF is set to 1, otherwise 0. Then Vx<<=1
    pub fn shl_schip(&mut self, x: u8) {
        self.registers[15] = self.registers[x as usize] >> 7;
        self.registers[x as usize] = self.registers[x as usize] << 1;
    }
    //TODO - Sadly, IDK how to properly scale CHIP's screen, so I guess I will do nothing (yet)
    ///00FF
    ///Turns on high res mode
    pub fn high(&mut self) {
        self.highres = true
    }
    ///00FE
    ///Turns off high res mode
    pub fn low(&mut self) {
        self.highres = false
    }
    ///FX30
    ///Points index reg to bighex font
    pub fn ld_index_bigfont(&mut self, x: u8) {
        self.i = 0x50 + 80 + 10 * self.registers[x as usize] as u16
    }
    ///00CN
    ///Scrolls display down by N lines
    pub fn scd(&mut self, n: u8) {
        if self.highres {
            for i in 0..n {
                self.h_screen[i as usize] = [false;128]
            }
            for i in n..128 {
                    self.h_screen[i as usize] = self.h_screen[(i-n) as usize]
            }
        } else {
            for i in 0..n {
                self.screen[i as usize] = [false;64]
            }
            for i in n..64 {
                    self.screen[i as usize] = self.screen[(i-n) as usize]
            }
        }
    }
    ///00FB
    ///Scrolls display right by 4
    pub fn scr(&mut self) {
        if self.highres {
            for row in 0..64 {
                for col in 0..4 {
                    self.h_screen[row][col] = false
                }
                for col in 4..128 {
                    self.h_screen[row][col] = self.h_screen[row][col - 4]
                }
            }
        } else {
            for row in 0..32 {
                for col in 0..4 {
                    self.h_screen[row][col] = false
                }
                for col in 4..64 {
                    self.h_screen[row][col] = self.h_screen[row][col - 4]
                }
            }
        }
    }
    ///00FC
    ///Scrolls display left by 4
    pub fn scl(&mut self) {
        if self.highres {
            for row in 0..64 {
                for col in 124..128 {
                    self.h_screen[row][col] = false
                }
                for col in 0..124 {
                    self.h_screen[row][col] = self.h_screen[row][col + 4]
                }
            }
        } else {
            for row in 0..32 {
                for col in 60..64 {
                    self.h_screen[row][col] = false
                }
                for col in 0..60 {
                    self.h_screen[row][col] = self.h_screen[row][col + 4]
                }
            }
        }
    }
    ///DXYN
    ///This is updated version of DRW command, as it works differently on SCHIP-48
    pub fn drw_schip(&mut self, x: u8, y: u8, n: u8){
        self.registers[15] = 0;
        let mut screen: Vec<Vec<bool>>; 
        if self.highres {
            screen = vec!(self.h_screen);
        } else {
            screen = self.screen.as_mut_slice();
        }
        let sprite = &self.ram[self.i as usize..(self.i + if n == 0 { 32 } else { n as u16} ) as usize];
        if n != 0 { 
        for row in 0..n {
            let mut alr_changed = false;
            for i in (0..8).rev() {
                let cords = (
                    ((7 - i) + self.registers[x as usize] as u16) as u8 % 64,
                    ((row as usize + self.registers[y as usize] as usize) % 32) as u8,
                );
                let prev = screen[cords.1 as usize][cords.0 as usize];
                screen[cords.1 as usize][cords.0 as usize] =
                    screen[cords.1 as usize][cords.0 as usize] as u8
                        ^ (sprite[row as usize] >> i & 1)
                        == 1;
                if prev != screen[cords.1 as usize][cords.0 as usize] {
                    self.redraw_needed = true;
                    //If value changed from 1 to 0, pixel was erased, so we change the value of VF
                    //to 1
                    if prev == true && !alr_changed{
                        self.registers[15] += 1;
                        alr_changed = true
                    }
                    
                }
            }
        }
        }
        else {
        for row in 0..16 {
            let mut alr_changed = false;
            for i in (0..16).rev() {
                let sprite_strip: u16 = ((sprite[row] as u16) << 8) + sprite[row+1] as u16;
                let cords = (
                    (((15 - i) + self.registers[x as usize] as usize) % 64) as u8,
                    ((row+ self.registers[y as usize] as usize) % 32) as u8,
                );
                let prev = screen[cords.1 as usize][cords.0 as usize];
                screen[cords.1 as usize][cords.0 as usize] =
                    screen[cords.1 as usize][cords.0 as usize] as u8
                        ^ ((sprite_strip as usize >> i) as u8 & 1)
                        == 1;
                if prev != screen[cords.1 as usize][cords.0 as usize] {
                    self.redraw_needed = true;
                    //If value changed from 1 to 0, pixel was erased, so we change the value of VF
                    //to 1
                    if prev == true && !alr_changed{
                        self.registers[15] += 1;
                        alr_changed = true
                    }
                    
                }
            }
        }
             
        }
    }
}
