use super::{props_to_string, tag_to_string};
use crate::schema::{HeadTag, SSRHeadPayload, TagPosition};
use std::collections::HashMap;

pub fn ssr_render_tags(tags: Vec<HeadTag>) -> SSRHeadPayload {
    let mut head_tags = String::new();
    let mut body_tags_open = String::new();
    let mut body_tags_close = String::new();
    let html_attrs = HashMap::new();
    let body_attrs = HashMap::new();

    // let lineBreaks = !options?.omitLineBreaks ? '\n' : ''
    let line_breaks = '\n';

    for tag in tags.into_iter() {
        // avoid rendering empty tags
        if tag.props.is_empty() && tag.inner_html.is_none() && tag.text_content.is_none() {
            continue;
        }
        //   if (tag.tag === 'htmlAttrs' || tag.tag === 'bodyAttrs') {
        //     Object.assign(schema[tag.tag], tag.props)
        //     continue
        //   }
        let s = tag_to_string(&tag);
        let tag_position = tag.tag_position.unwrap_or(TagPosition::Head);
        match tag_position {
            TagPosition::Head => {
                if head_tags.is_empty() {
                    head_tags += &format!("{line_breaks}{s}");
                } else {
                    head_tags += &s;
                }
            }
            TagPosition::BodyClose => {
                if body_tags_close.is_empty() {
                    body_tags_close += &format!("{line_breaks}{s}");
                } else {
                    body_tags_close += &s;
                }
            }
            TagPosition::BodyOpen => {
                if body_tags_open.is_empty() {
                    body_tags_open += &format!("{line_breaks}{s}");
                } else {
                    body_tags_open += &s;
                }
            }
        }
    }

    SSRHeadPayload {
        head_tags,
        body_tags: body_tags_close,
        body_tags_open,
        html_attrs: props_to_string(&html_attrs),
        body_attrs: props_to_string(&body_attrs),
    }
}
