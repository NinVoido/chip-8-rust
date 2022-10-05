//!All timer instructions
//!Beeping support for ST is NOT implemented in this file
impl crate::utilities::cpu::Cpu{
    ///FX07 instruction implementation
    ///Sets register VX to the current value of DT
    fn set_to_timer(mut self, x: u8){
        self.registers[x as usize] = self.dt
    }
    ///FX15 instruction implementation
    ///Sets the DT to the value in register VX
    fn set_delay_timer(mut self, x: u8){
        self.dt = self.registers[x as usize]
    }
    ///FX18 instruction implementation
    ///Sets the ST to the value in register VX
    fn set_sound_timer(mut self, x: u8){
        self.st = self.registers[x as usize]
    }
}
