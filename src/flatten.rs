// Copyright 2019-present, OVH SAS
// All rights reserved.
//
// This OVH Software is licensed to you under the MIT license <LICENSE-MIT
// https://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use std::collections::BTreeMap;

use serde_value::Value;

/// Serializer use to convert a struct which implement `Serialize` into a flatten `MapTree` ( which
/// means a dict with key/value of only one depth)
struct FlatSerializer {
    /// Key separator used to represent depths
    /// E.g: {"a": {"b": 5}} => {"a_b": 5} with key_separator="_"
    key_separator: String,
    /// Prefix to use on the first key level
    /// E.g: {"a": {"b": 5}} => {"_a_b": 5} with key_separator="_" and prefix="_"
    prefix: String,
}

impl FlatSerializer {
    /// Initialize the struct using the given configuration values
    pub fn new(key_separator: String, prefix: String) -> FlatSerializer {
        FlatSerializer { key_separator, prefix }
    }
    /// "Normal" key formatting
    #[cfg(not(feature = "with-schema"))]
    fn format_key(&self, xpath: &str, key: &str, _value: &Value) -> String {
        match (xpath, key) {
            (_, "") => String::new(),
            ("", k) => format!("{}{}", self.prefix, k),
            (x, k) => format!("{}{}{}", x, self.key_separator, k)
        }
    }
    /// Suffix the key name with the type of value
    /// * `bool` => `bool`
    /// * `u*` => `double`
    /// * `i*` => `long`
    /// * `f*` => `float`
    #[cfg(feature = "with-schema")]
    fn _schema_suffix(&self, value: &Value) -> String {
        match *value {
            Value::Bool(_) => format!("{}bool", self.key_separator),
            Value::U8(_) | Value::U16(_) | Value::U32(_) | Value::U64(_) => format!("{}double", self.key_separator),
            Value::I8(_) | Value::I16(_) | Value::I32(_) | Value::I64(_) => format!("{}long", self.key_separator),
            Value::F32(_) | Value::F64(_) => format!("{}float", self.key_separator),
            _ => "".into()
        }
    }
    /// Will suffix the key name with the schema
    #[cfg(feature = "with-schema")]
    fn format_key(&self, xpath: &str, key: &str, value: &Value) -> String {
        match (xpath, key) {
            (_, "") => String::new(),
            ("", k) => format!("{}{}{}", self.prefix, k, self._schema_suffix(value)),
            (x, k) => format!("{}{}{}{}", x, self.key_separator, k, self._schema_suffix(value)),
        }
    }
    /// Dissemble the struct attribute into a flatten key/ value pair.
    pub fn disassemble(&self, xpath: &str, key: &str, value: &Value) -> BTreeMap<Value, Value> {
        let mut parts = BTreeMap::new();
        match value {
            Value::Map(ref tree) => {
                for (k, v) in tree.iter() {
                    let subkey = match k {
                        Value::String(data) => format!("{}", data),
                        Value::Char(data) => format!("{}", data),
                        _ => panic!("Map keys MUST be strings or char")
                    };
                    parts.append(&mut self.disassemble(&self.format_key(xpath, &key, value), &subkey, v));
                };
            }
            Value::Seq(ref values) => {
                for (i, val) in values.iter().enumerate() {
                    parts.append(&mut self.disassemble(&mut self.format_key(xpath, key, value), &format!("{}", i), val));
                }
            }
            _ => {
                parts.insert(Value::String(self.format_key(xpath, key, value)), value.clone());
            }
        };
        parts
    }
}

/// Flatten any struct which implement `Serialize` into a `BTreeMap<serde_value::Value, serde_value::Value>`
/// with only one depth.
///
/// ```rust
/// #[macro_use]
/// extern crate serde_derive;
/// extern crate serde_json;
/// extern crate serde_value_utils;
///
/// #[derive(Serialize, Clone, Debug)]
/// struct SubFoo {
///     a: String,
///     b: u64,
/// }
///
/// #[derive(Serialize, Clone, Debug)]
/// struct Foo {
///     a: String,
///     b: f64,
///     c: Vec<i8>,
///     d: SubFoo,
/// }
///
/// fn main() {
///     let foo = Foo { a: "test".into(), b: 0.5, c: vec![5, 9], d: SubFoo { a: "subtest".into(), b: 695217 } };
///     let ser = serde_value_utils::to_flatten_maptree("_", Some("_"), &foo).unwrap();
///     println!("{}", serde_json::to_string_pretty(&ser).unwrap());
/// }
/// ```
/// **Output**:
/// ```json
///  {
///   "_a": "test",
///   "_b": 0.5,
///   "_c_0": 5,
///   "_c_1": 9,
///   "_d_a": "subtest",
///   "_d_b": 695217
/// }
/// ```
pub fn to_flatten_maptree<T: ?Sized>(key_separator: &str, prefix: Option<&str>, src: &T) -> Result<BTreeMap<serde_value::Value, serde_value::Value>, serde_value::SerializerError>
    where T: serde::Serialize {
    Ok(FlatSerializer::new(key_separator.into(), prefix.unwrap_or("").into())
        .disassemble("", "", &serde_value::to_value(src)?))
}

