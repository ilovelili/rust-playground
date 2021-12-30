# Cargo

## Create a new project using Cargo

```bash
cargo new hello_cargp
```

### Can assign source control in cargo (default is git)

```bash
cargo new --vcs=git
```

## Build with cargo

```bash
cargo build
```

This command creates an executable file in `target/debug/`

## Build and run with Cargo

```bash
cargo run
```

## Validate the code without compile into binary

```bash
cargo check
```

## Build for release

```bash
cargo build --release
```

release build will do some optimization while the compile time is longer than normal `cargo build`
