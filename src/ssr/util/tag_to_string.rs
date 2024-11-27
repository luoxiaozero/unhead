use crate::{schema::HeadTag, shared::SELF_CLOSING_TAGS, ssr::util::props_to_string};

pub fn tag_to_string(tag: &HeadTag) -> String {
    let attrs = props_to_string(&tag.props);
    let open_tag = format!("<{}{}>", tag.tag.as_str(), attrs);
    // get the encoding depending on the tag type
    // if (!TagsWithInnerContent.has(tag.tag))
    //   return SelfClosingTags.has(tag.tag) ? openTag : `${openTag}</${tag.tag}>`

    // // dangerously using innerHTML, we don't encode this
    // let content = String(tag.innerHTML || '')
    let content = String::new();
    // if (tag.textContent)
    //   // content needs to be encoded to avoid XSS, only for title
    //   content = escapeHtml(String(tag.textContent))

    if SELF_CLOSING_TAGS.contains(&tag.tag.as_str()) {
        open_tag
    } else {
        format!("{open_tag}{content}</{}>", tag.tag.as_str())
    }
}
