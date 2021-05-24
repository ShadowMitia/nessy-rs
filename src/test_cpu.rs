#[cfg(test)]
mod cpu_test {
    use crate::{nes_rom, nessy::Nessy};

    #[test]
    fn instr_misc_test() {
        let mut nessy = Nessy::new();
        let nestest = include_bytes!("../test_roms/nes-test-roms-master/instr_misc/instr_misc.nes");

        // Load ROM and decode header
        let rom = nestest;
        let nesfile = nes_rom::RomFile::new(rom);

        nessy.load(&nesfile);

        loop {
            nessy.execute();
        }
    }
}
