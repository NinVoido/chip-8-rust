//!SCHIP-48 instructions are implemented here
impl crate::utilities::cpu::Cpu {
    ///00FD instruction impl - stops the executing
    pub fn exit(&mut self) {
        self.reset();
        self.stopped = false
    }

    ///FX75 instruction implementation - store V0-Vx in user flags
    pub fn ld_r_vx(&mut self, x: u8) {
        for i in 0..=x as usize {
            self.fl_regs[i] = self.registers[i]
        }
    }

    ///Shr&Shl are done COSMAC-VIP way
    ///8XY6
    ///If least-significant bit of Vx is 1, VF is set to 1, otherwise 0. Then Vx>>=1
    pub fn shr_schip(&mut self, x: u8,) {
        self.registers[15] = self.registers[x as usize] & 0b1;
        self.registers[x as usize] = self.registers[x as usize] >> 1;
    }
    ///8XYE
    ///If most-significant bit of Vx is 1, VF is set to 1, otherwise 0. Then Vx<<=1
    pub fn shl_schip(&mut self, x: u8,) {
        self.registers[15] = self.registers[x as usize] >> 7;
        self.registers[x as usize] = self.registers[x as usize] << 1;
    }
}
