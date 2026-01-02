use crate::common::ValueType;

/// This function is a work in progress. It's designed to flatten a list of maps by using the
/// given key and verifier function, however it currently doesn't work and just needs to be
/// planned out a little more.
fn flatten_map_helper<F>(
    list: &Vec<ValueType>,
    key: &String,
    verifier: F,
) -> Result<Vec<ValueType>, String>
where
    F: Fn(&ValueType) -> Option<&ValueType>,
{
    let mut out = Vec::new();
    for item in list {
        let map = match item {
            ValueType::Map(Some(map)) => map,
            _ => return Err("Expected a map".to_owned()),
        };

        // Get the key, and verify that the value is the correct type. If so, push it.
        if let Some(value) = map.get(key).and_then(|v| verifier(v)) {
            out.push(value.clone());
        }
    }

    Ok(out)
}
