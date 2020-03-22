# Nuuro Game Template
This is the basic boilerplate for generating a
[nuuro](https://crates.io/crates/nuuro) project with targets on desktop and web
(`wasm32-unknown-unknown target`).

### Building

On desktop (Windows, Linux, MacOS) simply run:

```bash
cargo build --release
```

For web target, run:

```bash
cargo build --target=wasm32-unknown-unknown
```

### Running

On desktop (Windows, Linux, MacOS) simply run:

```bash
cargo run --release
```

For web target, first build with the instructions above and serve the html
folder with a static server. For example, if you have installed node, run:

```bash
npx serve html
```
