## gprobe

[![build](https://github.com/refcell/gprobe/actions/workflows/test.yml/badge.svg)](https://github.com/refcell/gprobe/actions/workflows/test.yml)
[![license: MIT](https://img.shields.io/badge/license-MIT-brightgreen.svg)](https://opensource.org/licenses/MIT)
[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/gprobe.svg
[crates-url]: https://crates.io/crates/gprobe

A verbose CLI to probe go-ethereum data structures, built in rust.

### Quickstart

_Prerequisites: Install Rust: https://www.rust-lang.org/tools/install_

Install as a global command:

```bash
cargo install --git github.com/refcell/gprobe --branch main
```

Then, you can run `gprobe` from anywhere. Alternatively, you can run `cargo run` from the project root or build the project with `cargo build`.

To use gprobe as a library, add the following to your `Cargo.toml`:

```toml
gprobe = { git = "https://github.com/refcell/gprobe", branch = "main" }
```

### Reference

```bash
grpobe 0.1.2
A verbose CLI to probe go-ethereum data structures.

USAGE:
    gprobe [OPTIONS] [SOURCE] [SUBCOMMAND]

ARGS:
    <SOURCE>    The data source to probe

OPTIONS:
    -h, --help       Print help information
    -p, --print      Prints out to the terminal
    -v, --verbose    Verbose output
    -V, --version    Print version information

SUBCOMMANDS:
    create        Create a new database
    decompress    Decompress a Tarball
    help          Print this message or the help of the given subcommand(s)
    tree          Traverse a database
```

### Documentation

`gprobe` exposes a number of useful utilities. The `tree` subcommand traverses a database and prints out the hex-encoded keys and values.


### Contributing

All contributions are welcome. Before opening a PR, please submit an issue detailing the bug or feature. When opening a PR, please ensure that your changes build with nightly rust, has been linted with `cargo fmt`, and contains tests when applicable.

### License

[MIT](https://choosealicense.com/licenses/mit/)

### Disclaimer

_This code is being provided as is. No guarantee, representation or warranty is being made, express or implied, as to the safety or correctness of the code. It has not been audited and as such there can be no assurance it will work as intended, and users may experience delays, failures, errors, omissions or loss of transmitted information. Nothing in this repo should be construed as investment advice or legal advice for any particular facts or circumstances and is not meant to replace competent counsel. It is strongly advised for you to contact a reputable attorney in your jurisdiction for any questions or concerns with respect thereto. The creators are not liable for any use of the foregoing, and users should proceed with caution and use at their own risk._

