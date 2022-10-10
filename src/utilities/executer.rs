use winit_input_helper::WinitInputHelper;

use crate::utilities::cpu::Cpu;
impl Cpu {
    pub fn execute(&mut self, input: &WinitInputHelper) -> Result<(), &'static str> {
        
        let instruction =
            ((self.ram[self.pc as usize] as u16) << 8) + self.ram[self.pc as usize + 1] as u16;
        let nb = Nibbles {
            nn: (instruction & 0x00ff) as u8,
            nnn: instruction & 0x0fff,
            x: ((instruction & 0x0f00) >> 8) as u8,
            y: ((instruction & 0x00f0) >> 4) as u8,
            n: (instruction & 0x000f) as u8,
            id: ((instruction & 0xf000) >> 12) as u8,
        };
        //TODO - is match temporary? I hope so. I will try to find a way to impl this using
        //function pointers, but that seems pretty sophisticated, because my instructions are
        //struct methods and giving a function pointer to them requires already given Cpu instance
        //and arguments\
        self.pc += 2;
        match nb.id {
            0 => match nb.nn {
                0xE0 => self.cls(),
                0xEE => self.ret()?,
                _ => (),
            },
            1 => self.jp(nb.nnn),
            2 => self.call(nb.nnn)?,
            3 => self.se(nb.x, nb.nn),
            4 => self.sne(nb.x, nb.nn),
            5 => self.se_reg(nb.x, nb.y),
            6 => self.ld(nb.x, nb.nn),
            7 => self.add(nb.x, nb.nn),
            8 => match nb.n {
                0 => self.ld_reg(nb.x, nb.y),
                1 => self.or(nb.x, nb.y),
                2 => self.and(nb.x, nb.y),
                3 => self.xor(nb.x, nb.y),
                4 => self.add_arithmetic(nb.x, nb.y),
                5 => self.sub(nb.x, nb.y),
                6 => (),
                7 => self.subn(nb.x, nb.y),
                0xE => (),
                _ => (),
            },
            9 => self.sne_reg(nb.x, nb.y),
            0xA => self.ld_index(nb.nnn),
            0xB => (),
            0xC => self.rnd(nb.x, nb.nn),
            0xD => self.drw(nb.x, nb.y, nb.n),
            0xE => match nb.nn {
                0x9E => self.skp(nb.x, input),
                0xA1 => self.sknp(nb.x, input),
                _ => (),
            },
            0xF => match nb.nn {
                7 => self.ld_from_delay_timer(nb.x),
                0xA => self.ld_keyboard(nb.x, input),
                0x15 => self.ld_to_delay_timer(nb.x),
                0x18 => self.ld_to_sound_timer(nb.x),
                0x1E => self.add_index(nb.x),
                0x29 => self.ld_index_font(nb.x),
                0x33 => self.ld_bcd(nb.x),
                0x55 => (),
                0x65 => (),
                _ => (),
            },
            _ => (),
        }
        Ok(())
    }
}
///Private struct that represent various components of the instruction
struct Nibbles {
    ///Lowest 8 bits of the instruction
    ///instruction & 0x00ff
    nn: u8,
    ///Lowest 12 bits of the instruction
    ///instruction & 0x0fff
    nnn: u16,
    ///Lower 4 bits of the high byte of the instruction
    ///(instruction & 0x0f00) >> 8
    x: u8,
    ///Upper 4 bits of the low byte of the instruction
    ///(instruction & 0x00f0) >> 4
    y: u8,
    ///Lowest 4 bits of the instruction
    ///instruction & 0x000f
    n: u8,
    ///Instruction identifier
    ///(instruction & 0xf000) >> 12
    id: u8,
}
