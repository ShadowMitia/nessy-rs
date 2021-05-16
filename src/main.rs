use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    thread::sleep,
    time::Duration,
};
mod rp2a03;
use rp2a03::{instructions::*, utils::RESET_VECTOR_ADDRESS, utils::*, Registers, *};

mod nes_rom;

fn main() {
    println!("Nessy 🐉!");

    let args: Vec<String> = std::env::args().collect();
    println!("{:#?}", args);

    // Initialise memory
    let mut memory = Memory::new();

    let mut is_nestest = false;
    let nestest = include_bytes!("../nestest.nes");

    // Load ROM and decode header
    let nesfile = if args.len() > 1 {
        let input = std::fs::File::open(&args[1]).unwrap();
        let mut buffered = BufReader::new(input);
        let mut rom = Vec::new();
        buffered.read_to_end(&mut rom).unwrap();
        let rom = rom.as_slice();
        nes_rom::RomFile::new(rom)
    } else {
        let rom = nestest;
        is_nestest = true;
        nes_rom::RomFile::new(rom)
    };

    // println!(
    //     "Num of 16k bytes PRG ROM {} ({}k bytes)\nNum of 8k CHR ROM {}",
    //     num_prgrom,
    //     16 * num_prgrom,
    //     num_chrrom
    // );

    // let rom = match nesfile {
    //     RomFile::Ines(_, ref data) => data,
    //     RomFile::Ines2(_, _) => {
    //         todo!()
    //     }
    // };
    let mut nestest_output = File::create("nestest.log").unwrap();

    // Load up memory
    nes_rom::mappers::load_rom(&mut memory, &nesfile);

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
    if is_nestest {
        registers.pc = 0xC000; // Becasue nestest starts here
    } else {
        registers.pc = reset_vector;
    }

    // POWER ON NES

    // CPU

    if is_nestest {
        registers.status = 0x24;
    } else {
        registers.status = 0x34;
    }

    memory.stack_pointer = 0xFD; // Stack is on page 1 only so 0xFF is actually 0x01FF

    // Memory

    memory.memory[0x4017] = 0x00; // Frame IRQ enabled
    memory.memory[0x4015] = 0x00; // All channels disabled
    memory.memory[0x4000..0x400F].copy_from_slice(&[0x0; 15]);
    memory.memory[0x4010..0x4013].copy_from_slice(&[0x0; 3]);

    // Disable decimal mode
    registers.set_flag(StatusFlag::D, false);

    // PPU

    // Set PPUCTRL
    memory.memory[0x2000] = 0x0;
    // Set PPUMASK
    memory.memory[0x2001] = 0x0;
    // PPUSTATUS
    memory.memory[0x2000] = 0b10100000;
    // OAMADDR
    memory.memory[0x2003] = 0x0;
    // PPUSCROLL
    memory.memory[0x2005] = 0x0;
    // PPUADDR
    memory.memory[0x2006] = 0x0;
    // PPUDATA
    memory.memory[0x2007] = 0x0;

    let reference = File::open("nestest_reference.log").unwrap();
    let reference_buffered = BufReader::new(reference);
    let mut reference_lines = reference_buffered.lines();

    let mut count = 0;

    let mut cycle = 7;

    loop {
        let byte = memory.memory[registers.pc as usize];
        let instruction = match_instruction(byte);

        let (instruction, addressing_mode, is_official_instruction) = match instruction {
            Instruction::Official(instr, addr) => (instr, addr, true),
            Instruction::Unofficial(instr, addr) => (instr, addr, false),
            Instruction::Unknown => {
                eprintln!(
                    "Unknown opcode {:#x}",
                    memory.memory[(registers.pc - 1) as usize]
                );
                break;
            }
        };

        let num_operands = num_operands_from_addressing(&addressing_mode) as u16;
        let ops = get_operands(&registers, &memory);

        let (low_byte, high_byte) = ops;
        let addr = apply_addressing(
            &memory,
            &registers,
            addressing_mode.clone(),
            low_byte,
            high_byte,
        )
        .unwrap_or(0);

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
                let low = memory.memory[address_from_bytes(low_byte, 0x0) as usize];
                let high =
                    memory.memory[address_from_bytes(low_byte.wrapping_add(1), 0x0) as usize];

                is_page_crossed(address_from_bytes(low, high) as u16, addr)
            }
            (_, AddressingMode::Relative) => is_page_crossed(
                registers.pc + 2, /* Include pc++ and operand read */
                (registers.pc as i16
                    + 2
                    + if addr >= 0x80 {
                        (addr as i32 - (1 << 8)) as i16
                    } else {
                        addr as i16
                    }) as u16,
            ),
            (_, AddressingMode::Absolute) => is_page_crossed(
                registers.pc + 2, /* Include pc++ and operand read */
                addr,
            ),
            _ => false,
        };

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

        if is_nestest {
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

            let addressing_stuff = match (addressing_mode.clone(), num_operands) {
                (AddressingMode::Relative, _) => format!(
                    "${:04X}",
                    registers
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
                        addr, memory.memory[mirror_addr as usize]
                    ),
                },
                (AddressingMode::AbsoluteIndirectWithX, _) => format!(
                    "${:04X},X @ {:04X} = {:02X}",
                    address_from_bytes(ops.0, ops.1),
                    address_from_bytes(ops.0, ops.1).wrapping_add(registers.x.into()),
                    memory.memory[mirror_addr as usize]
                ),
                (AddressingMode::AbsoluteIndirectWithY, _) => format!(
                    "${:04X},Y @ {:04X} = {:02X}",
                    address_from_bytes(ops.0, ops.1),
                    address_from_bytes(ops.0, ops.1).wrapping_add(registers.y.into()),
                    memory.memory[mirror_addr as usize]
                ),
                (AddressingMode::Immediate, _) => format!("#${:02X}", addr),
                (AddressingMode::Accumulator, _) => "A".to_string(),

                (AddressingMode::ZeroPageIndexedIndirect, _) => format!(
                    "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                    ops.0,
                    ops.0.wrapping_add(registers.x),
                    addr,
                    memory.memory[mirror_addr as usize]
                ),
                (AddressingMode::ZeroPageIndirectIndexedWithY, _) => format!(
                    "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                    ops.0,
                    address_from_bytes(
                        memory.memory[ops.0 as usize],
                        memory.memory[ops.0.wrapping_add(1) as usize]
                    ),
                    addr,
                    memory.memory[mirror_addr as usize]
                ),
                (AddressingMode::AbsoluteIndirect, _) => {
                    format!("(${:04X}) = {:04X}", address_from_bytes(ops.0, ops.1), addr)
                }
                (AddressingMode::ZeroPage, _) => format!(
                    "${:02X} = {:02X}",
                    addr, memory.memory[mirror_addr as usize]
                ),
                (AddressingMode::ZeroPageIndexedWithX, _) => format!(
                    "${:02X},X @ {:02X} = {:02X}",
                    ops.0,
                    ops.0.wrapping_add(registers.x),
                    memory.memory[mirror_addr as usize]
                ),
                (AddressingMode::ZeroPageIndexedWithY, _) => format!(
                    "${:02X},Y @ {:02X} = {:02X}",
                    ops.0,
                    ops.0.wrapping_add(registers.y),
                    memory.memory[mirror_addr as usize]
                ),
                _ => "".to_string(),
            };

            let res = format!(
                "{:04X}  {:02X} {} {} {} {:27} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:  0, 21 CYC:{}",
                registers.pc,
                byte,
                op1,
                op2,
                instr,
                addressing_stuff,
                registers.a, registers.x, registers.y, registers.status, memory.stack_pointer,
                // 0, 0,
                // " ",
                cycle,
            );

            let ref_line = reference_lines.next();
            let ref_columns_1;
            if let Some(ref_line) = ref_line {
                ref_columns_1 = ref_line.unwrap();
            } else {
                break;
            }

            let ref_columns = ref_columns_1
                .split(' ')
                // .filter(|&thing| !thing.is_empty())
                .collect::<Vec<&str>>();
            let output_columns = res
                .split(' ')
                // .filter(|&thing| !thing.is_empty())
                .collect::<Vec<&str>>();

            let mut matched = true;

            let ref_cyc = ref_columns.iter().last().unwrap();
            let cyc = output_columns.iter().last().unwrap();

            // for (&ref_col, output_col) in ref_columns.iter().zip(output_columns.clone()) {
            //     print!("\u{001b}[37m");
            //     if output_col.contains("PPU") {
            //         continue;
            //     }
            //     if ref_col == output_col {
            //         print!("\u{001b}[32m")
            //     } else {
            //         // print!("({} {})", ref_col, output_col);
            //         print!("\u{001b}[31m");
            //         matched = false;
            //     }
            //     print!("{} ", output_col);
            // }
            // println!();
            // if !matched {
            println!("\u{001b}[35m{} ", res);
            println!("\u{001b}[37m{} ", ref_columns_1);
            // writeln!(nestest_output, "#{}", &res);
            // writeln!(nestest_output, ">{}", ref_columns_1);
            // }

            if cyc != ref_cyc {
                break;
            }

            // sleep(Duration::from_millis(10));
        }

        // println!("addr {:X} mirror addr {:X}", addr, mirror_addr);

        let j_addr = addr;
        let addr = mirror_addr;

        registers.pc += 1; // READ instruction

        let mut branched = false;

        match instruction {
            InstructionName::SEI => {
                sei(&mut registers);
                registers.pc += num_operands;
            }
            InstructionName::CLD => {
                cld(&mut registers);
                registers.pc += num_operands;
            }
            InstructionName::LDA => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    memory.memory[addr as usize]
                };

                lda(&mut registers, data);
                registers.pc += num_operands;
            }
            InstructionName::BRK => {
                brk(&mut registers, &mut memory);
            }
            InstructionName::STA => {
                sta(&mut registers, &mut memory, addr);
                registers.pc += num_operands;
            }
            InstructionName::INC => {
                inc(&mut registers, &mut memory, addr);
                registers.pc += num_operands;
            }
            InstructionName::LDX => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    memory.memory[addr as usize]
                };
                ldx(&mut registers, data.into());
                registers.pc += num_operands;
            }
            InstructionName::TXS => {
                txs(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            InstructionName::AND => {
                if addressing_mode == AddressingMode::Accumulator {
                    and_acc(&mut registers);
                } else {
                    let data = if addressing_mode == AddressingMode::Immediate {
                        addr as u8
                    } else {
                        memory.memory[addr as usize]
                    };
                    and(&mut registers, data);
                }

                registers.pc += num_operands;
            }
            InstructionName::BEQ => {
                if !beq(&mut registers, addr) {
                    registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::CPX => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    memory.memory[addr as usize]
                };
                cpx(&mut registers, data);
                registers.pc += num_operands;
            }
            InstructionName::DEY => {
                dey(&mut registers);
                registers.pc += num_operands;
            }
            InstructionName::BPL => {
                if !bpl(&mut registers, addr) {
                    registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::PLA => {
                pla(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            InstructionName::TAY => {
                tay(&mut registers);
                registers.pc += num_operands;
            }
            InstructionName::CPY => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    memory.memory[addr as usize]
                };
                cpy(&mut registers, data);
                registers.pc += num_operands;
            }
            InstructionName::BNE => {
                if !bne(&mut registers, addr) {
                    registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::RTS => {
                rts(&mut registers, &mut memory);
            }
            InstructionName::JMP => {
                jmp(&mut registers, j_addr);
            }
            InstructionName::STX => {
                stx(&mut registers, &mut memory, addr);
                registers.pc += num_operands;
            }
            InstructionName::JSR => {
                jsr(&mut registers, &mut memory, j_addr);
            }
            InstructionName::NOP => {
                nop();
                registers.pc += num_operands;
            }
            InstructionName::SEC => {
                sec(&mut registers);
                registers.pc += num_operands;
            }
            InstructionName::BCS => {
                if !bcs(&mut registers, addr) {
                    registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::CLC => {
                clc(&mut registers);
                registers.pc += num_operands;
            }
            InstructionName::BCC => {
                if !bcc(&mut registers, addr) {
                    registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::PHP => {
                php(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            InstructionName::BIT => {
                bit(&mut registers, &mut memory, addr);
                registers.pc += num_operands;
            }
            InstructionName::BVS => {
                if !bvs(&mut registers, addr) {
                    registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::BVC => {
                if !bvc(&mut registers, addr) {
                    registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::LDY => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    memory.memory[addr as usize]
                };
                ldy(&mut registers, data);
                registers.pc += num_operands;
            }
            InstructionName::ASL => {
                if addressing_mode == AddressingMode::Accumulator {
                    asl_acc(&mut registers);
                } else {
                    let data = memory.memory[addr as usize];
                    asl(&mut registers, &mut memory, addr, data);
                }

                registers.pc += num_operands;
            }
            InstructionName::RTI => {
                rti(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            InstructionName::SBC => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    memory.memory[addr as usize]
                };
                sbc(&mut registers, data);
                registers.pc += num_operands;
            }
            InstructionName::SED => {
                sed(&mut registers);
                registers.pc += num_operands;
            }
            InstructionName::CMP => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    memory.memory[addr as usize]
                };
                cmp(&mut registers, data);
                registers.pc += num_operands;
            }
            InstructionName::PHA => {
                pha(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            InstructionName::PLP => {
                plp(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            InstructionName::BMI => {
                if !bmi(&mut registers, addr) {
                    registers.pc += num_operands;
                } else {
                    branched = true;
                }
            }
            InstructionName::ORA => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    memory.memory[addr as usize]
                };
                ora(&mut registers, data);
                registers.pc += num_operands;
            }
            InstructionName::CLV => {
                clv(&mut registers);
                registers.pc += num_operands;
            }
            InstructionName::EOR => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    memory.memory[addr as usize]
                };
                eor(&mut registers, data);
                registers.pc += num_operands;
            }
            InstructionName::ADC => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    memory.memory[addr as usize]
                };
                adc(&mut registers, data);

                registers.pc += num_operands;
            }
            InstructionName::STY => {
                sty(&mut registers, &mut memory, addr);
                registers.pc += num_operands;
            }
            InstructionName::INY => {
                iny(&mut registers);
                registers.pc += num_operands;
            }
            InstructionName::INX => {
                inx(&mut registers);
                registers.pc += num_operands;
            }
            InstructionName::TAX => {
                tax(&mut registers);
                registers.pc += num_operands;
            }
            InstructionName::TYA => {
                tya(&mut registers);
                registers.pc += num_operands;
            }
            InstructionName::TXA => {
                txa(&mut registers);
                registers.pc += num_operands;
            }
            InstructionName::TSX => {
                tsx(&mut registers, &mut memory);
                registers.pc += num_operands;
            }
            InstructionName::DEX => {
                dex(&mut registers);
                registers.pc += num_operands;
            }
            InstructionName::LSR => {
                if addressing_mode == AddressingMode::Accumulator {
                    lsr_acc(&mut registers);
                } else {
                    lsr(&mut registers, &mut memory, addr);
                }
                registers.pc += num_operands;
            }
            InstructionName::ROR => {
                if addressing_mode == AddressingMode::Accumulator {
                    ror_acc(&mut registers);
                } else {
                    ror(&mut registers, &mut memory, addr);
                }
                registers.pc += num_operands;
            }
            InstructionName::ROL => {
                if addressing_mode == AddressingMode::Accumulator {
                    rol_acc(&mut registers);
                } else {
                    let data = memory.memory[addr as usize];
                    rol(&mut registers, &mut memory, addr, data);
                }
                registers.pc += num_operands;
            }
            InstructionName::DEC => {
                dec(&mut registers, &mut memory, addr);
                registers.pc += num_operands;
            }

            // UNOFFICIAL Instructions
            InstructionName::LAX => {
                let data = if addressing_mode == AddressingMode::Immediate {
                    addr as u8
                } else {
                    memory.memory[addr as usize]
                };

                lda(&mut registers, data);
                ldx(&mut registers, data as u16);
                registers.pc += num_operands;
            }
            InstructionName::SAX => {
                memory.memory[addr as usize] = ((registers.a & registers.x) as i16) as u8;
                registers.pc += num_operands;
            }
            InstructionName::DCP => {
                dec(&mut registers, &mut memory, addr);
                cmp(&mut registers, memory.memory[addr as usize]);
                registers.pc += num_operands;
            }
            InstructionName::ISB => {
                inc(&mut registers, &mut memory, addr);
                sbc(&mut registers, memory.memory[addr as usize]);
                registers.pc += num_operands;
            }
            InstructionName::SLO => {
                let data = memory.memory[addr as usize];
                asl(&mut registers, &mut memory, addr, data);
                ora(&mut registers, memory.memory[addr as usize]);
                registers.pc += num_operands;
            }
            InstructionName::RLA => {
                let data = memory.memory[addr as usize];
                rol(&mut registers, &mut memory, addr, data);
                and(&mut registers, memory.memory[addr as usize]);
                registers.pc += num_operands;
            }
            InstructionName::SRE => {
                lsr(&mut registers, &mut memory, addr);
                eor(&mut registers, memory.memory[addr as usize]);
                registers.pc += num_operands;
            }
            InstructionName::RRA => {
                ror(&mut registers, &mut memory, addr);
                adc(&mut registers, memory.memory[addr as usize]);
                registers.pc += num_operands;
            }
        }

        let new_cycles = get_cycles(instruction, addressing_mode.clone(), page_crossed, branched);

        cycle += new_cycles as u32;
    }
}
