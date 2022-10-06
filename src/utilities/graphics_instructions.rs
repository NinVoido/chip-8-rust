impl crate::utilities::cpu::Cpu {
    ///FX29 instruction implementation
    ///Sets index register to the address of hex character resembling value in register VX
    fn ld_index_font(mut self, x: u8) {
        self.i = 50 + 5 * self.registers[x as usize] as u16
    }
}
