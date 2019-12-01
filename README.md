Python to C Compiler (ptc)
--------------------------
ptc is a source-to-source compiler for converting Python to C code. It aims to
allow converted code to be dropped into a project as if it had been
handwritten. It is written using the [Rust](https://www.rust-lang.org/)
programming language.

## Installation

The compiler can be installed by cloning the respository and using `cargo`:

```bash
git clone git@github.com:alexander-jackson/ptc.git
cd ptc/
cargo install --path .
```

## Usage

The compiler expects input as a file, using the `--filename` flag:

```bash
ptc --filename code.py
```
