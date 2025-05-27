# totorosay

![Build](https://github.com/nicomyerr/totorosay/workflows/Rust%20CI%20workflow/badge.svg)
[![License](https://img.shields.io/badge/License-MIT-a31f34)](LICENSE)

cowsay but with totoro

![totorosay example](./docs/totorosay-example.png)

## Usage

Open your favourite Terminal and type:

```bash
totorosay Hello world!
```

And Totoro says `Hello world!`.

Use the `-b` or `--big` flag to use **big** Totoro:

![totorosay example](./docs/totorosay-big-hello-world.png)

## Installation

TODO: crates.io and GitHub release

### From Source

Building from source requires Rust 1.85.0 or higher (see `rust-version` in [Cargo.toml](Cargo.toml)).

Then you can build it with `cargo`:

```shell
cargo install --locked --path .
```
