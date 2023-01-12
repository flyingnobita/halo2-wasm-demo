# Halo2 Wasm Demo

[Demo](https://halo2-wasm-demo.flyingnobita.com)

This repo demonstrates how to compile halo2 circuits into Wasm and incorporate them to a react app. It is based on the [halo2 Wasm guide](https://zcash.github.io/halo2/user/wasm-port.html) which in turn is based on [Zordle](https://github.com/nalinbhardwaj/zordle).

## Circuits

There are two simple circuits:

1. Circuit 1 - uses `square_chip.rs` to square the user input number
2. Circuit 2 - uses `cube_chip.rs` to cube the user input number

### Instructions

#### Build

> halo2 uses Rayon for parallel computations. In order to use Rayon in Wasm, the binding [`wasm-bindgen-rayon`](https://github.com/GoogleChromeLabs/wasm-bindgen-rayon) is required. `wasm-bindgen-rayon` requires setting the `build-std` flag in `.cargo/config`, which in turn require setting the build architecture `target` in the same [file](https://github.com/flyingnobita/halo2-wasm-demo/blob/6cea21c739cdf56a9b27fd236b4102e9249ca9e0/circuits/.cargo/config.toml#L12). You need to fill in the appropriate target of your computer before using this crate.

```rust
cargo build
```

#### Run

```rust
cargo run
```

Besides running the circuit, you can also generate the parameter file `params.bin` to be used in `/react_app`.

#### Test

```rust
cargo test -- --nocapture
```

---

## React App

### Instructions

#### Install

```bash
yarn
```

#### Link Package (For Development)

```bash
# Remove halo2-wasm-demo from package.json so changes will be picked up right
# away during development
yarn remove halo2-wasm-demo

cd public/pkg
yarn link

cd ../..  # i.e. in /halo2-wasm-demo/react_app
yarn link halo2-wasm-demo
```

#### Add Local Package (For Release w/o publishing package to NPM)

```bash
yarn add ./public/pkg
```

#### How to run

Before running the webapp, you must generate the parameter file `params.bin` first in `/circuits` with `cargo run`. The generated `params.bin` will be automatically copied to `/react_app` to be used.

```bash
yarn dev  # This will reflect changes made in rust and start react app
```

### Miscellaneous

#### Thread Pool Size

There is currently a [bug in Chromium](https://bugs.chromium.org/p/chromium/issues/detail?id=1228686&q=reporter%3Arreverser%40google.com&can=1) which will cause the prove and verify computations to be extremely slow with Apple M1 computers. Instead of using the maximum number of threads (default), a lower number is works much better (I find 4 to be the best on my M1 Pro).

This can be changed in the webapp under "Thread Pool Size".

#### SharedArrayBuffer

Wasm workers uses a brower feature called `SharedArrayBuffer` that is disabled by default for sercuity reasons. While this is enabled by default during development in `yarn start`, it needs to be explicitly enabled in the production HTTP server by allowing COOP (Cross Origin Opener Policy) and COEP (Cross Origin Embedder Policy). For Netlify, this is done in [`netlify.toml`](./netlify.toml).

#### Apple Safari

As stated in the [halo2 Book](https://zcash.github.io/halo2/user/wasm-port.html#safari), Safari is not currently supported.
