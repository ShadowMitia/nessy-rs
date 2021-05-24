/*!  Emulate a mos6502 Ricoh 2A03 microntroller */

pub mod instructions;
pub mod utils;


/**
A struct to represent MOS6502 registers

a is 8-bit accumulator register
x is 8-bit X index register
y is 8-bit Y index register
s is a 8-bit stack pointer, hardwired to memory page $01
pc is a 16-bit program counter
status hold processore flag bits (7 flags)
*/
#[derive(Debug, Clone)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub s: u8,
    pub pc: u16,
    pub status: u8,
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum StatusFlag {
    N = 7,
    V = 6,
    Unused = 5,
    B = 4,
    D = 3,
    I = 2,
    Z = 1,
    C = 0,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            s: 0,
            pc: 0,
            status: 0,
        }
    }

    pub fn set_flag(&mut self, flag: StatusFlag, activated: bool) {
        if activated {
            self.status |= 0x1 << flag as u8;
        } else {
            self.status &= !(0x1 << flag as u8);
        }
    }

    pub fn is_flag_set(&self, flag: StatusFlag) -> bool {
        let val = 0x1 << flag as u8;
        self.status & val == val
    }
}

/**
Represents a NES memory

Zero page    $0000 - $00FF
Stack    $0100 - $01FF
General-purpose    $0200 - $FFFF

*/
pub struct Memory {
    pub memory: Vec<u8>,
    pub ppu: Vec<u8>,
    pub stack_pointer: u16,
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Vec::new();
        memory.resize_with(0x10000, || 0);
        let mut ppu = Vec::new();
        ppu.resize_with(0x4000, || 0);

        Self {
            memory,
            ppu,
            stack_pointer: 0x01FF,
        }
    }

    pub fn stack_push(&mut self, val: u8) {
        self.memory[0x100 | self.stack_pointer as usize] = val;
        self.stack_pointer -= 1;
    }

    #[must_use]
    pub fn stack_pop(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.memory[0x100 | self.stack_pointer as usize]
    }
}

#[test]
fn stack_test() {
    let mut memory = Memory::new();

    memory.stack_push(0x42);
    assert_eq!(memory.stack_pointer, 0x01FE);

    let val = memory.stack_pop();
    assert_eq!(memory.stack_pointer, 0x01FF);
    assert_eq!(val, 0x42);
}

#[derive(Debug, Clone, PartialEq)]
pub enum AddressingMode {
    Accumulator,
    Implied,
    Immediate,
    Absolute,
    ZeroPage,
    Relative,         // ONLY USED BY JUMP INSTRUCTIONS
    AbsoluteIndirect, // ONLY USED BY 'JUMP'
    AbsoluteIndirectWithX,
    AbsoluteIndirectWithY,
    ZeroPageIndexedWithX,
    ZeroPageIndexedWithY,
    ZeroPageIndexedIndirect,
    ZeroPageIndirectIndexedWithY,
}
