use serde_json::Value;

use crate::{error::MavecError, result::Result};

/// Converts a `Value` to a `Vec<String>` if it's structure is
/// object-parsable, otherwise, returns an error.
///
/// ```
/// use mavec::core::to_vec;
/// use serde_json::json;
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

    Err(MavecError::JsonStructureParseError(
        "Map Could not be converted to object".to_string(),
    ))
}
