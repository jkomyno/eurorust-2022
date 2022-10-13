# No free lunch: Limits of Wasm as a bridge from Rust to JS

> Accompanying code for the talk I presented at EuroRust 2022 in Berlin, Germany

<p>
  <a href="https://github.com/jkomyno/react-native-user-inactivity/blob/master/LICENSE">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" target="_blank" />
  </a>
</p>

Slides for this talk are also available [here](http://jkomyno-eurorust-2022.vercel.app/).

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
- `cargo install -f wasm-bindgen-cli@0.2.83` (the version is important, as `wasm-bindgen-cli` doesn't yet follow semantic versioning. This version needs to match the version of the `wasm-bindgen` crate`)

In [`./nodejs`](./nodejs):

- `npm install`

### Build

In [`./rust`](./rust):

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

In [`./rust`](./rust):

- `demo-cli`: a CLI binary that uses the `rlib` libraries of the other demo crates to parse and validate schemas, trigger example panics, and showing serialized data structures with different libraries.

- `demo-serde-wasm`: library that defines a set of data structures to be accessed in Node.js via WebAssembly. It uses `serde` to serialize the data structures.

- `demo-tsify-wasm`: library that defines a set of data structures to be accessed in Node.js via WebAssembly. It uses `tsify` to serialize the data structures.

- `playground-wasm`: library that showcases examples of data structures to be accessed in Node.js via WebAssembly. It uses `wasm-bindgen` to serialize the data structures.

- `schema-parser`: library that defines a parser and a validator for a simple schema language inspired by [prisma](https://prisma.io). The data structures for the schema AST (Abstract Syntax Tree) are optionally serialized with `wasm-bindgen` + `tsify` via the `wasm` feature flag. `schema-parser` uses `nom` to parse the input.

- `schema-parser-wasm`: WebAssembly bindings for the `schema-parser` library (which is installed with the `wasm` feature flag). It uses `serde-json` to convert custom structs in a JavaScript error when needed.

## Demo

As a demonstration of defining reasonably complex in Rust and consuming it in Node.js via WebAssembly,
we will use a parser ([`schema-parser`](./rust/schema-parser)) for a simple schema language inspired by [prisma](https://prisma.io). Here's an example of a [schema](./prisma/schema.prisma):

```prisma
datasource db {
  provider = "postgres"
  url = env("DATABASE_URL")
  shadowDatabaseUrl = "postgres://optional-url"
}
```

This translates to the following value in Rust:

```rust
let ast = SchemaAST {
  datasources: vec!(
    Datasource::Db(
      DatasourceDb {
        provider: Provider::Postgres,
        url: Url::Env(
          String::from("DATABASE_URL"),
        ),
        shadow_database_url: Some(
          Url::Static(
            String::from("postgres://optional-url"),
          ),
        ),
      },
    ),
  ),
}
```

and we expect the same AST to be defined as the following for TypeScript:

```typescript
/* type definitions */

export type Provider = "postgres" | "cockroachdb" | "mysql" | "mariadb" | "sqlserver" | "sqlite" | "mongodb"

export type Url
  = { _tag: 'static', value: string } // => Static(String)
  | { _tag: 'env', value: string }    // => Env(String)

export type DatasourceDb = {
  provider: Provider
  url: Url
  shadowDatabaseUrl: Url | null
}

export type Datasource
  = { _tag: 'db', value: DatasourceDb }

export type SchemaAST = {
  datasources: Datasource[]
}

/* AST value */

const ast: SchemaAST = {
  datasources: [
    {
      _tag: 'db',
      value: {
        provider: 'postgres',
        url: {
          _tag: 'env',
          value: 'DATABASE_URL',
        },
        // shadowDatabaseUrl can be null 
        shadowDatabaseUrl: {
          _tag: 'static',
          value: 'postgres://optional-url',
        },
      },
    },
  ]
}
```

Let's see how schema parsing a validation work in Rust and Node.js.

### Schema parsing/validation in Rust

- cd `./rust`
- Parse the schema with:

```console
cargo run -p demo-cli -- parse --schema ../prisma/schema.prisma
```

- We expect the following output:

```console
Parsing schema...
Schema parsed successfully!
```

- Validate the schema with:

```console
cargo run -p demo-cli -- validate --schema ../prisma/schema.prisma
```

- We expect the following output:

```console
Parsing schema...
Schema parsed successfully!

Validating AST...
thread 'main' panicked at 'Environment variables are not yet supported for database URLs.', schema-parser/src/validate/validator.rs:52:20
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- We have a panic, but that's expected when using `env` URLs in the schema.
Let's change the [schema](./prisma/schema.prisma) to use to `static` URLs:

```diff
datasource db {
  provider = "postgres"
-   url = env("DATABASE_URL")
+   url = "static-url"
  shadowDatabaseUrl = "postgres://optional-url"
}
```

- Validate the schema with:

```console
cargo run -p demo-cli -- validate --schema ../prisma/schema.prisma
```

- We expect the following output:

```console
Parsing schema...
Schema parsed successfully!

Validating AST...
[rust:error]: Diagnostics { errors: ["\"postgres\" URLs must start with postgres://, received static-url"] }
Error: Custom { kind: InvalidData, error: "AST validation failed" }
```

- We now have a managed error that tells us we should use the `postgres://` protocol in URLs when the datasource db provider is `postgres`. Let's fix that in the [schema](./prisma/schema.prisma):

```diff
datasource db {
  provider = "postgres"
-   url = "static-url"
+   url = "postgres://static-url"
  shadowDatabaseUrl = "postgres://optional-url"
}
```

- We expect the following output:
```console
Parsing schema...
Schema parsed successfully!

Validating AST...
AST validated successfully!
```

We have both parsed and validated the schema successfully in Rust.
Let's now see how the same logic works in Node.js when imported via WebAssembly.

### Schema parsing/validation in Node.js via Wasm

- cd `./nodejs`
- Parse a predefined schema defined as

```prisma
datasource db {
  provider = "postgres"
  url = env("DATABASE_URL")
}
```

with:
```console
npx ts-node ./src/parse-schema.ts
```

- We expect the following output:
```console
Parsing schema...

Schema parsed successfully:

{
  "datasources": [
    {
      "_tag": "db",
      "value": {
        "provider": "postgres",
        "url": {
          "_tag": "env",
          "value": "DATABASE_URL"
        },
        "shadowDatabaseUrl": null
      }
    }
  ]
}
```

- Now it's time for validation. We have 3 predefined schemas:
  1. One that is valid:

```prisma
datasource db {
  provider = "sqlite"
  url = "file:./dev.db"
}
```

  2. One that results in a managed error:

```prisma
datasource db {
  provider = "cockroachdb"
  url = "postgres://jkomyno:prisma@localhost:5432"
}

datasource db {
  provider = "postgres"
  url = "mysql://jkomyno:prisma@localhost:5432"
}
```

  3. One that results in a panic:

```prisma
datasource db {
  provider = "cockroachdb"
  url = env("DATABASE_URL")
}
```

- Let's validate the first "success-case" schema with:

```console
npx ts-node ./src/validate-ast.ts success
```
- We expect the following output:

```console
Validating AST...

AST validated successfully!
```

- Let's validate the second "error-case" schema with:

```console
npx ts-node ./src/validate-ast.ts error
```

- We expect the following output:

```console
Validating AST...

[node:error] {
  errors: [
    `The provider "cockroachdb" is not yet supported. Supported providers are: '"sqlite"', '"postgres"'`,
    '"postgres" URLs must start with postgres://, received mysql://jkomyno:prisma@localhost:5432',
    'You defined more than one datasource. This is not supported yet.'
  ]
}

- Let's validate the third "error-case" schema with:

```console
npx ts-node ./src/validate-ast.ts panic
```

- We expect the following output:

```console
Validating AST...

[node:panic] RuntimeError: unreachable
    at __rust_start_panic (wasm://wasm/000e0f2a:wasm-function[485]:0x2a060)
    at rust_panic (wasm://wasm/000e0f2a:wasm-function[331]:0x295ba)
    at std::panicking::rust_panic_with_hook::hb09154fa23e06c37 (wasm://wasm/000e0f2a:wasm-function[233]:0x26c54)
    at std::panicking::begin_panic_handler::{{closure}}::h6091c197f0d08bf0 (wasm://wasm/000e0f2a:wasm-function[252]:0x27965)
    at std::sys_common::backtrace::__rust_end_short_backtrace::h004afb3e6a867c40 (wasm://wasm/000e0f2a:wasm-function[378]:0x29b34)
    at rust_begin_unwind (wasm://wasm/000e0f2a:wasm-function[317]:0x29316)
    at core::panicking::panic_fmt::h9e229748e3ae9f9d (wasm://wasm/000e0f2a:wasm-function[319]:0x29397)
    at schema_parser::validate::validator::validate_url::h660177b70e41ab86 (wasm://wasm/000e0f2a:wasm-function[59]:0x1578b)
    at schema_parser::validate::validator::validate_configuration::h25007314e5f4055e (wasm://wasm/000e0f2a:wasm-function[52]:0x13dbb)
    at schema_parser::validate_ast::h3ba8da369b9e2bb4 (wasm://wasm/000e0f2a:wasm-function[186]:0x240f5)
```

As we can see, we lose information on the original message passed to `panic!`, and the stacktrace is not very helpful.

## 👤 Author

**Alberto Schiabel**

* Twitter: [@jkomyno](https://twitter.com/jkomyno)
* Twitter: [@jkomyno](https://github.com/jkomyno)

## 📝 License

Built with ❤️ by [Alberto Schiabel](https://github.com/jkomyno).
This project is [MIT](https://github.com/jkomyno/eurorust-2022/blob/main/LICENSE) licensed.
