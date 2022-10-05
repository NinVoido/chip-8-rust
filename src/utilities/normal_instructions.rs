impl crate::utilities::cpu::Cpu{
    ///1NNN instruction implementation
    ///Jumps to a given address
    pub fn jump_to(mut self, address:u16){
        self.pc = address
    }
    ///2NNN instruction implementation
    ///Calls a subroutine at a given address
    ///Pushes current pc on stack and increments stack pointer
    pub fn call_subroutine(mut self, address: u16) -> Result<(), &'static str>{
        self.stack.push(self.pc)?;
        Ok(())
    }
    ///2NNN instruction implementation
    ///Returns from a subroutine by performing a pop on a stack
    pub fn return_from_subroutine(mut self) -> Result<(), &'static str>{
        self.pc = self.stack.stack[self.stack.sp as usize-1];
        self.stack.pop()?;
        Ok(())
    }
    ///3XNN instruction implementation
    ///Skips one instruction if value in register VX is equal to NN
    pub fn skip_if_equal_to(mut self, x: u8, nn: u8){
        if self.registers[x as usize] == nn{
            self.pc += 2;
        }
    }
    ///4XNN instruction implementation
    ///Skips one instruction if value in register VX is NOT equal to NNN
    pub fn skip_if_not_equal_to(mut self, x: u8, nn: u8){
        if self.registers[x as usize] != nn{
            self.pc += 2;
        }
    }
    ///5XY0 instruction implementation
    ///Skips one instruction if value in register VX is equal to value in VY
    pub fn skip_if_equal(mut self, x: u8, y: u8){
        if self.registers[x as usize] == self.registers[y as usize]{
            self.pc += 2;
        }
    }
    ///9XY0 instruction implementation
    ///Skips one instruction if value in register VX is NOT equal to value in VY
    pub fn skip_if_not_equal(mut self, x: u8, y: u8){
        if self.registers[x as usize] != self.registers[y as  usize]{
            self.pc += 2;
        }
    }
    ///6XNN instruction implementation
    ///Sets the register VX to the value of NN
    pub fn set_register(mut self, x: u8, nn: u8){
        self.registers[x as usize] = nn;
    }
    ///7XNN instruction implementation
    ///Adds the value NN to register VX
    ///If attempts to add with overflow, only lower 8 bits will be stored 
    pub fn add(mut self, x: u8, nn: u8){
        self.registers[x as usize] = (self.registers[x as usize] as u16 + nn as u16).to_be_bytes()[1];
    }
    ///ANNN instruction implementation
    ///Sets the index register to the value of NNN
    pub fn set_index(mut self, nnn: u16){
        self.i = nnn
    }
    //TODO - BNNN jump with offset and random
    ///FX1E instruction implementation
    ///Adds value of register VX to index register
    ///Sets VF to 1 if addition overflowed 12 bits
    //TODO - make this behaviour changeable?
    fn add_to_index(mut self, x: u8){
        self.i += self.registers[x as usize] as u16;
        if self.i >= 0x1000{
            self.registers[15] = 1
        }
    }
}
