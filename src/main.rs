mod rp2a03 {
    /*!  Emulate a mos6502 Ricoh 2A03 microntroller */
    /**
    A struct to represent MOS6502 registers

    a is 8-bit accumulator register
    x is 8-bit X index register
    y is 8-bit Y index register
     is a 8-bit stack pointer, hardwired to memory page $01
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

    Zero page	$0000 - $00FF
    Stack	$0100 - $01FF
    General-purpose	$0200 - $FFFF

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
            ppu.resize_with(0x3F21, || 0);

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

    pub mod instructions {
        use crate::{address_from_bytes, BREAK_VECTOR_ADDDRESS, STACK_ADDRESS};

        use super::*;

        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
            Unknown,
        }

        #[must_use]
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
                // CPX
                0xEC => (Instructions::CPX, AddressingMode::Absolute),
                0xE0 => (Instructions::CPX, AddressingMode::Immediate),
                0xE4 => (Instructions::CPX, AddressingMode::ZeroPage),
                // DEY
                0x88 => (Instructions::DEY, AddressingMode::Implied),
                // BPL
                0x10 => (Instructions::BPL, AddressingMode::Relative),
                // PLA
                0x68 => (Instructions::PLA, AddressingMode::Implied),
                // TAY
                0xA8 => (Instructions::TAY, AddressingMode::Implied),
                // CPY
                0xCC => (Instructions::CPY, AddressingMode::Absolute),
                0xC0 => (Instructions::CPY, AddressingMode::Immediate),
                0xC4 => (Instructions::CPY, AddressingMode::ZeroPage),
                // BNE
                0xD0 => (Instructions::BNE, AddressingMode::Relative),
                // RTS
                0x60 => (Instructions::RTS, AddressingMode::Implied),
                // JMP
                0x4C => (Instructions::JMP, AddressingMode::Absolute),
                0x6C => (Instructions::JMP, AddressingMode::AbsoluteIndirect),
                // STX
                0x8E => (Instructions::STX, AddressingMode::Absolute),
                0x86 => (Instructions::STX, AddressingMode::ZeroPage),
                0x96 => (Instructions::STX, AddressingMode::ZeroPageIndexedWithY),
                // JSR
                0x20 => (Instructions::JSR, AddressingMode::Absolute),
                // NOP
                0xEA => (Instructions::NOP, AddressingMode::Implied),
                // SEC
                0x38 => (Instructions::SEC, AddressingMode::Implied),
                // BCS
                0xB0 => (Instructions::BCS, AddressingMode::Relative),
                // CLC
                0x18 => (Instructions::CLC, AddressingMode::Implied),
                // BCC
                0x90 => (Instructions::BCC, AddressingMode::Relative),
                // PHP
                0x08 => (Instructions::PHP, AddressingMode::Implied),
                // BIT
                0x2C => (Instructions::BIT, AddressingMode::Absolute),
                0x89 => (Instructions::BIT, AddressingMode::Immediate),
                0x24 => (Instructions::BIT, AddressingMode::ZeroPage),
                // BVS
                0x70 => (Instructions::BVS, AddressingMode::Relative),
                //BVC
                0x50 => (Instructions::BVC, AddressingMode::Relative),
                // LDY
                0xAC => (Instructions::LDY, AddressingMode::Absolute),
                0xBC => (Instructions::LDY, AddressingMode::AbsoluteIndirectWithX),
                0xA0 => (Instructions::LDY, AddressingMode::Immediate),
                0xA4 => (Instructions::LDY, AddressingMode::ZeroPage),
                0xB4 => (Instructions::LDY, AddressingMode::ZeroPageIndexedWithX),
                // ASL
                0x0E => (Instructions::ASL, AddressingMode::Absolute),
                0x1E => (Instructions::ASL, AddressingMode::AbsoluteIndirectWithX),
                0x0A => (Instructions::ASL, AddressingMode::Accumulator),
                0x06 => (Instructions::ASL, AddressingMode::ZeroPage),
                0x16 => (Instructions::ASL, AddressingMode::ZeroPageIndexedWithX),
                // RTI
                0x40 => (Instructions::RTI, AddressingMode::Implied),
                // SBC
                0xED => (Instructions::SBC, AddressingMode::Absolute),
                0xFD => (Instructions::SBC, AddressingMode::AbsoluteIndirectWithX),
                0xF9 => (Instructions::SBC, AddressingMode::AbsoluteIndirectWithY),
                0xE9 => (Instructions::SBC, AddressingMode::Immediate),
                0xE5 => (Instructions::SBC, AddressingMode::ZeroPage),
                0xE1 => (Instructions::SBC, AddressingMode::ZeroPageIndexedIndirect),
                0xF5 => (Instructions::SBC, AddressingMode::ZeroPageIndexedWithX),
                0xF1 => (
                    Instructions::SBC,
                    AddressingMode::ZeroPageIndirectIndexedWithY,
                ),
                // SED
                0xF8 => (Instructions::SED, AddressingMode::Implied),
                // CMP
                0xCD => (Instructions::CMP, AddressingMode::Absolute),
                0xDD => (Instructions::CMP, AddressingMode::AbsoluteIndirectWithX),
                0xD9 => (Instructions::CMP, AddressingMode::AbsoluteIndirectWithY),
                0xC9 => (Instructions::CMP, AddressingMode::Immediate),
                0xC5 => (Instructions::CMP, AddressingMode::ZeroPage),
                0xC1 => (Instructions::CMP, AddressingMode::ZeroPageIndexedIndirect),
                0xD5 => (Instructions::CMP, AddressingMode::ZeroPageIndexedWithX),
                0xD1 => (
                    Instructions::CMP,
                    AddressingMode::ZeroPageIndirectIndexedWithY,
                ),
                // PHA
                0x48 => (Instructions::PHA, AddressingMode::Implied),
                // PLP
                0x28 => (Instructions::PLP, AddressingMode::Implied),
                // BMI
                0x30 => (Instructions::BMI, AddressingMode::Relative),
                // ORA
                0x0D => (Instructions::ORA, AddressingMode::Absolute),
                0x1D => (Instructions::ORA, AddressingMode::AbsoluteIndirectWithX),
                0x19 => (Instructions::ORA, AddressingMode::AbsoluteIndirectWithY),
                0x09 => (Instructions::ORA, AddressingMode::Immediate),
                0x05 => (Instructions::ORA, AddressingMode::ZeroPage),
                0x01 => (Instructions::ORA, AddressingMode::ZeroPageIndexedIndirect),
                0x15 => (Instructions::ORA, AddressingMode::ZeroPageIndexedWithX),
                0x11 => (
                    Instructions::ORA,
                    AddressingMode::ZeroPageIndirectIndexedWithY,
                ),
                // CLV
                0xB8 => (Instructions::CLV, AddressingMode::Implied),
                // EOR
                0x4D => (Instructions::EOR, AddressingMode::Absolute),
                0x5D => (Instructions::EOR, AddressingMode::AbsoluteIndirectWithX),
                0x59 => (Instructions::EOR, AddressingMode::AbsoluteIndirectWithY),
                0x49 => (Instructions::EOR, AddressingMode::Immediate),
                0x45 => (Instructions::EOR, AddressingMode::ZeroPage),
                0x41 => (Instructions::EOR, AddressingMode::ZeroPageIndexedIndirect),
                0x55 => (Instructions::EOR, AddressingMode::ZeroPageIndexedWithX),
                0x51 => (
                    Instructions::EOR,
                    AddressingMode::ZeroPageIndirectIndexedWithY,
                ),
                // ADC
                0x6D => (Instructions::ADC, AddressingMode::Absolute),
                0x7D => (Instructions::ADC, AddressingMode::AbsoluteIndirectWithX),
                0x79 => (Instructions::ADC, AddressingMode::AbsoluteIndirectWithY),
                0x69 => (Instructions::ADC, AddressingMode::Immediate),
                0x65 => (Instructions::ADC, AddressingMode::ZeroPage),
                0x61 => (Instructions::ADC, AddressingMode::ZeroPageIndexedIndirect),
                0x75 => (Instructions::ADC, AddressingMode::ZeroPageIndexedWithX),
                0x71 => (
                    Instructions::ADC,
                    AddressingMode::ZeroPageIndirectIndexedWithY,
                ),
                // STY
                0x8C => (Instructions::STY, AddressingMode::Absolute),
                0x84 => (Instructions::STY, AddressingMode::ZeroPage),
                0x94 => (Instructions::STY, AddressingMode::ZeroPageIndexedIndirect),
                // INY
                0xC8 => (Instructions::INY, AddressingMode::Implied),
                // INX
                0xE8 => (Instructions::INX, AddressingMode::Implied),
                // TAX
                0xAA => (Instructions::TAX, AddressingMode::Implied),
                // TYA
                0x98 => (Instructions::TYA, AddressingMode::Implied),
                // TXA
                0x8A => (Instructions::TXA, AddressingMode::Implied),
                // TSX
                0xBA => (Instructions::TSX, AddressingMode::Implied),
                // DEX
                0xCA => (Instructions::DEX, AddressingMode::Implied),
                // LSR
                0x4A => (Instructions::LSR, AddressingMode::Accumulator),
                0x46 => (Instructions::LSR, AddressingMode::ZeroPage),
                0x56 => (Instructions::LSR, AddressingMode::ZeroPageIndexedWithX),
                0x4E => (Instructions::LSR, AddressingMode::Absolute),
                0x5E => (Instructions::LSR, AddressingMode::AbsoluteIndirectWithX),
                _ => (Instructions::Unknown, AddressingMode::Implied),
            }
        }

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

        pub fn lda(registers: &mut Registers, operand: u16) {
            registers.a = operand as u8;
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
        }

        pub fn brk(registers: &mut Registers, memory: &mut Memory) {
            registers.pc += 2;
            memory.stack_push(((registers.pc >> 8) & 0xFF) as u8);
            memory.stack_push((registers.pc & 0xFF) as u8);
            registers.status |= 0b00010100; // Enable B and I registers
            memory.stack_push(registers.status);
            registers.pc = address_from_bytes(
                memory.memory[BREAK_VECTOR_ADDDRESS as usize],
                memory.memory[(BREAK_VECTOR_ADDDRESS + 1) as usize],
            );
        }

        #[test]
        fn brk_test() {
            let mut registers = Registers::new();
            let mut memory = Memory::new();
            memory.memory[BREAK_VECTOR_ADDDRESS as usize] = 0x42;
            memory.memory[(BREAK_VECTOR_ADDDRESS + 1) as usize] = 0x0;
            registers.pc += 1; // Simulate reading insruction
            brk(&mut registers, &mut memory);
            assert_eq!(registers.status, 0b00010100);
            assert_eq!(memory.memory[0x01FE], 3);
            assert_eq!(memory.memory[0x01FF], 0);
            assert_eq!(registers.pc, 0x42);
        }

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

        pub fn ldx(registers: &mut Registers, addr: u16) {
            registers.x = addr as u8;
            registers.set_flag(StatusFlag::Z, registers.x == 0);
            registers.set_flag(StatusFlag::N, registers.x >= 0x80);
        }

        #[test]
        fn ldx_test() {
            let mut registers = Registers::new();
            let mut memory = Memory::new();

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

        pub fn and(registers: &mut Registers, value: u8) {
            registers.a &= value;

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

        pub fn cpx(registers: &mut Registers, value: u16) {
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

        pub fn cpy(registers: &mut Registers, value: u16) {
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

        pub fn rts(registers: &mut Registers, memory: &mut Memory) {
            let low = memory.stack_pop();
            let high = memory.stack_pop();
            let addr = address_from_bytes(low, high);
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

        pub fn nop() {}

        #[test]
        fn nop_test() {
            // HOW CAN I TEST THIS :D
        }

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

        pub fn php(registers: &mut Registers, memory: &mut Memory) {
            registers.set_flag(StatusFlag::B, true);
            memory.stack_push(registers.status);
            registers.set_flag(StatusFlag::B, false);
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

        pub fn ldy(registers: &mut Registers, operand: u8) {
            registers.y = operand;
            if operand == 0 {
                registers.set_flag(StatusFlag::Z, true);
            } else {
                registers.set_flag(StatusFlag::Z, false);
            };
            if operand >= 0x80 {
                registers.set_flag(StatusFlag::N, true);
            } else {
                registers.set_flag(StatusFlag::N, false);
            };
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

        pub fn asl(registers: &mut Registers, memory: &mut Memory, addr: u16, val: u16) {
            let mut m = val;
            let c = (m & 0b10000000) as u8;

            m <<= 1;
            memory.memory[addr as usize] = m as u8;
            registers.status |= c;

            if m == 0 {
                registers.set_flag(StatusFlag::Z, true);
            } else {
                registers.set_flag(StatusFlag::Z, false);
            };
            if m >= 0x80 {
                registers.set_flag(StatusFlag::N, true);
            } else {
                registers.set_flag(StatusFlag::N, false);
            };
        }

        #[test]
        fn asl_test() {
            let mut registers = Registers::new();
            let mut memory = Memory::new();
            registers.pc += 1; // Simulate reading insruction
            asl(&mut registers, &mut memory, 0x2, 0x2);
            assert_eq!(memory.memory[0x2], 0x4);
        }

        pub fn rti(registers: &mut Registers, memory: &mut Memory) {
            let status = memory.stack_pop();
            let pc_lsb = memory.stack_pop();
            let pc_msb = memory.stack_pop();
            let pc = address_from_bytes(pc_lsb, pc_msb);

            registers.status = status;
            registers.set_flag(StatusFlag::Unused, true);
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
            assert_eq!(registers.status, 0b10101010);
            assert_eq!(registers.pc, 0x2);

            memory.stack_push(0xCE);
            memory.stack_push(0xCE);
            memory.stack_push(0x87);
            registers.pc += 1; // Simulate reading insruction
            rti(&mut registers, &mut memory);
            assert_eq!(registers.status, 0xA7);
            assert_eq!(registers.pc, 0xCECE);
        }

        pub fn sbc(registers: &mut Registers, addr: u16) {
            let carry = if registers.is_flag_set(StatusFlag::C) {
                0
            } else {
                1
            } as u8;

            let a = registers.a;
            let m = addr as u8;

            let temp = (a as i16 - m as i16 - carry as i16) as u16;

            registers.a = temp as u8;

            registers.set_flag(StatusFlag::C, (1 << 8) & temp != (1 << 8));
            registers.set_flag(
                StatusFlag::V,
                a >= 0x80 && m <= 0x80 && temp < 0x80 || a < 0x80 && m > 0x80 && temp >= 0x80,
            );
            registers.set_flag(StatusFlag::Z, registers.a == 0);
            registers.set_flag(StatusFlag::N, registers.a >= 0x80);
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

        pub fn cmp(registers: &mut Registers, value: u16) {
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

        pub fn plp(registers: &mut Registers, memory: &mut Memory) {
            let old_registers = registers.clone();
            registers.status = memory.stack_pop();

            registers.set_flag(StatusFlag::B, false);
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

        pub fn eor(registers: &mut Registers, addr: u16) {
            registers.a ^= addr as u8;

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

        pub fn adc(registers: &mut Registers, addr: u16) {
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
            let m = addr as u8;

            let temp = a as u16 + m as u16 + carry as u16;

            registers.a = temp as u8;

            registers.set_flag(StatusFlag::C, (1 << 8) & temp == (1 << 8));
            registers.set_flag(
                StatusFlag::V,
                a >= 0x80 && m >= 0x80 && temp < 0x80 || a < 0x80 && m < 0x80 && temp >= 0x80,
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

        pub fn dex(registers: &mut Registers, addr: u16) {
            registers.x = (registers.x as i16 - 1) as u8;
            registers.set_flag(StatusFlag::Z, registers.x == 0);
            registers.set_flag(StatusFlag::N, registers.x >= 0x80);
        }

        #[test]
        fn dex_test() {
            let mut registers = Registers::new();

            dex(&mut registers, 0x1);
            assert_eq!(registers.x, 0xFF);
            assert_eq!(registers.status, 0b10000000);

            registers.x = 0x43;
            dex(&mut registers, 0x1);
            assert_eq!(registers.x, 0x42);
            assert_eq!(registers.status, 0b00000000);
        }

        pub fn lsr(registers: &mut Registers, memory: &mut Memory, addr: u16) {
            let m = memory.memory[addr as usize];
            let carry = m as u8 & 0b1 == 0b1;
            let m = m >> 1;
            memory.memory[addr as usize] = m;
            registers.set_flag(StatusFlag::C, carry);
            registers.set_flag(StatusFlag::Z, m == 0);
            registers.set_flag(StatusFlag::N, m >= 0x80);
        }

        pub fn lsr_acc(registers: &mut Registers) {
            let m = registers.a;
            let carry = m as u8 & 0b1== 0b1;
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
                    Some(addr as u16)
                }
                AddressingMode::ZeroPage => {
                    let addr = low_byte;
                    Some(*memory.get(addr as usize).unwrap() as u16)
                }
                AddressingMode::Relative => Some(low_byte as u16),
                AddressingMode::AbsoluteIndirect => {
                    let addr = address_from_bytes(low_byte, high_byte);
                    let addr2 = addr + 1;
                    Some(address_from_bytes(
                        memory[addr as usize],
                        memory[addr2 as usize],
                    ))
                }
                AddressingMode::AbsoluteIndirectWithX => {
                    let addr = address_from_bytes(low_byte, high_byte) + registers.x as u16;
                    Some(*memory.get(addr as usize).unwrap() as u16)
                }
                AddressingMode::AbsoluteIndirectWithY => {
                    let addr = address_from_bytes(low_byte, high_byte) + registers.y as u16;
                    Some(*memory.get(addr as usize).unwrap() as u16)
                }
                AddressingMode::ZeroPageIndexedWithX => {
                    let addr = low_byte + registers.x;
                    Some(*memory.get(addr as usize).unwrap() as u16)
                }
                AddressingMode::ZeroPageIndexedWithY => {
                    let addr = low_byte + registers.y;
                    Some(*memory.get(addr as usize).unwrap() as u16)
                }
                AddressingMode::ZeroPageIndexedIndirect => {
                    let addr = low_byte + registers.x;
                    let low = *memory.get(addr as usize).unwrap();
                    let high = *memory.get((addr + 1) as usize).unwrap();
                    let addr = address_from_bytes(low, high);
                    Some(addr as u16)
                }
                AddressingMode::ZeroPageIndirectIndexedWithY => {
                    let addr = address_from_bytes(low_byte, high_byte);
                    let low_byte = *memory.get(addr as usize).unwrap();
                    let high_byte = *memory.get((addr + 1) as usize).unwrap();
                    let addr = address_from_bytes(low_byte, high_byte) + registers.y as u16;
                    Some(*memory.get(addr as usize).unwrap() as u16)
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
        #[test]
        fn apply_addressing_test() {
            let mut memory = Memory::new();

            let mut registers = Registers::new();

            // Accumulator
            let res = apply_addressing(&memory, &registers, AddressingMode::Accumulator, 0x0, 0x0);
            assert_eq!(res, None);

            // IMPLIED
            let res = apply_addressing(&memory, &registers, AddressingMode::Implied, 0x0, 0x0);
            assert_eq!(res, None);

            // IMMEDIATE
            let res = apply_addressing(&memory, &registers, AddressingMode::Immediate, 0x22, 0x0);
            assert_eq!(res, Some(0x22));

            let res = apply_addressing(&memory, &registers, AddressingMode::Immediate, 0x81, 0x42);
            assert_eq!(res, Some(0x81));

            // ABSOLUTE
            memory.memory[0x201] = 42;
            let res = apply_addressing(&memory, &registers, AddressingMode::Absolute, 0x10, 0xD0);
            assert_eq!(res, Some(0xD010));

            // ZERO PAGE
            memory.memory[0x4] = 43;
            let res = apply_addressing(&memory, &registers, AddressingMode::ZeroPage, 0x4, 0x0);
            assert_eq!(res, Some(43));

            // Relative
            let res = apply_addressing(&memory, &registers, AddressingMode::Relative, 0x42, 0x0);
            assert_eq!(res, Some(0x42));

            // AbsoluteIndirect
            memory.memory[0xA001] = 0xFF;
            memory.memory[0xA002] = 0x00;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::AbsoluteIndirect,
                0x01,
                0xA0,
            );
            assert_eq!(res, Some(0x00FF));

            // AbsoluteIndirectWithX
            memory.memory[0xC003] = 0x5A;
            registers.x = 0x2;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::AbsoluteIndirectWithX,
                0x1,
                0xC0,
            );
            assert_eq!(res, Some(0x5A));

            // AbsoluteIndirectWithY
            memory.memory[0xF004] = 0xEF;
            registers.y = 0x3;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::AbsoluteIndirectWithY,
                0x1,
                0xF0,
            );
            assert_eq!(res, Some(0xEF));

            // ZeroPageIndexedWithX
            memory.memory[0x3] = 0xA5;
            registers.x = 0x2;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::ZeroPageIndexedWithX,
                0x1,
                0x0,
            );
            assert_eq!(res, Some(0xA5));

            // ZeroPageIndexedWithY
            memory.memory[0x4] = 0xE3;
            registers.y = 3;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::ZeroPageIndexedWithY,
                0x1,
                0x0,
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
                0x15,
                0x0,
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
                0x2A,
                0x0,
            );
            assert_eq!(res, Some(0x2F));
        }
    }
}

use std::{
    fmt::write,
    fs::File,
    io::{BufRead, BufReader, Write},
};

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

mod nes_rom {
    use super::*;

    pub mod mappers {

        use super::*;

        pub(crate) fn load_rom(memory: &mut Memory, rom: &[u8], mapper: Mapper) {
            match mapper {
                Mapper::Nrom => {
                    memory.memory[PRG_ROM_START..PRG_ROM_START + 16384]
                        .copy_from_slice(&rom[16..16 + 16384]);
                    memory.memory[0xC000..=0xFFFF].copy_from_slice(&rom[16..16 + 16384]);
                }
            }
        }
        pub enum Mapper {
            Nrom,
        }
    }

    struct Ines2 {}

    #[derive(Debug)]
    pub enum SupportedFormat {
        ines2,
        ines,
        unsupported,
    }

    pub struct NESFile {
        pub mapper: mappers::Mapper,
        pub num_prgrom: u8,
        pub num_chrrom: u8,
        pub data: Vec<u8>,
    }

    pub const HEADER_START: usize = 0;
    pub const TRAINER_START: usize = 17;
    pub const PRG_ROM_START: usize = 529;

    impl NESFile {
        pub fn new(rom: &[u8]) -> Self {
            let NES = &rom[0..4];
            let num_prgrom = rom[4];
            let num_chrrom = rom[5];
            let flags6 = rom[6];

            let mirroring = flags6 & 0x1 == 0x1;
            let persistent_memory = flags6 & 0x2 == 0x2;
            let has_trainer = flags6 & 0x4 == 0x4;
            let four_screen_vram = flags6 & 0x8 & 0x8;
            let mapper_lsb = flags6 & 0xF0 >> 4;

            let flags7 = rom[7];

            let vs = flags7 & 0x1 == 0x1;
            let playchoice = flags7 & 0x2 == 0x2;
            let mapper_msb = (flags7 & 0xF0) >> 4;

            let prgram_size = if rom[8] == 0 { 1 } else { rom[8] };

            let flags9 = rom[9];
            let tv_system = flags9 & 0x1 == 0x1;

            let flags10 = rom[10];
            let tv_system2 = flags10 & 0x3;
            let has_prg_ram = flags10 & 0b10000 == 0b10000;
            let has_bus_conflict = flags10 & 0b100000 == 0b100000;

            let padding = &rom[11..16];
            // TODO: there are checks to do in padding in some cases
            // TODO: See http://wiki.nesdev.com/w/index.php/INES before variant comparison

            ////

            let mapper = (mapper_msb << 4) | mapper_lsb;

            println!("Mapper number {}", mapper);
            println!(
                "Num PRG ROM {} ({}KB)",
                num_prgrom,
                num_prgrom as u32 * 16384
            );
            println!(
                "Num CHR ROM {} ({}KB)",
                num_chrrom,
                num_chrrom as u32 * 8192
            );
            println!("Has trainer {}", has_trainer);
            println!("Has PRG RAM {}", has_prg_ram);

            let format = NESFile::get_file_format(rom);

            Self {
                mapper: mappers::Mapper::Nrom,
                num_prgrom,
                num_chrrom,
                data: rom.to_vec(),
            }
        }

        fn get_file_format(header: &[u8]) -> SupportedFormat {
            let ines_format = header[0] as char == 'N'
                && header[1] as char == 'E'
                && header[2] as char == 'S'
                && header[3] == 0x1A; // MS-DOS end of file

            let nes2 = ines_format && (header[7] & 0x0C) == 0x08;
            // TODO: check proper size of ROM image "size taking into account byte 9 does not exceed the actual size of the ROM image, then NES 2.0."

            if nes2 {
                SupportedFormat::ines2
            } else if ines_format {
                SupportedFormat::ines
            } else {
                SupportedFormat::unsupported
            }
        }
    }
}

fn print_nestest_log(
    instruction: Instructions,
    registers: &Registers,
    memory: &Memory,
    num_operands: u16,
    addr: u16,
    ops: (u8, u8),
) {
    // P : Processor status
    // A : Accumulator
    // X : register X
    //
    print!(
        "{:04X}  {:02X} ",
        registers.pc, memory.memory[registers.pc as usize]
    );
    if num_operands > 0 {
        print!("{:02X} ", ops.0);
    } else {
        print!("   ");
    }
    if num_operands >= 2 {
        print!("{:02X} ", ops.1);
    } else {
        print!("   ");
    }
    print!("{:4?} ", instruction);

    match num_operands {
        1 => match instruction {
            Instructions::BCC
            | Instructions::BCS
            | Instructions::BNE
            | Instructions::BPL
            | Instructions::BEQ => {
                print!("{:04X} ", registers.pc + addr + 2)
            }
            _ => print!("{:04X} ", addr),
        },
        2 => match instruction {
            Instructions::BCC
            | Instructions::BCS
            | Instructions::BNE
            | Instructions::BPL
            | Instructions::BEQ => {
                print!("{:04X} ", registers.pc + addr)
            }
            _ => print!("{:04X} ", addr),
        },
        _ => print!("     "),
    }

    print!(
        "A:{:2X} X:{:2X} Y:{:2X} P:{:2X} SP:{:2X} PPU: 0, 00 CYC:0",
        registers.a, registers.x, registers.y, registers.status, memory.stack_pointer
    );

    println!();
}

fn main() {
    println!("Nessy 🐉!");

    // let args: Vec<String> = std::env::args().collect();
    // println!("{:#?}", args);

    // Initialise memory
    let mut memory = Memory::new();

    // Load ROM and decode header
    let rom = include_bytes!("../nestest.nes");

    let nesfile = nes_rom::NESFile::new(rom);

    println!(
        "{}{}{} {}",
        rom[0] as char, rom[1] as char, rom[2] as char, rom[3]
    );

    // println!(
    //     "Num of 16k bytes PRG ROM {} ({}k bytes)\nNum of 8k CHR ROM {}",
    //     num_prgrom,
    //     16 * num_prgrom,
    //     num_chrrom
    // );

    let mut nestest_output = File::create("nestest.log").unwrap();

    // Load up memory
    nes_rom::mappers::load_rom(&mut memory, rom, nesfile.mapper);

    // let bank_seven = 16 + 7 * 16384;

    // for index in 0..16384 {
    //     memory.memory[(0xC000 + index) as usize] = rom[(bank_seven + index) as usize];
    // }

    // Get the RESET vector to find start of the game
    let reset_vector_low = memory.memory[RESET_VECTOR_ADDRESS as usize];
    let reset_vector_high = memory.memory[(RESET_VECTOR_ADDRESS + 1) as usize];

    let reset_vector = address_from_bytes(reset_vector_low, reset_vector_high);

    println!("Reset vector {:x}", reset_vector);

    // Set up registers
    let mut registers = Registers::new();
    registers.pc = 0xC000; // Becasue nestest starts here
                           // registers.pc = reset_vector;

    // POWER ON NES
    registers.status = 0x24;

    memory.stack_pointer = 0xFD; // Stack is on page 1 only so 0xFF is actually 0x01FF

    memory.memory[0x4017] = 0x00; // Frame IRQ enabled
    memory.memory[0x4015] = 0x00; // All channels disabled
    memory.memory[0x4000..0x400F].copy_from_slice(&[0x0; 15]);
    memory.memory[0x4010..0x4013].copy_from_slice(&[0x0; 3]);

    let reference = File::open("nestest_reference.log").unwrap();
    let reference_buffered = BufReader::new(reference);
    let mut reference_lines = reference_buffered.lines();

    loop {
        let byte = memory.memory[registers.pc as usize];
        let (instruction, addressing_mode) = match_instruction(byte);

        let num_operands = num_operands_from_addressing(&addressing_mode) as u16;
        let ops = get_operands(&registers, &memory);

        let (low_byte, high_byte) = ops;
        let bytes = address_from_bytes(low_byte, high_byte);
        let addr = apply_addressing(
            &memory,
            &registers,
            addressing_mode.clone(),
            low_byte,
            high_byte,
        )
        .unwrap_or(0);

        // print_nestest_log(instruction, &registers, &memory, num_operands, addr, ops);

        // PC
        write!(nestest_output, "{:04X}", registers.pc).unwrap();

        write!(nestest_output, "  ").unwrap();

        write!(
            nestest_output,
            "{:02X}",
            memory.memory[registers.pc as usize]
        )
        .unwrap();

        write!(nestest_output, " ").unwrap();

        if num_operands > 0 {
            write!(nestest_output, "{:02X} ", ops.0).unwrap();
        } else {
            write!(nestest_output, "   ").unwrap();
        }
        if num_operands >= 2 {
            write!(nestest_output, "{:02X} ", ops.1).unwrap();
        } else {
            write!(nestest_output, "   ").unwrap();
        }

        write!(nestest_output, " ").unwrap();
        write!(nestest_output, "{:3?} ", instruction).unwrap();

        match num_operands {
            1 => match instruction {
                Instructions::BCC
                | Instructions::BCS
                | Instructions::BNE
                | Instructions::BPL
                | Instructions::BVS
                | Instructions::BVC
                | Instructions::BMI
                | Instructions::BEQ => {
                    write!(nestest_output, "${:02X}   ", registers.pc + addr + 2).unwrap()
                }

                _ => match addressing_mode {
                    AddressingMode::Immediate => {
                        write!(nestest_output, "#${:02X}    ", addr).unwrap()
                    }
                    // AddressingMode::Relative => write!(nestest_output, "${:04X}    ", registers.pc + 1 + addr),
                    AddressingMode::Absolute => write!(
                        nestest_output,
                        "${:04X} = {:02}",
                        addr, memory.memory[addr as usize]
                    )
                    .unwrap(),
                    AddressingMode::ZeroPage => write!(
                        nestest_output,
                        "${:02X} = {:02X}",
                        ops.0, memory.memory[addr as usize]
                    )
                    .unwrap(),
                    _ => write!(nestest_output, "    ").unwrap(),
                },
            },
            2 => match instruction {
                Instructions::BCC
                | Instructions::BCS
                | Instructions::BNE
                | Instructions::BPL
                | Instructions::BVS
                | Instructions::BVC
                | Instructions::BMI
                | Instructions::BEQ => {
                    write!(nestest_output, "${:04X}    ", registers.pc + addr).unwrap()
                }
                // Have absolute adressing but don't show the = {} part
                Instructions::JSR | Instructions::JMP => {
                    write!(nestest_output, "${:04X}   ", addr).unwrap()
                }
                _ => match addressing_mode {
                    AddressingMode::Immediate => {
                        write!(nestest_output, "#${:04X}  ", addr).unwrap()
                    }
                    AddressingMode::Absolute => write!(
                        nestest_output,
                        "${:04X} = {:02X}",
                        addr, memory.memory[addr as usize]
                    )
                    .unwrap(),

                    AddressingMode::ZeroPage => write!(
                        nestest_output,
                        "${:02X} = {:02X}",
                        ops.0, memory.memory[addr as usize]
                    )
                    .unwrap(),
                    _ => write!(nestest_output, "        ").unwrap(),
                },
            },
            _ => match addressing_mode {
                AddressingMode::Accumulator => write!(nestest_output, "A       ").unwrap(),
                _ => write!(nestest_output, "        ").unwrap(),
            },
        };

        write!(nestest_output, "                    ").unwrap();

        write!(
            nestest_output,
            "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:  0, 00 CYC:0",
            registers.a, registers.x, registers.y, registers.status, memory.stack_pointer
        )
        .unwrap();

        writeln!(nestest_output).unwrap();

        let output = File::open("nestest.log").unwrap();
        let output_buffered = BufReader::new(output);

        let ref_line = reference_lines.next().unwrap();

        let output_line = output_buffered.lines().last().unwrap();
        let ref_columns_1 = ref_line.unwrap();
        let ref_columns = ref_columns_1
            .split(' ')
            // .filter(|&thing| !thing.is_empty())
            .collect::<Vec<&str>>();
        let output_columns = output_line.unwrap();
        let output_columns = output_columns
            .split(' ')
            // .filter(|&thing| !thing.is_empty())
            .collect::<Vec<&str>>();

        let mut matched = true;

        for ((index, &ref_col), output_col) in
            ref_columns.iter().enumerate().zip(output_columns.clone())
        {
            // if index == 0 {
            //     println!("\u{001b}[37m{:?}", output_columns);
            //     println!("\u{001b}[37m{:?}", ref_columns);
            // }
            if ref_col == output_col {
                print!("\u{001b}[32m")
            } else {
                // print!("({} {})", ref_col, output_col);
                print!("\u{001b}[31m");
                matched = false;
            }

            print!("{} ", output_col);
        }
        println!();
        if !matched {
            println!("\u{001b}[37m{} ", ref_columns_1);
            // break;
        }

        registers.pc += 1; // READ instruction

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
                let addr = if addressing_mode == AddressingMode::Absolute {
                    memory.memory[addr as usize] as u16
                } else {
                    addr as u16
                };
                lda(&mut registers, addr);
                registers.pc += num_operands;
            }
            Instructions::BRK => {
                brk(&mut registers, &mut memory);
                // NOTE: Shouldn't change pc as set by brk instruction
                // TODO: Check if need to advance pc by 1, but probs not
            }
            Instructions::STA => {
                sta(&mut registers, &mut memory, addr);
                registers.pc += num_operands;
            }
            Instructions::INC => {
                inc(&mut registers, &mut memory, addr);
                registers.pc += num_operands;
            }
            Instructions::LDX => {
                let addr = if addressing_mode == AddressingMode::Absolute {
                    memory.memory[addr as usize] as u16
                } else {
                    addr as u16
                };
                ldx(&mut registers, addr);
                registers.pc += num_operands;
            }
            Instructions::TXS => {
                txs(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            Instructions::AND => {
                and(&mut registers, addr as u8);
                registers.pc += num_operands;
            }
            Instructions::BEQ => {
                if !beq(&mut registers, addr) {
                    registers.pc += num_operands;
                }
            }
            Instructions::CPX => {
                cpx(&mut registers, addr);
                registers.pc += num_operands;
            }
            Instructions::DEY => {
                dey(&mut registers);
                registers.pc += num_operands;
            }
            Instructions::BPL => {
                if !bpl(&mut registers, addr) {
                    registers.pc += num_operands;
                }
            }
            Instructions::PLA => {
                pla(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            Instructions::TAY => {
                tay(&mut registers);
                registers.pc += num_operands;
            }
            Instructions::CPY => {
                cpy(&mut registers, addr);
                registers.pc += num_operands;
            }
            Instructions::BNE => {
                if !bne(&mut registers, addr) {
                    registers.pc += num_operands;
                }
            }
            Instructions::RTS => {
                rts(&mut registers, &mut memory);
            }
            Instructions::JMP => {
                jmp(&mut registers, addr);
            }
            Instructions::STX => {
                stx(&mut registers, &mut memory, addr);
                registers.pc += num_operands;
            }
            Instructions::JSR => {
                jsr(&mut registers, &mut memory, addr);
            }
            Instructions::NOP => {
                nop();
                registers.pc += num_operands;
            }
            Instructions::SEC => {
                sec(&mut registers);
                registers.pc += num_operands;
            }
            Instructions::BCS => {
                if !bcs(&mut registers, addr) {
                    registers.pc += num_operands;
                }
            }
            Instructions::CLC => {
                clc(&mut registers);
                registers.pc += num_operands;
            }
            Instructions::BCC => {
                if !bcc(&mut registers, addr) {
                    registers.pc += num_operands;
                }
            }
            Instructions::PHP => {
                php(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            Instructions::BIT => {
                bit(&mut registers, &mut memory, addr);
                registers.pc += num_operands;
            }
            Instructions::BVS => {
                if !bvs(&mut registers, addr) {
                    registers.pc += num_operands;
                }
            }
            Instructions::BVC => {
                if !bvc(&mut registers, addr) {
                    registers.pc += num_operands;
                }
            }
            Instructions::LDY => {
                ldy(&mut registers, addr as u8);
                registers.pc += num_operands;
            }
            Instructions::ASL => {
                asl(&mut registers, &mut memory, bytes, addr);
                registers.pc += num_operands;
            }
            Instructions::RTI => {
                rti(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            Instructions::SBC => {
                sbc(&mut registers, addr);
                registers.pc += num_operands;
            }
            Instructions::SED => {
                sed(&mut registers);
                registers.pc += num_operands;
            }
            Instructions::CMP => {
                cmp(&mut registers, addr);
                registers.pc += num_operands;
            }
            Instructions::PHA => {
                pha(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            Instructions::PLP => {
                plp(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            Instructions::BMI => {
                if !bmi(&mut registers, addr) {
                    registers.pc += num_operands;
                }
            }
            Instructions::ORA => {
                ora(&mut registers, addr as u8);
                registers.pc += num_operands;
            }
            Instructions::CLV => {
                clv(&mut registers);
                registers.pc += num_operands;
            }
            Instructions::EOR => {
                eor(&mut registers, addr);
                registers.pc += num_operands;
            }
            Instructions::ADC => {
                adc(&mut registers, addr);
                registers.pc += num_operands;
            }
            Instructions::STY => {
                sty(&mut registers, &mut memory, addr);
                registers.pc += num_operands;
            }
            Instructions::INY => {
                iny(&mut registers);
                registers.pc += num_operands;
            }
            Instructions::INX => {
                inx(&mut registers);
                registers.pc += num_operands;
            }
            Instructions::TAX => {
                tax(&mut registers);
                registers.pc += num_operands;
            }
            Instructions::TYA => {
                tya(&mut registers);
                registers.pc += num_operands;
            }
            Instructions::TXA => {
                txa(&mut registers);
                registers.pc += num_operands;
            }
            Instructions::TSX => {
                tsx(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            Instructions::DEX => {
                dex(&mut registers, addr);
                registers.pc += num_operands;
            }
            Instructions::LSR => {
                if addressing_mode == AddressingMode::Accumulator {
                    lsr_acc(&mut registers);
                } else {
                    lsr(&mut registers, &mut memory, addr);
                }

                registers.pc += num_operands;
            }

            Instructions::Unknown => {
                eprintln!(
                    "Unknown opcode {:#x}",
                    memory.memory[(registers.pc - 1) as usize]
                );
                break;
            }
        }
    }

    // let reference = File::open("nestest_reference.log").unwrap();
    // let reference_buffered = BufReader::new(reference);

    // let output = File::open("nestest.log").unwrap();
    // let output_buffered = BufReader::new(output);

    // for (count, (ref_line, output_line)) in reference_buffered
    //     .lines()
    //     .zip(output_buffered.lines())
    //     .enumerate()
    // {
    //     let ref_columns_1 = ref_line.unwrap();
    //     let ref_columns = ref_columns_1
    //         .split(' ')
    //         // .filter(|&thing| !thing.is_empty())
    //         .collect::<Vec<&str>>();
    //     let output_columns = output_line.unwrap();
    //     let output_columns = output_columns
    //         .split(' ')
    //         // .filter(|&thing| !thing.is_empty())
    //         .collect::<Vec<&str>>();

    //     let mut matched = true;

    //     for ((index, &ref_col), output_col) in
    //         ref_columns.iter().enumerate().zip(output_columns.clone())
    //     {
    //         // if index == 0 {
    //         //     println!("\u{001b}[37m{:?}", output_columns);
    //         //     println!("\u{001b}[37m{:?}", ref_columns);
    //         // }
    //         if ref_col == output_col {
    //             print!("\u{001b}[32m")
    //         } else {
    //             // print!("({} {})", ref_col, output_col);
    //             print!("\u{001b}[31m");
    //             matched = false;
    //         }

    //         print!("{} ", output_col);
    //     }
    //     println!();
    //     if !matched {
    //         println!("\u{001b}[37m{} ", ref_columns_1);
    //         // break;
    //     }

    //     // if count == 35 {
    //     //     break;
    //     // }
    // }
}
