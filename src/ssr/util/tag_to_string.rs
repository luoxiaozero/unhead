use crate::{
    schema::HeadTag,
    shared::{SELF_CLOSING_TAGS, TAGS_WITH_INNER_CONTENT},
    ssr::util::props_to_string,
};

fn escape_html(str: &String) -> String {
    let mut html = String::new();

    for c in str.chars() {
        match c {
            '&' => html.push_str("&amp;"),
            '<' => html.push_str("&lt;"),
            '>' => html.push_str("&gt;"),
            '"' => html.push_str("&quot;"),
            '\'' => html.push_str("&#x27;"),
            '/' => html.push_str("&#x2F;"),
            _ => html.push(c),
        }
    }
    html
}

pub fn tag_to_string(tag: &HeadTag) -> String {
    let attrs = props_to_string(&tag.props);
    let tag_name = tag.tag.as_str();
    let open_tag = format!("<{}{}>", tag_name, attrs);
    // get the encoding depending on the tag type
    if !TAGS_WITH_INNER_CONTENT.contains(&tag_name) {
        if SELF_CLOSING_TAGS.contains(&tag_name) {
            return open_tag;
        } else {
            return format!("{open_tag}</${tag_name}>");
        }
    }

    // dangerously using innerHTML, we don't encode this
    let content = if let Some(inner_html) = &tag.inner_html {
        inner_html.clone()
    } else if let Some(text_context) = &tag.text_content {
        // content needs to be encoded to avoid XSS, only for title
        escape_html(text_context)
    } else {
        String::new()
    };

    if SELF_CLOSING_TAGS.contains(&tag_name) {
        open_tag
    } else {
        format!("{open_tag}{content}</{}>", tag_name)
    }
}
