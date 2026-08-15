# Snake

A Snake game written in Rust with Macroquad.

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

Open <http://localhost:8000> in a browser.

## Deploy to GitHub Pages

The workflow in `.github/workflows/deploy.yml` builds the WASM bundle and deploys it whenever `main` changes.

In the repository settings, set **Pages > Build and deployment > Source** to **GitHub Actions**. The deployed site will be available at:

```text
https://<owner>.github.io/<repository>/
```

The generated `dist/` directory is a portable static bundle and can also be published by another static host later.
