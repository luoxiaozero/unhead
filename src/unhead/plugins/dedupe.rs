use super::{HeadHook, HeadPlugin};
use crate::{
    schema::{HeadTag, TagKey},
    shared::{hash_tag, tag_dedupe_key},
};
use std::collections::HashMap;

pub struct DedupePlugin;

impl HeadHook for DedupePlugin {
    fn tag_normalise(&self, tag: &mut HeadTag) {
        if let Some(generated_key) = tag_dedupe_key(tag) {
            if !generated_key.starts_with("meta:og:") && !generated_key.starts_with("meta:twitter:")
            {
                tag.key = None;
            }
        }
    }

    fn tags_resolve(&self, tags: &mut Vec<HeadTag>) {
        let mut deduping = HashMap::<String, HeadTag>::new();

        tags.drain(..).into_iter().for_each(|mut tag| {
            let dedupe_key = if let Some(key) = &tag.key {
                format!("{}:{}", tag.tag.as_str(), key)
            } else {
                hash_tag(&tag)
            };

            if let Some(duped_tag) = deduping.get_mut(&dedupe_key) {
                let old_props = &mut duped_tag.props;
                if let Some(old_style) = old_props.get_mut("style") {
                    if let Some(style) = tag.props.get_mut("style") {
                        if !old_style.ends_with(";") {
                            *old_style += ";";
                        }
                        *style = format!("{old_style} {style}");
                    }
                }

                if let Some(old_class) = old_props.get_mut("class") {
                    if let Some(class) = tag.props.get_mut("class") {
                        *class = format!("{old_class} {class}");
                    } else {
                        tag.props.insert("class".to_string(), old_class.clone());
                    }
                }

                tag.props.iter().for_each(|(key, value)| {
                    old_props.insert(key.clone(), value.clone());
                });

                return;
            }

            deduping.insert(dedupe_key, tag);
        });

        *tags = deduping
            .into_values()
            .filter(|t| {
                !(t.tag == TagKey::Meta
                    && (t.props.contains_key("name") || t.props.contains_key("property"))
                    && !t.props.contains_key("content"))
            })
            .collect();
    }
}

impl HeadPlugin for DedupePlugin {
    fn key(&self) -> &'static str {
        "dedupe"
    }
}
