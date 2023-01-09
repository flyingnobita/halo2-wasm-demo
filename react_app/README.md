## Instructions

### Install

```bash
yarn
```

### Link Package (For Development)

```bash
cd public/pkg
yarn link

cd ../..  # in /react_app
yarn link halo2_playground
```

### Add Local Package (For Release w/o NPM)

```bash
yarn add ./public/pkg
```

### Reflect changes made in rust and start react app

```bash
yarn dev
```

## Miscellaneous

### WASM Target File



### SharedArrayBuffer

WASM workers uses a brower feature called `SharedArrayBuffer` that is disabled by default for sercuity reasons. While this is enabled by default during development in `yarn start`, it needs to be explicitly enabled in the production HTTP server by allowing COOP (Cross Origin Opener Policy) and COEP (Cross Origin Embedder Policy). For Netlify, this is done in the [`netlify.toml`](./netlify.toml).
