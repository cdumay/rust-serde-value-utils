// Copyright 2019-present, OVH SAS
// All rights reserved.
//
// This OVH Software is licensed to you under the MIT license <LICENSE-MIT
// https://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.
//

use serde_value::Value;

/// Function to try to detect value type from String
///
/// Only the following types are recognized :
/// * `bool`
/// * `u64`
/// * `i64`
/// * `f64`
/// * `String` (as fallback)
/// 
/// ```rust
/// extern crate serde_value_utils;
///
/// use serde_value_utils::try_detect_type;
///
/// fn main() {
///     println!("{:?}", try_detect_type("6.5"));
/// }
/// ```
/// **Output**:
/// ```
/// F64(6.5)
/// ```
pub fn try_detect_type(raw: &str) -> Value {
    if let Ok(data) = raw.parse::<bool>() {
        return Value::Bool(data);
    }
    if let Ok(data) = raw.parse::<u64>() {
        return Value::U64(data);
    }
    if let Ok(data) = raw.parse::<i64>() {
        return Value::I64(data);
    }
    if let Ok(data) = raw.parse::<f64>() {
        return Value::F64(data);
    }
    Value::String(raw.to_string())
}