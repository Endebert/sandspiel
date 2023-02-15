# Sandspiel

A prototype implementation of the classic [Falling-sand game](https://en.wikipedia.org/wiki/Falling-sand_game) written in Rust.

## Usage
### In Terminal
```bash
cargo run --package terminal_renderer
```
Press Enter to forward the simulation. You can change the initial state in [main.rs](terminal_renderer%2Fsrc%2Fmain.rs)
### In a window (using [pixels](https://github.com/parasyte/pixels))
```bash
cargo run --package pixels_renderer
```