use serde_json::Value;

use crate::{error::MavecError, result::Result};

/// Converts a `Value` to a `Vec<String>` if it's structure is
/// object-or-array-parsable, otherwise, returns an `JsonStructureParseError` error.
///
/// ```
/// use mavec::core::to_vec;
/// use serde_json::{json, Value};
/// use serde_json::Value
///
/// let value = json!({
/// "Jeff": true,
/// "Rose": "Mary",
/// "Miguel": 17,
///  });
///
/// assert_eq!(
/// to_vec(value).unwrap(),
/// Vec::from([
///     "Jeff".to_string(),
///     "true".to_string(),
///     "Rose".to_string(),
///     "Mary".to_string(),
///     "Miguel".to_string(),
///     "17".to_string()
/// ])
/// );
/// 
/// let value = Value::Array(vec![
/// json!(1),
/// Value::Bool(true),
/// Value::String(String::from("Mavec")),
/// Value::Bool(false),
/// ]);
///
/// assert_eq!(
/// to_vec(value).unwrap(),
/// Vec::from([
///     "1".to_string(),
///     "true".to_string(),
///     "Mavec".to_string(),
///     "false".to_string(),
/// ])
/// );
/// ```
pub fn to_vec(value: Value) -> Result<Vec<String>> {
    if let Some(object) = value.as_object() {
        let result: Vec<String> = object
            .iter()
            .flat_map(|(key, value)| {
                let value_str = match value {
                    Value::Number(num) => num.to_string(),
                    Value::String(s) => s.clone(),
                    Value::Bool(b) => b.to_string(),
                    Value::Null => "null".to_string(),
                    _ => "".to_string(),
                };
                vec![key.clone(), value_str]
            })
            .collect();

        return Ok(result);
    }

    if let Some(object) = value.as_array() {
        let result: Vec<String> = object
            .iter()
            .flat_map(|key| {
                let value_str = match key {
                    Value::Number(num) => num.to_string(),
                    Value::String(s) => s.clone(),
                    Value::Bool(b) => b.to_string(),
                    Value::Null => "null".to_string(),
                    _ => "".to_string(),
                };
                vec![value_str]
            })
            .collect();

        return Ok(result);
    }

    Err(MavecError::JsonStructureParseError(
        "Value could not be converted to Vec".to_string(),
    ))
}
