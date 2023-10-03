# arg_attr

[![Crates.io](https://img.shields.io/crates/v/arg_attr)](https://crates.io/crates/arg_attr)
[![Downloads](https://img.shields.io/crates/d/arg_attr.svg)](https://crates.io/crates/arg_attr)
[![Documentation](https://docs.rs/arg_attr/badge.svg)](https://docs.rs/arg_attr)
[![License](https://img.shields.io/crates/l/arg_attr)](https://crates.io/crates/arg_attr)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/arg_attr/status.svg)](https://deps.rs/repo/github/JohnScience/arg_attr)

> Specify the accepted arguments depending on configuration conditional checks.

## Examples

Simple yet silly example:

```rust
#![allow(dead_code)]

use arg_attr::args;

#[args(_url: String)]
fn silly_drop() {
    drop(_url);
}
```

In the example above, `args` attribute unconditionally specifies the arguments for the `silly_drop` function. On it's own it is fairly useless, but it can be used in combination with [`#[cfg_attr(...)]`](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute) for conditional compilation.

```rust,ignore
use pyo3::prelude::Python;
use qualifier_attr::qualifiers;
use arg_attr::args;

// At the moment of writing, `pyo3` does not support async functions.
// https://github.com/PyO3/pyo3/issues/1632
#[cfg_attr(feature="python", args(py: Python<'_>, url: &'str))]
#[cfg_attr(not(feature="python"), args(url: &'str), qualifiers(async))]
fn fetch() {
    todo!()
}
```

## Related crates

* [`qualifier_attr`](https://crates.io/crates/qualifier_attr).
* [`const_fn`](https://crates.io/crates/const_fn).
* [`cfg_if`](https://crates.io/crates/cfg-if).
