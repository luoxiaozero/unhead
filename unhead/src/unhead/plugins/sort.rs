use std::cmp::Ordering;
use crate::{
    schema::{HeadTag, TagPriority},
    shared::tag_weight,
};
use super::{HeadHook, HeadPlugin};

pub struct SortPlugin;

impl HeadPlugin for SortPlugin {
    fn key(&self) -> &'static str {
        "sort"
    }
}

impl HeadHook for SortPlugin {
    fn tags_resolve(&self, tags: &mut Vec<HeadTag>) {
        // 2a. Sort based on priority
        // now we need to check render priority for each before: rule and use the dedupe key index
        let dp: Vec<_> = tags
            .iter()
            .filter_map(|tag| {
                if let Some(d) = tag.d.clone() {
                    if let Some(p) = tag.p {
                        return Some((d, p));
                    }
                }
                None
            })
            .collect();
        for tag in tags.iter_mut() {
            if let Some(tag_priority) = &tag.tag_priority {
                if let TagPriority::Before(key) = tag_priority {
                    let position = dp.iter().find_map(|(d, p)| (d == key).then_some(p));
                    if let Some(position) = position {
                        tag.p = Some(position - 1);
                        break;
                    }
                } else if let TagPriority::After(key) = tag_priority {
                    let position = dp.iter().find_map(|(d, p)| (d == key).then_some(p));
                    if let Some(position) = position {
                        tag.p = Some(position + 1);
                        break;
                    }
                }
            }
        }

        tags.sort_by(move |a, b| {
            let a_weight = tag_weight(a);
            let b_weight = tag_weight(b);
            // 2c. sort based on critical tags
            if a_weight < b_weight {
                return Ordering::Less;
            } else if a_weight > b_weight {
                return Ordering::Greater;
            }

            match a.p.unwrap_or_default() - b.p.unwrap_or_default() {
                0 => Ordering::Equal,
                1.. => Ordering::Greater,
                ..0 => Ordering::Less,
            }
        });
    }
}
