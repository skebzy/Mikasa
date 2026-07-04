# Mikasa CLI

A small command-line tool for scaffolding front‑end projects.

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

## Installation
```bash
# Build and install locally
cargo install --path .
```
Or run directly without installing:
```bash
cargo run --
```

## Usage example
```bash
# Create a Next.js project named "example"
mikasa
# (follow the prompts for package manager, TypeScript, Tailwind, etc.)
```

## Development
```bash
# Run tests (if any)
cargo test
# Run the binary in development mode
cargo run -- --help
```
