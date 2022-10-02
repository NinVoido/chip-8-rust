///Main CPU struct
///
///RAM, stack, etc. are stored in this struct as pub fields.
pub struct Cpu{
    pub ram: [u8;4096],
    pub i: u16,
    pub registers: [u8;16],
    pub dt: u8,
    pub st: u8,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16;16],
}

