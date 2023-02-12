# improved-broccoli

WIP [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) emulator written in Rust, for fun.

## Crates

- chip8
  - Emulator logic for the [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)
- gui
  - GUI for the emulator using [egui](https://docs.rs/egui/latest/egui/) and [eframe](https://docs.rs/eframe/latest/eframe/)
  - This is based on the eframe_template project

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

## Why "improved-broccoli"?

The name was just a random GitHub repo name suggestion which I thought was funny. Disclaimer: this project does not claim to improve upon, or have any other effect on, broccoli in any way, shape, or form.
