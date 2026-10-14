## Setting up the environment

This repository is a standard [Cargo](https://doc.rust-lang.org/cargo/) package. Install Rust 1.85 or later with [rustup](https://rustup.rs), then run:

```sh
./scripts/bootstrap
```

## Modifying or adding code

Most of the SDK is generated code, and changes to generated files will be overwritten by the next generation, unless they're sealed as custom code. The generator never modifies `src/lib/` or `examples/`, so they're good places for hand-written code and examples.

## Running tests and lints

```sh
./scripts/test
./scripts/lint
```

`./scripts/format` fixes formatting.

## Running examples

Add examples to `examples/` and run them with:

```sh
cargo run --example <name>
```
