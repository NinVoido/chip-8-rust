use rand::random;
impl crate::utilities::cpu::Cpu {
    ///CXNN instruction implementation
    ///Generates a random integer using rand, bin-ands it with NN and puts the result into register
    ///VX
    pub fn rnd(mut self, x: u8, nn: u8) {
        self.registers[x as usize] = nn & random::<u8>()
    }
}
