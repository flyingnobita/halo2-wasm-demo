## Instructions

### Install

```bash
yarn
```

### Link Package (For Development)

```bash
# Remove halo2_playground fro package.json so changes will be picked up right
# away during development
yarn remove halo2_playground

cd public/pkg
yarn link

cd ../..  # in /react_app
yarn link halo2_playground
```

### Add Local Package (For Release w/o publishing package to NPM)

```bash
yarn add ./public/pkg
```

### Reflect changes made in rust and start react app

```bash
yarn dev
```

## Miscellaneous

### Thread Pool Size

There is currently a [bug in Chromium](https://bugs.chromium.org/p/chromium/issues/detail?id=1228686&q=reporter%3Arreverser%40google.com&can=1) which will cause the prove and verify computations to be extremely slow with Apple M1 computers. Instead of using the maximum number of threads (default), a lower number is works much better (I find 4 to be the best on my M1 Pro).

This can be changed on the webapp under "Thread Pool Size".

### SharedArrayBuffer

WASM workers uses a brower feature called `SharedArrayBuffer` that is disabled by default for sercuity reasons. While this is enabled by default during development in `yarn start`, it needs to be explicitly enabled in the production HTTP server by allowing COOP (Cross Origin Opener Policy) and COEP (Cross Origin Embedder Policy). For Netlify, this is done in [`netlify.toml`](./netlify.toml).
