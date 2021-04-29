mod rp2a03 {
    /*!  Emulate a mos6502 Ricoh 2A03 microntroller */
    /**
    A struct to represent MOS6502 registers

    a is 8-bit accumulator register
    x is 8-bit X index register
    y is 8-bit Y index register
    s is a 8-bit stack pointer, hardwired to memory page $01
    pc is a 16-bit program counter
    status hold processore flag bits (7 flags)
    */
    pub struct Registers {
        pub a: u8,
        pub x: u8,
        pub y: u8,
        pub s: u8,
        pub pc: u16,
        pub status: u8,
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
    }

    /**
    Represents a NES memory

    Zero page	$0000 - $00FF
    Stack	$0100 - $01FF
    General-purpose	$0200 - $FFFF

    */
    pub struct Memory {
        pub memory: Vec<u8>,
        pub ppu: Vec<u8>,
    }

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

    pub mod Instructions {

        use crate::address_from_bytes;

        use super::*;

        pub enum Instructions {
            SEI,
            CLD,
            LDA,
        }

        pub fn match_instruction(opcode: u8) -> (Instructions, AddressingMode) {
            match opcode {
                // LDA
                0xAD => (Instructions::LDA, AddressingMode::Absolute),
                0xBD => (Instructions::LDA, AddressingMode::AbsoluteIndirectWithX),
                0xB9 => (Instructions::LDA, AddressingMode::AbsoluteIndirectWithY),
                0xA9 => (Instructions::LDA, AddressingMode::Immediate),
                0xA5 => (Instructions::LDA, AddressingMode::ZeroPage),
                0xA1 => (Instructions::LDA, AddressingMode::ZeroPageIndexedIndirect),
                0xB5 => (Instructions::LDA, AddressingMode::ZeroPageIndexedWithX),
                0xB1 => (
                    Instructions::LDA,
                    AddressingMode::ZeroPageIndirectIndexedWithY,
                ),
                // SEI
                0x78 => (Instructions::SEI, AddressingMode::Implied),
                0xd8 => (Instructions::CLD, AddressingMode::Implied),
                _ => panic!("Unknown opcode {:#x}", opcode),
            }
        }

        pub fn sei(registers: &mut Registers) {
            registers.status |= 0b00000100;
        }

        #[test]
        fn sei_test() {
            let mut registers = Registers::new();
            sei(&mut registers);
            assert_eq!(registers.status, 0b00000100);
        }

        pub fn cld(registers: &mut Registers) {
            registers.status &= 0b11110111;
        }

        #[test]
        fn cld_test() {
            let mut registers = Registers::new();
            registers.status |= 0b00001000;
            cld(&mut registers);
            assert_eq!(registers.status, 0b00000000);
        }

        pub fn lda(registers: &mut Registers, operand: u8) {
            registers.a = operand;
            registers.status = if operand == 0 {
                registers.status | 0b00000010
            } else {
                registers.status & 0b11111101
            };
            registers.status = if operand >= 0x80 {
                registers.status | 0b10000000
            } else {
                registers.status & 0b01111111
            };
        }

        #[test]
        fn lda_test() {
            let mut registers = Registers::new();
            lda(&mut registers, 0x42);
            assert_eq!(registers.a, 0x42);
            lda(&mut registers, 0x0);
            assert_eq!(registers.a, 0x0);
            assert_eq!(registers.status & 0b00000010, 0b00000010);
            lda(&mut registers, 0x80);
            assert_eq!(registers.a, 0x80);
            assert_eq!(registers.status & 0b10000000, 0b10000000);
        }

        /**
        Applies addressing mode rules to operands and gives out 16-bit results
         */
        pub fn apply_addressing(
            memory: &[u8],
            registers: &Registers,
            adressing_mode: AddressingMode,
            low_byte: Option<u8>,
            high_byte: Option<u8>,
        ) -> Option<u16> {
            match adressing_mode {
                AddressingMode::Accumulator => None,
                AddressingMode::Implied => None,
                AddressingMode::Immediate => {
                    Some(address_from_bytes(low_byte.unwrap(), high_byte.unwrap()))
                }
                AddressingMode::Absolute => {
                    let addr = address_from_bytes(low_byte.unwrap(), high_byte.unwrap());
                    Some(*memory.get(addr as usize).unwrap() as u16)
                }
                AddressingMode::ZeroPage => {
                    let addr = low_byte.unwrap();
                    Some(*memory.get(addr as usize).unwrap() as u16)
                }
                AddressingMode::Relative => Some(low_byte.unwrap() as u16),
                AddressingMode::AbsoluteIndirect => {
                    let addr = address_from_bytes(low_byte.unwrap(), high_byte.unwrap());
                    let addr2 = address_from_bytes(low_byte.unwrap() + 1, high_byte.unwrap());
                    Some(address_from_bytes(
                        memory[addr as usize],
                        memory[addr2 as usize],
                    ))
                }
                AddressingMode::AbsoluteIndirectWithX => {
                    let addr = address_from_bytes(low_byte.unwrap(), high_byte.unwrap())
                        + registers.x as u16;
                    Some(*memory.get(addr as usize).unwrap() as u16)
                }
                AddressingMode::AbsoluteIndirectWithY => {
                    let addr = address_from_bytes(low_byte.unwrap(), high_byte.unwrap())
                        + registers.y as u16;
                    Some(*memory.get(addr as usize).unwrap() as u16)
                }
                AddressingMode::ZeroPageIndexedWithX => {
                    let addr = low_byte.unwrap() + registers.x;
                    Some(*memory.get(addr as usize).unwrap() as u16)
                }
                AddressingMode::ZeroPageIndexedWithY => {
                    let addr = low_byte.unwrap() + registers.y;
                    Some(*memory.get(addr as usize).unwrap() as u16)
                }
                AddressingMode::ZeroPageIndexedIndirect => {
                    let addr = low_byte.unwrap() + registers.x;
                    let low = *memory.get(addr as usize).unwrap();
                    let high = *memory.get((addr + 1) as usize).unwrap();
                    Some(address_from_bytes(low, high))
                }
                AddressingMode::ZeroPageIndirectIndexedWithY => {
                    let addr = address_from_bytes(low_byte.unwrap(), high_byte.unwrap());
                    let low_byte = *memory.get(addr as usize).unwrap();
                    let high_byte = *memory.get((addr + 1) as usize).unwrap();
                    let addr = address_from_bytes(low_byte, high_byte) + registers.y as u16;
                    Some(memory[addr as usize] as u16)
                }
            }
        }

        #[test]
        fn apply_addressing_test() {
            let mut memory = Vec::new();
            memory.resize_with(0x10000, || 0);

            let mut registers = Registers::new();

            // IMPLIED
            let res =
                apply_addressing(&memory, &registers, AddressingMode::Accumulator, None, None);
            assert_eq!(res, None);

            // IMPLIED
            let res = apply_addressing(&memory, &registers, AddressingMode::Implied, None, None);
            assert_eq!(res, None);

            // IMMEDIATE
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::Immediate,
                Some(0x1),
                Some(0x2),
            );
            assert_eq!(res, Some(0x201));

            // ABSOLUTE
            memory[0x201] = 42;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::Absolute,
                Some(0x1),
                Some(0x2),
            );
            assert_eq!(res, Some(42));

            // ZERO PAGE
            memory[0x0] = 43;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::ZeroPage,
                Some(0x0),
                None,
            );
            assert_eq!(res, Some(43));

            // Relative
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::Relative,
                Some(0x42),
                None,
            );
            assert_eq!(res, Some(0x42));

            // AbsoluteIndirect
            memory[0xA001] = 0xFF;
            memory[0xA002] = 0x00;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::AbsoluteIndirect,
                Some(0x01),
                Some(0xA0),
            );
            assert_eq!(res, Some(0x00FF));

            // AbsoluteIndirectWithX
            memory[0xC003] = 0x5A;
            registers.x = 0x2;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::AbsoluteIndirectWithX,
                Some(0x1),
                Some(0xC0),
            );
            assert_eq!(res, Some(0x5A));

            // AbsoluteIndirectWithY
            memory[0xF004] = 0xEF;
            registers.y = 0x3;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::AbsoluteIndirectWithY,
                Some(0x1),
                Some(0xF0),
            );
            assert_eq!(res, Some(0xEF));

            // ZeroPageIndexedWithX
            memory[0x3] = 0xA5;
            registers.x = 0x2;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::ZeroPageIndexedWithX,
                Some(0x1),
                None,
            );
            assert_eq!(res, Some(0xA5));

            // ZeroPageIndexedWithY
            memory[0x4] = 0xE3;
            registers.y = 3;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::ZeroPageIndexedWithY,
                Some(0x1),
                None,
            );
            assert_eq!(res, Some(0xE3));

            // Zero Page Indexed Indirect
            memory[0x17] = 0x10;
            memory[0x18] = 0xD0;
            registers.x = 2;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::ZeroPageIndexedIndirect,
                Some(0x15),
                None,
            );
            assert_eq!(res, Some(0xD010));

            // Zero Page Indexed Indirect with Y
            memory[0x002A] = 0x35;
            memory[0x002B] = 0xC2;
            memory[0xC238] = 0x2F;
            registers.y = 0x3;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::ZeroPageIndirectIndexedWithY,
                Some(0x2A),
                Some(0x00),
            );
            assert_eq!(res, Some(0x2F));
        }
    }
}

use rp2a03::*;

use crate::rp2a03::Instructions::apply_addressing;

fn decode(memory: &mut [u8], registers: &mut Registers) {}

fn address_from_bytes(low_byte: u8, high_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | low_byte as u16
}

const ZEROPAGE_ADDRESS: u32 = 0x0;
const STACK_ADDRESS: u32 = 0x0100;
const RAM_ADDRESS: u32 = 0x0200;
const EXPANSION_ROM_ADDRESS: u32 = 0x4020;
const SRAM_ADDRESS: u32 = 0x6000;
const PRGROM_ADDRESS: u32 = 0x8000;
const NMI_VECTOR_ADDRESS: u32 = 0xFFFA;
const RESET_VECTOR_ADDRESS: u32 = 0xFFFC;
const BREAK_VECTOR_ADDDRESS: u32 = 0xFFFE;

fn main() {
    println!("Nessy 🐉!");

    // Initialise memory
    let mut memory = Vec::new();
    memory.resize_with(0x10000, || 0);

    // Load ROM and decode header
    let rom =
        include_bytes!("/home/dimitri/development/nessy-rs/Legend of Zelda, The (Europe).nes");

    println!(
        "{}{}{} {}",
        rom[0] as char, rom[1] as char, rom[2] as char, rom[3]
    );

    let num_prgrom = rom[4];
    let num_chrrom = if rom[5] == 0 { 1 } else { rom[5] };

    println!(
        "Num of 16k bytes PRG ROM {} ({}k bytes)\nNum of 8k CHR ROM {}",
        num_prgrom,
        16 * num_prgrom,
        num_chrrom
    );

    // Load up memory
    for index in 0..(0x10000 - PRGROM_ADDRESS) {
        memory[(PRGROM_ADDRESS + index) as usize] = rom[index as usize];
    }

    let bank_seven = 16 + 7 * 16384;

    for index in 0..16384 {
        memory[(0xC000 + index) as usize] = rom[(bank_seven + index) as usize];
    }

    // Get the RESET vector to find start of the game
    let reset_vector_low = memory[0xFFFC];
    let reset_vector_high = memory[0xFFFD];

    println!("hi {:x} lo {:x}", reset_vector_high, reset_vector_low);

    let reset_vector = address_from_bytes(reset_vector_low, reset_vector_high);

    println!("Reset vector {:x}", reset_vector);

    // Set up registers
    let mut registers = Registers::new();
    registers.pc = reset_vector;

    loop {
        let byte = memory[registers.pc as usize];
        let (instruction, addressing_mode) = Instructions::match_instruction(byte);
        match instruction {
            Instructions::Instructions::SEI => {
                Instructions::sei(&mut registers);
                registers.pc += 1;
            }
            Instructions::Instructions::CLD => {
                Instructions::cld(&mut registers);
                registers.pc += 1;
            }
            Instructions::Instructions::LDA => {
                let low_byte = memory[registers.pc as usize];
                let high_byte = memory[registers.pc as usize];
                let k = apply_addressing(
                    &memory,
                    &registers,
                    addressing_mode,
                    Some(low_byte),
                    Some(high_byte),
                );

                Instructions::lda(&mut registers, (k.unwrap() & 0x00FF) as u8);
                registers.pc += 1;
            }
        }
    }
}
