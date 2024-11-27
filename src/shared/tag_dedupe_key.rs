use super::UNIQUE_TAGS;
use crate::schema::HeadTag;

static ALLOWED_META_PROPERTIES: [&str; 3] = ["name", "property", "http-equiv"];

pub fn tag_dedupe_key(tag: &HeadTag) -> Option<String> {
    let tag_name = tag.tag.as_str();
    let props = &tag.props;
    // must only be a single base so we always dedupe
    if UNIQUE_TAGS.contains(&tag_name) {
        return Some(tag_name.to_string());
    }

    // support only a single canonical
    if tag_name == "link" {
        let canonical: Option<String> = Some("canonical".to_string());
        if props.get("rel") == canonical.as_ref() {
            return canonical;
        }
    }

    if props.contains_key("charset") {
        return Some("charset".to_string());
    }

    if let Some(id) = props.get("id") {
        return Some(format!("{tag_name}:id:{id}"));
    }

    for n in ALLOWED_META_PROPERTIES.iter() {
        // open graph props can have multiple tags with the same property
        if let Some(value) = props.get(*n) {
            // for example: meta-name-description
            return Some(format!("{tag_name}:{n}:{value}"));
        }
    }

    None
}
