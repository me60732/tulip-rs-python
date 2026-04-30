use std::collections::HashMap;
use tulip_rs::types::Info;

/// Converts a TulipRS Info struct into a Python-compatible HashMap
/// This provides a consistent way to expose indicator metadata to Python
pub fn info_to_hashmap(info: Info) -> HashMap<String, String> {
    let mut result = HashMap::new();

    result.insert("name".to_string(), info.name.to_string());
    result.insert("full_name".to_string(), info.full_name.to_string());
    result.insert(
        "display_type".to_string(),
        format!("{:?}", info.display_type),
    );
    result.insert(
        "indicator_type".to_string(),
        format!("{:?}", info.indicator_type),
    );
    result.insert("inputs".to_string(), format!("{:?}", info.inputs));
    result.insert("options".to_string(), format!("{:?}", info.options));
    result.insert("outputs".to_string(), format!("{:?}", info.outputs));
    result.insert(
        "optional_outputs".to_string(),
        format!("{:?}", info.optional_outputs),
    );

    result
}
