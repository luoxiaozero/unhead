mod dedupe;

pub use dedupe::*;

use crate::schema::HeadTag;

pub trait HeadPlugin: HeadHook {
    fn key(&self) -> &'static str;
}

pub trait HeadHook {
    fn tag_normalise(&self, tag: &mut HeadTag) {
        let _ = tag;
    }

    fn tags_resolve(&self, tags: &mut Vec<HeadTag>) {
        let _ = tags;
    }
}
