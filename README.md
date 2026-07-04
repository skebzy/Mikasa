# Mikasa CLI

A small command‑line tool for scaffolding front‑end projects.

## What it does
- Creates a **Next.js** or **React (Vite)** project.
- Detects the available package manager (bun, pnpm, npm) and uses it.
- Normalises the project name to lowercase to avoid issues on the scaffolding scripts.
- Uses the modern `npm create` command, avoiding deprecated `npx` usage.

## Available commands
```
mikasa          # interactive mode – choose framework and options
mikasa next     # scaffold a Next.js project (prompts for name, TS, Tailwind, etc.)
mikasa react    # scaffold a React (Vite) project (same options)
```

## Prerequisites
- Rust toolchain (stable) – install via https://rustup.rs
- Node.js (>= 18) with npm (or bun/pnpm) on the PATH

## Compilation / installation
```bash
# Build the binary
cargo build --release
# Install locally (adds `mikasa` to Cargo bin directory)
cargo install --path .
```
Or run without installing:
```bash
cargo run --
```

## Usage example
```bash
# Create a Next.js project named "example"
mikasa
# Follow the prompts for package manager, TypeScript, Tailwind, etc.
```

## Development helpers
```bash
# Run tests (if any)
cargo test
# Run the binary in development mode with help output
cargo run -- --help
```

## License
MIT © 2026 Skebzy
