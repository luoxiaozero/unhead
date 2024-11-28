use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum TagKey {
    Base,
    Meta,
    Link,
    Noscript,
    Script,
    Style,
    Title,
}

impl TagKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Meta => "meta",
            Self::Link => "link",
            Self::Noscript => "noscript",
            Self::Script => "script",
            Self::Style => "style",
            Self::Title => "title",
        }
    }
}

#[derive(Debug, Clone)]
pub enum TagPosition {
    Head,
    BodyClose,
    BodyOpen,
}

#[derive(Debug, Clone)]
pub struct HeadTag {
    pub tag: TagKey,
    pub props: HashMap<String, String>,
    pub key: Option<String>,
    pub inner_html: Option<String>,
    pub text_content: Option<String>,
    pub tag_position: Option<TagPosition>,
}
