mod dedupe;
mod sort;

pub use dedupe::*;
pub use sort::*;

use crate::schema::HeadTag;

pub enum RuntimeMode {
    Server,
    Client,
}

pub trait HeadPlugin: HeadHook {
    fn key(&self) -> &'static str;

    fn mode(&self) -> Option<RuntimeMode> {
        None
    }
}

pub trait HeadHook {
    fn tag_normalise(&self, tag: &mut HeadTag) {
        let _ = tag;
    }

    fn tags_resolve(&self, tags: &mut Vec<HeadTag>) {
        let _ = tags;
    }
}

// TODO
impl<T: HeadPlugin> HeadHook for Vec<T> {
    fn tag_normalise(&self, tag: &mut HeadTag) {
        self.iter().for_each(|p| p.tag_normalise(tag));
    }

    fn tags_resolve(&self, tags: &mut Vec<HeadTag>) {
        self.iter().for_each(|p| p.tags_resolve(tags));
    }
}
