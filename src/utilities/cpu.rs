///Main CPU struct
///
///RAM, stack, etc. are stored in this struct as pub fields.
pub struct Cpu{
    ///4Kb RAM accesable by CPU
    pub ram: [u8;4096],
    ///Index register
    pub i: u16,
    ///CPU's registers
    ///Worth mentioning that VF (16th register) should never be used in programs,
    ///as it is considered as a flag by some instructions
    pub registers: [u8;16],
    ///Delay timer, while it's value is above 0, decrements at a rate of 60 Hz
    pub dt: u8,
    ///Sound timer, beeps while decrementing like delay timer
    pub st: u8,
    ///Program counter
    pub pc: u16,
    ///Stack
    pub stack: Stack, 
}
///Stack struct, which contains 16-element max stack and stack pointer
pub struct Stack{
    ///Stack
    pub stack: [u16;16],
    ///Stack pointer
    sp: u8,
}
impl Stack{
    ///Push implementation for stack struct
    ///Panics with stack overflow if stack pointer is already 16
    pub fn push(mut self, subroutine_adress: u16)->Result<(), &'static str>{
        if self.sp==15{
            return Err("Stack overflow")
        }
        self.sp += 1;
        self.stack[self.sp as usize] = subroutine_adress;
        Ok(())
    }
    ///Creates an empty instance of Stack
    pub fn new()->Stack{
        Stack{
            stack: [0u16;16],
            sp: 0,
        }
    }
    ///Pop implementation for stack struct
    ///Panics if tries popping from empty stack
    pub fn pop(mut self)->Result<(), &'static str>{
        if self.sp==0{
            return Err("Popping from empty stack")
        } 
        self.stack[self.sp as usize] = 0;
        self.sp -= 1;
        Ok(())
    }
}
/*todo!("Fonts (perhaps initialize them at Cpu::new()?: 
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
0xF0, 0x80, 0xF0, 0x80, 0x80  // F");
`
todo!("
1) init a CPU (Cpu::new()), load fonts to ram, set default values
2)Wait for user tio choose a file, settings(Instruction behaviors, clock rate, etc.), press run button
3)Load ROM into ram
4)Launch an executing loop
5)Debug menu, etc");

*/

impl Cpu{
    ///Function creates an empty chip-8 CPU structure
    ///Fonts are put into RAM
    ///Program counter is set 512 to match the first instruction adress
    ///Everything else is left zero
    pub fn new()->Cpu{

        let mut chip = Cpu{
            ram: [0u8;4096],
            pc: 512,
            stack: Stack::new(),
            i: 0,
            registers: [0u8;16],
            dt: 0,
            st: 0
        };
        let fonts: [u8;80] =   [
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
            0xF0, 0x80, 0xF0, 0x80, 0x80];// F
        for i in 0..=79{
            chip.ram[i] = fonts[i]
        }
        return chip
    }
}
