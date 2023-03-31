# Project Cube Collection Pages

## Build

### Prepare

1. Read [*Unofficial Bevy Cheat Book*](https://bevy-cheatbook.github.io/platforms/wasm.html) and install `wasm32-unknown-unknown`.
   - Using: `rustup target install wasm32-unknown-unknown`.
2. Make a `out` folder.
3. (Optional) Download `wasm-opt.exe` and put it into `out`.

### Generate ASM file

1. Checkout `bevy` branch: `git checkout bevy`.
2. Build wasm target: `cargo build --release --target wasm32-unknown-unknown`.
3. Build wasm file: `wasm-bindgen --out-name cube-collection --out-dir out --target web .\target\wasm32-unknown-unknown\release\cube-collection.wasm`.
4. (Optional) Optimize size: `.\out\wasm-opt.exe -Oz --output .\out\cube-collection_bg.wasm .\out\cube-collection_bg.wasm`.

## Miscellaneous

Upgrade tools:

```powershell
rustup update
```

Update dependent libraries:

```powershell
cargo update
```
