# onehot & onehot-derive

Rust crates for one-hot encoding structs and enums. Provides a trait and a proc macro for
auto deriving.

[![Cargo](https://img.shields.io/crates/v/onehot.svg)](https://crates.io/crates/onehot)
[![Documentation](https://docs.rs/onehot/badge.svg)](https://docs.rs/onehot)

## Features

- Generic trait for encoding arbitrary (including composite) types.
- Proc macro for auto deriving.
- Optional [bitmatrix](https://docs.rs/bitmatrix) support, including vertical and
  horizontal encoding.
- No unsafe code.

## Licence

`onehot` is licenced under the [MIT Licence](http://opensource.org/licenses/MIT).
