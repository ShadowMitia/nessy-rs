use super::*;

/// Represents the three letter name of an instruction
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum InstructionName {
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
    CPX,
    DEY,
    BPL,
    PLA,
    TAY,
    CPY,
    BNE,
    RTS,
    JMP,
    STX,
    JSR,
    NOP,
    SEC,
    BCS,
    CLC,
    BCC,
    PHP,
    BIT,
    BVS,
    BVC,
    LDY,
    ASL,
    RTI,
    SBC,
    SED,
    CMP,
    PHA,
    PLP,
    BMI,
    ORA,
    CLV,
    EOR,
    ADC,
    STY,
    INY,
    INX,
    TAX,
    TYA,
    TXA,
    TSX,
    DEX,
    LSR,
    ROR,
    ROL,
    DEC,
    // UNOFFICIALS
    LAX,
    SAX,
    DCP,
    ISB, // Sometimes designated ISC
    SLO,
    RLA,
    SRE,
    RRA,
}

/// Associates an InstructionName to an AddressingMode, used by `match_instruction`
/// to convert opcodes to instruction and adressing mode
/// Also differentiates Official from Unofficial opcodes, for clarity
pub enum Instruction {
    Official(InstructionName, AddressingMode),
    Unofficial(InstructionName, AddressingMode),
    Unknown,
}

/// Given an `u8` opcode, returns the `Instruction` corresponding to the instruction and adressing mode
///
/// Manages all official and unoffical unstrictions. Retursn `Unknown` if opcode is invalid.
#[must_use]
pub fn match_instruction(opcode: u8) -> Instruction {
    match opcode {
        // LDA
        0xA9 => Instruction::Official(InstructionName::LDA, AddressingMode::Immediate),
        0xA5 => Instruction::Official(InstructionName::LDA, AddressingMode::ZeroPage),
        0xB5 => Instruction::Official(InstructionName::LDA, AddressingMode::ZeroPageIndexedWithX),
        0xAD => Instruction::Official(InstructionName::LDA, AddressingMode::Absolute),
        0xBD => Instruction::Official(InstructionName::LDA, AddressingMode::AbsoluteIndirectWithX),
        0xB9 => Instruction::Official(InstructionName::LDA, AddressingMode::AbsoluteIndirectWithY),
        0xA1 => Instruction::Official(
            InstructionName::LDA,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0xB1 => Instruction::Official(
            InstructionName::LDA,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        // SEI
        0x78 => Instruction::Official(InstructionName::SEI, AddressingMode::Implied),
        0xd8 => Instruction::Official(InstructionName::CLD, AddressingMode::Implied),
        // BRK
        0x0 => Instruction::Official(InstructionName::BRK, AddressingMode::Implied),
        // STA
        0x8d => Instruction::Official(InstructionName::STA, AddressingMode::Absolute),
        0x9d => Instruction::Official(InstructionName::STA, AddressingMode::AbsoluteIndirectWithX),
        0x99 => Instruction::Official(InstructionName::STA, AddressingMode::AbsoluteIndirectWithY),
        0x85 => Instruction::Official(InstructionName::STA, AddressingMode::ZeroPage),
        0x81 => Instruction::Official(
            InstructionName::STA,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0x95 => Instruction::Official(InstructionName::STA, AddressingMode::ZeroPageIndexedWithX),
        0x91 => Instruction::Official(
            InstructionName::STA,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        // INC
        0xEE => Instruction::Official(InstructionName::INC, AddressingMode::Absolute),
        0xFE => Instruction::Official(InstructionName::INC, AddressingMode::AbsoluteIndirectWithX),
        0xE6 => Instruction::Official(InstructionName::INC, AddressingMode::ZeroPage),
        0xF6 => Instruction::Official(InstructionName::INC, AddressingMode::ZeroPageIndexedWithX),
        // LDX
        0xAE => Instruction::Official(InstructionName::LDX, AddressingMode::Absolute),
        0xBE => Instruction::Official(InstructionName::LDX, AddressingMode::AbsoluteIndirectWithY),
        0xA2 => Instruction::Official(InstructionName::LDX, AddressingMode::Immediate),
        0xA6 => Instruction::Official(InstructionName::LDX, AddressingMode::ZeroPage),
        0xB6 => Instruction::Official(InstructionName::LDX, AddressingMode::ZeroPageIndexedWithY),
        // TXS
        0x9a => Instruction::Official(InstructionName::TXS, AddressingMode::Implied),
        // AND
        0x29 => Instruction::Official(InstructionName::AND, AddressingMode::Immediate),
        0x25 => Instruction::Official(InstructionName::AND, AddressingMode::ZeroPage),
        0x35 => Instruction::Official(InstructionName::AND, AddressingMode::ZeroPageIndexedWithX),
        0x2D => Instruction::Official(InstructionName::AND, AddressingMode::Absolute),
        0x3D => Instruction::Official(InstructionName::AND, AddressingMode::AbsoluteIndirectWithX),
        0x39 => Instruction::Official(InstructionName::AND, AddressingMode::AbsoluteIndirectWithY),
        0x21 => Instruction::Official(
            InstructionName::AND,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0x31 => Instruction::Official(
            InstructionName::AND,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        // BEQ
        0xF0 => Instruction::Official(InstructionName::BEQ, AddressingMode::Relative),
        // CPX
        0xEC => Instruction::Official(InstructionName::CPX, AddressingMode::Absolute),
        0xE0 => Instruction::Official(InstructionName::CPX, AddressingMode::Immediate),
        0xE4 => Instruction::Official(InstructionName::CPX, AddressingMode::ZeroPage),
        // DEY
        0x88 => Instruction::Official(InstructionName::DEY, AddressingMode::Implied),
        // BPL
        0x10 => Instruction::Official(InstructionName::BPL, AddressingMode::Relative),
        // PLA
        0x68 => Instruction::Official(InstructionName::PLA, AddressingMode::Implied),
        // TAY
        0xA8 => Instruction::Official(InstructionName::TAY, AddressingMode::Implied),
        // CPY
        0xCC => Instruction::Official(InstructionName::CPY, AddressingMode::Absolute),
        0xC0 => Instruction::Official(InstructionName::CPY, AddressingMode::Immediate),
        0xC4 => Instruction::Official(InstructionName::CPY, AddressingMode::ZeroPage),
        // BNE
        0xD0 => Instruction::Official(InstructionName::BNE, AddressingMode::Relative),
        // RTS
        0x60 => Instruction::Official(InstructionName::RTS, AddressingMode::Implied),
        // JMP
        0x4C => Instruction::Official(InstructionName::JMP, AddressingMode::Absolute),
        0x6C => Instruction::Official(InstructionName::JMP, AddressingMode::AbsoluteIndirect),
        // STX
        0x8E => Instruction::Official(InstructionName::STX, AddressingMode::Absolute),
        0x86 => Instruction::Official(InstructionName::STX, AddressingMode::ZeroPage),
        0x96 => Instruction::Official(InstructionName::STX, AddressingMode::ZeroPageIndexedWithY),
        // JSR
        0x20 => Instruction::Official(InstructionName::JSR, AddressingMode::Absolute),
        // NOP
        0xEA => Instruction::Official(InstructionName::NOP, AddressingMode::Implied),
        // SEC
        0x38 => Instruction::Official(InstructionName::SEC, AddressingMode::Implied),
        // BCS
        0xB0 => Instruction::Official(InstructionName::BCS, AddressingMode::Relative),
        // CLC
        0x18 => Instruction::Official(InstructionName::CLC, AddressingMode::Implied),
        // BCC
        0x90 => Instruction::Official(InstructionName::BCC, AddressingMode::Relative),
        // PHP
        0x08 => Instruction::Official(InstructionName::PHP, AddressingMode::Implied),
        // BIT
        0x2C => Instruction::Official(InstructionName::BIT, AddressingMode::Absolute),
        0x89 => Instruction::Official(InstructionName::BIT, AddressingMode::Immediate),
        0x24 => Instruction::Official(InstructionName::BIT, AddressingMode::ZeroPage),
        // BVS
        0x70 => Instruction::Official(InstructionName::BVS, AddressingMode::Relative),
        //BVC
        0x50 => Instruction::Official(InstructionName::BVC, AddressingMode::Relative),
        // LDY
        0xAC => Instruction::Official(InstructionName::LDY, AddressingMode::Absolute),
        0xBC => Instruction::Official(InstructionName::LDY, AddressingMode::AbsoluteIndirectWithX),
        0xA0 => Instruction::Official(InstructionName::LDY, AddressingMode::Immediate),
        0xA4 => Instruction::Official(InstructionName::LDY, AddressingMode::ZeroPage),
        0xB4 => Instruction::Official(InstructionName::LDY, AddressingMode::ZeroPageIndexedWithX),
        // ASL
        0x0E => Instruction::Official(InstructionName::ASL, AddressingMode::Absolute),
        0x1E => Instruction::Official(InstructionName::ASL, AddressingMode::AbsoluteIndirectWithX),
        0x0A => Instruction::Official(InstructionName::ASL, AddressingMode::Accumulator),
        0x06 => Instruction::Official(InstructionName::ASL, AddressingMode::ZeroPage),
        0x16 => Instruction::Official(InstructionName::ASL, AddressingMode::ZeroPageIndexedWithX),
        // RTI
        0x40 => Instruction::Official(InstructionName::RTI, AddressingMode::Implied),
        // SBC
        0xED => Instruction::Official(InstructionName::SBC, AddressingMode::Absolute),
        0xFD => Instruction::Official(InstructionName::SBC, AddressingMode::AbsoluteIndirectWithX),
        0xF9 => Instruction::Official(InstructionName::SBC, AddressingMode::AbsoluteIndirectWithY),
        0xE9 => Instruction::Official(InstructionName::SBC, AddressingMode::Immediate),
        0xE5 => Instruction::Official(InstructionName::SBC, AddressingMode::ZeroPage),
        0xE1 => Instruction::Official(
            InstructionName::SBC,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0xF5 => Instruction::Official(InstructionName::SBC, AddressingMode::ZeroPageIndexedWithX),
        0xF1 => Instruction::Official(
            InstructionName::SBC,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        // SED
        0xF8 => Instruction::Official(InstructionName::SED, AddressingMode::Implied),
        // CMP
        0xCD => Instruction::Official(InstructionName::CMP, AddressingMode::Absolute),
        0xDD => Instruction::Official(InstructionName::CMP, AddressingMode::AbsoluteIndirectWithX),
        0xD9 => Instruction::Official(InstructionName::CMP, AddressingMode::AbsoluteIndirectWithY),
        0xC9 => Instruction::Official(InstructionName::CMP, AddressingMode::Immediate),
        0xC5 => Instruction::Official(InstructionName::CMP, AddressingMode::ZeroPage),
        0xC1 => Instruction::Official(
            InstructionName::CMP,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0xD5 => Instruction::Official(InstructionName::CMP, AddressingMode::ZeroPageIndexedWithX),
        0xD1 => Instruction::Official(
            InstructionName::CMP,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        // PHA
        0x48 => Instruction::Official(InstructionName::PHA, AddressingMode::Implied),
        // PLP
        0x28 => Instruction::Official(InstructionName::PLP, AddressingMode::Implied),
        // BMI
        0x30 => Instruction::Official(InstructionName::BMI, AddressingMode::Relative),
        // ORA
        0x0D => Instruction::Official(InstructionName::ORA, AddressingMode::Absolute),
        0x1D => Instruction::Official(InstructionName::ORA, AddressingMode::AbsoluteIndirectWithX),
        0x19 => Instruction::Official(InstructionName::ORA, AddressingMode::AbsoluteIndirectWithY),
        0x09 => Instruction::Official(InstructionName::ORA, AddressingMode::Immediate),
        0x05 => Instruction::Official(InstructionName::ORA, AddressingMode::ZeroPage),
        0x01 => Instruction::Official(
            InstructionName::ORA,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0x15 => Instruction::Official(InstructionName::ORA, AddressingMode::ZeroPageIndexedWithX),
        0x11 => Instruction::Official(
            InstructionName::ORA,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        // CLV
        0xB8 => Instruction::Official(InstructionName::CLV, AddressingMode::Implied),
        // EOR
        0x4D => Instruction::Official(InstructionName::EOR, AddressingMode::Absolute),
        0x5D => Instruction::Official(InstructionName::EOR, AddressingMode::AbsoluteIndirectWithX),
        0x59 => Instruction::Official(InstructionName::EOR, AddressingMode::AbsoluteIndirectWithY),
        0x49 => Instruction::Official(InstructionName::EOR, AddressingMode::Immediate),
        0x45 => Instruction::Official(InstructionName::EOR, AddressingMode::ZeroPage),
        0x41 => Instruction::Official(
            InstructionName::EOR,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0x55 => Instruction::Official(InstructionName::EOR, AddressingMode::ZeroPageIndexedWithX),
        0x51 => Instruction::Official(
            InstructionName::EOR,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        // ADC
        0x6D => Instruction::Official(InstructionName::ADC, AddressingMode::Absolute),
        0x7D => Instruction::Official(InstructionName::ADC, AddressingMode::AbsoluteIndirectWithX),
        0x79 => Instruction::Official(InstructionName::ADC, AddressingMode::AbsoluteIndirectWithY),
        0x69 => Instruction::Official(InstructionName::ADC, AddressingMode::Immediate),
        0x65 => Instruction::Official(InstructionName::ADC, AddressingMode::ZeroPage),
        0x61 => Instruction::Official(
            InstructionName::ADC,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0x75 => Instruction::Official(InstructionName::ADC, AddressingMode::ZeroPageIndexedWithX),
        0x71 => Instruction::Official(
            InstructionName::ADC,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        // STY
        0x8C => Instruction::Official(InstructionName::STY, AddressingMode::Absolute),
        0x84 => Instruction::Official(InstructionName::STY, AddressingMode::ZeroPage),
        0x94 => Instruction::Official(InstructionName::STY, AddressingMode::ZeroPageIndexedWithX),
        // INY
        0xC8 => Instruction::Official(InstructionName::INY, AddressingMode::Implied),
        // INX
        0xE8 => Instruction::Official(InstructionName::INX, AddressingMode::Implied),
        // TAX
        0xAA => Instruction::Official(InstructionName::TAX, AddressingMode::Implied),
        // TYA
        0x98 => Instruction::Official(InstructionName::TYA, AddressingMode::Implied),
        // TXA
        0x8A => Instruction::Official(InstructionName::TXA, AddressingMode::Implied),
        // TSX
        0xBA => Instruction::Official(InstructionName::TSX, AddressingMode::Implied),
        // DEX
        0xCA => Instruction::Official(InstructionName::DEX, AddressingMode::Implied),
        // LSR
        0x4A => Instruction::Official(InstructionName::LSR, AddressingMode::Accumulator),
        0x46 => Instruction::Official(InstructionName::LSR, AddressingMode::ZeroPage),
        0x56 => Instruction::Official(InstructionName::LSR, AddressingMode::ZeroPageIndexedWithX),
        0x4E => Instruction::Official(InstructionName::LSR, AddressingMode::Absolute),
        0x5E => Instruction::Official(InstructionName::LSR, AddressingMode::AbsoluteIndirectWithX),
        // ROR
        0x6A => Instruction::Official(InstructionName::ROR, AddressingMode::Accumulator),
        0x66 => Instruction::Official(InstructionName::ROR, AddressingMode::ZeroPage),
        0x76 => Instruction::Official(InstructionName::ROR, AddressingMode::ZeroPageIndexedWithX),
        0x6E => Instruction::Official(InstructionName::ROR, AddressingMode::Absolute),
        0x7E => Instruction::Official(InstructionName::ROR, AddressingMode::AbsoluteIndirectWithX),
        // ROL
        0x2A => Instruction::Official(InstructionName::ROL, AddressingMode::Accumulator),
        0x26 => Instruction::Official(InstructionName::ROL, AddressingMode::ZeroPage),
        0x36 => Instruction::Official(InstructionName::ROL, AddressingMode::ZeroPageIndexedWithX),
        0x2E => Instruction::Official(InstructionName::ROL, AddressingMode::Absolute),
        0x3E => Instruction::Official(InstructionName::ROL, AddressingMode::AbsoluteIndirectWithX),
        // DEC
        0xC6 => Instruction::Official(InstructionName::DEC, AddressingMode::ZeroPage),
        0xD6 => Instruction::Official(InstructionName::DEC, AddressingMode::ZeroPageIndexedWithX),
        0xCE => Instruction::Official(InstructionName::DEC, AddressingMode::Absolute),
        0xDE => Instruction::Official(InstructionName::DEC, AddressingMode::AbsoluteIndirectWithX),

        // UNOFFICIAL OPCODES
        // NOP
        0x04 => Instruction::Unofficial(InstructionName::NOP, AddressingMode::ZeroPage),
        0x44 => Instruction::Unofficial(InstructionName::NOP, AddressingMode::ZeroPage),
        0x64 => Instruction::Unofficial(InstructionName::NOP, AddressingMode::ZeroPage),
        0x0C => Instruction::Unofficial(InstructionName::NOP, AddressingMode::Absolute),
        0x14 => Instruction::Unofficial(InstructionName::NOP, AddressingMode::ZeroPageIndexedWithX),
        0x34 => Instruction::Unofficial(InstructionName::NOP, AddressingMode::ZeroPageIndexedWithX),
        0x54 => Instruction::Unofficial(InstructionName::NOP, AddressingMode::ZeroPageIndexedWithX),
        0x74 => Instruction::Unofficial(InstructionName::NOP, AddressingMode::ZeroPageIndexedWithX),
        0xd4 => Instruction::Unofficial(InstructionName::NOP, AddressingMode::ZeroPageIndexedWithX),
        0xF4 => Instruction::Unofficial(InstructionName::NOP, AddressingMode::ZeroPageIndexedWithX),
        0x1A => Instruction::Unofficial(InstructionName::NOP, AddressingMode::Implied),
        0x3A => Instruction::Unofficial(InstructionName::NOP, AddressingMode::Implied),
        0x5A => Instruction::Unofficial(InstructionName::NOP, AddressingMode::Implied),
        0x7A => Instruction::Unofficial(InstructionName::NOP, AddressingMode::Implied),
        0xDA => Instruction::Unofficial(InstructionName::NOP, AddressingMode::Implied),
        0xFA => Instruction::Unofficial(InstructionName::NOP, AddressingMode::Implied),
        0x80 => Instruction::Unofficial(InstructionName::NOP, AddressingMode::Immediate),
        0x1C => {
            Instruction::Unofficial(InstructionName::NOP, AddressingMode::AbsoluteIndirectWithX)
        }
        0x3C => {
            Instruction::Unofficial(InstructionName::NOP, AddressingMode::AbsoluteIndirectWithX)
        }
        0x5C => {
            Instruction::Unofficial(InstructionName::NOP, AddressingMode::AbsoluteIndirectWithX)
        }
        0x7C => {
            Instruction::Unofficial(InstructionName::NOP, AddressingMode::AbsoluteIndirectWithX)
        }
        0xDC => {
            Instruction::Unofficial(InstructionName::NOP, AddressingMode::AbsoluteIndirectWithX)
        }
        0xFC => {
            Instruction::Unofficial(InstructionName::NOP, AddressingMode::AbsoluteIndirectWithX)
        }
        // LAX
        0xA3 => Instruction::Unofficial(
            InstructionName::LAX,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0xA7 => Instruction::Unofficial(InstructionName::LAX, AddressingMode::ZeroPage),
        0xAF => Instruction::Unofficial(InstructionName::LAX, AddressingMode::Absolute),
        0xB3 => Instruction::Unofficial(
            InstructionName::LAX,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        0xB7 => Instruction::Unofficial(InstructionName::LAX, AddressingMode::ZeroPageIndexedWithY),
        0xBF => {
            Instruction::Unofficial(InstructionName::LAX, AddressingMode::AbsoluteIndirectWithY)
        }
        // SAX
        0x83 => Instruction::Unofficial(
            InstructionName::SAX,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0x87 => Instruction::Unofficial(InstructionName::SAX, AddressingMode::ZeroPage),
        0x8F => Instruction::Unofficial(InstructionName::SAX, AddressingMode::Absolute),
        0x97 => Instruction::Unofficial(InstructionName::SAX, AddressingMode::ZeroPageIndexedWithY),
        // SBC
        0xEB => Instruction::Unofficial(InstructionName::SBC, AddressingMode::Immediate),
        // DCP
        0xC3 => Instruction::Unofficial(
            InstructionName::DCP,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0xC7 => Instruction::Unofficial(InstructionName::DCP, AddressingMode::ZeroPage),
        0xCF => Instruction::Unofficial(InstructionName::DCP, AddressingMode::Absolute),
        0xDF => {
            Instruction::Unofficial(InstructionName::DCP, AddressingMode::AbsoluteIndirectWithX)
        }
        0xDB => {
            Instruction::Unofficial(InstructionName::DCP, AddressingMode::AbsoluteIndirectWithY)
        }
        0xD7 => Instruction::Unofficial(InstructionName::DCP, AddressingMode::ZeroPageIndexedWithX),
        0xD3 => Instruction::Unofficial(
            InstructionName::DCP,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        // ISC
        0xE3 => Instruction::Unofficial(
            InstructionName::ISB,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0xE7 => Instruction::Unofficial(InstructionName::ISB, AddressingMode::ZeroPage),
        0xEF => Instruction::Unofficial(InstructionName::ISB, AddressingMode::Absolute),
        0xF3 => Instruction::Unofficial(
            InstructionName::ISB,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        0xF7 => Instruction::Unofficial(InstructionName::ISB, AddressingMode::ZeroPageIndexedWithX),
        0xFB => {
            Instruction::Unofficial(InstructionName::ISB, AddressingMode::AbsoluteIndirectWithY)
        }
        0xFF => {
            Instruction::Unofficial(InstructionName::ISB, AddressingMode::AbsoluteIndirectWithX)
        }
        // SLO
        0x03 => Instruction::Unofficial(
            InstructionName::SLO,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0x07 => Instruction::Unofficial(InstructionName::SLO, AddressingMode::ZeroPage),
        0x0F => Instruction::Unofficial(InstructionName::SLO, AddressingMode::Absolute),
        0x17 => Instruction::Unofficial(InstructionName::SLO, AddressingMode::ZeroPageIndexedWithX),
        0x1F => {
            Instruction::Unofficial(InstructionName::SLO, AddressingMode::AbsoluteIndirectWithX)
        }
        0x1B => {
            Instruction::Unofficial(InstructionName::SLO, AddressingMode::AbsoluteIndirectWithY)
        }
        0x13 => Instruction::Unofficial(
            InstructionName::SLO,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        // RLA
        0x27 => Instruction::Unofficial(InstructionName::RLA, AddressingMode::ZeroPage),
        0x37 => Instruction::Unofficial(InstructionName::RLA, AddressingMode::ZeroPageIndexedWithX),
        0x2F => Instruction::Unofficial(InstructionName::RLA, AddressingMode::Absolute),
        0x3F => {
            Instruction::Unofficial(InstructionName::RLA, AddressingMode::AbsoluteIndirectWithX)
        }
        0x3B => {
            Instruction::Unofficial(InstructionName::RLA, AddressingMode::AbsoluteIndirectWithY)
        }
        0x23 => Instruction::Unofficial(
            InstructionName::RLA,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0x33 => Instruction::Unofficial(
            InstructionName::RLA,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        // SRE
        0x47 => Instruction::Unofficial(InstructionName::SRE, AddressingMode::ZeroPage),
        0x57 => Instruction::Unofficial(InstructionName::SRE, AddressingMode::ZeroPageIndexedWithX),
        0x4F => Instruction::Unofficial(InstructionName::SRE, AddressingMode::Absolute),
        0x5F => {
            Instruction::Unofficial(InstructionName::SRE, AddressingMode::AbsoluteIndirectWithX)
        }
        0x5B => {
            Instruction::Unofficial(InstructionName::SRE, AddressingMode::AbsoluteIndirectWithY)
        }
        0x43 => Instruction::Unofficial(
            InstructionName::SRE,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0x53 => Instruction::Unofficial(
            InstructionName::SRE,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        // RRA
        0x67 => Instruction::Unofficial(InstructionName::RRA, AddressingMode::ZeroPage),
        0x77 => Instruction::Unofficial(InstructionName::RRA, AddressingMode::ZeroPageIndexedWithX),
        0x6F => Instruction::Unofficial(InstructionName::RRA, AddressingMode::Absolute),
        0x7F => {
            Instruction::Unofficial(InstructionName::RRA, AddressingMode::AbsoluteIndirectWithX)
        }
        0x7B => {
            Instruction::Unofficial(InstructionName::RRA, AddressingMode::AbsoluteIndirectWithY)
        }
        0x63 => Instruction::Unofficial(
            InstructionName::RRA,
            AddressingMode::ZeroPageIndexedIndirect,
        ),
        0x73 => Instruction::Unofficial(
            InstructionName::RRA,
            AddressingMode::ZeroPageIndirectIndexedWithY,
        ),
        // UNKNOWN
        _ => Instruction::Unknown,
    }
}

/// Set Interrupt Disable (SEI)
///
/// Status I flag is set to 1
pub fn sei(registers: &mut Registers) {
    registers.status |= 0b00000100;
}

#[test]
fn sei_test() {
    let mut registers = Registers::new();
    registers.pc += 1; // Simulate reading insruction
    sei(&mut registers);
    assert_eq!(registers.status, 0b00000100);
}

/// Clear Decimal Mode (CLD)
///
/// Status D flag is set to 0
pub fn cld(registers: &mut Registers) {
    registers.status &= 0b11110111;
}

#[test]
fn cld_test() {
    let mut registers = Registers::new();
    registers.status |= 0b00001000;
    registers.pc += 1; // Simulate reading insruction
    cld(&mut registers);
    assert_eq!(registers.status, 0b00000000);
}

/// Load Accumulator (LDA)
///
/// Loads a byte of memory into the accumulator.
/// Sets the zero and negative flags as appropriate.
pub fn lda(registers: &mut Registers, operand: u8) {
    registers.a = operand as u8;
    registers.set_flag(StatusFlag::Z, registers.a == 0);
    registers.set_flag(StatusFlag::N, registers.a >= 0x80);
}

#[test]
fn lda_test() {
    let mut registers = Registers::new();
    registers.pc += 1; // Simulate reading insruction
    lda(&mut registers, 0x42);
    assert_eq!(registers.a, 0x42);
    registers.pc += 1; // Simulate reading insruction
    lda(&mut registers, 0x0);
    assert_eq!(registers.a, 0x0);
    assert_eq!(registers.status & 0b00000010, 0b00000010);
    registers.pc += 1; // Simulate reading insruction
    lda(&mut registers, 0x80);
    assert_eq!(registers.a, 0x80);
    assert_eq!(registers.status & 0b10000000, 0b10000000);
    registers.pc += 1; // Simulate reading insruction
    lda(&mut registers, 0x80);
    assert_eq!(registers.a, 0x80);
    assert_eq!(registers.status & 0b10000000, 0b10000000);
}

/// Force Interrupt (BRK)
///
/// The BRK instruction forces the generation of an interrupt request.
/// The program counter and processor status are pushed on the stack then
/// the IRQ interrupt vector at $FFFE/F is loaded into the PC
/// and the break flag in the status set to one.
pub fn brk(registers: &mut Registers, memory: &mut Memory) {
    registers.pc += 1;
    memory.stack_push(((registers.pc >> 8) & 0xFF) as u8);
    memory.stack_push((registers.pc & 0xFF) as u8);

    registers.set_flag(StatusFlag::B, true);
    registers.set_flag(StatusFlag::Unused, true);
    memory.stack_push(registers.status);
    registers.set_flag(StatusFlag::I, true);
    registers.pc = utils::address_from_bytes(
        memory.memory[utils::BREAK_VECTOR_ADDDRESS as usize],
        memory.memory[(utils::BREAK_VECTOR_ADDDRESS + 1) as usize],
    );
    // registers.set_flag(StatusFlag::B, false);
    // registers.set_flag(StatusFlag::Unused, false);
}

#[test]
fn brk_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();
    memory.memory[utils::BREAK_VECTOR_ADDDRESS as usize] = 0x42;
    memory.memory[(utils::BREAK_VECTOR_ADDDRESS + 1) as usize] = 0x0;
    registers.pc += 1; // Simulate reading insruction
    brk(&mut registers, &mut memory);
    assert_eq!(registers.status, 0b00110100);
    assert_eq!(memory.memory[0x01FE], 2);
    assert_eq!(memory.memory[0x01FF], 0);
    assert_eq!(registers.pc, 0x42);
}

/// Store Accumulator (STA)
///
/// Stores the contents of the accumulator into memory.
pub fn sta(registers: &mut Registers, memory: &mut Memory, addr: u16) {
    memory.memory[addr as usize] = registers.a;
}

#[test]
fn sta_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();
    registers.a = 0x42;
    registers.pc += 1; // Simulate reading insruction
    sta(&mut registers, &mut memory, 0x12);
    assert_eq!(memory.memory[0x12], 0x42);
}

/// Increment Memory (INC)
///
/// Adds one to the value held at a specified memory location.
/// Sets the zero and negative flags as appropriate.
pub fn inc(registers: &mut Registers, memory: &mut Memory, addr: u16) {
    let operand = memory.memory[addr as usize] as u16;
    if operand == 0xFF {
        memory.memory[addr as usize] = 0;
    } else {
        memory.memory[addr as usize] += 1;
    }

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
    registers.pc += 1; // Simulate reading insruction
    inc(&mut registers, &mut memory, 0x0);
    assert_eq!(memory.memory[0x0], 42);
}

/// Load X Register (LDX)
///
/// Loads a byte of memory into the X register
/// Sets the zero and negative flags as appropriate.
pub fn ldx(registers: &mut Registers, addr: u16) {
    registers.x = addr as u8;
    registers.set_flag(StatusFlag::Z, registers.x == 0);
    registers.set_flag(StatusFlag::N, registers.x >= 0x80);
}

#[test]
fn ldx_test() {
    let mut registers = Registers::new();

    registers.pc += 1; // Simulate reading insruction
    ldx(&mut registers, 0x42);
    assert_eq!(registers.x, 0x42);
    registers.pc += 1; // Simulate reading insruction
    ldx(&mut registers, 0x0);
    assert_eq!(registers.x, 0x0);
    assert_eq!(registers.status & 0b00000010, 0b00000010);
    registers.pc += 1; // Simulate reading insruction
    ldx(&mut registers, 0x80);
    assert_eq!(registers.x, 0x80);
    assert_eq!(registers.status & 0b10000000, 0b10000000);
}

/// Transfer X to stack pointer (TXS)
///
/// Copies the current contents of the X register into the stack register.
pub fn txs(registers: &mut Registers, memory: &mut Memory) {
    memory.stack_pointer = registers.x as u16;
}

#[test]
fn txs_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    registers.x = 42;
    registers.pc += 1; // Simulate reading insruction
    txs(&mut registers, &mut memory);

    assert_eq!(memory.stack_pointer, 42);
}

/// Logical And (AND)
///
/// A logical AND is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
pub fn and(registers: &mut Registers, value: u8) {
    registers.a &= value;

    registers.set_flag(StatusFlag::Z, registers.a == 0);
    registers.set_flag(StatusFlag::N, registers.a >= 0x80);
}

/// Logical And (AND) with immediate value
///
/// A logical AND is performed, bit by bit, on the accumulator contents using the immediate value.
pub fn and_acc(registers: &mut Registers) {
    registers.a &= registers.a;
    registers.set_flag(StatusFlag::Z, registers.a == 0);
    registers.set_flag(StatusFlag::N, registers.a >= 0x80);
}

#[test]
fn and_test() {
    let mut registers = Registers::new();

    registers.a = 0b00000001;
    registers.pc += 1; // Simulate reading insruction
    and(&mut registers, 0x1);
    assert_eq!(registers.a, 1);

    registers.a = 0b00000000;
    registers.pc += 1; // Simulate reading insruction
    and(&mut registers, 0x1);
    assert_eq!(registers.a, 0);

    registers.a = 0x6F;
    registers.pc += 1; // Simulate reading insruction
    and(&mut registers, 0xEF);
    assert_eq!(registers.a, 0x6F);
}

/// Branch If Equal (BEQ)
///
/// If the carry flag is set then add the relative displacement to the program counter to cause a branch to a new location.
#[must_use]
pub fn beq(registers: &mut Registers, value: u16) -> bool {
    // Check if zero flag is enabled
    if registers.is_flag_set(StatusFlag::Z) {
        if value >= 0x80 {
            let value = (value as i32 - (1 << 8)) as i16;
            registers.pc = 1 + (registers.pc as i16 + value) as u16;
        } else {
            registers.pc = 1 + (registers.pc as i16 + value as i16) as u16;
        }
        true
    } else {
        false
    }
}

#[test]
fn beq_test() {
    let mut registers = Registers::new();

    registers.status = 0b00000000;
    registers.pc += 1; // Simulate reading insruction
    let _ = beq(&mut registers, 0x10);
    assert_eq!(registers.pc, 0x1);

    registers.pc = 0x0;
    registers.status = 0b00000010;
    registers.pc += 1; // Simulate reading insruction
    let _ = beq(&mut registers, 0x10);
    assert_eq!(registers.pc, 0x12);

    registers.pc = 0x43;
    registers.status = 0b00000010;
    registers.pc += 1; // Simulate reading insruction
    let _ = beq(&mut registers, 0xFD);
    assert_eq!(registers.pc, 0x42);
}

/// Compare X Register (CPX)
///
/// This instruction compares the contents of the X register with another memory held value.
/// Sets the zero and carry flags as appropriate.
pub fn cpx(registers: &mut Registers, value: u8) {
    registers.set_flag(StatusFlag::C, false);
    registers.set_flag(StatusFlag::Z, false);

    match registers.x.cmp(&(value as u8)) {
        std::cmp::Ordering::Less => {
            // registers.status &= 0b00000000;
        }
        std::cmp::Ordering::Equal => {
            registers.set_flag(StatusFlag::C, true);
            registers.set_flag(StatusFlag::Z, true);
        }
        std::cmp::Ordering::Greater => registers.set_flag(StatusFlag::C, true),
    }

    let res = if value >= 0x80 {
        let value = value as i16 - (1 << 8);
        (registers.x as i16 - value as i16) as u8
    } else {
        (registers.x as i16 - value as i16) as u8
    };
    registers.set_flag(StatusFlag::N, res >= 0x80);
}

#[test]
fn cpx_test() {
    let mut registers = Registers::new();

    registers.x = 0x10;
    registers.pc += 1; // Simulate reading insruction
    cpx(&mut registers, 0x10);
    assert_eq!(registers.status, 0b00000011);

    registers.x = 0x9;
    registers.pc += 1; // Simulate reading insruction
    cpx(&mut registers, 0x10);
    assert_eq!(registers.status, 0b10000000);

    registers.x = 0x10;
    registers.pc += 1; // Simulate reading insruction
    cpx(&mut registers, 0x9);
    assert_eq!(registers.status, 0b00000001);

    registers.x = 0xFF;
    registers.pc += 1; // Simulate reading insruction
    cpx(&mut registers, 0x10);
    assert_eq!(registers.status, 0b10000001);
}

///  Decrement Y Register (DEY)
///
/// Subtracts one from the Y register.
/// Sets the zero and negative flags as appropriate.
pub fn dey(registers: &mut Registers) {
    registers.y = (registers.y as i16 - 1) as u8;

    registers.status = if registers.y == 0 {
        registers.status | 0b00000010
    } else {
        registers.status & 0b11111101
    };
    registers.status = if registers.y >= 0x80 {
        registers.status | 0b10000000
    } else {
        registers.status & 0b01111111
    };
}

#[test]
fn dey_test() {
    let mut registers = Registers::new();

    registers.y = 0x43;
    registers.pc += 1; // Simulate reading insruction
    dey(&mut registers);
    assert_eq!(registers.y, 0x42);

    registers.y = 0x0;
    registers.pc += 1; // Simulate reading insruction
    dey(&mut registers);
    assert_eq!(registers.y, 0xFF);
}

/// Branch if Positive (BPL)
///
/// If the negative flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
#[must_use]
pub fn bpl(registers: &mut Registers, value: u16) -> bool {
    if !registers.is_flag_set(StatusFlag::N) {
        if value >= 0x80 {
            let value = (value as i32 - (1 << 8)) as i16;
            registers.pc = 1 + (registers.pc as i16 + value) as u16;
        } else {
            registers.pc = 1 + (registers.pc as i16 + value as i16) as u16;
        }
        true
    } else {
        false
    }
}

#[test]
fn bpl_test() {
    let mut registers = Registers::new();

    registers.y = 0x43;
    registers.pc += 1; // Simulate reading insruction
    dey(&mut registers);
    assert_eq!(registers.y, 0x42);

    registers.y = 0x0;
    registers.pc += 1; // Simulate reading insruction
    dey(&mut registers);
    assert_eq!(registers.y, 0xFF);
}

/// Pull Accumulator (PLA)
///
/// Pulls an 8 bit value from the stack and into the accumulator.
/// The zero and negative flags are set as appropriate.
pub fn pla(registers: &mut Registers, memory: &mut Memory) {
    registers.a = memory.stack_pop();

    registers.set_flag(StatusFlag::Z, registers.a == 0);
    registers.set_flag(StatusFlag::N, registers.a >= 0x80);
}

#[test]
fn pla_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    memory.stack_push(0x42);
    registers.pc += 1; // Simulate reading insruction
    pla(&mut registers, &mut memory);
    assert_eq!(registers.a, 0x42);
    assert_eq!(memory.stack_pointer, 0x1FF);

    memory.stack_push(0x6F);
    registers.status = 0x6F;
    registers.pc += 1; // Simulate reading insruction
    pla(&mut registers, &mut memory);
    assert_eq!(registers.a, 0x6F);
    assert_eq!(memory.stack_pointer, 0x1FF);
    assert_eq!(registers.status, 0x6D);
}

/// Transfer Accumulator to Y (TAY)
///
/// Copies the current contents of the accumulator into the X register.
/// Sets the zero and negative flags as appropriate.
pub fn tay(registers: &mut Registers) {
    registers.y = registers.a;
    registers.set_flag(StatusFlag::Z, registers.a == 0);
    registers.set_flag(StatusFlag::N, registers.a >= 0x80);
}

#[test]
fn tay_test() {
    let mut registers = Registers::new();
    registers.a = 0x42;
    registers.pc += 1; // Simulate reading insruction
    tay(&mut registers);
    assert_eq!(registers.a, 0x42);
    assert_eq!(registers.y, 0x42);

    registers.a = 0x99;
    registers.pc += 1; // Simulate reading insruction
    tay(&mut registers);
    assert_eq!(registers.a, 0x99);
    assert_eq!(registers.y, 0x99);
}

/// Compare Y Register (CPY)
///
/// This instruction compares the contents of the Y register with another memory held value.
/// Sets the zero and carry flags as appropriate.
pub fn cpy(registers: &mut Registers, value: u8) {
    registers.set_flag(StatusFlag::C, false);
    registers.set_flag(StatusFlag::Z, false);

    match registers.y.cmp(&(value as u8)) {
        std::cmp::Ordering::Less => {
            // registers.status &= 0b00000000;
        }
        std::cmp::Ordering::Equal => {
            registers.set_flag(StatusFlag::C, true);
            registers.set_flag(StatusFlag::Z, true);
        }
        std::cmp::Ordering::Greater => registers.set_flag(StatusFlag::C, true),
    }

    let res = if value >= 0x80 {
        let value = (value as i32 - (1 << 8)) as i16;
        (registers.y as i16 - value as i16) as u8
    } else {
        (registers.y as i16 - value as i16) as u8
    };
    registers.set_flag(StatusFlag::N, res >= 0x80);
}

#[test]
fn cpy_test() {
    let mut registers = Registers::new();

    registers.y = 0x10;
    registers.pc += 1; // Simulate reading insruction
    cpy(&mut registers, 0x10);
    assert_eq!(registers.status, 0b00000011);

    registers.y = 0x9;
    registers.pc += 1; // Simulate reading insruction
    cpy(&mut registers, 0x10);
    assert_eq!(registers.status, 0b10000000);

    registers.y = 0xFF;
    registers.pc += 1; // Simulate reading insruction
    cpy(&mut registers, 0x10);
    assert_eq!(registers.status, 0b10000001);
}

/// Branch if Not Equal (BNE)
///
/// If the zero flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
#[must_use]
pub fn bne(registers: &mut Registers, value: u16) -> bool {
    // Check if zero flag is not enabled
    if !registers.is_flag_set(StatusFlag::Z) {
        if value >= 0x80 {
            let value = (value as i16 - (1 << 8)) as i16;
            registers.pc = 1 + (registers.pc as i16 + value) as u16;
        } else {
            registers.pc = 1 + (registers.pc as i16 + value as i16) as u16;
        }
        true
    } else {
        false
    }
}

#[test]
fn bne_test() {
    let mut registers = Registers::new();

    registers.status = 0b00000010;
    registers.pc += 1; // Simulate reading insruction
    let _ = bne(&mut registers, 0x10);
    assert_eq!(registers.pc, 0x1);

    registers.status = 0b00000000;
    registers.pc = 0;
    registers.pc += 1; // Simulate reading insruction
    let _ = bne(&mut registers, 0x10);
    assert_eq!(registers.pc, 0x12);

    registers.status = 0xE4;
    registers.pc = 0xC957;
    registers.pc += 1; // Simulate reading insruction
    let _ = bne(&mut registers, 0x5);
    assert_eq!(registers.pc, 0xC95E);
}

/// Return from Subroutine (RTS)
///
/// Used at the end of a subroutine to return to the calling routine.
/// It pulls the program counter (minus one) from the stack.
pub fn rts(registers: &mut Registers, memory: &mut Memory) {
    let low = memory.stack_pop();
    let high = memory.stack_pop();
    let addr = utils::address_from_bytes(low, high);
    registers.pc = addr;
    registers.pc += 1;
}

#[test]
fn rts_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    memory.stack_push(0x0);
    memory.stack_push(0x4);

    registers.pc += 1; // Simulate reading insruction
    rts(&mut registers, &mut memory);
    assert_eq!(registers.pc, 0x5);
}

/// Jump (JMP)
///
/// Sets the program counter to the address specified by the operand.
pub fn jmp(registers: &mut Registers, addr: u16) {
    registers.pc = addr;
}

#[test]
fn jmp_test() {
    let mut registers = Registers::new();

    registers.pc += 1; // Simulate reading insruction
    jmp(&mut registers, 0x42);
    assert_eq!(registers.pc, 0x42);
}

/// Store X Register (STX)
///
/// Stores the contents of the X register into memory.
pub fn stx(registers: &mut Registers, memory: &mut Memory, addr: u16) {
    memory.memory[addr as usize] = registers.x;
}

#[test]
fn stx_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    registers.x = 0x42;
    registers.pc += 1; // Simulate reading insruction
    stx(&mut registers, &mut memory, 0x30);
    assert_eq!(memory.memory[0x30], 0x42);
}

/// Jump to Subroutine (JSR)
///
/// Pushes the address (minus one) of the return point on to the stack.
/// Sets the program counter to the target memory address.
pub fn jsr(registers: &mut Registers, memory: &mut Memory, addr: u16) {
    registers.pc += 1;
    memory.stack_push(((registers.pc >> 8) & 0xFF) as u8);
    memory.stack_push((registers.pc & 0xFF) as u8);
    registers.pc = addr;
}

#[test]
fn jsr_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();
    registers.pc = 0x42;
    registers.pc += 1; // Simulate reading insruction
    jsr(&mut registers, &mut memory, 0x100);
    assert_eq!(registers.pc, 0x100);
}

/// No Operation (NOP)
///
/// The NOP instruction causes no changes to the processor other than the normal incrementing of the program counter to the next instruction.
pub fn nop() {}

#[test]
fn nop_test() {
    // HOW CAN I TEST THIS :D
}

/// Set Carry Flag (SEC)
///
/// Set the carry flag to one.
pub fn sec(registers: &mut Registers) {
    registers.status |= 0x1;
}

#[test]
fn sec_test() {
    let mut registers = Registers::new();

    registers.pc += 1; // Simulate reading insruction
    sec(&mut registers);
    assert_eq!(registers.status & 0x1, 0x1);
}

///  Branch if Carry Set (BCS)
///
/// If the carry flag is set then add the relative displacement to the program counter to cause a branch to a new location.
#[must_use]
pub fn bcs(registers: &mut Registers, addr: u16) -> bool {
    if registers.is_flag_set(StatusFlag::C) {
        if addr >= 0x80 {
            let value = (addr as i32 - (1 << 8)) as i16;
            registers.pc = 1 + (registers.pc as i16 + value) as u16;
        } else {
            registers.pc = 1 + (registers.pc as i16 + addr as i16) as u16;
        }
        true
    } else {
        false
    }
}

#[test]
fn bcs_test() {
    let mut registers = Registers::new();

    registers.pc += 0xC72F;

    registers.pc += 1; // Simulate reading instruction
    let _ = bcs(&mut registers, 0x20);
    assert_eq!(registers.pc, 0xC730);

    registers.status |= 0x1;

    registers.pc = 0xC72F;
    registers.pc += 1; // Simulate reading insruction
    let _ = bcs(&mut registers, 0x4);
    assert_eq!(registers.pc, 0xC735);
}

/// Clear Carry Flag (CLC)
///
/// Set the carry flag to zero.
pub fn clc(registers: &mut Registers) {
    registers.status &= 0b11111110;
}

#[test]
fn clc_test() {
    let mut registers = Registers::new();
    registers.status = 0b1;
    registers.pc += 1; // Simulate reading insruction
    let _ = clc(&mut registers);
    assert_eq!(registers.status, 0x0);
}

/// Branch if Carry Clear (BCC)
///
/// If the carry flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
#[must_use]
pub fn bcc(registers: &mut Registers, addr: u16) -> bool {
    if !registers.is_flag_set(StatusFlag::C) {
        if addr >= 0x80 {
            let value = (addr as i32 - (1 << 8)) as i16;
            registers.pc = 1 + (registers.pc as i16 + value) as u16;
        } else {
            registers.pc = 1 + (registers.pc as i16 + addr as i16) as u16;
        }
        true
    } else {
        false
    }
}

#[test]
fn bcc_test() {
    let mut registers = Registers::new();
    registers.status = 0b1;
    registers.pc += 1; // Simulate reading insruction
    let _ = bcc(&mut registers, 0x42);
    assert_eq!(registers.pc, 0x1);

    registers.status = 0b0;
    registers.pc = 0;
    registers.pc += 1; // Simulate reading insruction
    let _ = bcc(&mut registers, 0x42);
    assert_eq!(registers.pc, 0x44);

    registers.status = 0b0;
    registers.pc = 0xC74D;
    registers.pc += 1; // Simulate reading insruction
    let _ = bcc(&mut registers, 0x4);
    assert_eq!(registers.pc, 0xC753);
}

/// Push Processor Status (PHP)
///
/// Pushes a copy of the status flags on to the stack.
pub fn php(registers: &mut Registers, memory: &mut Memory) {
    registers.set_flag(StatusFlag::B, true);
    registers.set_flag(StatusFlag::Unused, true);
    memory.stack_push(registers.status);
    registers.set_flag(StatusFlag::B, false);
    // registers.set_flag(StatusFlag::Unused, false);
}

#[test]
fn php_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();
    registers.status = 0b10101010;
    registers.pc += 1; // Simulate reading insruction
    php(&mut registers, &mut memory);
    assert_eq!(memory.memory[0x01FF], 0b10111010);
}

/// Bit Test (BIT)
///
/// This instructions is used to test if one or more bits are set in a target memory location. 
/// The mask pattern in A is ANDed with the value in memory to set or clear the zero flag, but the result is not kept. 
/// Bits 7 and 6 of the value from memory are copied into the N and V flags.
pub fn bit(registers: &mut Registers, memory: &mut Memory, addr: u16) {
    let m = memory.memory[addr as usize];
    let test = registers.a & m;
    if test == 0 {
        registers.set_flag(StatusFlag::Z, true);
    } else {
        registers.set_flag(StatusFlag::Z, false);
    }
    let v = m & 0b01000000 == 0b01000000;
    registers.set_flag(StatusFlag::V, v);
    let n = m & 0b10000000 == 0b10000000;
    registers.set_flag(StatusFlag::N, n);
}

#[test]
fn bit_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();
    registers.pc += 1; // Simulate reading insruction
    bit(&mut registers, &mut memory, 0x42);
    assert_eq!(registers.status, 0b00000010);

    memory.memory[0x42] = 0x1;
    registers.a = 0x1;
    registers.pc += 1; // Simulate reading insruction
    bit(&mut registers, &mut memory, 0x42);
    assert_eq!(registers.status, 0b00000000);

    memory.memory[0x42] = 0xF2;
    registers.a = 0xFF;
    registers.pc += 1; // Simulate reading insruction
    bit(&mut registers, &mut memory, 0x42);
    assert_eq!(registers.status, 0b11000000);
}

/// Branch if Overflow Set (BVS)
///
/// If the overflow flag is set then add the relative displacement to the program counter to cause a branch to a new location.
pub fn bvs(registers: &mut Registers, addr: u16) -> bool {
    if registers.is_flag_set(StatusFlag::V) {
        if addr >= 0x80 {
            let value = (addr as i32 - (1 << 8)) as i16;
            registers.pc = 1 + (registers.pc as i16 + value) as u16;
        } else {
            registers.pc = 1 + (registers.pc as i16 + addr as i16) as u16;
        }
        true
    } else {
        false
    }
}

#[test]
fn bvs_test() {
    let mut registers = Registers::new();
    registers.pc += 1; // Simulate reading insruction
    let _ = bvs(&mut registers, 0x42);
    assert_eq!(registers.pc, 0x1);

    registers.status = 0b01000000;
    registers.pc = 0;
    registers.pc += 1; // Simulate reading insruction
    let _ = bvs(&mut registers, 0x42);
    assert_eq!(registers.pc, 0x44);
}

/// Branch if Overflow Clear (BVC)
///
/// If the overflow flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
pub fn bvc(registers: &mut Registers, addr: u16) -> bool {
    if !registers.is_flag_set(StatusFlag::V) {
        if addr >= 0x80 {
            let value = (addr as i32 - (1 << 8)) as i16;
            registers.pc = 1 + (registers.pc as i16 + value) as u16;
        } else {
            registers.pc = 1 + (registers.pc as i16 + addr as i16) as u16;
        }
        true
    } else {
        false
    }
}

#[test]
fn bvc_test() {
    let mut registers = Registers::new();

    registers.status = 0b01000000;
    registers.pc += 1; // Simulate reading insruction
    let _ = bvc(&mut registers, 0x42);
    assert_eq!(registers.pc, 0x1);

    registers.status = 0b00000000;
    registers.pc = 0;
    registers.pc += 1; // Simulate reading insruction
    let _ = bvc(&mut registers, 0x42);
    assert_eq!(registers.pc, 0x44);
}

/// Load Y Register (LDY)
///
/// Loads a byte of memory into the Y register setting the zero and negative flags as appropriate.
pub fn ldy(registers: &mut Registers, operand: u8) {
    registers.y = operand;
    registers.set_flag(StatusFlag::Z, operand == 0);
    registers.set_flag(StatusFlag::N, operand >= 0x80);
}

#[test]
fn ldy_test() {
    let mut registers = Registers::new();
    registers.pc += 1; // Simulate reading insruction
    ldy(&mut registers, 0x42);
    assert_eq!(registers.y, 0x42);
    registers.pc += 1; // Simulate reading insruction
    ldy(&mut registers, 0x0);
    assert_eq!(registers.y, 0x0);
    assert_eq!(registers.status & 0b00000010, 0b00000010);
    registers.pc += 1; // Simulate reading insruction
    ldy(&mut registers, 0x80);
    assert_eq!(registers.y, 0x80);
    assert_eq!(registers.status & 0b10000000, 0b10000000);
}

/// Arithmetic Shift Left (ASL)
///
/// This operation shifts all the bits of the memory contents one bit left. 
/// Bit 0 is set to 0 and bit 7 is placed in the carry flag. 
/// The effect of this operation is to multiply the memory contents by 2 (ignoring 2's complement considerations), setting the carry if the result will not fit in 8 bits.
pub fn asl(registers: &mut Registers, memory: &mut Memory, addr: u16, val: u8) {
    let mut m = val;
    let c = (m & 0b10000000) as u8 == 0b10000000;

    m <<= 1;
    memory.memory[addr as usize] = m as u8;

    registers.set_flag(StatusFlag::Z, m == 0);
    registers.set_flag(StatusFlag::N, m >= 0x80);
    registers.set_flag(StatusFlag::C, c);
}

/// Arithmetic Shift Left (ASL) with accumulator
///
/// This operation shifts all the bits of the accumulator contents one bit left. 
/// Bit 0 is set to 0 and bit 7 is placed in the carry flag. 
/// The effect of this operation is to multiply the memory contents by 2 (ignoring 2's complement considerations), setting the carry if the result will not fit in 8 bits.
pub fn asl_acc(registers: &mut Registers) {
    let mut m = registers.a;
    let c = (m & 0b10000000) as u8 == 0b10000000;
    m <<= 1;
    registers.a = m as u8;

    registers.set_flag(StatusFlag::Z, m == 0);
    registers.set_flag(StatusFlag::N, m >= 0x80);
    registers.set_flag(StatusFlag::C, c);
}

#[test]
fn asl_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();
    registers.pc += 1; // Simulate reading insruction
    asl(&mut registers, &mut memory, 0x2, 0x2);
    assert_eq!(memory.memory[0x2], 0x4);
}

/// Return from Interrupt (RTI)
///
/// The RTI instruction is used at the end of an interrupt processing routine.
/// It pulls the processor flags from the stack followed by the program counter.
pub fn rti(registers: &mut Registers, memory: &mut Memory) {
    let status = memory.stack_pop();
    let pc_lsb = memory.stack_pop();
    let pc_msb = memory.stack_pop();
    let pc = utils::address_from_bytes(pc_lsb, pc_msb);

    let old_registers = registers.clone();

    registers.status = status;
    registers.set_flag(StatusFlag::B, old_registers.is_flag_set(StatusFlag::B));
    registers.set_flag(
        StatusFlag::Unused,
        old_registers.is_flag_set(StatusFlag::Unused),
    );
    registers.pc = pc;
}

#[test]
fn rti_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    memory.stack_push(0x0);
    memory.stack_push(0x2);
    memory.stack_push(0b10101010);
    registers.pc += 1; // Simulate reading insruction
    rti(&mut registers, &mut memory);

    assert_eq!(registers.status, 0b10001010);
    assert_eq!(registers.pc, 0x2);

    memory.stack_push(0xCE);
    memory.stack_push(0xCE);
    memory.stack_push(0x87);
    registers.pc += 1; // Simulate reading insruction
    rti(&mut registers, &mut memory);
    assert_eq!(registers.status, 0x87);
    assert_eq!(registers.pc, 0xCECE);
}

/// Subtract with Carry (SBC)
///
/// This instruction subtracts the contents of a memory location to the accumulator together with the not of the carry bit. 
/// If overflow occurs the carry bit is clear, this enables multiple byte subtraction to be performed.
pub fn sbc(registers: &mut Registers, value: u8) {
    adc(registers, !value);
}

#[test]
fn sbc_test() {
    let mut registers = Registers::new();

    registers.status = 0x65;
    registers.a = 0x40;
    registers.pc += 1; // Simulate instruction READ
    sbc(&mut registers, 0x40);
    assert_eq!(registers.a, 0x0);
    assert_eq!(registers.status, 0x27);

    registers.status = 0xE5;
    registers.a = 0x40;
    registers.pc += 1; // Simulate instruction READ
    sbc(&mut registers, 0x41);
    assert_eq!(registers.a, 0xFF);
    assert_eq!(registers.status, 0xA4);
}

/// Set Decimal Flag (SED)
///
/// Set the decimal mode flag to one.
pub fn sed(registers: &mut Registers) {
    registers.set_flag(StatusFlag::D, true);
}

#[test]
fn sed_test() {
    let mut registers = Registers::new();
    registers.pc += 1; // Simulate instruction READ
    sed(&mut registers);
    assert_eq!(registers.status, 0x8)
}

/// Compare (CMP)
///
/// This instruction compares the contents of the accumulator with another memory held value and sets the zero and carry flags as appropriate.

pub fn cmp(registers: &mut Registers, value: u8) {
    registers.set_flag(StatusFlag::N, false);
    registers.set_flag(StatusFlag::C, false);
    registers.set_flag(StatusFlag::Z, false);

    match registers.a.cmp(&(value as u8)) {
        std::cmp::Ordering::Less => {
            // registers.status &= 0b00000000;
        }
        std::cmp::Ordering::Equal => {
            registers.set_flag(StatusFlag::C, true);
            registers.set_flag(StatusFlag::Z, true);
        }
        std::cmp::Ordering::Greater => registers.set_flag(StatusFlag::C, true),
    }

    let res = if value >= 0x80 {
        let value = (value as i32 - (1 << 8)) as i16;
        (registers.a as i16 - value as i16) as u8
    } else {
        (registers.a as i16 - value as i16) as u8
    };
    registers.set_flag(StatusFlag::N, res >= 0x80);
}

#[test]
fn cmp_test() {
    let mut registers = Registers::new();

    registers.a = 0x10;
    registers.pc += 1; // Simulate reading insruction
    cmp(&mut registers, 0x10);
    assert_eq!(registers.status, 0b00000011);

    registers.a = 0x9;
    registers.pc += 1; // Simulate reading insruction
    cmp(&mut registers, 0x10);
    assert_eq!(registers.status, 0b10000000);

    registers.a = 0x10;
    registers.pc += 1; // Simulate reading insruction
    cmp(&mut registers, 0x9);
    assert_eq!(registers.status, 0b00000001);

    registers.a = 0xFF;
    registers.pc += 1; // Simulate reading insruction
    cmp(&mut registers, 0x10);

    registers.a = 0x7F;
    registers.pc += 1; // Simulate reading insruction
    cmp(&mut registers, 0x6F);
    assert_eq!(registers.status, 0b00000001);

    registers.a = 0x40;
    registers.pc += 1; // Simulate reading insruction
    registers.status = 0x25;
    cmp(&mut registers, 0x41);
    assert_eq!(registers.status, 0xA4);

    registers.a = 0xFF;
    registers.pc += 1; // Simulate reading insruction
    registers.status = 0xA4;
    cmp(&mut registers, 0xFF);
    assert_eq!(registers.status, 0x27);
}

/// Push Accumulator (PHA)
///
/// Pushes a copy of the accumulator on to the stack.
pub fn pha(registers: &mut Registers, memory: &mut Memory) {
    memory.stack_push(registers.a);
}

#[test]
fn pha_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    registers.a = 0x42;
    pha(&mut registers, &mut memory);
    assert_eq!(memory.stack_pop(), 0x42);
}

/// Pull Processor Status (PLP)
///
/// Pulls an 8 bit value from the stack and into the processor flags. 
/// The flags will take on new states as determined by the value pulled.
pub fn plp(registers: &mut Registers, memory: &mut Memory) {
    let old_registers = registers.clone();
    registers.status = memory.stack_pop();

    registers.set_flag(StatusFlag::B, old_registers.is_flag_set(StatusFlag::B));
    registers.set_flag(
        StatusFlag::Unused,
        old_registers.is_flag_set(StatusFlag::Unused),
    );
}

#[test]
fn plp_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    memory.stack_push(0xFF);
    plp(&mut registers, &mut memory);
    assert_eq!(registers.status, 0xCF);

    memory.stack_push(0xFF);
    registers.set_flag(StatusFlag::Unused, true);
    plp(&mut registers, &mut memory);
    assert_eq!(registers.status, 0xEF);
}

/// Branch if Minus (BMI)
///
/// If the negative flag is set then add the relative displacement to the program counter to cause a branch to a new location.
#[must_use]
pub fn bmi(registers: &mut Registers, addr: u16) -> bool {
    if registers.is_flag_set(StatusFlag::N) {
        if addr >= 0x80 {
            let value = (addr as i32 - (1 << 8)) as i16;
            registers.pc = 1 + (registers.pc as i16 + value) as u16;
        } else {
            registers.pc = 1 + (registers.pc as i16 + addr as i16) as u16;
        }
        true
    } else {
        false
    }
}

#[test]
fn bmi_test() {
    let mut registers = Registers::new();

    let _ = bmi(&mut registers, 0x42);
    assert_eq!(registers.pc, 0x0);

    registers.set_flag(StatusFlag::N, true);
    let _ = bmi(&mut registers, 0x42);
    assert_eq!(registers.pc, 0x43);
}

///  Logical Inclusive OR (OR)
///
/// An inclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
pub fn ora(registers: &mut Registers, value: u8) {
    registers.a |= value;
    registers.set_flag(StatusFlag::Z, registers.a == 0);
    registers.set_flag(StatusFlag::N, registers.a >= 0x80);
}

#[test]
fn ora_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    registers.a = 0b00000001;
    memory.memory[0x1] = 0b00000001;
    registers.pc += 1; // Simulate reading insruction
    ora(&mut registers, 0x1);
    assert_eq!(registers.a, 1);

    registers.a = 0b00000010;
    memory.memory[0x1] = 0b00000001;
    registers.pc += 1; // Simulate reading insruction
    ora(&mut registers, 0x1);
    assert_eq!(registers.a, 0b11);
}

/// Clear Overflow Flag (CLV)
/// 
/// Clears the overflow flag.
pub fn clv(registers: &mut Registers) {
    registers.set_flag(StatusFlag::V, false);
}

#[test]
fn clv_test() {
    let mut registers = Registers::new();
    registers.set_flag(StatusFlag::V, true);
    registers.pc += 1; // Simulate reading insruction
    clv(&mut registers);
    assert_eq!(registers.is_flag_set(StatusFlag::V), false);
}

/// Exclusive OR (EOR)
///
/// An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
pub fn eor(registers: &mut Registers, value: u8) {
    registers.a ^= value;

    registers.set_flag(StatusFlag::Z, registers.a == 0);
    registers.set_flag(StatusFlag::N, registers.a >= 0x80);
}

#[test]
fn eor_test() {
    let mut registers = Registers::new();

    registers.a = 0b1;
    eor(&mut registers, 0x1);
    assert_eq!(registers.a, 0b0);

    registers.a = 2;
    eor(&mut registers, 0x1);
    assert_eq!(registers.a, 0b11);

    registers.a = 0x5F;
    eor(&mut registers, 0xAA);
    assert_eq!(registers.a, 0xF5);
}

/// Add with Carry (ADC)
///
/// This instruction adds the contents of a memory location to the accumulator together with the carry bit. 
/// If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.
pub fn adc(registers: &mut Registers, value: u8) {
    // ~CARRY
    let carry = if registers.is_flag_set(StatusFlag::C) {
        1
    } else {
        0
    } as u8;

    // let a = if registers.a >= 0x80 {
    //     (registers.a as i32 - (1 << 8)) as i16
    // } else {
    //     registers.a as i16
    // };

    // let m = addr;
    // let m = if m >= 0x80 {
    //     (m as i32 - (1 << 8)) as i16
    // } else {
    //     m as i16
    // };

    let a = registers.a;
    let m = value;

    let temp = a as u16 + m as u16 + carry as u16;

    registers.a = temp as u8;

    registers.set_flag(StatusFlag::C, temp > 0xFF);
    registers.set_flag(
        StatusFlag::V,
        // NOTE: found here https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
        // NOTE: but unsure why this works and the previous and why I had issues with it...
        !(a ^ value) & (a ^ temp as u8) & 0x80 == 0x80,
    );
    registers.set_flag(StatusFlag::Z, registers.a == 0);
    registers.set_flag(StatusFlag::N, registers.a >= 0x80);
}

#[test]
fn adc_test() {
    let mut registers = Registers::new();

    registers.a = 0x2;
    adc(&mut registers, 0x40);
    assert_eq!(registers.a, 0x42);

    registers.a = 0x2;
    adc(&mut registers, 0xFF);
    assert_eq!(registers.a, 0x1);

    registers.a = 0x2;
    registers.set_flag(StatusFlag::C, true);
    adc(&mut registers, 0x40);
    assert_eq!(registers.a, 0x43);

    registers.a = 0x7F;
    registers.status = 0x25;
    adc(&mut registers, 0x7F);
    assert_eq!(registers.a, 0xFF);
    assert_eq!(registers.status, 0xE4);

    registers.a = 0x01;
    registers.status = 0x6D;
    adc(&mut registers, 0x69);
    assert_eq!(registers.a, 0x6B);
    assert_eq!(registers.status, 0x2C);
}

/// Store Y Register (STY)
///
/// Stores the contents of the Y register into memory.
pub fn sty(registers: &mut Registers, memory: &mut Memory, addr: u16) {
    memory.memory[addr as usize] = registers.y;
}

#[test]
fn sty_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    registers.y = 0x42;
    registers.pc += 1; // Simulate reading insruction
    sty(&mut registers, &mut memory, 0x30);
    assert_eq!(memory.memory[0x30], 0x42);
}

/// Increment Y Register (INY)
///
/// Adds one to the Y register setting the zero and negative flags as appropriate.
pub fn iny(registers: &mut Registers) {
    let operand = registers.y as u16;
    if operand == 0xFF {
        registers.y = 0;
    } else {
        registers.y += 1;
    }

    let operand = registers.y;

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
fn iny_test() {
    let mut registers = Registers::new();

    registers.y = 41;
    registers.pc += 1; // Simulate reading insruction
    iny(&mut registers);
    assert_eq!(registers.y, 42);
}

/// Increment X Register (INX)
///
/// Adds one to the X register setting the zero and negative flags as appropriate.
pub fn inx(registers: &mut Registers) {
    let operand = registers.x as u16;
    if operand == 0xFF {
        registers.x = 0;
    } else {
        registers.x += 1;
    }

    let operand = registers.x;

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
fn inx_test() {
    let mut registers = Registers::new();

    registers.x = 41;
    registers.pc += 1; // Simulate reading insruction
    inx(&mut registers);
    assert_eq!(registers.x, 42);
}

/// Transfer Accumulator to X (TAX)
///
/// Copies the current contents of the accumulator into the X register and sets the zero and negative flags as appropriate.
pub fn tax(registers: &mut Registers) {
    registers.x = registers.a;

    let operand = registers.x;

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
fn tax_test() {
    let mut registers = Registers::new();

    registers.a = 42;
    registers.pc += 1; // Simulate reading insruction
    tax(&mut registers);
    assert_eq!(registers.x, 42);
}

/// Transfer Y to Accumulator (TYA)
///
/// Copies the current contents of the Y register into the accumulator and sets the zero and negative flags as appropriate.
pub fn tya(registers: &mut Registers) {
    registers.a = registers.y;

    let operand = registers.a;

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
fn tya_test() {
    let mut registers = Registers::new();

    registers.y = 42;
    registers.pc += 1; // Simulate reading insruction
    tya(&mut registers);
    assert_eq!(registers.a, 42);
}

/// Transfer X to Accumulator
/// 
/// Copies the current contents of the X register into the accumulator and sets the zero and negative flags as appropriate.
pub fn txa(registers: &mut Registers) {
    registers.a = registers.x;

    let operand = registers.a;

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
fn txa_test() {
    let mut registers = Registers::new();

    registers.x = 42;
    registers.pc += 1; // Simulate reading insruction
    txa(&mut registers);
    assert_eq!(registers.a, 42);
}

/// Transfer Stack Pointer to X (TSX)
///
/// Copies the current contents of the stack register into the X register and sets the zero and negative flags as appropriate.
pub fn tsx(registers: &mut Registers, memory: &mut Memory) {
    registers.x = memory.stack_pointer as u8;

    registers.set_flag(StatusFlag::Z, registers.x == 0);
    registers.set_flag(StatusFlag::N, registers.x >= 0x80);
}

#[test]
fn tsx_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    registers.pc += 1; // Simulate reading insruction
    tsx(&mut registers, &mut memory);
    assert_eq!(registers.x, memory.stack_pointer as u8);
}

/// Decrement X Register (DEX)
///
/// Subtracts one from the X register setting the zero and negative flags as appropriate.
pub fn dex(registers: &mut Registers) {
    registers.x = (registers.x as i16 - 1) as u8;
    registers.set_flag(StatusFlag::Z, registers.x == 0);
    registers.set_flag(StatusFlag::N, registers.x >= 0x80);
}

#[test]
fn dex_test() {
    let mut registers = Registers::new();

    dex(&mut registers);
    assert_eq!(registers.x, 0xFF);
    assert_eq!(registers.status, 0b10000000);

    registers.x = 0x43;
    dex(&mut registers);
    assert_eq!(registers.x, 0x42);
    assert_eq!(registers.status, 0b00000000);
}

/// Logical Shift Right (LSR)
/// 
/// Each of the bits in M is shift one place to the right. 
/// The bit that was in bit 0 is shifted into the carry flag. Bit 7 is set to zero.
pub fn lsr(registers: &mut Registers, memory: &mut Memory, addr: u16) {
    let m = memory.memory[addr as usize];
    let carry = m as u8 & 0b1 == 0b1;
    let m = m >> 1;
    memory.memory[addr as usize] = m;
    registers.set_flag(StatusFlag::C, carry);
    registers.set_flag(StatusFlag::Z, m == 0);
    registers.set_flag(StatusFlag::N, m >= 0x80);
}

/// Logical Shift Right (LSR) with accumulator
/// 
/// Each of the bits in A is shift one place to the right. 
/// The bit that was in bit 0 is shifted into the carry flag. Bit 7 is set to zero.
pub fn lsr_acc(registers: &mut Registers) {
    let m = registers.a;
    let carry = m as u8 & 0b1 == 0b1;
    let m = m >> 1;
    registers.a = m;
    registers.set_flag(StatusFlag::C, carry);
    registers.set_flag(StatusFlag::Z, registers.a == 0);
    registers.set_flag(StatusFlag::N, registers.a >= 0x80);
}

#[test]
fn lsr_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    memory.memory[0x42] = 0x4;
    lsr(&mut registers, &mut memory, 0x42);
    assert_eq!(memory.memory[0x42], 0x2);

    registers.a = 0x4;
    lsr_acc(&mut registers);
    assert_eq!(registers.a, 0x2);
}

/// Rotate Right (ROR)
///
/// Move each of the bits in either M one place to the right. 
/// Bit 7 is filled with the current value of the carry flag whilst the old bit 0 becomes the new carry flag value.
pub fn ror(registers: &mut Registers, memory: &mut Memory, addr: u16) {
    let m = memory.memory[addr as usize];
    let bit0 = m as u8 & 0b1 == 0b1;
    let mut m = m >> 1;
    let carry = registers.is_flag_set(StatusFlag::C);
    m |= if carry { 1 << 7 } else { 0 };
    memory.memory[addr as usize] = m;
    registers.set_flag(StatusFlag::C, bit0);
    registers.set_flag(StatusFlag::Z, m == 0);
    registers.set_flag(StatusFlag::N, m >= 0x80);
}

/// Rotate Right (ROR) with accumulator
///
/// Move each of the bits in either A one place to the right. 
/// Bit 7 is filled with the current value of the carry flag whilst the old bit 0 becomes the new carry flag value.
pub fn ror_acc(registers: &mut Registers) {
    let m = registers.a;
    let bit0 = m as u8 & 0b1 == 0b1;
    let mut m = m >> 1;
    let carry = registers.is_flag_set(StatusFlag::C);
    m |= if carry { 1 << 7 } else { 0 };
    registers.a = m;
    registers.set_flag(StatusFlag::C, bit0);
    registers.set_flag(StatusFlag::Z, registers.a == 0);
    registers.set_flag(StatusFlag::N, registers.a >= 0x80);
}

#[test]
fn ror_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    memory.memory[0x42] = 0x4;
    ror(&mut registers, &mut memory, 0x42);
    assert_eq!(memory.memory[0x42], 0x2);

    registers.a = 0x4;
    ror_acc(&mut registers);
    assert_eq!(registers.a, 0x2);

    registers.a = 0x4;
    registers.set_flag(StatusFlag::C, true);
    ror_acc(&mut registers);
    assert_eq!(registers.a, 0x82);
}

/// Rotate Left (ROL)
///
/// Move each of the bits in either M one place to the left. 
/// Bit 0 is filled with the current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
pub fn rol(registers: &mut Registers, memory: &mut Memory, addr: u16, value: u8) {
    let m = value;
    let bit7 = m as u8 & 0b10000000 == 0b10000000;
    let mut m = m << 1;
    let carry = registers.is_flag_set(StatusFlag::C);
    m |= if carry { 1 } else { 0 };
    memory.memory[addr as usize] = m;
    registers.set_flag(StatusFlag::C, bit7);
    registers.set_flag(StatusFlag::Z, m == 0);
    registers.set_flag(StatusFlag::N, m >= 0x80);
}

/// Rotate Left (ROL) with accumulator
///
/// Move each of the bits in either A one place to the left. 
/// Bit 0 is filled with the current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
pub fn rol_acc(registers: &mut Registers) {
    let m = registers.a;
    let bit7 = m as u8 & 0b10000000 == 0b10000000;
    let mut m = m << 1;
    let carry = registers.is_flag_set(StatusFlag::C);
    m |= if carry { 1 } else { 0 };
    registers.a = m;
    registers.set_flag(StatusFlag::C, bit7);
    registers.set_flag(StatusFlag::Z, registers.a == 0);
    registers.set_flag(StatusFlag::N, registers.a >= 0x80);
}

#[test]
fn rol_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    memory.memory[0x42] = 0x4;
    rol(&mut registers, &mut memory, 0x42, 0x4);
    assert_eq!(memory.memory[0x42], 0x8);

    registers.a = 0x4;
    rol_acc(&mut registers);
    assert_eq!(registers.a, 0x8);

    registers.a = 0x4;
    registers.set_flag(StatusFlag::C, true);
    rol_acc(&mut registers);
    assert_eq!(registers.a, 0x9);
}

/// Decrement Memory (DEC)
///
/// Subtracts one from the value held at a specified memory location setting the zero and negative flags as appropriate.
pub fn dec(registers: &mut Registers, memory: &mut Memory, addr: u16) {
    memory.memory[addr as usize] = memory.memory[addr as usize].wrapping_sub(1);
    registers.set_flag(StatusFlag::Z, memory.memory[addr as usize] == 0);
    registers.set_flag(StatusFlag::N, memory.memory[addr as usize] >= 0x80);
}

#[test]
fn dec_test() {
    let mut registers = Registers::new();
    let mut memory = Memory::new();

    memory.memory[0x42] = 0x4;
    dec(&mut registers, &mut memory, 0x42);
    assert_eq!(memory.memory[0x42], 0x3);
}
