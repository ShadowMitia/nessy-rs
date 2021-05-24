#[cfg(test)]
mod nestest {
    use crate::{nes_rom, nessy::Nessy};

    #[test]
    fn main() {
        let mut nessy = Nessy::new();
        let nestest = include_bytes!("../test_roms/nes-test-roms-master/other/nestest.nes");

        // Load ROM and decode header
        let rom = nestest;
        let nesfile = nes_rom::RomFile::new(rom);

        nessy.load_nestest(&nesfile);

        let reference = include_str!("../test_roms/nestest.log");
        let mut reference_lines = reference.lines();

        loop {
            let ref_line = reference_lines.next();
            let ref_columns_1 = if let Some(ref_line) = ref_line {
                ref_line
            } else {
                break;
            };

            let res = nessy.get_nestest_output();

            let ref_columns = ref_columns_1.split(' ').collect::<Vec<&str>>();
            let output_columns = res.split(' ').collect::<Vec<&str>>();

            let mut matched = true;

            for (&ref_col, output_col) in ref_columns.iter().zip(output_columns.clone()) {
                // print!("\u{001b}[37m");
                if ref_col == output_col {
                    // print!("\u{001b}[32m")
                } else {
                    // print!("\u{001b}[31m");
                    matched = false;
                }
                // print!("{} ", output_col);
            }
            // println!();
            if !matched {
                // println!("\u{001b}[37m{} ", ref_columns_1);
                unreachable!();
            }

            // print!("\u{001b}[0m");

            nessy.execute();
        }
    }
}
