# serde-value-utils


[![Build Status](https://travis-ci.org/cdumay/rust-serde-value-utils.svg?branch=master)](https://travis-ci.org/cdumay/rust-serde-value-utils) 
[![Latest version](https://img.shields.io/crates/v/serde-value-utils.svg)](https://crates.io/crates/serde-value-utils)
[![Documentation](https://docs.rs/serde-value-utils/badge.svg)](https://docs.rs/serde-value-utils) 
![License](https://img.shields.io/crates/l/serde-value-utils.svg)

Bundle of tools to use with serde_value.

## Quickstart

You can start using it by first adding it to your `Cargo.toml`:

```toml
[dependencies]
serde = "1.0"
serde_derive = "1.0"
serde_value_utils = "0.1"
```

Then, create a structure which implement the `serde::Serialize` trait and use it with any
serde lib.

## Example: to_flatten_maptree

```rust
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_value_utils;

#[derive(Serialize, Clone, Debug)]
struct SubFoo {
    a: String,
    b: u64,
}

#[derive(Serialize, Clone, Debug)]
struct Foo {
    a: String,
    b: f64,
    c: Vec<i8>,
    d: SubFoo,
}

fn main() {
    let foo = Foo { a: "test".into(), b: 0.5, c: vec![5, 9], d: SubFoo { a: "subtest".into(), b: 695217 } };
    let ser = serde_value_utils::to_flatten_maptree("_", Some("_"), &foo).unwrap();
    println!("{}", serde_json::to_string_pretty(&ser).unwrap());
}
```
**Output**:
```json
 {
  "_a": "test",
  "_b": 0.5,
  "_c_0": 5,
  "_c_1": 9,
  "_d_a": "subtest",
  "_d_b": 695217
}
```
### Feature with-schema

The feature `with-schema` allow to suffix fields names to suits to the
[LDP naming conventions](https://docs.ovh.com/fr/logs-data-platform/field-naming-conventions/).

In your `Cargo.toml`, set:

```toml
[dependencies]
serde_value_utils = { version = "0.1", features = ["with-schema"] }
```

Re-run the previous example, and now the output will be :

```json
{
  "_a": "test",
  "_b_float": 0.5,
  "_c_0_long": 5,
  "_c_1_long": 9,
  "_d_a": "subtest",
  "_d_b_double": 695217
}
```
## Example: try_detect_type

```rust
extern crate serde_value_utils;

use serde_value_utils::try_detect_type;

fn main() {
    println!("{:?}", try_detect_type("6.5"));
}
```
**Output**:
```rust
F64(6.5)
```



License: BSD-3-Clause
