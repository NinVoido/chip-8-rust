impl crate::utilities::cpu::Cpu {
    ///Method for loading a ROM into RAM
    ///Panics if ROM is bigger than available 3.5Kb of RAM or if std::fs threw an error
    pub fn load_rom(mut self, path: String)->Result<(), &'static str>{
        let rom_file = std::fs::read(path).expect("Error occured while reading a file");
        //Panic if ROM can't fit into RAM
        if rom_file.len() > 3584{
            return Err("ROM was too big")
        }
        for i in 512..512+rom_file.len(){
            self.ram[i] = rom_file[i-512];
        }
        Ok(())
    }
}



