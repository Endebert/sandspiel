# Sandspiel

A prototype implementation of the classic [Falling-sand game](https://en.wikipedia.org/wiki/Falling-sand_game) written in Rust.

I have started this project to get familiar with Rust. It has multithreading and the simulation is renderer-agnostic.

At the moment, a few materials are implemented, as well as various renderers. Simulation speed is dependent on monitor refresh rate.

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
---
Webgpu renderer is unfinished, and Wasm-pack renderer is currently broken due to lack of support for threads.
