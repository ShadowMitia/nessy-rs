#[cfg(test)]
mod cpu {
    use crate::{nes_rom, nessy::Nessy};

    #[test]
    fn instr_test_v5_offical() {
        let mut nessy = Nessy::new();
        let nestest = include_bytes!("../test_roms/instr_test-v5/official_only.nes");

        // Load ROM and decode header
        let rom = nestest;
        let nesfile = nes_rom::RomFile::new(rom);

        nessy.load(&nesfile);

        loop {
            nessy.execute();
        }
    }

    #[test]
    fn instr_misc() {
        let mut nessy = Nessy::new();
        let nestest = include_bytes!("../test_roms/instr_misc/instr_misc.nes");

        // Load ROM and decode header
        let rom = nestest;
        let nesfile = nes_rom::RomFile::new(rom);

        nessy.load(&nesfile);

        loop {
            nessy.execute();
        }
    }

    // #[test]
    // fn cpu_exec_space() {
    //     let mut nessy = Nessy::new();
    //     let nestest =
    //         include_bytes!("../test_roms/cpu_exec_space/test_cpu_exec_space_ppuio.nes");

    //     // Load ROM and decode header
    //     let rom = nestest;
    //     let nesfile = nes_rom::RomFile::new(rom);

    //     nessy.load(&nesfile);

    //     loop {
    //         nessy.execute();
    //     }
    // }
}
