# improved-broccoli

WIP [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) emulator written in Rust, for fun.

## Running

### Running locally

Run `cargo run --release` in this directory.

### Running on the web

Install the `wasm32-unknown-unknown` target:

```bash
rustup target add wasm32-unknown-unknown
```

Install Trunk:

```bash
cargo install --locked trunk
```

Build and serve locally:

```bash
cd gui
trunk server
```

Navigate to <http://127.0.0.1:8080/index.html#dev> in a browser.

## Building

### Building locally

Run `cargo build --release` in this directory. Your executable will be somewhere in the `target/` directory.

### Building for web

Install the `wasm32-unknown-unknown` target:

```bash
rustup target add wasm32-unknown-unknown
```

Install Trunk:

```bash
cargo install --locked trunk
```

Build with Trunk:

```bash
cd gui
trunk build --release
```

The folder `gui/dist/` will contain a static HTML website.

## Crates

- chip8
  - Emulator logic for the [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)
  - No unsafe code (has `#[forbid(unsafe_code)]`)
  - Supports `no_std` using only `core` (no `alloc` needed!)
- gui
  - GUI for the emulator using [egui](https://docs.rs/egui/latest/egui/) and [eframe](https://docs.rs/eframe/latest/eframe/)
  - Based on the [eframe_template](https://github.com/emilk/eframe_template/) project
  - Supports running in the browser via WASM
  - [Internationalization](https://en.wikipedia.org/wiki/Internationalization_and_localization) using [rust-i18n](https://github.com/longbridgeapp/rust-i18n)

## Goals

- A decent GUI
  - <https://en.wikipedia.org/wiki/KISS_principle>
- Support standard COSMAC VIP CHIP-8 programs and behavior
  - This may change in the future but for now support for extensions is not planned and hasn't been considered during development
- Standard nice emulator features such as:
  - Key binding
  - Savestates
  - Pausing
  - Simulation speed
  - Frame advance
  - Graphics options such as which colors are used instead of black and white
  - Sound options such as changing the volume, pitch, and sound played
- Support cool reverse engineering features such as:
  - Memory viewing and editing
  - Debugging
  - Dissassembing
    - Using either Octo scripting syntax and a commonplace assembly style syntax

## FAQ

### Why "improved-broccoli"?

The name was just a random GitHub repo name suggestion which I thought was funny. Disclaimer: this project does not claim to improve upon, or have any other effect on, broccoli in any way, shape, or form.

### License?

Currently undecided so please don't contribute or use code from this repository without agreeing a license.

## FAQ for nerds

### Why not implement the `Error` trait on error types?

The error types in the `chip8` crate don't implement the `std::error::Error` trait, because it's currently not supported in `no_std` environments, and because all the methods on it are either deprecated, experimental, or wouldn't be used by the types so there's not much point in supporting it in the first place.
