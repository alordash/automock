# rsubstitute

Library for mocking static functions, traits and structures in Rust.

[![Build Status](https://github.com/alordash/rsubstitute/actions/workflows/ci.yml/badge.svg)](https://github.com/alordash/rsubstitute/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/rsubstitute.svg)](https://crates.io/crates/rsubstitute)
[![Documentation](https://docs.rs/rsubstitute/badge.svg)](https://docs.rs/rsubstitute)

## Overview

This library exposes `mock` attribute that generates all infrastructure required for creating mocks and an API for their
configuration. No changes for source code are needed (apart from adding `#[cfg_attr(test, mock)]` attribute).

## Usage

Add `rsubsitute` to your `dev-dependencies`:

```toml
[dev-dependencies]
rsubsitute = "0.1.6"
```

Import `rsubstitute::*` and apply `mock` attribute on your function, trait, structure, or `impl` block.  
Here's an example of how to test function `use_trait` using `Trait` mock:

```rust
#[cfg(test)]
use rsubstitute::*;

#[cfg_attr(test, mock)]
trait Trait {
    fn work(&self, v: i32) -> i32;
}

fn use_trait(t: &dyn Trait, v: i32) -> i32 {
    t.work(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trait_test() {
        // Arrange
        let mut mock = TraitMock::new();
        mock.setup().work(10).returns(20);

        // Act
        let result = use_trait(&mock, 10);

        // Assert
        assert_eq!(result, 20);
        mock.received().work(10, 1.time());
    }
}
```

For more information about features and caveats of `rsubstitute` refer to
[crate documentation](https://docs.rs/rsubstitute).

# Minimum Supported Rust Version (MSRV)

`rsubstitute` is supported on Rust 1.88.0 and higher. `rsubstitute`'s MSRV will not be changed in the future without
bumping the major or minor version.

# License

`rsubstitute` is distributed under the terms of MIT license. See [license.txt](license.txt) for details.

# Acknowledgements

`rsubstitute` was heavily inspired by two mocking libraries: [mockall](https://github.com/asomers/mockall) (Rust)
and [NSubstitute](https://github.com/nsubstitute/NSubstitute) (C#). A lot of documentation and features were based on
`mockall` (including this README's structure). Errors format and API structure were basically borrowed from
`NSubstitute` (as well as name).