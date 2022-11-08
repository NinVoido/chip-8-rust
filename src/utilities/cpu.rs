//!Main CPU architecture and methods are written here.
///Main CPU struct\
///RAM, stack, etc. are stored in this struct as pub fields.
pub struct Cpu {
    ///4Kb RAM accesable by CPU
    pub ram: [u8; 4096],
    ///Index register
    pub i: u16,
    ///CPU's registers\
    ///Worth mentioning that VF (16th register) should never be used in programs,
    ///as it is considered as a flag by some instructions
    pub registers: [u8; 16],
    ///Delay timer, while it's value is above 0, decrements at a rate of 60 Hz
    pub dt: u8,
    ///Sound timer, beeps while decrementing like delay timer
    pub st: u8,
    ///Program counter
    pub pc: u16,
    ///Stack
    pub stack: Stack,
    ///Flag used while rendering, turns true if VF is set to 1 after drawing a sprite or after
    ///clearing the screen
    pub redraw_needed: bool,
    ///Frame buffer of CHIP-8 CPU\
    ///If redraw_needed flag is up, contents of this fb are written to main frame provided by pixels
    pub screen: Vec<Vec<bool>>,
    ///Flag needed for key scan instrucion implementation
    pub scan_info: (bool, u8),
    ///Flag for audio playback
    pub should_beep: bool,
    ///CPU's input buffer
    pub keypad: [bool; 16],
    ///Flag registers
    pub fl_regs: [u8; 16],
    ///Is highres on
    pub highres: bool,
    ///Did the execution stop
    pub stopped: bool,
}
///Stack struct, which contains 16-element max stack and stack pointer
#[derive(Copy, Clone)]
pub struct Stack {
    ///Stack
    pub stack: [u16; 16],
    ///Stack pointer
    pub sp: u8,
}
impl Stack {
    ///Push implementation for stack struct\
    ///Returns stack overflow if stack pointer is already 16
    pub fn push(&mut self, subroutine_adress: u16) -> Result<(), &'static str> {
        if self.sp == 16 {
            return Err("Stack overflow");
        }
        self.stack[self.sp as usize] = subroutine_adress;
        self.sp += 1;
        Ok(())
    }
    ///Creates an empty instance of Stack
    pub fn new() -> Stack {
        Stack {
            stack: [0u16; 16],
            sp: 0,
        }
    }
    ///Pop implementation for stack struct\
    ///Returns an error if tries popping from empty stack
    pub fn pop(&mut self) -> Result<(), &'static str> {
        if self.sp == 0 {
            return Err("Popping from empty stack");
        }
        self.sp -= 1;
        Ok(())
    }
}

impl Cpu {
    ///Function creates an empty chip-8 CPU structure\
    ///Fonts are put into RAM\
    ///Program counter is set 512 to match the first instruction adress\
    ///Everything else is left zero
    pub fn new() -> Cpu {
        let mut chip = Cpu {
            ram: [0u8; 4096],
            pc: 512,
            stack: Stack::new(),
            i: 0,
            registers: [0u8; 16],
            dt: 0,
            st: 0,
            redraw_needed: false,
            screen: vec![vec![false; 64]; 32],
            scan_info: (false, 0),
            should_beep: false,
            keypad: [false; 16],
            fl_regs: [0u8; 16],
            highres: false,
            stopped: false,
        };
        let fonts: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80,
        ]; // F
        let bigfonts: [u8; 160] = [
            0xFF, 0xFF, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, 0x18, 0x78, 0x78, 0x18,
            0x18, 0x18, 0x18, 0x18, 0xFF, 0xFF, 0xFF, 0xFF, 0x3, 0x3, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF,
            0xFF, 0xFF, 0xFF, 0x3, 0x3, 0xFF, 0xFF, 0x3, 0x3, 0xFF, 0xFF, 0xC3, 0xC3, 0xC3, 0xC3,
            0xFF, 0xFF, 0x3, 0x3, 0x3, 0x3, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0x3, 0x3, 0xFF,
            0xFF, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0xFF, 0xFF, 0x3, 0x3,
            0x6, 0xC, 0x18, 0x18, 0x18, 0x18, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF,
            0xFF, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0x3, 0x3, 0xFF, 0xFF, 0x7E, 0xFF, 0xC3, 0xC3,
            0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xC3, 0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, 0xC3, 0xC3,
            0xFC, 0xFC, 0x3C, 0xFF, 0xC3, 0xC0, 0xC0, 0xC0, 0xC0, 0xC3, 0xFF, 0x3C, 0xFC, 0xFE,
            0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFE, 0xFC, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF,
            0xC0, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xC0, 0xC0,
        ];

        for i in 0..=79 {
            chip.ram[i + 0x50] = fonts[i]
        }

        for i in 0..=159 {
            chip.ram[i + 0x50 + 80] = bigfonts[i]
        }
        return chip;
    }
    ///Reset entire Cpu
    pub fn reset(&mut self) {
        *self = Cpu::new()
    }
}
