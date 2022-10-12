# No free lunch: Limits of Wasm as a bridge from Rust to JS

> Accompanying code for the talk I presented at EuroRust 2022 in Berlin, Germany

## Abstract

Although Rust code can supposedly run in JS runtimes via WebAssembly, concepts like serialization, panic handling, and type-safety aren’t supported out of the box. Using a parser written in Rust and consumed by Node.js, we’ll discuss limitations and alternatives to guide you through a Wasm project.

Traditionally, Node.js has delegated the task of writing complex CPU-intensive logic to C++, but the increasing adoption of Rust and WebAssembly has led to a paradigm shift. In fact, Rust code can be compiled to WASM and be imported in a JavaScript (or even TypeScript) source file - for instance, [Seed](https://github.com/seed-rs/seed) and [Prisma](https://github.com/prisma/prisma) follow this approach -, but that doesn’t come without limitations and gotchas, even for relatively small projects that abide to standard Rust patterns. From silenced warnings and obfuscated panic errors to structs that cannot be serialized and typed automatically, the path of porting Rust code to a JS app in production is a steep one, even when we don’t consider the I/O limitations that WASI should address.

In this presentation, we will look at a language parser written in Rust, compiled with [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) and interpreted by Node.js, observing some limitations for production use cases and discussing alternatives. There’s no free lunch: WebAssembly - albeit useful - comes with its own set of challenges, and I trust this talk will help you navigate potential issues before they impact your deadlines.

## Get Started

### Requirements

- `cargo@1.64.0` or superior
- `nodejs@18.8.0` or superior

### Install Dependencies

- `cargo update -p wasm-bindgen`
- `cargo install -f wasm-bindgen-cli@0.2.83` (this version is important, as `wasm-bindgen-cli` doesn't yet follow semantic versioning. This version needs to match the version of the `wasm-bindgen` crate`)

In [`./nodejs`]:

- `npm install`

### Build

In [`./rust`]:

- Run unit tests:
```
cargo test
```

- Build the `rlib` libraries and the demo CLI binary:
```
cargo build --release
````

- Build the `cdylib` libraries in the [`./nodes/src/wasm`](./nodes/src/wasm) folder:

```
`./scripts/wasm_all.sh`
```

## What's in this repository

In [`./rust`](rust):

- `demo-cli`: a CLI binary that uses the `rlib` libraries of the other demo crates to parse and validate schemas, trigger example panics, and showing serialized data structures with different libraries.

- `demo-serde-wasm`: library that defines a set of data structures to be accessed in Node.js via WebAssembly. It uses `serde` to serialize the data structures.

- `demo-tsify-wasm`: library that defines a set of data structures to be accessed in Node.js via WebAssembly. It uses `tsify` to serialize the data structures.

- `playground-wasm`: library that showcases examples of data structures to be accessed in Node.js via WebAssembly. It uses `wasm-bindgen` to serialize the data structures.

- `schema-parser`: library that defines a parser and a validator for a simple schema language inspired by [prisma](https://prisma.io). The data structures for the schema AST (Abstract Syntax Tree) are optionally serialized with `wasm-bindgen` + `tsify` via the `wasm` feature flag. `schema-parser` uses `nom` to parse the input.

- `schema-parser-wasm`: WebAssembly bindings for the `schema-parser` library (which is installed with the `wasm` feature flag). It uses `serde-json` to convert custom structs in a JavaScript error when needed.
