use std::collections::HashMap;

fn encode_attribute(value: &String) -> String {
    value.replacen('"', "&quot;", value.len())
}

pub fn props_to_string(props: &HashMap<String, String>) -> String {
    let mut attrs = String::new();

    for (key, value) in props.into_iter() {
        if value.is_empty() {
            attrs += &format!(" {key}");
        } else {
            attrs += &format!(" {key}=\"{}\"", encode_attribute(value));
        }
    }

    attrs
}
