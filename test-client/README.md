## Link Package (For Development)

```bash
cd public/pkg
yarn link

cd ../..
yarn link halo2_playground
```

## Add Local Package (For Release w/o NPM)

```bash
yarn add ./public/pkg
```

## Reflect changes made in rust and start react app

```bash
yarn dev
```
