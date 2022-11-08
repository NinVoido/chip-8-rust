//!Arithmetic (8___) instructions are implemented here
impl crate::utilities::cpu::Cpu {
    ///8XY0\
    ///Sets register VX to the value of VY
    pub fn ld_reg(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize]
    }
    ///8XY1\
    ///Sets register VX to the VX | VY
    pub fn or(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
    }
    ///8XY2\
    ///Sets register VX to the VX && VY
    pub fn and(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
    }
    ///8XY3\
    ///Sets register VX to the VX ^ VY
    pub fn xor(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize]
    }
    ///8XY4\
    ///Adds value of register VY to the VX\
    ///Sets VF flag to 1 if the addition overflows 8 bits, to 0 if doesn't
    pub fn add_arithmetic(&mut self, x: u8, y: u8) {
        let temp =
            (self.registers[x as usize] as u16 + self.registers[y as usize] as u16).to_be_bytes();
        if temp[0] != 0 {
            self.registers[15] = 1;
        } else {
            self.registers[15] = 0;
        }
        self.registers[x as usize] = temp[1];
    }
    ///8XY5\
    ///Substracts value of register VY from VX and puts the result into VX
    pub fn sub(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] < self.registers[y as usize] {
            self.registers[x as usize] = (0x100 - self.registers[y as usize] as u16
                + self.registers[x as usize] as u16) as u8;
            self.registers[15] = 0
        } else {
            self.registers[x as usize] = self.registers[x as usize] - self.registers[y as usize];
            self.registers[15] = 1
        }
    }
    ///8XY7\
    ///Substracts value of register VX from VY and puts the result into VX
    pub fn subn(&mut self, x: u8, y: u8) {
        if self.registers[y as usize] < self.registers[x as usize] {
            self.registers[x as usize] = (0x100 - self.registers[x as usize] as u16
                + self.registers[y as usize] as u16) as u8;
            self.registers[15] = 0
        } else {
            self.registers[x as usize] = self.registers[y as usize] - self.registers[x as usize];
            self.registers[15] = 1
        }
    }
    ///8XY6\
    ///If least-significant bit of Vx is 1, VF is set to 1, otherwise 0. Then Vx>>=1
    pub fn shr(&mut self, x: u8, y: u8) {
        self.registers[15] = self.registers[y as usize] & 0b1;
        self.registers[x as usize] = self.registers[x as usize] >> 1;
    }
    ///8XYE\
    ///If most-significant bit of Vx is 1, VF is set to 1, otherwise 0. Then Vx<<=1
    pub fn shl(&mut self, x: u8, y: u8) {
        self.registers[15] = self.registers[y as usize] >> 7;
        self.registers[x as usize] = self.registers[y as usize] << 1;
    }
}
