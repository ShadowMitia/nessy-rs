use super::{*, instructions::InstructionName};

pub fn address_from_bytes(low_byte: u8, high_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | low_byte as u16
}

pub const NMI_VECTOR_ADDRESS: u32 = 0xFFFA;
pub const RESET_VECTOR_ADDRESS: u32 = 0xFFFC;
pub const BREAK_VECTOR_ADDDRESS: u32 = 0xFFFE;

pub fn get_operands(registers: &Registers, memory: &Memory) -> (u8, u8) {
    let low = memory.memory[(registers.pc + 1) as usize];
    let high = memory.memory[(registers.pc + 2) as usize];
    (low, high)
}

/// Test if addr1 and addr2 are on different 6502 pages
pub fn is_page_crossed(addr1: u16, addr2: u16) -> bool {
    // println!(
    //     "page crossed {} ({:X} {:X} | {:X} {:X})",
    //     addr1 & 0xFF00 != addr2 & 0xFF00,
    //     addr1,
    //     addr2,
    //     addr1 & 0xFF00,
    //     addr2 & 0xFF00
    // );
    addr1 & 0xFF00 != addr2 & 0xFF00
}

pub fn get_cycles(
    instruction: InstructionName,
    addressing_mode: AddressingMode,
    page_crossed: bool,
    branches: bool,
) -> u8 {
    let page_cross = if page_crossed { 1 } else { 0 };
    match (instruction, addressing_mode) {
        (InstructionName::SEI, AddressingMode::Implied) => 2,
        (InstructionName::CLD, AddressingMode::Implied) => 2,
        (InstructionName::LDA, AddressingMode::Immediate) => 2,
        (InstructionName::LDA, AddressingMode::Absolute) => 4,
        (InstructionName::LDA, AddressingMode::ZeroPage) => 3,
        (InstructionName::LDA, AddressingMode::AbsoluteIndirectWithX) => 4 + page_cross,
        (InstructionName::LDA, AddressingMode::AbsoluteIndirectWithY) => 4 + page_cross,
        (InstructionName::LDA, AddressingMode::ZeroPageIndexedWithX) => 4,
        (InstructionName::LDA, AddressingMode::ZeroPageIndexedIndirect) => 6,
        (InstructionName::LDA, AddressingMode::ZeroPageIndirectIndexedWithY) => 5 + page_cross,
        (InstructionName::BRK, AddressingMode::Implied) => 7,
        (InstructionName::STA, AddressingMode::Absolute) => 4,
        (InstructionName::STA, AddressingMode::ZeroPage) => 3,
        (InstructionName::STA, AddressingMode::AbsoluteIndirectWithX) => 5,
        (InstructionName::STA, AddressingMode::AbsoluteIndirectWithY) => 5,
        (InstructionName::STA, AddressingMode::ZeroPageIndexedWithX) => 4,
        (InstructionName::STA, AddressingMode::ZeroPageIndexedIndirect) => 6,
        (InstructionName::STA, AddressingMode::ZeroPageIndirectIndexedWithY) => 6,
        (InstructionName::INC, AddressingMode::Absolute) => 6,
        (InstructionName::INC, AddressingMode::ZeroPage) => 5,
        (InstructionName::INC, AddressingMode::AbsoluteIndirectWithX) => 7,
        (InstructionName::INC, AddressingMode::ZeroPageIndexedWithX) => 6,
        (InstructionName::LDX, AddressingMode::Immediate) => 2,
        (InstructionName::LDX, AddressingMode::Absolute) => 4,
        (InstructionName::LDX, AddressingMode::ZeroPage) => 3,
        (InstructionName::LDX, AddressingMode::AbsoluteIndirectWithY) => 4 + page_cross,
        (InstructionName::LDX, AddressingMode::ZeroPageIndexedWithY) => 4,
        (InstructionName::TXS, AddressingMode::Implied) => 2,
        (InstructionName::AND, AddressingMode::Immediate) => 2,
        (InstructionName::AND, AddressingMode::Absolute) => 4,
        (InstructionName::AND, AddressingMode::ZeroPage) => 3,
        (InstructionName::AND, AddressingMode::AbsoluteIndirectWithX) => 4 + page_cross,
        (InstructionName::AND, AddressingMode::AbsoluteIndirectWithY) => 4 + page_cross,
        (InstructionName::AND, AddressingMode::ZeroPageIndexedWithX) => 4,
        (InstructionName::AND, AddressingMode::ZeroPageIndexedIndirect) => 6,
        (InstructionName::AND, AddressingMode::ZeroPageIndirectIndexedWithY) => 5 + page_cross,
        (InstructionName::BEQ, AddressingMode::Relative) => {
            2 + if branches {
                if page_crossed {
                    2
                } else {
                    1
                }
            } else {
                0
            }
        }
        (InstructionName::CPX, AddressingMode::Immediate) => 2,
        (InstructionName::CPX, AddressingMode::Absolute) => 4,
        (InstructionName::CPX, AddressingMode::ZeroPage) => 3,
        (InstructionName::DEY, AddressingMode::Implied) => 2,
        (InstructionName::BPL, AddressingMode::Relative) => {
            2 + if branches {
                if page_crossed {
                    2
                } else {
                    1
                }
            } else {
                0
            }
        }
        (InstructionName::PLA, AddressingMode::Implied) => 4,
        (InstructionName::TAY, AddressingMode::Implied) => 2,
        (InstructionName::CPY, AddressingMode::Immediate) => 2,
        (InstructionName::CPY, AddressingMode::Absolute) => 4,
        (InstructionName::CPY, AddressingMode::ZeroPage) => 3,
        (InstructionName::BNE, AddressingMode::Relative) => {
            2 + if branches {
                if page_crossed {
                    2
                } else {
                    1
                }
            } else {
                0
            }
        }
        (InstructionName::RTS, AddressingMode::Implied) => 6,
        (InstructionName::JMP, AddressingMode::Absolute) => 3,
        (InstructionName::JMP, AddressingMode::AbsoluteIndirect) => 5,
        (InstructionName::STX, AddressingMode::Absolute) => 4,
        (InstructionName::STX, AddressingMode::ZeroPage) => 3,
        (InstructionName::STX, AddressingMode::ZeroPageIndexedWithY) => 4,
        (InstructionName::JSR, AddressingMode::Absolute) => 6,
        (InstructionName::NOP, AddressingMode::Implied) => 2,
        (InstructionName::NOP, AddressingMode::Immediate) => 2,
        (InstructionName::NOP, AddressingMode::Absolute) => 4,
        (InstructionName::NOP, AddressingMode::AbsoluteIndirectWithX) => 4 + page_cross,
        (InstructionName::NOP, AddressingMode::ZeroPage) => 3,
        (InstructionName::NOP, AddressingMode::ZeroPageIndexedWithX) => 4 + page_cross,
        (InstructionName::SEC, AddressingMode::Implied) => 2,
        (InstructionName::BCS, AddressingMode::Relative) => {
            2 + if branches {
                if page_crossed {
                    2
                } else {
                    1
                }
            } else {
                0
            }
        }
        (InstructionName::CLC, AddressingMode::Implied) => 2,
        (InstructionName::BCC, AddressingMode::Relative) => {
            2 + if branches {
                if page_crossed {
                    2
                } else {
                    1
                }
            } else {
                0
            }
        }
        (InstructionName::PHP, AddressingMode::Implied) => 3,
        (InstructionName::BIT, AddressingMode::Absolute) => 4,
        (InstructionName::BIT, AddressingMode::ZeroPage) => 3,
        (InstructionName::BVS, AddressingMode::Relative) => {
            2 + if branches {
                if page_crossed {
                    2
                } else {
                    1
                }
            } else {
                0
            }
        }
        (InstructionName::BVC, AddressingMode::Relative) => {
            2 + if branches {
                if page_crossed {
                    2
                } else {
                    1
                }
            } else {
                0
            }
        }
        (InstructionName::LDY, AddressingMode::Immediate) => 2,
        (InstructionName::LDY, AddressingMode::Absolute) => 4,
        (InstructionName::LDY, AddressingMode::ZeroPage) => 3,
        (InstructionName::LDY, AddressingMode::AbsoluteIndirectWithX) => 4 + page_cross,
        (InstructionName::LDY, AddressingMode::ZeroPageIndexedWithX) => 4,
        (InstructionName::ASL, AddressingMode::Accumulator) => 2,
        (InstructionName::ASL, AddressingMode::Absolute) => 6,
        (InstructionName::ASL, AddressingMode::ZeroPage) => 5,
        (InstructionName::ASL, AddressingMode::AbsoluteIndirectWithX) => 7,
        (InstructionName::ASL, AddressingMode::ZeroPageIndexedWithX) => 6,
        (InstructionName::RTI, AddressingMode::Implied) => 6,
        (InstructionName::SBC, AddressingMode::Immediate) => 2,
        (InstructionName::SBC, AddressingMode::Absolute) => 4,
        (InstructionName::SBC, AddressingMode::ZeroPage) => 3,
        (InstructionName::SBC, AddressingMode::AbsoluteIndirectWithX) => 4 + page_cross,
        (InstructionName::SBC, AddressingMode::AbsoluteIndirectWithY) => 4 + page_cross,
        (InstructionName::SBC, AddressingMode::ZeroPageIndexedWithX) => 4,
        (InstructionName::SBC, AddressingMode::ZeroPageIndexedIndirect) => 6,
        (InstructionName::SBC, AddressingMode::ZeroPageIndirectIndexedWithY) => 5 + page_cross,
        (InstructionName::SED, AddressingMode::Implied) => 2,
        (InstructionName::CMP, AddressingMode::Immediate) => 2,
        (InstructionName::CMP, AddressingMode::Absolute) => 4,
        (InstructionName::CMP, AddressingMode::ZeroPage) => 3,
        (InstructionName::CMP, AddressingMode::AbsoluteIndirectWithX) => 4 + page_cross,
        (InstructionName::CMP, AddressingMode::AbsoluteIndirectWithY) => 4 + page_cross,
        (InstructionName::CMP, AddressingMode::ZeroPageIndexedWithX) => 4,
        (InstructionName::CMP, AddressingMode::ZeroPageIndexedIndirect) => 6,
        (InstructionName::CMP, AddressingMode::ZeroPageIndirectIndexedWithY) => 5 + page_cross,
        (InstructionName::PHA, AddressingMode::Implied) => 3,
        (InstructionName::PLP, AddressingMode::Implied) => 4,
        (InstructionName::BMI, AddressingMode::Relative) => {
            2 + if branches {
                if page_crossed {
                    2
                } else {
                    1
                }
            } else {
                0
            }
        }
        (InstructionName::ORA, AddressingMode::Immediate) => 2,
        (InstructionName::ORA, AddressingMode::Absolute) => 4,
        (InstructionName::ORA, AddressingMode::ZeroPage) => 3,
        (InstructionName::ORA, AddressingMode::AbsoluteIndirectWithX) => 4 + page_cross,
        (InstructionName::ORA, AddressingMode::AbsoluteIndirectWithY) => 4 + page_cross,
        (InstructionName::ORA, AddressingMode::ZeroPageIndexedWithX) => 4,
        (InstructionName::ORA, AddressingMode::ZeroPageIndexedIndirect) => 6,
        (InstructionName::ORA, AddressingMode::ZeroPageIndirectIndexedWithY) => 5 + page_cross,
        (InstructionName::CLV, AddressingMode::Implied) => 2,
        (InstructionName::EOR, AddressingMode::Immediate) => 2,
        (InstructionName::EOR, AddressingMode::Absolute) => 4,
        (InstructionName::EOR, AddressingMode::ZeroPage) => 3,
        (InstructionName::EOR, AddressingMode::AbsoluteIndirectWithX) => 4 + page_cross,
        (InstructionName::EOR, AddressingMode::AbsoluteIndirectWithY) => 4 + page_cross,
        (InstructionName::EOR, AddressingMode::ZeroPageIndexedWithX) => 4,
        (InstructionName::EOR, AddressingMode::ZeroPageIndexedIndirect) => 6,
        (InstructionName::EOR, AddressingMode::ZeroPageIndirectIndexedWithY) => 5 + page_cross,
        (InstructionName::ADC, AddressingMode::Immediate) => 2,
        (InstructionName::ADC, AddressingMode::Absolute) => 4,
        (InstructionName::ADC, AddressingMode::ZeroPage) => 3,
        (InstructionName::ADC, AddressingMode::AbsoluteIndirectWithX) => 4 + page_cross,
        (InstructionName::ADC, AddressingMode::AbsoluteIndirectWithY) => 4 + page_cross,
        (InstructionName::ADC, AddressingMode::ZeroPageIndexedWithX) => 4,
        (InstructionName::ADC, AddressingMode::ZeroPageIndexedIndirect) => 6,
        (InstructionName::ADC, AddressingMode::ZeroPageIndirectIndexedWithY) => 5 + page_cross,
        (InstructionName::STY, AddressingMode::Absolute) => 4,
        (InstructionName::STY, AddressingMode::ZeroPage) => 3,
        (InstructionName::STY, AddressingMode::ZeroPageIndexedWithX) => 4,
        (InstructionName::INY, AddressingMode::Implied) => 2,
        (InstructionName::INX, AddressingMode::Implied) => 2,
        (InstructionName::TAX, AddressingMode::Implied) => 2,
        (InstructionName::TYA, AddressingMode::Implied) => 2,
        (InstructionName::TXA, AddressingMode::Implied) => 2,
        (InstructionName::TSX, AddressingMode::Implied) => 2,
        (InstructionName::DEX, AddressingMode::Implied) => 2,
        (InstructionName::LSR, AddressingMode::Accumulator) => 2,
        (InstructionName::LSR, AddressingMode::Absolute) => 6,
        (InstructionName::LSR, AddressingMode::ZeroPage) => 5,
        (InstructionName::LSR, AddressingMode::AbsoluteIndirectWithX) => 7,
        (InstructionName::LSR, AddressingMode::ZeroPageIndexedWithX) => 6,
        (InstructionName::ROR, AddressingMode::Accumulator) => 2,
        (InstructionName::ROR, AddressingMode::Absolute) => 6,
        (InstructionName::ROR, AddressingMode::ZeroPage) => 5,
        (InstructionName::ROR, AddressingMode::AbsoluteIndirectWithX) => 7,
        (InstructionName::ROR, AddressingMode::ZeroPageIndexedWithX) => 6,
        (InstructionName::ROL, AddressingMode::Accumulator) => 2,
        (InstructionName::ROL, AddressingMode::Absolute) => 6,
        (InstructionName::ROL, AddressingMode::ZeroPage) => 5,
        (InstructionName::ROL, AddressingMode::AbsoluteIndirectWithX) => 7,
        (InstructionName::ROL, AddressingMode::ZeroPageIndexedWithX) => 6,
        (InstructionName::DEC, AddressingMode::Absolute) => 6,
        (InstructionName::DEC, AddressingMode::ZeroPage) => 5,
        (InstructionName::DEC, AddressingMode::AbsoluteIndirectWithX) => 7,
        (InstructionName::DEC, AddressingMode::ZeroPageIndexedWithX) => 6,
        (InstructionName::LAX, AddressingMode::Immediate) => 2,
        (InstructionName::LAX, AddressingMode::Absolute) => 4,
        (InstructionName::LAX, AddressingMode::ZeroPage) => 3,
        (InstructionName::LAX, AddressingMode::AbsoluteIndirectWithY) => 4 + page_cross,
        (InstructionName::LAX, AddressingMode::ZeroPageIndexedWithY) => 4,
        (InstructionName::LAX, AddressingMode::ZeroPageIndexedIndirect) => 6,
        (InstructionName::LAX, AddressingMode::ZeroPageIndirectIndexedWithY) => 5 + page_cross,
        (InstructionName::SAX, AddressingMode::Absolute) => 4,
        (InstructionName::SAX, AddressingMode::ZeroPage) => 3,
        (InstructionName::SAX, AddressingMode::ZeroPageIndexedWithY) => 4,
        (InstructionName::SAX, AddressingMode::ZeroPageIndexedIndirect) => 6,
        (InstructionName::SAX, AddressingMode::ZeroPageIndirectIndexedWithY) => 6,
        (InstructionName::DCP, AddressingMode::Absolute) => 6,
        (InstructionName::DCP, AddressingMode::ZeroPage) => 5,
        (InstructionName::DCP, AddressingMode::AbsoluteIndirectWithX) => 7,
        (InstructionName::DCP, AddressingMode::AbsoluteIndirectWithY) => 7,
        (InstructionName::DCP, AddressingMode::ZeroPageIndexedWithX) => 6,
        (InstructionName::DCP, AddressingMode::ZeroPageIndexedIndirect) => 8,
        (InstructionName::DCP, AddressingMode::ZeroPageIndirectIndexedWithY) => 8,
        (InstructionName::ISB, AddressingMode::Absolute) => 6,
        (InstructionName::ISB, AddressingMode::ZeroPage) => 5,
        (InstructionName::ISB, AddressingMode::AbsoluteIndirectWithX) => 7,
        (InstructionName::ISB, AddressingMode::AbsoluteIndirectWithY) => 7,
        (InstructionName::ISB, AddressingMode::ZeroPageIndexedWithX) => 6,
        (InstructionName::ISB, AddressingMode::ZeroPageIndexedIndirect) => 8,
        (InstructionName::ISB, AddressingMode::ZeroPageIndirectIndexedWithY) => 8,
        (InstructionName::SLO, AddressingMode::Absolute) => 6,
        (InstructionName::SLO, AddressingMode::ZeroPage) => 5,
        (InstructionName::SLO, AddressingMode::AbsoluteIndirectWithX) => 7,
        (InstructionName::SLO, AddressingMode::AbsoluteIndirectWithY) => 7,
        (InstructionName::SLO, AddressingMode::ZeroPageIndexedWithX) => 6,
        (InstructionName::SLO, AddressingMode::ZeroPageIndexedIndirect) => 8,
        (InstructionName::SLO, AddressingMode::ZeroPageIndirectIndexedWithY) => 8,
        (InstructionName::RLA, AddressingMode::Absolute) => 6,
        (InstructionName::RLA, AddressingMode::ZeroPage) => 5,
        (InstructionName::RLA, AddressingMode::AbsoluteIndirectWithX) => 7,
        (InstructionName::RLA, AddressingMode::AbsoluteIndirectWithY) => 7,
        (InstructionName::RLA, AddressingMode::ZeroPageIndexedWithX) => 6,
        (InstructionName::RLA, AddressingMode::ZeroPageIndexedIndirect) => 8,
        (InstructionName::RLA, AddressingMode::ZeroPageIndirectIndexedWithY) => 8,
        (InstructionName::SRE, AddressingMode::Absolute) => 6,
        (InstructionName::SRE, AddressingMode::ZeroPage) => 5,
        (InstructionName::SRE, AddressingMode::AbsoluteIndirectWithX) => 7,
        (InstructionName::SRE, AddressingMode::AbsoluteIndirectWithY) => 7,
        (InstructionName::SRE, AddressingMode::ZeroPageIndexedWithX) => 6,
        (InstructionName::SRE, AddressingMode::ZeroPageIndexedIndirect) => 8,
        (InstructionName::SRE, AddressingMode::ZeroPageIndirectIndexedWithY) => 8,
        (InstructionName::RRA, AddressingMode::Absolute) => 6,
        (InstructionName::RRA, AddressingMode::ZeroPage) => 5,
        (InstructionName::RRA, AddressingMode::AbsoluteIndirectWithX) => 7,
        (InstructionName::RRA, AddressingMode::AbsoluteIndirectWithY) => 7,
        (InstructionName::RRA, AddressingMode::ZeroPageIndexedWithX) => 6,
        (InstructionName::RRA, AddressingMode::ZeroPageIndexedIndirect) => 8,
        (InstructionName::RRA, AddressingMode::ZeroPageIndirectIndexedWithY) => 8,
        _ => unreachable!(),
    }
}

/**
Applies addressing mode rules to operands and gives out 16-bit results
 */
pub fn apply_addressing(
    memory: &Memory,
    registers: &Registers,
    adressing_mode: AddressingMode,
    low_byte: u8,
    high_byte: u8,
) -> Option<u16> {
    let memory = &memory.memory;
    let addr = match adressing_mode {
        AddressingMode::Accumulator => None,
        AddressingMode::Implied => None,
        AddressingMode::Immediate => Some(low_byte.into()),
        AddressingMode::Absolute => {
            let addr = address_from_bytes(low_byte, high_byte);
            Some(addr)
        }
        AddressingMode::ZeroPage => {
            let addr = low_byte;
            Some(addr as u16)
        }
        AddressingMode::Relative => Some(low_byte as u16),
        AddressingMode::AbsoluteIndirect => {
            let addr = address_from_bytes(low_byte, high_byte);
            // NOTE: Handle hardware bug for JMP with absolute indirect
            if low_byte == 0xFF {
                let addr2 = address_from_bytes(0x0, high_byte);

                Some(address_from_bytes(
                    memory[addr as usize],
                    memory[addr2 as usize],
                ))
            } else {
                let addr2 = addr + 1;
                let res = address_from_bytes(memory[addr as usize], memory[addr2 as usize]);
                Some(res)
            }
        }
        AddressingMode::AbsoluteIndirectWithX => {
            let tmp = address_from_bytes(low_byte, high_byte);
            let addr = tmp.wrapping_add(registers.x.into()) as u16;
            Some(addr as u16)
        }
        AddressingMode::AbsoluteIndirectWithY => {
            let tmp = address_from_bytes(low_byte, high_byte);
            let addr = tmp.wrapping_add(registers.y.into()) as u16;
            Some(addr as u16)
        }
        AddressingMode::ZeroPageIndexedWithX => {
            let tmp = low_byte;
            let addr = low_byte.wrapping_add(registers.x);
            Some(addr as u16)
        }
        AddressingMode::ZeroPageIndexedWithY => {
            let tmp = low_byte;
            let addr = low_byte.wrapping_add(registers.y);
            Some(addr as u16)
        }
        AddressingMode::ZeroPageIndexedIndirect => {
            let base = low_byte.wrapping_add(registers.x);
            let addr = memory[base as usize];
            let addr2 = memory[base.wrapping_add(1) as usize];
            let res = address_from_bytes(addr, addr2);
            Some(res as u16)
        }
        AddressingMode::ZeroPageIndirectIndexedWithY => {
            let addr = low_byte;
            let low_byte = *memory.get(addr as usize).unwrap();
            let high_byte = *memory.get((addr.wrapping_add(1)) as usize).unwrap();
            let addr =
                address_from_bytes(low_byte, high_byte).wrapping_add(registers.y.into()) as u16;
            Some(addr as u16)
        }
    };

    addr
}

pub fn num_operands_from_addressing(adressing_mode: &AddressingMode) -> u8 {
    match adressing_mode {
        AddressingMode::Accumulator => 0,
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

// TODO: add some missing cases (negative cases, ...)
// #[test]
// fn apply_addressing_test() {
//     let mut memory = Memory::new();

//     let mut registers = Registers::new();

//     // Accumulator
//     let res = apply_addressing(&memory, &registers, AddressingMode::Accumulator, 0x0, 0x0);
//     assert_eq!(res, None);

//     // IMPLIED
//     let res = apply_addressing(&memory, &registers, AddressingMode::Implied, 0x0, 0x0);
//     assert_eq!(res, None);

//     // IMMEDIATE
//     let res = apply_addressing(&memory, &registers, AddressingMode::Immediate, 0x22, 0x0);
//     assert_eq!(res, Some(0x22));

//     let res = apply_addressing(&memory, &registers, AddressingMode::Immediate, 0x81, 0x42);
//     assert_eq!(res, Some(0x81));

//     // ABSOLUTE
//     memory.memory[0x201] = 42;
//     let res = apply_addressing(&memory, &registers, AddressingMode::Absolute, 0x10, 0xD0);
//     assert_eq!(res, Some(0xD010));

//     // ZERO PAGE
//     let res = apply_addressing(&memory, &registers, AddressingMode::ZeroPage, 0x4, 0x0);
//     assert_eq!(res, Some(0x4));

//     // Relative
//     let res = apply_addressing(&memory, &registers, AddressingMode::Relative, 0x42, 0x0);
//     assert_eq!(res, Some(0x42));

//     // AbsoluteIndirect
//     memory.memory[0xA001] = 0xFF;
//     memory.memory[0xA002] = 0x00;
//     let res = apply_addressing(
//         &memory,
//         &registers,
//         AddressingMode::AbsoluteIndirect,
//         0x01,
//         0xA0,
//     );
//     assert_eq!(res, Some(0x00FF));

//     // AbsoluteIndirectWithX
//     registers.x = 0x2;
//     let res = apply_addressing(
//         &memory,
//         &registers,
//         AddressingMode::AbsoluteIndirectWithX,
//         0x1,
//         0xC0,
//     );
//     assert_eq!(res, Some(0xC003));

//     // AbsoluteIndirectWithY
//     registers.y = 0x3;
//     let res = apply_addressing(
//         &memory,
//         &registers,
//         AddressingMode::AbsoluteIndirectWithY,
//         0x1,
//         0xF0,
//     );
//     assert_eq!(res, Some(0xF004));

//     // ZeroPageIndexedWithX
//     registers.x = 0x2;
//     let res = apply_addressing(
//         &memory,
//         &registers,
//         AddressingMode::ZeroPageIndexedWithX,
//         0x1,
//         0x0,
//     );
//     assert_eq!(res, Some(0x03));

//     // ZeroPageIndexedWithY
//     registers.y = 3;
//     let res = apply_addressing(
//         &memory,
//         &registers,
//         AddressingMode::ZeroPageIndexedWithY,
//         0x1,
//         0x0,
//     );
//     assert_eq!(res, Some(0x04));

//     // Zero Page Indexed Indirect
//     memory.memory[0x17] = 0x10;
//     memory.memory[0x18] = 0xD0;
//     registers.x = 2;
//     let res = apply_addressing(
//         &memory,
//         &registers,
//         AddressingMode::ZeroPageIndexedIndirect,
//         0x15,
//         0x0,
//     );
//     assert_eq!(res, Some(0xD010));

//     // Zero Page Indexed Indirect with Y
//     memory.memory[0x002A] = 0x35;
//     memory.memory[0x002B] = 0xC2;
//     registers.y = 0x3;
//     let res = apply_addressing(
//         &memory,
//         &registers,
//         AddressingMode::ZeroPageIndirectIndexedWithY,
//         0x2A,
//         0x0,
//     );
//     assert_eq!(res, Some(0xC238));
// }
