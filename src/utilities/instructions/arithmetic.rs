impl crate::utilities::cpu::Cpu {
    ///8XY0 instruction implementation
    ///Sets register VX to the value of VY
    pub fn ld_reg(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize]
    }
    ///8XY1 instruction implementation
    ///Sets register VX to the VX | VY
    pub fn or(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
    }
    ///8XY2 instruction implementation
    ///Sets register VX to the VX && VY
    pub fn and(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
    }
    ///8XY3 instruction implementation
    ///Sets register VX to the VX ^ VY
    pub fn xor(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize]
    }
    ///8XY4 instruction implementation
    ///Adds value of register VY to the VX
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
    ///8XY5 instruction implementation
    ///Substracts value of register VY from VX and puts the result into VX
    pub fn sub(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] > self.registers[y as usize] {
            self.registers[15] = 1
        } else {
            self.registers[15] = 0
        }
        //IDK how to properly handle this, so i guess I'll just use to_be_bytes
        self.registers[x as usize] = (self.registers[x as usize] as i16
            - self.registers[y as usize] as i16)
            .to_be_bytes()[1];
    }
    ///8XY7 instruction implementation
    ///Substracts value of register VX from VY and puts the result into VX
    pub fn subn(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] > self.registers[y as usize] {
            self.registers[15] = 1
        } else {
            self.registers[15] = 0
        }
        //IDK how to properly handle this, so i guess I'll just use to_be_bytes
        self.registers[x as usize] = (self.registers[y as usize] as i16
            - self.registers[x as usize] as i16)
            .to_be_bytes()[1];
    }
    //8XY6
}
