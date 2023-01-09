# Halo2 Playground

A repo to learn and experiment with Halo2.

## Circuits

### Circuit 1

Uses `square_chip.rs` to square the user input number.

### Circuit 2

Uses `cube_chip.rs` to cube the user input number.

## Setup

### Build Target

halo2 uses Rayon for parallel computations. In order to use Rayon in wasm, the binding [`wasm-bindgen-rayon`](https://github.com/GoogleChromeLabs/wasm-bindgen-rayon) is required. `wasm-bindgen-rayon` requires setting the `build-std` flag in `.cargo/config`, which in turn require setting the build architecture `target` in the same [file](https://github.com/flyingnobita/halo2-playground/blob/6cea21c739cdf56a9b27fd236b4102e9249ca9e0/circuits/.cargo/config.toml#L12). You need to fill in the appropriate target of your computer before using this crate.

### Parameter File

The parameter file `params.bin` is generated as part `main()`. This file is copied and used in the React app.

## Build

```rust
cargo build
```

## Run

```rust
cargo run
```

## Test

```rust
cargo test -- --nocapture
```
