# bit-tower

A frontend for QBitTorrent written in Rust with Leptos.

Leptos is a web front-end framework for Rust that is architecturally similar to SolidJS, based on reactive signals. It's well suited for web applications that need to make frequent fine-grained updates to the UI.

## Running

```bash
cargo leptos watch
```

## Compiling for Release
```bash
cargo leptos build --release
```

## Testing
```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

