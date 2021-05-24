use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};
mod cpu;
pub mod nessy;
mod test_cpu;
mod test_nestest;
use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
    prelude::*,
    prelude::{App, IntoSystem},
    reflect::TypeUuid,
    DefaultPlugins,
};
use cpu::{instructions::*, utils::RESET_VECTOR_ADDRESS, utils::*, Memory, *};

mod ppu;
use nes_rom::RomFile;

use crate::nessy::Nessy;
mod nes_rom;

#[derive(TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct NESRomAsset {
    pub rom: nes_rom::RomFile,
}

#[derive(Default)]
pub struct NESRomAssetLoader;

impl AssetLoader for NESRomAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = NESRomAsset {
                rom: nes_rom::RomFile::new(bytes),
            };
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["nes"]
    }
}

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

    App::build()
        .add_plugins(DefaultPlugins)
        .add_asset::<NESRomAsset>()
        .add_startup_system(setup.system())
        .run();

    loop {
        nessy.execute();
    }
}

fn setup() {}
