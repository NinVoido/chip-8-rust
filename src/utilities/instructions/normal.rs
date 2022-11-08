//!Most of the instructions that don't rely on external components and cannot be sorted into groups
//!are implemented here
impl crate::utilities::cpu::Cpu {
    ///1NNN\
    ///Jumps to a given address
    pub fn jp(&mut self, address: u16) {
        self.pc = address
    }
    ///2NNN\
    ///Calls a subroutine at a given address\
    ///Pushes current pc on stack and increments stack pointer
    pub fn call(&mut self, address: u16) -> Result<(), &'static str> {
        self.stack.push(self.pc)?;
        self.pc = address;
        Ok(())
    }
    ///00EE\
    ///Returns from a subroutine by performing a pop on a stack
    pub fn ret(&mut self) -> Result<(), &'static str> {
        self.stack.pop()?;
        self.pc = self.stack.stack[self.stack.sp as usize];
        Ok(())
    }
    ///3XNN\
    ///Skips one instruction if value in register VX is equal to NN
    pub fn se(&mut self, x: u8, nn: u8) {
        if self.registers[x as usize] == nn {
            self.pc += 2;
        }
    }
    ///4XNN\
    ///Skips one instruction if value in register VX is NOT equal to NNN
    pub fn sne(&mut self, x: u8, nn: u8) {
        if self.registers[x as usize] != nn {
            self.pc += 2;
        }
    }
    ///5XY0\
    ///Skips one instruction if value in register VX is equal to value in VY
    pub fn se_reg(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] == self.registers[y as usize] {
            self.pc += 2;
        }
    }
    ///9XY0\
    ///Skips one instruction if value in register VX is NOT equal to value in VY
    pub fn sne_reg(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] != self.registers[y as usize] {
            self.pc += 2;
        }
    }
    ///6XNN\
    ///Sets the register VX to the value of NN
    pub fn ld(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] = nn;
    }
    ///7XNN\
    ///Adds the value NN to register VX
    ///If attempts to add with overflow, only lower 8 bits will be stored
    pub fn add(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] =
            ((self.registers[x as usize] as u16 + nn as u16) & 0x00ff) as u8;
    }
    ///ANNN\
    ///Sets the index register to the value of NNN
    pub fn ld_index(&mut self, nnn: u16) {
        self.i = nnn
    }
    ///BNNN\
    ///Jump to location NNN + V0
    pub fn jp_with(&mut self, nnn: u16) {
        self.pc = nnn + self.registers[0] as u16
    }
    ///FX1E\
    ///Adds value of register VX to index register\
    ///Sets VF to 1 if addition overflowed 12 bits
    pub fn add_index(&mut self, x: u8) {
        self.i += self.registers[x as usize] as u16;
        if self.i >= 0x1000 {
            self.registers[15] = 1
        }
    }
    ///FX33\
    ///Takes the value of register VX and converts it to three decimal digits, to store them after
    ///register I
    pub fn ld_bcd(&mut self, x: u8) {
        let mut temp = self.registers[x as usize];
        self.ram[self.i as usize + 2] = temp % 10;
        temp /= 10;
        self.ram[self.i as usize + 1] = temp % 10;
        temp /= 10;
        self.ram[self.i as usize] = temp;
    }
    ///FX55\
    ///Stores registers V0 through Vx into memory from I
    pub fn ld_i_vx(&mut self, x: u8) {
        for j in 0..=x {
            self.ram[(self.i + j as u16) as usize] = self.registers[j as usize]
        }
        self.i += x as u16 //+ 1
    }
    ///FX65\
    ///Read registers V0 through VX from memory starting at I
    pub fn ld_vx_i(&mut self, x: u8) {
        for j in 0..=x {
            self.registers[j as usize] = self.ram[(self.i + j as u16) as usize]
        }
        self.i += x as u16 //+ 1
    }
}
