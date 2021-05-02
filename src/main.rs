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
        stack_pointer: u16,
    }

    impl Memory {
        pub fn new() -> Self {
            let mut memory = Vec::new();
            memory.resize_with(0x10000, || 0);
            let mut ppu = Vec::new();
            ppu.resize_with(0x3F21, || 0);

            Self {
                memory,
                ppu,
                stack_pointer: 0x01FF,
            }
        }

        pub fn stack_push(&mut self, val: u8) {
            self.memory[self.stack_pointer as usize] = val;
            self.stack_pointer -= 1;
        }
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

    pub mod instructions {

        use crate::address_from_bytes;

        use super::*;

        pub enum Instructions {
            SEI,
            CLD,
            LDA,
            BRK,
            STA,
            INC,
            LDX,
            TXS,
            AND,
            BEQ,
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
                // BRK
                0x0 => (Instructions::BRK, AddressingMode::Implied),
                // STA
                0x8d => (Instructions::STA, AddressingMode::Absolute),
                0x9d => (Instructions::STA, AddressingMode::AbsoluteIndirectWithX),
                0x99 => (Instructions::STA, AddressingMode::AbsoluteIndirectWithY),
                0x85 => (Instructions::STA, AddressingMode::ZeroPage),
                0x81 => (Instructions::STA, AddressingMode::ZeroPageIndexedIndirect),
                0x95 => (Instructions::STA, AddressingMode::ZeroPageIndexedWithX),
                0x91 => (
                    Instructions::STA,
                    AddressingMode::ZeroPageIndirectIndexedWithY,
                ),
                // INC
                0xEE => (Instructions::INC, AddressingMode::Absolute),
                0xFE => (Instructions::INC, AddressingMode::AbsoluteIndirectWithX),
                0xE6 => (Instructions::INC, AddressingMode::ZeroPage),
                0xF6 => (Instructions::INC, AddressingMode::ZeroPageIndexedWithX),
                // LDX
                0xAE => (Instructions::LDX, AddressingMode::Absolute),
                0xBE => (Instructions::LDX, AddressingMode::AbsoluteIndirectWithY),
                0xA2 => (Instructions::LDX, AddressingMode::Immediate),
                0xA6 => (Instructions::LDX, AddressingMode::ZeroPage),
                0xB6 => (Instructions::LDX, AddressingMode::ZeroPageIndexedWithY),
                // TXS
                0x9a => (Instructions::TXS, AddressingMode::Implied),
                // AND
                0x2D => (Instructions::AND, AddressingMode::Absolute),
                0x3D => (Instructions::AND, AddressingMode::AbsoluteIndirectWithX),
                0x39 => (Instructions::AND, AddressingMode::AbsoluteIndirectWithY),
                0x29 => (Instructions::AND, AddressingMode::Immediate),
                0x2A => (Instructions::AND, AddressingMode::Accumulator),
                0x26 => (Instructions::AND, AddressingMode::ZeroPage),
                0x36 => (Instructions::AND, AddressingMode::ZeroPageIndexedWithX),
                // BEQ
                0xF0 => (Instructions::BEQ, AddressingMode::Relative),
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

        pub fn brk(registers: &mut Registers, memory: &mut Memory) {
            registers.pc += 2;
            memory.stack_push(((registers.pc >> 8) & 0xFF) as u8);
            memory.stack_push((registers.pc & 0xFF) as u8);
            registers.status |= 0b00010100; // Enable B and I registers
            memory.stack_push(registers.status);
            registers.pc = address_from_bytes(memory.memory[0xFFFE], memory.memory[0xFFFF])
        }

        #[test]
        fn brk_test() {
            let mut registers = Registers::new();
            let mut memory = Memory::new();

            brk(&mut registers, &mut memory);
            assert_eq!(registers.status, 0b00010100);
            assert_eq!(memory.memory[0x01FF], 0);
            assert_eq!(memory.memory[0x01FE], 2);
        }

        pub fn sta(registers: &mut Registers, memory: &mut Memory, addr: u16) {
            memory.memory[addr as usize] = registers.a;
        }

        #[test]
        fn sta_test() {
            let mut registers = Registers::new();
            let mut memory = Memory::new();
            registers.a = 0x42;
            sta(&mut registers, &mut memory, 0x12);
            assert_eq!(memory.memory[0x12], 0x42);
        }

        pub fn inc(registers: &mut Registers, memory: &mut Memory, addr: u16) {
            memory.memory[addr as usize] += 1;

            let operand = memory.memory[addr as usize];

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
        fn inc_test() {
            let mut registers = Registers::new();
            let mut memory = Memory::new();

            memory.memory[0x0] = 41;
            inc(&mut registers, &mut memory, 0x0);
            assert_eq!(memory.memory[0x0], 42);
        }

        pub fn ldx(registers: &mut Registers, operand: u8) {
            registers.x = operand;
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
        fn ldx_test() {
            let mut registers = Registers::new();
            ldx(&mut registers, 0x42);
            assert_eq!(registers.x, 0x42);
            ldx(&mut registers, 0x0);
            assert_eq!(registers.x, 0x0);
            assert_eq!(registers.status & 0b00000010, 0b00000010);
            ldx(&mut registers, 0x80);
            assert_eq!(registers.x, 0x80);
            assert_eq!(registers.status & 0b10000000, 0b10000000);
        }

        pub fn txs(registers: &mut Registers, memory: &mut Memory) {
            memory.stack_push(registers.x);
        }

        #[test]
        fn txs_test() {
            let mut registers = Registers::new();
            let mut memory = Memory::new();

            registers.x = 42;
            txs(&mut registers, &mut memory);

            assert_eq!(memory.memory[0x01FF], 42);
        }

        pub fn and(registers: &mut Registers, value: u8) {
            registers.a &= value;
        }

        #[test]
        fn and_test() {
            let mut registers = Registers::new();
            let mut memory = Memory::new();

            registers.a = 0b00000001;
            memory.memory[0x1] = 0b00000001;
            and(&mut registers, 0x1);
            assert_eq!(registers.a, 1);

            registers.a = 0b00000000;
            memory.memory[0x1] = 0b00000001;
            and(&mut registers, 0x1);
            assert_eq!(registers.a, 0);
        }

        pub fn beq(registers: &mut Registers, value: u16) -> bool {
            // Check if zero flag is enabled
            if (registers.status & 0b00000010) == 0b00000010 {
                let mut value = value as i32;
                if value >= 0x80 {
                    // BEQ only has relative addressing mode, so only low bytes used
                    value -= 1 << 8;
                }
                registers.pc = 2 + (registers.pc as i32 + value as i32) as u16;
                true
            } else {
                false
            }
        }

        #[test]
        fn beq_test() {
            let mut registers = Registers::new();

            registers.status = 0b00000000;
            beq(&mut registers, 0x10);
            assert_eq!(registers.pc, 0x0);

            registers.pc = 0x0;
            registers.status = 0b00000010;
            beq(&mut registers, 0x10);
            assert_eq!(registers.pc, 0x12);

            registers.pc = 0x43;
            registers.status = 0b00000010;
            beq(&mut registers, 0xFD);
            assert_eq!(registers.pc, 0x42);
        }

        // fn convert_u8_hex_to_dex(val: i8) -> i8 {
        //     let mut table = [0i8; 256];

        //     table
        //         .iter_mut()
        //         .enumerate()
        //         .map(|(i, _)| {
        //             let val = i as u8;
        //             if val & 0b10000000 == 0b10000000 {
        //                 val - (1 << 7)
        //             } else {
        //                 val
        //             }
        //         })
        //         .collect::<[i8; 256]>();

        //     table[val as usize]
        // }

        /**
        Applies addressing mode rules to operands and gives out 16-bit results
         */
        pub fn apply_addressing(
            memory: &Memory,
            registers: &Registers,
            adressing_mode: AddressingMode,
            low_byte: Option<u8>,
            high_byte: Option<u8>,
        ) -> Option<u16> {
            let memory = &memory.memory;
            let addr = match adressing_mode {
                AddressingMode::Accumulator => None,
                AddressingMode::Implied => None,
                AddressingMode::Immediate => Some(low_byte.unwrap().into()),
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
            };

            addr
        }

        pub fn num_operands_from_addressing(adressing_mode: &AddressingMode) -> u8 {
            match adressing_mode {
                AddressingMode::Accumulator => 1,
                AddressingMode::Implied => 0,
                AddressingMode::Immediate => 1,
                AddressingMode::Absolute => 2,
                AddressingMode::ZeroPage => 1,
                AddressingMode::Relative => 1,
                AddressingMode::AbsoluteIndirect => 2,
                AddressingMode::AbsoluteIndirectWithX => 2,
                AddressingMode::AbsoluteIndirectWithY => 2,
                AddressingMode::ZeroPageIndexedWithX => 1,
                AddressingMode::ZeroPageIndexedWithY => 1,
                AddressingMode::ZeroPageIndexedIndirect => 1,
                AddressingMode::ZeroPageIndirectIndexedWithY => 1,
            }
        }

        #[test]
        fn apply_addressing_test() {
            let mut memory = Memory::new();

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
                None,
            );
            assert_eq!(res, Some(0x1));

            // ABSOLUTE
            memory.memory[0x201] = 42;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::Absolute,
                Some(0x1),
                Some(0x2),
            );
            assert_eq!(res, Some(42));

            // ZERO PAGE
            memory.memory[0x0] = 43;
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
            memory.memory[0xA001] = 0xFF;
            memory.memory[0xA002] = 0x00;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::AbsoluteIndirect,
                Some(0x01),
                Some(0xA0),
            );
            assert_eq!(res, Some(0x00FF));

            // AbsoluteIndirectWithX
            memory.memory[0xC003] = 0x5A;
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
            memory.memory[0xF004] = 0xEF;
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
            memory.memory[0x3] = 0xA5;
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
            memory.memory[0x4] = 0xE3;
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
            memory.memory[0x17] = 0x10;
            memory.memory[0x18] = 0xD0;
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
            memory.memory[0x002A] = 0x35;
            memory.memory[0x002B] = 0xC2;
            memory.memory[0xC238] = 0x2F;
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

use rp2a03::{instructions::*, Registers, *};
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

fn get_operands(registers: &Registers, memory: &Memory) -> (u8, u8) {
    let low = memory.memory[(registers.pc + 1) as usize];
    let high = memory.memory[(registers.pc + 2) as usize];
    (low, high)
}

fn main() {
    println!("Nessy 🐉!");

    // Initialise memory
    let mut memory = Memory::new();

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
        memory.memory[(PRGROM_ADDRESS + index) as usize] = rom[index as usize];
    }

    let bank_seven = 16 + 7 * 16384;

    for index in 0..16384 {
        memory.memory[(0xC000 + index) as usize] = rom[(bank_seven + index) as usize];
    }

    // Get the RESET vector to find start of the game
    let reset_vector_low = memory.memory[RESET_VECTOR_ADDRESS as usize];
    let reset_vector_high = memory.memory[(RESET_VECTOR_ADDRESS + 1) as usize];

    println!("hi {:x} lo {:x}", reset_vector_high, reset_vector_low);

    let reset_vector = address_from_bytes(reset_vector_low, reset_vector_high);

    println!("Reset vector {:x}", reset_vector);

    // Set up registers
    let mut registers = Registers::new();
    registers.pc = reset_vector;

    loop {
        let byte = memory.memory[registers.pc as usize];
        let (instruction, addressing_mode) = match_instruction(byte);

        let num_operands = num_operands_from_addressing(&addressing_mode) as u16 + 1;

        match instruction {
            Instructions::SEI => {
                sei(&mut registers);
                registers.pc += num_operands;
            }
            Instructions::CLD => {
                cld(&mut registers);
                registers.pc += num_operands;
            }
            Instructions::LDA => {
                let (low_byte, high_byte) = get_operands(&registers, &memory);
                let addr = apply_addressing(
                    &memory,
                    &registers,
                    addressing_mode,
                    Some(low_byte),
                    Some(high_byte),
                )
                .unwrap();

                lda(&mut registers, addr as u8);
                registers.pc += num_operands;
            }
            Instructions::BRK => {
                brk(&mut registers, &mut memory);
                // NOTE: Shouldn't change pc as set by brk instruction
                // TODO: Check if need to advance pc by 1, but probs not
            }
            Instructions::STA => {
                let (low_byte, high_byte) = get_operands(&registers, &memory);
                let addr = apply_addressing(
                    &memory,
                    &registers,
                    addressing_mode,
                    Some(low_byte),
                    Some(high_byte),
                )
                .unwrap();
                sta(&mut registers, &mut memory, addr);
                registers.pc += num_operands;
            }
            Instructions::INC => {
                let (low_byte, high_byte) = get_operands(&registers, &memory);
                let addr = apply_addressing(
                    &memory,
                    &registers,
                    addressing_mode,
                    Some(low_byte),
                    Some(high_byte),
                )
                .unwrap();
                inc(&mut registers, &mut memory, addr);
                registers.pc += num_operands;
            }
            Instructions::LDX => {
                let (low_byte, high_byte) = get_operands(&registers, &memory);
                let addr = apply_addressing(
                    &memory,
                    &registers,
                    addressing_mode,
                    Some(low_byte),
                    Some(high_byte),
                )
                .unwrap();

                ldx(&mut registers, addr as u8);

                registers.pc += num_operands;
            }
            Instructions::TXS => {
                txs(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            Instructions::AND => {
                let (low_byte, high_byte) = get_operands(&registers, &memory);
                let addr = apply_addressing(
                    &memory,
                    &registers,
                    addressing_mode,
                    Some(low_byte),
                    Some(high_byte),
                )
                .unwrap();

                and(&mut registers, memory.memory[addr as usize]);
                registers.pc += num_operands;
            }
            Instructions::BEQ => {
                let (low_byte, high_byte) = get_operands(&registers, &memory);
                let addr = apply_addressing(
                    &memory,
                    &registers,
                    addressing_mode,
                    Some(low_byte),
                    Some(high_byte),
                )
                .unwrap();

                if !beq(&mut registers, addr) {
                    registers.pc += num_operands;
                }
            }
        }
    }
}
