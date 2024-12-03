use crate::schema::{HeadTag, TagKey, TagPriority};

pub fn tag_weight(tag: &HeadTag) -> i32 {
    let priority = &tag.tag_priority;
    if let Some(TagPriority::Number(priority)) = priority {
        return *priority;
    }
    let mut weight = 100;
    if tag.tag == TagKey::Meta {
        // CSP needs to be as it effects the loading of assets
        if tag
            .props
            .get("http-equiv")
            .is_some_and(|k| k == "content-security-policy")
        {
            weight = -30
            // charset must come early in case there's non-utf8 characters in the HTML document
        } else if tag.props.contains_key("charset") {
            weight = -20
        } else if tag.props.get("name").is_some_and(|k| k == "viewport") {
            weight = -15
        }
    } else if tag.tag == TagKey::Link && tag.props.get("rel").is_some_and(|k| k == "preconnect") {
        // preconnects should almost always come first
        weight = 20
    } else if tag.tag == TagKey::Base {
        weight = -10
    } else if tag.tag == TagKey::Title {
        weight = 10
    }

    if let Some(priority) = priority {
        match priority {
            TagPriority::Critical => weight - 80,
            TagPriority::High => weight - 10,
            TagPriority::Low => weight + 20,
            TagPriority::Number(_) => unreachable!(),
            TagPriority::Before(_) => weight,
            TagPriority::After(_) => weight,
        }
    } else {
        weight
    }
}
