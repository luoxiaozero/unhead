use std::collections::HashMap;

#[derive(PartialEq)]
pub enum TagKey {
    Title,
    Meta,
    Style,
}

impl TagKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Meta => "meta",
            Self::Style => "style",
        }
    }
}

#[derive(Debug)]
pub enum TagPosition {
    Head,
    BodyClose,
    BodyOpen,
}

pub struct HeadTag {
    pub tag: TagKey,
    pub props: HashMap<String, String>,
    pub key: Option<String>,
    pub tag_position: Option<TagPosition>,
}
