use std::collections::HashMap;

#[derive(PartialEq)]
pub enum TagKey {
    Title,
    Meta,
}

impl TagKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Meta => "meta",
        }
    }
}

pub struct HeadTag {
    pub tag: TagKey,
    pub props: HashMap<String, String>,
    pub key: Option<String>,
}
