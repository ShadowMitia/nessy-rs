use self::mappers::Mapper;

use super::*;

pub mod mappers {

    use super::*;

    pub fn load_rom(memory: &mut cpu::cpu::Memory, ppu_memory: &mut ppu::Memory, nesfile: &RomFile) {
        match nesfile {
            RomFile::Ines(nesfile, data) => match nesfile.mapper {
                Mapper::Nrom => {
                    memory.memory[0x8000..0x8000 + 16384].copy_from_slice(&data[16..16 + 16384]);

                    memory.memory[0xC000..=0xFFFF].copy_from_slice(
                        &data[(16 + 16384 * (nesfile.num_prgrom - 1) as usize)
                            ..16 + 16384 * (nesfile.num_prgrom) as usize],
                    );

                    ppu_memory.memory[0x0000..0x1FFF].copy_from_slice(
                        &data[(16 + 16384 * (nesfile.num_prgrom as usize) + 1)
                            ..(16
                                + 16384 * (nesfile.num_prgrom as usize)
                                + (nesfile.num_chrrom as usize) * 8192)
                                as usize],
                    )
                }
                Mapper::Unknown => panic!("Unknown mapper"),
            },
            _ => unreachable!(),
        }
    }
    #[repr(u32)]
    #[derive(PartialEq, Clone, Copy)]
    pub enum Mapper {
        Nrom,
        Unknown = u32::MAX,
    }

    impl From<u8> for Mapper {
        fn from(from: u8) -> Self {
            match from {
                0 => Mapper::Nrom,
                _ => Mapper::Unknown,
            }
        }
    }
}

pub enum RomFile {
    Ines(Ines, Vec<u8>),
    Ines2(Ines2, Vec<u8>),
}

pub struct Ines2 {}

pub struct Ines {
    num_prgrom: u8,
    num_chrrom: u8,
    mirroring: bool,
    persistent_memory: bool,
    has_trainer: bool,
    four_screen_vram: u8,
    mapper_lsb: u8,
    vs: bool,
    playchoice: bool,
    mapper_msb: u8,
    prgram_size: u8,
    tv_system: bool,

    tv_system2: u8,
    has_prg_ram: bool,
    has_bus_conflict: bool,
    padding: Vec<u8>,
    mapper: Mapper,
}
#[derive(Debug, PartialEq)]
pub enum SupportedFormat {
    ines,
    unsupported,
}

impl RomFile {
    pub fn new(rom: &[u8]) -> Self {
        let nes = &rom[0..4];

        println!("{}{}{}", nes[0] as char, nes[1] as char, nes[2] as char);

        let format = RomFile::get_file_format(rom);

        let file = if format == SupportedFormat::ines {
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
            let mapper = (mapper_msb << 4) | mapper_lsb;

            let ines = Ines {
                num_prgrom,
                num_chrrom,
                mirroring,
                persistent_memory,
                has_trainer,
                four_screen_vram,
                mapper_lsb,
                vs,
                playchoice,
                mapper_msb,
                prgram_size,
                tv_system,
                tv_system2,
                has_prg_ram,
                has_bus_conflict,
                padding: padding.to_vec(),
                mapper: mapper.into(),
            };

            println!("Format {:?}", format);
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

            RomFile::Ines(ines, rom.to_vec())
        } else {
            panic!("Unsupport file type");
        };

        file
    }

    fn get_file_format(header: &[u8]) -> SupportedFormat {
        let ines_format = header[0] as char == 'N'
            && header[1] as char == 'E'
            && header[2] as char == 'S'
            && header[3] == 0x1A; // MS-DOS end of file

        let nes2 = ines_format && (header[7] & 0x0C) == 0x08;
        // TODO: check proper size of ROM image "size taking into account byte 9 does not exceed the actual size of the ROM image, then NES 2.0."

        if ines_format {
            SupportedFormat::ines
        } else {
            SupportedFormat::unsupported
        }
    }
}
