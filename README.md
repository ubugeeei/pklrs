# pklrs

Rust bindings for [Apple Pkl](https://pkl-lang.org/) — a configuration-as-code language.

This library communicates with `pkl server` via MessagePack IPC, providing a native Rust interface to evaluate Pkl modules and deserialize results into Rust types through serde.

## Crates

| Crate | Description |
|-------|-------------|
| `pkl` | Core library — IPC protocol, pkl-binary decoder, serde Deserializer |
| `pkl-derive` | `#[derive(FromPkl)]` and `pkl!` proc-macros |
| `pkl-codegen` | CLI tool (`pkl-gen-rust`) to generate Rust structs from `.pkl` schemas |

## Quick Start

```toml
[dependencies]
pkl = "0.1"
serde = { version = "1", features = ["derive"] }
```

```rust
use pkl::{EvaluatorManager, EvaluatorOptions, ModuleSource};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Server {
    host: String,
    port: i64,
}

fn main() -> Result<(), pkl::Error> {
    let mut manager = EvaluatorManager::new()?;
    let evaluator = manager.new_evaluator(EvaluatorOptions::preconfigured())?;

    let source = ModuleSource::text(r#"
        host = "localhost"
        port = 8080
    "#);

    let server: Server = manager.evaluate_module_typed(&evaluator, source)?;
    println!("{server:?}"); // Server { host: "localhost", port: 8080 }

    manager.close_evaluator(&evaluator)?;
    Ok(())
}
```

### `pkl!` macro

Write PKL configuration as tokens directly in Rust — no string literals needed:

```rust
use pkl::pkl;

fn main() -> Result<(), pkl::Error> {
    let value = pkl! {
        host = "localhost"
        port = 8080
        database {
            url = "postgres://localhost/mydb"
            maxConnections = 10
        }
    }?;

    println!("{value:#?}");
    Ok(())
}
```

The macro supports most PKL constructs:

```rust
let value = pkl! {
    // Classes and type annotations
    class Server {
        host: String
        port: UInt16
    }

    // Functions
    function double(x) = x * 2

    // Modifiers
    local basePort = 8000

    // Duration and data size units
    timeout = 30.s
    maxSize = 512.mb

    // Control flow
    port = if (debug) 3000 else 8080

    // Operators
    items = data |> filter(it > 0)
    name = input ?? "default"
    valid = value is String
}?;
```

> **Limitations:** PKL raw strings (`#"..."#`), string interpolation (`\(expr)`),
> and multi-line strings (`"""..."""`) are not supported in the macro.
> Use `ModuleSource::text(...)` with a raw string literal for these cases.

You can also evaluate `.pkl` files directly:

```rust
let source = ModuleSource::file("config.pkl");
```

Or evaluate a specific expression:

```rust
let value = manager.evaluate_expression(&evaluator, source, Some("output.host"))?;
```

## Prerequisites

The `pkl` CLI must be installed and available on your `PATH`.

```sh
# macOS
brew install pkl

# or download from https://pkl-lang.org
```

## Code Generation

Generate Rust types from Pkl schema files:

```sh
cargo install pkl-codegen
pkl-gen-rust schema.pkl -o src/generated/
```

## License

MIT
