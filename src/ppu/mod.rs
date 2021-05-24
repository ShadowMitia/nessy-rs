/*!  Emulate a Ricoh 2C02 microntroller used for PPU */

pub struct Memory {
    pub memory: Vec<u8>,
    pub oam: Vec<u8>, // Object Attribute Memory
}

impl Memory {
    pub fn new() -> Self {

        let mut memory = Vec::new();
        memory.resize_with(0x10000, || 0);

        let mut oam = Vec::new();
        oam.resize_with(256, || 0);

        Self {
            memory,
            oam,
        }
    }
}

pub struct Registers {
    pub ctrl: Ctrl,
    pub mask: Mask,
    pub status: Status,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ctrl: Ctrl::new(),
            mask: Mask::new(),
            status: Status::new(),
        }
    }
}

pub struct Mask {
    color_emphasis: u8,
    sprite_enable: bool,
    background_enable: bool,
    sprite_left_column_enable: bool,
    background_left_column_enable: bool,
    greyscale: bool,
}

impl Mask {
    pub fn new() -> Self {
        Self::new_from(0x0)
    }

    pub fn new_from(byte: u8) -> Self {
        let color_emphasis = (byte & 0b11100000) >> 5;
        let sprite_enable = byte & 0b10000 == 0b10000;
        let background_enable = byte & 0b1000 == 0b1000;
        let sprite_left_column_enable = byte & 0b100 == 0b100;
        let background_left_column_enable = byte & 0b10 == 0b10;
        let greyscale = byte & 0b1 == 0b1;

        Self {
            color_emphasis,
            sprite_enable,
            background_enable,
            sprite_left_column_enable,
            background_left_column_enable,
            greyscale,
        }
    }
}

// Represents the state of the PPU Control Register (0x2000)
pub struct Ctrl {
    nmi_enable: bool,
    ppu_master_slave: bool, // Not used by NES
    sprite_height: u8,
    sprite_tile_select: bool,
    background_tile_select: bool,
    increment_mode: bool,
    nametable_select: u8,
}

impl Ctrl {
    pub fn new() -> Self {
        Self::new_from(0x0)
    }

    pub fn new_from(byte: u8) -> Self {
        let nmi_enable = byte & 0b10000000 == 0b10000000;
        let ppu_master_slave = byte & 0b01000000 == 0b01000000;
        let sprite_height = byte & 0b00100000;
        let background_tile_select = byte & 0b00010000 == 0b00010000;
        let sprite_tile_select = byte & 0b00001000 == 0b00001000;
        let increment_mode = byte & 0b00000100 == 0b00000100;
        let nametable_select = byte & 0b11;

        Self {
            nmi_enable,
            ppu_master_slave,
            sprite_height,
            sprite_tile_select,
            background_tile_select,
            increment_mode,
            nametable_select,
        }
    }
}

pub struct Status {
    vblank: bool,
    sprite_0_hit: bool,
    sprite_overflow: bool,
}

impl Status {
    pub fn new() -> Self {
        Self::new_from(0x0)
    }

    pub fn new_from(byte: u8) -> Self {
        let vblank = byte & 0b10000000 == 0b10000000;
        let sprite_0_hit = byte & 0b1000000 == 0b1000000;
        let sprite_overflow = byte & 0b100000 == 0b100000;

        Self {
            vblank,
            sprite_0_hit,
            sprite_overflow,
        }
    }
}
