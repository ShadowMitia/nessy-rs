use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};
mod cpu;
pub mod nessy;
mod test_nestest;
mod test_cpu;
use cpu::{instructions::*, utils::RESET_VECTOR_ADDRESS, utils::*, Memory, *};

mod ppu;
use nes_rom::RomFile;
use ppu::{Memory as PPUMemory, *};

use crate::nessy::Nessy;

mod nes_rom;


fn main() {
    println!("Nessy 🐉!");

    let args: Vec<String> = std::env::args().collect();
    println!("{:#?}", args);

    let mut nessy = Nessy::new();

    // Load ROM and decode header
    let nesfile = if args.len() > 1 {
        let input = std::fs::File::open(&args[1]).unwrap();
        let mut buffered = BufReader::new(input);
        let mut rom = Vec::new();
        buffered.read_to_end(&mut rom).unwrap();
        let rom = rom.as_slice();
        nes_rom::RomFile::new(rom)
    } else {
        panic!("No ROM file provided");
    };

    nessy.load(&nesfile);

    loop {
        nessy.execute();
    }
}
