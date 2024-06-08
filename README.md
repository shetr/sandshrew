# Sandshrew

Yet another sand game. Heavily inspired by [Sandspiel](https://github.com/MaxBittker/sandspiel) and [Noita](https://noitagame.com/).

Build wasm:

```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name sandshrew --out-dir www/target --target web target/wasm32-unknown-unknown/release/sandshrew.wasm
```