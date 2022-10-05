impl crate::utilities::cpu::Cpu{
    ///8XY0 instruction implementation
    ///Sets register VX to the value of VY
    fn set_reg(mut self, x: u8, y: u8){
        self.registers[x as usize] = self.registers[y as usize]
    }
    ///8XY1 instruction implementation
    ///Sets register VX to the VX | VY
    fn bin_or(mut self, x: u8, y: u8){
        self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
    }
    ///8XY2 instruction implementation
    ///Sets register VX to the VX && VY
    fn bin_and(mut self, x: u8, y: u8){
        self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
    }
    ///8XY3 instruction implementation
    ///Sets register VX to the VX ^ VY
    fn bin_xor(mut self, x: u8, y: u8){
        self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize]
    }
    ///8XY4 instruction implementation
    ///Adds value of register VY to the VX
    ///Sets VF flag to 1 if the addition overflows 8 bits, to 0 if doesn't
    fn ar_add(mut self, x: u8, y: u8){
        let temp = (self.registers[x as usize] as u16 + self.registers[y as usize] as u16)
            .to_be_bytes();
        if temp[0] != 0 {
            self.registers[15] = 1;
        }else{
            self.registers[15] = 0;
        }
        self.registers[x as usize] = temp[1];
    }
    
}
