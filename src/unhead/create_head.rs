use crate::schema::HeadTag;

use super::plugins::{DedupePlugin, HeadPlugin};

pub struct Unhead<T> {
    entries: Vec<HeadEntry<T>>,
    plugins: Vec<Box<dyn HeadPlugin>>,
}

impl<T> Unhead<T> {
    pub fn new() -> Self {
        let head = Self::new_core();

        head
    }

    fn new_core() -> Self {
        let entries = vec![];
        let mut head = Self {
            entries,
            plugins: vec![],
        };

        head.plugin(DedupePlugin);

        head
    }

    pub fn plugin(&mut self, p: impl HeadPlugin + 'static) {
        if self.plugins.iter().any(|plugin| plugin.key() == p.key()) {
            return;
        }

        self.plugins.push(Box::new(p));
    }

    pub fn push(&mut self, input: T) {
        let entry = HeadEntry { input };

        self.entries.push(entry);
    }

    pub fn resolve_tags(&self) -> Vec<HeadTag> {
        let mut tags: Vec<HeadTag> = vec![];
        let entries = &self.entries;

        // self.entries.iter().for_each(|entry| );

        self.plugins.iter().for_each(|p| p.tags_resolve(&mut tags));

        tags
    }
}

struct HeadEntry<Input> {
    input: Input,
}
