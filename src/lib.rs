// Copyright 2019-present, OVH SAS
// All rights reserved.
//
// This OVH Software is licensed to you under the MIT license <LICENSE-MIT
// https://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

//! Bundle of tools to use with serde_value.
//!
//! # Quickstart
//!
//! You can start using it by first adding it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! serde = "1.0"
//! serde_derive = "1.0"
//! serde_value_utils = "0.1"
//! ```
//!
//! Then, create a structure which implement the `serde::Serialize` trait and use it with any
//! serde lib.
//!
//! # Example: to_flatten_maptree
//!
//! ```rust
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_json;
//! extern crate serde_value_utils;
//!
//! #[derive(Serialize, Clone, Debug)]
//! struct SubFoo {
//!     a: String,
//!     b: u64,
//! }
//!
//! #[derive(Serialize, Clone, Debug)]
//! struct Foo {
//!     a: String,
//!     b: f64,
//!     c: Vec<i8>,
//!     d: SubFoo,
//! }
//!
//! fn main() {
//!     let foo = Foo { a: "test".into(), b: 0.5, c: vec![5, 9], d: SubFoo { a: "subtest".into(), b: 695217 } };
//!     let ser = serde_value_utils::to_flatten_maptree("_", Some("_"), &foo).unwrap();
//!     println!("{}", serde_json::to_string_pretty(&ser).unwrap());
//! }
//! ```
//! **Output**:
//! ```json
//!  {
//!   "_a": "test",
//!   "_b": 0.5,
//!   "_c_0": 5,
//!   "_c_1": 9,
//!   "_d_a": "subtest",
//!   "_d_b": 695217
//! }
//! ```
//! ## Feature with-schema
//!
//! The feature `with-schema` allow to suffix fields names to suits to the
//! [LDP naming conventions](https://docs.ovh.com/fr/logs-data-platform/field-naming-conventions/).
//!
//! In your `Cargo.toml`, set:
//!
//! ```toml
//! [dependencies]
//! serde_value_utils = { version = "0.1", features = ["with-schema"] }
//! ```
//!
//! Re-run the previous example, and now the output will be :
//!
//! ```json
//! {
//!   "_a": "test",
//!   "_b_float": 0.5,
//!   "_c_0_long": 5,
//!   "_c_1_long": 9,
//!   "_d_a": "subtest",
//!   "_d_b_double": 695217
//! }
//! ```
//! # Example: try_detect_type
//!
//! ```rust
//! extern crate serde_value_utils;
//!
//! use serde_value_utils::try_detect_type;
//!
//! fn main() {
//!     println!("{:?}", try_detect_type("6.5"));
//! }
//! ```
//! **Output**:
//! ```
//! F64(6.5)
//! ```
#![doc(
html_logo_url = "https://eu.api.ovh.com/images/com-square-bichro.png",
html_favicon_url = "https://www.ovh.com/favicon.ico",
)]
#![deny(warnings, missing_docs)]
extern crate serde;
extern crate serde_value;

pub use detect::try_detect_type;
pub use flatten::to_flatten_maptree;

mod flatten;
mod detect;
