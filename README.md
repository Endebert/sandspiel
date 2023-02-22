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
### In a Browser (using [wasm-pack](https://rustwasm.github.io/wasm-pack/))
#### Demo
Try it here: [sandspiel.ende.pro](https://sandspiel.ende.pro) (might be outdated)
#### Install & Run
1. install wasm-pack (see website)
```bash
cd wasm-pack-renderer
wasm-pack build
cd www
npm install
export NODE_OPTIONS=--openssl-legacy-provider
npm start
```
OpenSSL legacy provider is required, unfortunately. The option is only needed if you are using a newer version of nodejs.