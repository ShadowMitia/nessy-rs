# A NES emulator

## Usage (WIP)

You can run the emulot and test it against nestest by running without argument

```
cargo run
```

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

## 6502 NES CPU

Passes [nestest.nes](https://wiki.nesdev.com/w/index.php/Emulator_tests?source=post_page) (99.9%)
Still missing a few unofficial instructions though...
No cycle or timing handling for now.
Right now no ambition to make a full 6502 CPU, just NES one.

## Supported Features

- Load ROMS

### Mappers

- Mapper 0
