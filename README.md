# Snake
<!-- Rust -->
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)

A Snake game written in Rust with [Macroquad](https://macroquad.rs/).

## Run locally

Run the native game with:

```bash
cargo run
```

Build the portable static site with:

```bash
./scripts/build-web.sh
```

Serve `dist/` through a local HTTP server. Opening `dist/index.html` directly is not supported because browsers restrict WASM module loading from `file://` URLs.

For example:

```bash
python3 -m http.server --directory dist 8000
```

Open <http://localhost:8000> in a browser and voala.

