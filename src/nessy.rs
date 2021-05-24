use crate::{cpu::{self, AddressingMode, Memory, StatusFlag, instructions::{match_instruction, Instruction, InstructionName, *}, utils::{RESET_VECTOR_ADDRESS, address_from_bytes, apply_addressing, get_cycles, get_operands, is_page_crossed, num_operands_from_addressing}}, nes_rom::{self, RomFile}, ppu::{self, Ctrl, Mask, Status}};

pub struct Nessy {
    pub memory: Memory,
    pub registers: cpu::Registers,
    pub ppu_registers: ppu::Registers,
    pub ppu_memory: ppu::Memory,
    pub reset_vector: u16,
    pub cycle: usize,
    pub ppu_cycle: usize,
    pub frames: usize,
}

impl Nessy {
    #[must_use]
    pub fn new() -> Self {
        // Initialise memory
        let mut memory = Memory::new();
        // Set up registers
        let mut registers = cpu::Registers::new();

        // PPU
        let ppu_registers = ppu::Registers::new();
        let ppu_memory = ppu::Memory::new();

        // Get the RESET vector to find start of the game
        let reset_vector_low = memory.memory[RESET_VECTOR_ADDRESS as usize];
        let reset_vector_high = memory.memory[(RESET_VECTOR_ADDRESS + 1) as usize];

        let reset_vector = address_from_bytes(reset_vector_low, reset_vector_high);

        memory.stack_pointer = 0xFF; // Stack is on page 1 only so 0xFF is actually 0x01FF

        // Memory
        memory.memory[0x4017] = 0x00; // Frame IRQ enabled
        memory.memory[0x4015] = 0x00; // All channels disabled
        memory.memory[0x4000..0x400F].copy_from_slice(&[0x0; 15]);
        memory.memory[0x4010..0x4013].copy_from_slice(&[0x0; 3]);

        // Disable decimal mode
        registers.set_flag(StatusFlag::D, false);

        let cycle = 7;
        let ppu_cycle = 21;
        let frames = 0;

        Self {
            memory,
            registers,
            ppu_registers,
            ppu_memory,
            reset_vector,

            cycle,
            ppu_cycle,
            frames,
        }
    }

    pub fn load(&mut self, nesfile: &RomFile) {
        nes_rom::mappers::load_rom(&mut self.memory, &mut self.ppu_memory, &nesfile);

        self.registers.pc = self.reset_vector;
        self.registers.status = 0x34;
    }

    pub fn load_nestest(&mut self, nesfile: &RomFile) {
        nes_rom::mappers::load_rom(&mut self.memory, &mut self.ppu_memory, &nesfile);

        self.registers.pc = 0xC000;
        self.registers.status = 0x24;
    }

    #[must_use]
    pub fn get_opcode(&self) -> u8 {
        self.memory.memory[self.registers.pc as usize]
    }

    pub fn execute(&mut self) {
        let opcode = self.get_opcode();
        let instruction = match_instruction(opcode);

        let (instruction, addressing_mode, _) = match instruction {
            Instruction::Official(instr, addr) => (instr, addr, true),
            Instruction::Unofficial(instr, addr) => (instr, addr, false),
            Instruction::Unknown => {
                panic!(
                    "Unknown opcode {:#x}",
                    self.memory.memory[(self.registers.pc - 1) as usize]
                );
            }
        };

        let num_operands = num_operands_from_addressing(&addressing_mode) as u16;
        let ops = get_operands(&self.registers, &self.memory);

        let (low_byte, high_byte) = ops;
        let addr = apply_addressing(
            &self.memory,
            &self.registers,
            addressing_mode.clone(),
            low_byte,
            high_byte,
        )
        .unwrap_or(0);

        // RAM MIRORRING AND
        let mirror_addr = if addr < 0x2000 {
            // System memory is mirrored
            addr % 0x0800
        } else if (0x2000..0x4000).contains(&addr) {
            // PPU I/O rgisters are mirrored
            if addr > 0x007 {
                addr % 0x2008 + 0x2000
            } else {
                addr
            }
        } else {
            addr
        };

        let j_addr = addr;
        let addr = mirror_addr;

        let page_crossed = match (instruction, addressing_mode.clone()) {
            (InstructionName::INC, AddressingMode::AbsoluteIndirectWithX)
            | (InstructionName::INC, AddressingMode::AbsoluteIndirectWithY)
            | (InstructionName::ADC, AddressingMode::AbsoluteIndirectWithX)
            | (InstructionName::ADC, AddressingMode::AbsoluteIndirectWithY)
            | (InstructionName::LDA, AddressingMode::AbsoluteIndirectWithX)
            | (InstructionName::LDA, AddressingMode::AbsoluteIndirectWithY)
            | (InstructionName::LDY, AddressingMode::AbsoluteIndirectWithX)
            | (InstructionName::LDY, AddressingMode::AbsoluteIndirectWithY)
            | (InstructionName::LDX, AddressingMode::AbsoluteIndirectWithX)
            | (InstructionName::LDX, AddressingMode::AbsoluteIndirectWithY)
            | (InstructionName::NOP, AddressingMode::AbsoluteIndirectWithX)
            | (InstructionName::NOP, AddressingMode::AbsoluteIndirectWithY) => {
                is_page_crossed(address_from_bytes(low_byte, high_byte), addr)
            }
            (InstructionName::ADC, AddressingMode::ZeroPageIndirectIndexedWithY)
            | (InstructionName::LDA, AddressingMode::ZeroPageIndirectIndexedWithY)
            | (InstructionName::LDY, AddressingMode::ZeroPageIndirectIndexedWithY)
            | (InstructionName::LDX, AddressingMode::ZeroPageIndirectIndexedWithY)
            | (InstructionName::INC, AddressingMode::ZeroPageIndirectIndexedWithY)
            | (InstructionName::LAX, AddressingMode::ZeroPageIndirectIndexedWithY) => {
                let low = self.memory.memory[address_from_bytes(low_byte, 0x0) as usize];
                let high =
                    self.memory.memory[address_from_bytes(low_byte.wrapping_add(1), 0x0) as usize];

                is_page_crossed(address_from_bytes(low, high) as u16, addr)
            }
            (_, AddressingMode::Relative) => is_page_crossed(
                self.registers.pc + 2, /* Include pc++ and operand read */
                (self.registers.pc as i16
                    + 2
                    + if addr >= 0x80 {
                        (addr as i32 - (1 << 8)) as i16
                    } else {
                        addr as i16
                    }) as u16,
            ),
            (_, AddressingMode::Absolute) => is_page_crossed(
                self.registers.pc + 2, /* Include pc++ and operand read */
                addr,
            ),
            _ => false,
        };

        self.registers.pc += 1; // READ instruction

        let mut branched = false;

        match instruction {
            InstructionName::SEI => {
                sei(&mut self.registers);
                self.registers.pc += num_operands;
            }
            InstructionName::CLD => {
                cld(&mut self.registers);
                self.registers.pc += num_operands;
            }
            InstructionName::LDA => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    self.memory.memory[addr as usize]
                };

                lda(&mut self.registers, data);
                self.registers.pc += num_operands;
            }
            InstructionName::BRK => {
                brk(&mut self.registers, &mut self.memory);
            }
            InstructionName::STA => {
                sta(&mut self.registers, &mut self.memory, addr);
                self.registers.pc += num_operands;
            }
            InstructionName::INC => {
                inc(&mut self.registers, &mut self.memory, addr);
                self.registers.pc += num_operands;
            }
            InstructionName::LDX => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    self.memory.memory[addr as usize]
                };
                ldx(&mut self.registers, data.into());
                self.registers.pc += num_operands;
            }
            InstructionName::TXS => {
                txs(&mut self.registers, &mut self.memory);
                self.registers.pc += num_operands;
            }
            InstructionName::AND => {
                if addressing_mode == AddressingMode::Accumulator {
                    and_acc(&mut self.registers);
                } else {
                    let data = if addressing_mode == AddressingMode::Immediate {
                        addr as u8
                    } else {
                        self.memory.memory[addr as usize]
                    };
                    and(&mut self.registers, data);
                }

                self.registers.pc += num_operands;
            }
            InstructionName::BEQ => {
                if !beq(&mut self.registers, addr) {
                    self.registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::CPX => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    self.memory.memory[addr as usize]
                };
                cpx(&mut self.registers, data);
                self.registers.pc += num_operands;
            }
            InstructionName::DEY => {
                dey(&mut self.registers);
                self.registers.pc += num_operands;
            }
            InstructionName::BPL => {
                if !bpl(&mut self.registers, addr) {
                    self.registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::PLA => {
                pla(&mut self.registers, &mut self.memory);
                self.registers.pc += num_operands;
            }
            InstructionName::TAY => {
                tay(&mut self.registers);
                self.registers.pc += num_operands;
            }
            InstructionName::CPY => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    self.memory.memory[addr as usize]
                };
                cpy(&mut self.registers, data);
                self.registers.pc += num_operands;
            }
            InstructionName::BNE => {
                if !bne(&mut self.registers, addr) {
                    self.registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::RTS => {
                rts(&mut self.registers, &mut self.memory);
            }
            InstructionName::JMP => {
                jmp(&mut self.registers, j_addr);
            }
            InstructionName::STX => {
                stx(&mut self.registers, &mut self.memory, addr);
                self.registers.pc += num_operands;
            }
            InstructionName::JSR => {
                jsr(&mut self.registers, &mut self.memory, j_addr);
            }
            InstructionName::NOP => {
                nop();
                self.registers.pc += num_operands;
            }
            InstructionName::SEC => {
                sec(&mut self.registers);
                self.registers.pc += num_operands;
            }
            InstructionName::BCS => {
                if !bcs(&mut self.registers, addr) {
                    self.registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::CLC => {
                clc(&mut self.registers);
                self.registers.pc += num_operands;
            }
            InstructionName::BCC => {
                if !bcc(&mut self.registers, addr) {
                    self.registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::PHP => {
                php(&mut self.registers, &mut self.memory);
                self.registers.pc += num_operands;
            }
            InstructionName::BIT => {
                bit(&mut self.registers, &mut self.memory, addr);
                self.registers.pc += num_operands;
            }
            InstructionName::BVS => {
                if !bvs(&mut self.registers, addr) {
                    self.registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::BVC => {
                if !bvc(&mut self.registers, addr) {
                    self.registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::LDY => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    self.memory.memory[addr as usize]
                };
                ldy(&mut self.registers, data);
                self.registers.pc += num_operands;
            }
            InstructionName::ASL => {
                if addressing_mode == AddressingMode::Accumulator {
                    asl_acc(&mut self.registers);
                } else {
                    let data = self.memory.memory[addr as usize];
                    asl(&mut self.registers, &mut self.memory, addr, data);
                }

                self.registers.pc += num_operands;
            }
            InstructionName::RTI => {
                rti(&mut self.registers, &mut self.memory);
                self.registers.pc += num_operands;
            }
            InstructionName::SBC => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    self.memory.memory[addr as usize]
                };
                sbc(&mut self.registers, data);
                self.registers.pc += num_operands;
            }
            InstructionName::SED => {
                sed(&mut self.registers);
                self.registers.pc += num_operands;
            }
            InstructionName::CMP => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    self.memory.memory[addr as usize]
                };
                cmp(&mut self.registers, data);
                self.registers.pc += num_operands;
            }
            InstructionName::PHA => {
                pha(&mut self.registers, &mut self.memory);
                self.registers.pc += num_operands;
            }
            InstructionName::PLP => {
                plp(&mut self.registers, &mut self.memory);
                self.registers.pc += num_operands;
            }
            InstructionName::BMI => {
                if !bmi(&mut self.registers, addr) {
                    self.registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::ORA => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    self.memory.memory[addr as usize]
                };
                ora(&mut self.registers, data);
                self.registers.pc += num_operands;
            }
            InstructionName::CLV => {
                clv(&mut self.registers);
                self.registers.pc += num_operands;
            }
            InstructionName::EOR => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    self.memory.memory[addr as usize]
                };
                eor(&mut self.registers, data);
                self.registers.pc += num_operands;
            }
            InstructionName::ADC => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    self.memory.memory[addr as usize]
                };
                adc(&mut self.registers, data);

                self.registers.pc += num_operands;
            }
            InstructionName::STY => {
                sty(&mut self.registers, &mut self.memory, addr);
                self.registers.pc += num_operands;
            }
            InstructionName::INY => {
                iny(&mut self.registers);
                self.registers.pc += num_operands;
            }
            InstructionName::INX => {
                inx(&mut self.registers);
                self.registers.pc += num_operands;
            }
            InstructionName::TAX => {
                tax(&mut self.registers);
                self.registers.pc += num_operands;
            }
            InstructionName::TYA => {
                tya(&mut self.registers);
                self.registers.pc += num_operands;
            }
            InstructionName::TXA => {
                txa(&mut self.registers);
                self.registers.pc += num_operands;
            }
            InstructionName::TSX => {
                tsx(&mut self.registers, &mut self.memory);
                self.registers.pc += num_operands;
            }
            InstructionName::DEX => {
                dex(&mut self.registers);
                self.registers.pc += num_operands;
            }
            InstructionName::LSR => {
                if addressing_mode == AddressingMode::Accumulator {
                    lsr_acc(&mut self.registers);
                } else {
                    lsr(&mut self.registers, &mut self.memory, addr);
                }
                self.registers.pc += num_operands;
            }
            InstructionName::ROR => {
                if addressing_mode == AddressingMode::Accumulator {
                    ror_acc(&mut self.registers);
                } else {
                    ror(&mut self.registers, &mut self.memory, addr);
                }
                self.registers.pc += num_operands;
            }
            InstructionName::ROL => {
                if addressing_mode == AddressingMode::Accumulator {
                    rol_acc(&mut self.registers);
                } else {
                    let data = self.memory.memory[addr as usize];
                    rol(&mut self.registers, &mut self.memory, addr, data);
                }
                self.registers.pc += num_operands;
            }
            InstructionName::DEC => {
                dec(&mut self.registers, &mut self.memory, addr);
                self.registers.pc += num_operands;
            }

            // UNOFFICIAL Instructions
            InstructionName::LAX => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    self.memory.memory[addr as usize]
                };

                lda(&mut self.registers, data);
                ldx(&mut self.registers, data as u16);
                self.registers.pc += num_operands;
            }
            InstructionName::SAX => {
                self.memory.memory[addr as usize] =
                    ((self.registers.a & self.registers.x) as i16) as u8;
                self.registers.pc += num_operands;
            }
            InstructionName::DCP => {
                dec(&mut self.registers, &mut self.memory, addr);
                cmp(&mut self.registers, self.memory.memory[addr as usize]);
                self.registers.pc += num_operands;
            }
            InstructionName::ISB => {
                inc(&mut self.registers, &mut self.memory, addr);
                sbc(&mut self.registers, self.memory.memory[addr as usize]);
                self.registers.pc += num_operands;
            }
            InstructionName::SLO => {
                let data = self.memory.memory[addr as usize];
                asl(&mut self.registers, &mut self.memory, addr, data);
                ora(&mut self.registers, self.memory.memory[addr as usize]);
                self.registers.pc += num_operands;
            }
            InstructionName::RLA => {
                let data = self.memory.memory[addr as usize];
                rol(&mut self.registers, &mut self.memory, addr, data);
                and(&mut self.registers, self.memory.memory[addr as usize]);
                self.registers.pc += num_operands;
            }
            InstructionName::SRE => {
                lsr(&mut self.registers, &mut self.memory, addr);
                eor(&mut self.registers, self.memory.memory[addr as usize]);
                self.registers.pc += num_operands;
            }
            InstructionName::RRA => {
                ror(&mut self.registers, &mut self.memory, addr);
                adc(&mut self.registers, self.memory.memory[addr as usize]);
                self.registers.pc += num_operands;
            }
        }

        let new_cycles = get_cycles(instruction, addressing_mode, page_crossed, branched);
        self.cycle += new_cycles as usize;

        // PPU

        for _ in 0..(new_cycles * 3) {
            // let get_oam_byte = |n: i32, m: i32| 4 * n + m;

            if mirror_addr == 0x2000 {
                // PPUCTRL register
                self.ppu_registers.ctrl = Ctrl::new_from(self.memory.memory[0x2000]);
            } else if mirror_addr == 0x2001 {
                // PPUMASK register
                self.ppu_registers.mask = Mask::new_from(self.memory.memory[0x2001]);
            } else if mirror_addr == 0x2002 {
                // PPUSTATUS register
                self.ppu_registers.status = Status::new_from(self.memory.memory[0x2002]);
            }
            // } else if mirror_addr == 0x2003 {
            //     // OAMADDR register
            //     let oamaddr = self.memory.memory[0x2003];
            // } else if mirror_addr == 0x2004 {
            //     // OAMDATA register
            //     let oamdata = self.memory.memory[0x2004];
            // } else if mirror_addr == 0x2005 {
            //     // PPUSCROLL register
            //     let ppuscroll = self.memory.memory[0x2005];
            // } else if mirror_addr == 0x2006 {
            //     // PPUADDR regsiter
            //     let ppuaddr = self.memory.memory[0x2006];
            // } else if mirror_addr == 0x2007 {
            //     // PPUDATA register
            //     let ppudata = self.memory.memory[0x2007];
            // } else if mirror_addr == 0x4014 {
            //     // OAMDATA register
            //     let oamdata = self.memory.memory[0x4014];
            // }

            self.ppu_cycle += 1;
            if self.ppu_cycle > 340 {
                self.frames += 1;
            }
            self.ppu_cycle %= 341;
        }
    }

    pub fn get_nestest_output(&self) -> String {
        let opcode = self.get_opcode();
        let instruction = match_instruction(opcode);

        let (instruction, addressing_mode, is_official_instruction) = match instruction {
            Instruction::Official(instr, addr) => (instr, addr, true),
            Instruction::Unofficial(instr, addr) => (instr, addr, false),
            Instruction::Unknown => {
                unreachable!()
            }
        };

        let num_operands = num_operands_from_addressing(&addressing_mode) as u16;
        let ops = get_operands(&self.registers, &self.memory);

        let (low_byte, high_byte) = ops;
        let addr = apply_addressing(
            &self.memory,
            &self.registers,
            addressing_mode.clone(),
            low_byte,
            high_byte,
        )
        .unwrap_or(0);

        // RAM MIRORRING AND
        let mirror_addr = if addr < 0x2000 {
            // System memory is mirrored
            addr % 0x0800
        } else if (0x2000..0x4000).contains(&addr) {
            // PPU I/O rgisters are mirrored
            if addr > 0x007 {
                addr % 0x2008 + 0x2000
            } else {
                addr
            }
        } else {
            addr
        };

        let op1 = if num_operands >= 1 {
            format!("{:02X}", ops.0)
        } else {
            "  ".to_string()
        };

        let op2 = if num_operands > 1 {
            format!("{:02X}", ops.1)
        } else {
            "  ".to_string()
        };

        let instr = if !is_official_instruction {
            format!("*{:?}", instruction)
        } else {
            format!(" {:?}", instruction)
        };

        let addressing_stuff = match (addressing_mode, num_operands) {
            (AddressingMode::Relative, _) => format!(
                "${:04X}",
                self.registers
                    .pc
                    .wrapping_add(if addr >= 0x80 {
                        (addr as i32 - (1 << 8)) as u16
                    } else {
                        addr
                    })
                    .wrapping_add(2)
            ),
            (AddressingMode::Absolute, _) => match instruction {
                InstructionName::JMP
                | InstructionName::BCS
                | InstructionName::JSR
                | InstructionName::BCC
                | InstructionName::BEQ
                | InstructionName::BMI
                | InstructionName::BNE
                | InstructionName::BPL
                | InstructionName::BVC => format!("${:04X}", addr),
                _ => format!(
                    "${:04X} = {:02X}",
                    addr, self.memory.memory[mirror_addr as usize]
                ),
            },
            (AddressingMode::AbsoluteIndirectWithX, _) => format!(
                "${:04X},X @ {:04X} = {:02X}",
                address_from_bytes(ops.0, ops.1),
                address_from_bytes(ops.0, ops.1).wrapping_add(self.registers.x.into()),
                self.memory.memory[mirror_addr as usize]
            ),
            (AddressingMode::AbsoluteIndirectWithY, _) => format!(
                "${:04X},Y @ {:04X} = {:02X}",
                address_from_bytes(ops.0, ops.1),
                address_from_bytes(ops.0, ops.1).wrapping_add(self.registers.y.into()),
                self.memory.memory[mirror_addr as usize]
            ),
            (AddressingMode::Immediate, _) => format!("#${:02X}", addr),
            (AddressingMode::Accumulator, _) => "A".to_string(),

            (AddressingMode::ZeroPageIndexedIndirect, _) => format!(
                "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                ops.0,
                ops.0.wrapping_add(self.registers.x),
                addr,
                self.memory.memory[mirror_addr as usize]
            ),
            (AddressingMode::ZeroPageIndirectIndexedWithY, _) => format!(
                "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                ops.0,
                address_from_bytes(
                    self.memory.memory[ops.0 as usize],
                    self.memory.memory[ops.0.wrapping_add(1) as usize]
                ),
                addr,
                self.memory.memory[mirror_addr as usize]
            ),
            (AddressingMode::AbsoluteIndirect, _) => {
                format!("(${:04X}) = {:04X}", address_from_bytes(ops.0, ops.1), addr)
            }
            (AddressingMode::ZeroPage, _) => format!(
                "${:02X} = {:02X}",
                addr, self.memory.memory[mirror_addr as usize]
            ),
            (AddressingMode::ZeroPageIndexedWithX, _) => format!(
                "${:02X},X @ {:02X} = {:02X}",
                ops.0,
                ops.0.wrapping_add(self.registers.x),
                self.memory.memory[mirror_addr as usize]
            ),
            (AddressingMode::ZeroPageIndexedWithY, _) => format!(
                "${:02X},Y @ {:02X} = {:02X}",
                ops.0,
                ops.0.wrapping_add(self.registers.y),
                self.memory.memory[mirror_addr as usize]
            ),
            _ => "".to_string(),
        };

        format!(
            "{:04X}  {:02X} {} {} {} {:27} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:{:3},{:3} CYC:{}",
            self.registers.pc,
            opcode,
            op1,
            op2,
            instr,
            addressing_stuff,
            self.registers.a, self.registers.x, self.registers.y, self.registers.status, self.memory.stack_pointer,
            self.frames,
            self.ppu_cycle,
            self.cycle,
        )
    }
}
