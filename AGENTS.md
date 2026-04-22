# AGENTS.md

## Workspace

Two-crate Cargo workspace (`resolver = "2"` for wgpu/Bevy):

- **`cube-core`** — pure Rust game logic library (no dependencies). LGPL 3.0.
- **`cube-collection`** — Bevy 0.18.1 frontend, depends on `cube-core`. MIT.

## Commands

```sh
cargo run --release -p cube-collection   # run the game (--release needed; Bevy unusably slow in debug)
cargo test -p cube-core                   # all tests live here (inline in cube-core/src/lib.rs)
cargo test                                # workspace-wide (equivalent; cube-collection has no tests)
```

The `-p` flag is required; this is a workspace, not a single crate.

## Dev profile optimizations

`Cargo.toml` sets `opt-level = 1` for dev and `opt-level = 3` for dev dependencies — intentional for Bevy. Do not remove.

## WASM build & gh-pages deploy

Full workflow from `current` branch to published `gh-pages`:

```sh
# 1. Build (on current branch)
cargo build --profile wasm-release --target wasm32-unknown-unknown -p cube-collection

# 2. Generate JS/WASM bindings
wasm-bindgen --out-name cube-collection --out-dir out --target web target/wasm32-unknown-unknown/release/cube-collection.wasm

# 3. Optimize WASM binary
wasm-opt -Oz --output out/cube-collection_bg.wasm out/cube-collection_bg.wasm

# 4. Switch to gh-pages and deploy
git checkout gh-pages
cp out/cube-collection.js out/cube-collection_bg.wasm out/cube-collection_bg.wasm.d.ts out/cube-collection.d.ts .
git checkout current -- cube-collection/assets    # extract assets into working tree
mv cube-collection/assets assets                  # move to root (gh-pages layout)
rm -rf cube-collection                            # clean up
git add -A && git commit -m "feat: upgrade to bevy X.Y.Z"
git push
git checkout current
```

Key constraints:

- wasm-bindgen CLI version must match the `wasm-bindgen` crate version exactly, else build fails. Install matching version: `cargo install wasm-bindgen-cli --version <crate_version>`
- The build uses a custom `wasm-release` profile (`lto = "fat"`, `opt-level = 'z'`) defined in workspace `Cargo.toml` for smaller WASM output. The `out/` dir path under `target/` will be `release/` (it inherits from release).
- `gh-pages` branch holds only: `index.html`, `cube-collection.js`, `cube-collection_bg.wasm`, `*.d.ts`, and `assets/`. Do not commit build tools, Rust source, or `server.ps1` there.
- `fit_canvas_to_parent: true` is set in `main.rs` for responsive WASM canvas; the HTML must have `html/body` at 100% width/height with no margin.

## Levels

Level files are TOML in `cube-collection/assets/level/`. Adding a level requires:

1. Create the `.toml` file in that directory.
2. Add the filename (without extension) to `name_list` in `cube-collection/assets/level/index.toml`.

Invalid level files cause the game to stop loading and log an error.

## Architecture

- `cube-core`: `Seed` → `CubeCore` (game state machine). `commit(movement)` advances state, returns `Diff`s. `remake()` is undo/retry.
- `cube-collection/src/main.rs` → `plugin::ScenePlugin` (Bevy plugin). Plugins in `src/plugin/` handle loading, scene, and shapes.
- Cube colors are defined in `scene_plugin/common/style.rs::cube_color()`.
