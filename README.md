# A NES emulator

## Usage (WIP)

You can pass a ROM, but it won't do anything interesting for now.

```
cargo run [PATH_TO_ROM]
```

## Development

```
git clone github.com/shadowMitia/nessy-rs
cd nessy-rs
cargo build
```

## Testing

There is an attempt to test as much as possible.
Instructions and some parts of the emulator are being unit-tested.
Some NES test roms are being used (notably nestest.nes).

```
cargo test
```

## 6502 NES CPU

Passes [nestest.nes](https://wiki.nesdev.com/w/index.php/Emulator_tests?source=post_page) (99.9%)
(Since there is no APU and PPU, some things are still missing at the end of the ROM).

Still missing a few unofficial instructions...
No cycle or timing handling for now. Cycle seems to be correct according to nestest though, so hopefully shouldn't be too difficult to setup accurately.
Right now no ambition to make a full 6502 CPU, just NES one.

## Supported Features

- Load ROMS

### Support mappers

- Mapper 0
- Mapper 1 (very WIP)
