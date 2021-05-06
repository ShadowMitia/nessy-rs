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
            self.memory[self.stack_pointer as usize] = val;
            self.stack_pointer -= 1;
        }

        #[must_use]
        pub fn stack_pop(&mut self) -> u8 {
            self.stack_pointer += 1;
            self.memory[self.stack_pointer as usize]
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

    #[derive(Debug, Clone)]
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

        use crate::{address_from_bytes, BREAK_VECTOR_ADDDRESS};

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
                0xC0 => (Instructions::CPX, AddressingMode::Immediate),
                0xC4 => (Instructions::CPX, AddressingMode::ZeroPage),
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

            brk(&mut registers, &mut memory);
            assert_eq!(registers.status, 0b00010100);
            assert_eq!(memory.memory[0x01FE], 2);
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

        #[must_use]
        pub fn beq(registers: &mut Registers, value: u16) -> bool {
            // Check if zero flag is enabled
            if (registers.status & 0b00000010) == 0b00000010 {
                if value >= 0x80 {
                    let value = (value as i32 - (1 << 8)) as i16;
                    registers.pc = 2 + (registers.pc as i16 + value) as u16;
                } else {
                    registers.pc = 2 + (registers.pc as i16 + value as i16) as u16;
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
            let _ = beq(&mut registers, 0x10);
            assert_eq!(registers.pc, 0x0);

            registers.pc = 0x0;
            registers.status = 0b00000010;
            let _ = beq(&mut registers, 0x10);
            assert_eq!(registers.pc, 0x12);

            registers.pc = 0x43;
            registers.status = 0b00000010;
            let _ = beq(&mut registers, 0xFD);
            assert_eq!(registers.pc, 0x42);
        }

        pub fn cpx(registers: &mut Registers, memory: &mut Memory, value: u16) {
            registers.status &= 0b01111100;

            match registers.x.cmp(&memory.memory[value as usize]) {
                std::cmp::Ordering::Less => {
                    // registers.status &= 0b00000000;
                }
                std::cmp::Ordering::Equal => {
                    registers.status |= 0b00000011;
                }
                std::cmp::Ordering::Greater => registers.status |= 0b00000001,
            }

            if (registers.x as i32 - memory.memory[value as usize] as i32) < 0 {
                registers.status |= 0b10000000;
            }
        }

        #[test]
        fn cpx_test() {
            let mut registers = Registers::new();
            let mut memory = Memory::new();

            registers.x = 0x10;
            memory.memory[0x42] = 0x10;
            cpx(&mut registers, &mut memory, 0x42);
            assert_eq!(registers.status, 0b00000011);

            registers.x = 0x9;
            memory.memory[0x42] = 0x10;
            cpx(&mut registers, &mut memory, 0x42);
            assert_eq!(registers.status, 0b10000000);

            registers.x = 0x10;
            memory.memory[0x42] = 0x9;
            cpx(&mut registers, &mut memory, 0x42);
            assert_eq!(registers.status, 0b00000001);

            registers.x = 0xFF;
            memory.memory[0x42] = 0x10;
            cpx(&mut registers, &mut memory, 0x42);
            assert_eq!(registers.status, 0b00000001);
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
            dey(&mut registers);
            assert_eq!(registers.y, 0x42);

            registers.y = 0x0;
            dey(&mut registers);
            assert_eq!(registers.y, 0xFF);
        }

        #[must_use]
        pub fn bpl(registers: &mut Registers, value: u16) -> bool {
            // Check if zero flag is enabled
            if (registers.status & 0b10000000) == 0b00000000 {
                if value >= 0x80 {
                    let value = (value as i32 - (1 << 8)) as i16;
                    registers.pc = 2 + (registers.pc as i16 + value) as u16;
                } else {
                    registers.pc = 2 + (registers.pc as i16 + value as i16) as u16;
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
            dey(&mut registers);
            assert_eq!(registers.y, 0x42);

            registers.y = 0x0;
            dey(&mut registers);
            assert_eq!(registers.y, 0xFF);
        }

        pub fn apl(registers: &mut Registers, memory: &mut Memory) {
            registers.a = memory.stack_pop();
        }

        #[test]
        fn apl_test() {
            let mut registers = Registers::new();
            let mut memory = Memory::new();

            memory.stack_push(0x42);
            apl(&mut registers, &mut memory);
            assert_eq!(registers.a, 0x42);
        }

        pub fn tay(registers: &mut Registers) {
            registers.a = registers.y;
            registers.status = if registers.a == 0 {
                registers.status | 0b00000010
            } else {
                registers.status & 0b11111101
            };
            registers.status = if registers.a >= 0x80 {
                registers.status | 0b10000000
            } else {
                registers.status & 0b01111111
            };
        }

        #[test]
        fn tay_test() {
            let mut registers = Registers::new();
            registers.y = 0x42;

            tay(&mut registers);
            assert_eq!(registers.a, 0x42);
        }

        pub fn cpy(registers: &mut Registers, memory: &mut Memory, value: u16) {
            registers.status &= 0b01111100;

            match registers.y.cmp(&memory.memory[value as usize]) {
                std::cmp::Ordering::Less => {
                    // registers.status &= 0b00000000;
                }
                std::cmp::Ordering::Equal => {
                    registers.status |= 0b00000011;
                }
                std::cmp::Ordering::Greater => registers.status |= 0b00000001,
            }

            if (registers.y as i32 - memory.memory[value as usize] as i32) < 0 {
                registers.status |= 0b10000000;
            }
        }

        #[test]
        fn cpy_test() {
            let mut registers = Registers::new();
            let mut memory = Memory::new();

            registers.y = 0x10;
            memory.memory[0x42] = 0x10;
            cpy(&mut registers, &mut memory, 0x42);
            assert_eq!(registers.status, 0b00000011);

            registers.y = 0x9;
            memory.memory[0x42] = 0x10;
            cpy(&mut registers, &mut memory, 0x42);
            assert_eq!(registers.status, 0b10000000);

            registers.y = 0xFF;
            memory.memory[0x42] = 0x10;
            cpy(&mut registers, &mut memory, 0x42);
            assert_eq!(registers.status, 0b00000001);
        }

        #[must_use]
        pub fn bne(registers: &mut Registers, value: u16) -> bool {
            // Check if zero flag is enabled
            if (registers.status & 0b00000010) == 0b00000010 {
                if value >= 0x80 {
                    let value = (value as i32 - (1 << 8)) as i16;
                    registers.pc = 2 + (registers.pc as i16 + value) as u16;
                } else {
                    registers.pc = 2 + (registers.pc as i16 + value as i16) as u16;
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
            let _ = bne(&mut registers, 0x10);
            assert_eq!(registers.pc, 0x12);

            registers.status = 0b00000000;
            let _ = bne(&mut registers, 0x10);
            assert_eq!(registers.pc, 0x12);
        }

        pub fn rts(registers: &mut Registers, memory: &mut Memory) {
            let low = memory.stack_pop();
            let high = memory.stack_pop();
            let addr = address_from_bytes(low, high);
            println!("{val:b} {val}", val = addr);
            registers.pc = addr;
            registers.pc += 1;
        }

        #[test]
        fn rts_test() {
            let mut registers = Registers::new();
            let mut memory = Memory::new();

            memory.stack_push(0x0);
            memory.stack_push(0x4);

            rts(&mut registers, &mut memory);
            assert_eq!(registers.pc, 0x5);
        }

        pub fn jmp(registers: &mut Registers, addr: u16) {
            registers.pc = addr;
        }

        #[test]
        fn jmp_test() {
            let mut registers = Registers::new();

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
            stx(&mut registers, &mut memory, 0x30);
            assert_eq!(memory.memory[0x30], 0x42);
        }

        pub fn jsr(registers: &mut Registers, memory: &mut Memory, addr: u16) {
            registers.pc += 2;
            memory.stack_push(((registers.pc >> 8) & 0xFF) as u8);
            memory.stack_push((registers.pc & 0xFF) as u8);
            registers.pc = addr;
        }

        #[test]
        fn jsr_test() {
            let mut registers = Registers::new();
            let mut memory = Memory::new();
            registers.pc = 0x42;
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

            sec(&mut registers);
            assert_eq!(registers.status & 0x1, 0x1);
        }

        #[must_use]
        pub fn bcs(registers: &mut Registers, addr: u16) -> bool {
            if registers.status & 0x1 == 0b1 {
                if addr >= 0x80 {
                    let value = (addr as i32 - (1 << 8)) as i16;
                    println!("value {:X} ", value);
                    registers.pc = 1 + (registers.pc as i16 + value) as u16;
                } else {
                    registers.pc = 1 + (registers.pc as i16 + addr as i16) as u16;
                }
                println!("new pc {:X} ", registers.pc);
                true
            } else {
                false
            }
        }

        #[test]
        fn bcs_test() {
            let mut registers = Registers::new();

            registers.pc += 0xC72F;

            let _ = bcs(&mut registers, 0x20);
            assert_eq!(registers.pc, 0xC72F);

            registers.status |= 0x1;

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
            let _ = clc(&mut registers);
            assert_eq!(registers.status, 0x0);
        }

        #[must_use]
        pub fn bcc(registers: &mut Registers, addr: u16) -> bool {
            if registers.status & 0x1 == 0x0 {
                if addr >= 0x80 {
                    let value = (addr as i32 - (1 << 8)) as i16;
                    registers.pc = 2 + (registers.pc as i16 + value) as u16;
                } else {
                    registers.pc = 2 + (registers.pc as i16 + addr as i16) as u16;
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
            let _ = bcc(&mut registers, 0x42);
            assert_eq!(registers.pc, 0x0);

            registers.status = 0b0;
            let _ = bcc(&mut registers, 0x42);
            assert_eq!(registers.pc, 0x44);
        }

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
                    Some(addr)
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

        // TODO: add some missing cases (negative cases, ...)
        #[test]
        fn apply_addressing_test() {
            let mut memory = Memory::new();

            let mut registers = Registers::new();

            // Accumulator
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

            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::Immediate,
                Some(0x81),
                None,
            );
            assert_eq!(res, Some(0x81));

            // ABSOLUTE
            memory.memory[0x201] = 42;
            let res = apply_addressing(
                &memory,
                &registers,
                AddressingMode::Absolute,
                Some(0x1),
                Some(0x2),
            );
            assert_eq!(res, Some(0x0201));

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

    /*
        print!(
        "{:4X} {:2X} ",
        registers.pc, memory.memory[registers.pc as usize]
    );
    if num_operands > 0 {
        print!("{:2X} ", ops.0);
    } else {
        print!("   ");
    }
    if num_operands >= 2 {
        print!("{:2X} ", ops.1);
    } else {
        print!("   ");
    }
    print!("{:4?} ", instruction);

    match num_operands {
        1 => print!("{:4X} ", ops.0),
        2 => print!(
            "{:4X} ",
            apply_addressing(
                &memory,
                &registers,
                addressing_mode.clone(),
                Some(ops.0),
                Some(ops.1),
            )
            .unwrap_or(0)
        ),
        _ => print!("     "),
    }

    print!(
        "A:{:2X} X:{:2X} Y:{:2X} P:{:2X} SP:{:4X} PPU: SOMETHING, SOMETHING SOMETHING",
        registers.a, registers.x, registers.y, registers.status, memory.stack_pointer
    );

    println!();
    */
    registers.pc = 0xC000; // Becasue nestest starts here

    loop {
        let byte = memory.memory[registers.pc as usize];
        let (instruction, addressing_mode) = match_instruction(byte);

        let num_operands = num_operands_from_addressing(&addressing_mode) as u16;
        let ops = get_operands(&registers, &memory);

        let (low_byte, high_byte) = ops;
        let addr = apply_addressing(
            &memory,
            &registers,
            addressing_mode,
            Some(low_byte),
            Some(high_byte),
        )
        .unwrap_or(0);

        print!(
            "{:4X} {:2X} ",
            registers.pc, memory.memory[registers.pc as usize]
        );
        if num_operands > 0 {
            print!("{:2X} ", ops.0);
        } else {
            print!("   ");
        }
        if num_operands >= 2 {
            print!("{:2X} ", ops.1);
        } else {
            print!("   ");
        }
        print!("{:4?} ", instruction);

        match num_operands {
            1 => print!("{:4X} ", ops.0),
            2 => print!("{:4X} ", addr),
            _ => print!("     "),
        }

        print!(
            "A:{:2X} X:{:2X} Y:{:2X} P:{:2X} SP:{:4X} PPU: SOMETHING, SOMETHING SOMETHING",
            registers.a, registers.x, registers.y, registers.status, memory.stack_pointer
        );

        println!();

        registers.pc += 1;

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
                lda(&mut registers, addr as u8);
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
                ldx(&mut registers, addr as u8);
                registers.pc += num_operands;
            }
            Instructions::TXS => {
                txs(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            Instructions::AND => {
                and(&mut registers, memory.memory[addr as usize]);
                registers.pc += num_operands;
            }
            Instructions::BEQ => {
                if !beq(&mut registers, addr) {
                    registers.pc += num_operands;
                }
            }
            Instructions::CPX => {
                cpx(&mut registers, &mut memory, addr);
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
                apl(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            Instructions::TAY => {
                tay(&mut registers);
                registers.pc += num_operands;
            }
            Instructions::CPY => {
                cpy(&mut registers, &mut memory, addr);
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
        }
    }
}
