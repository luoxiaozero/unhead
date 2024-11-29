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

/// Specify where to render the tag.
#[derive(Debug, Clone)]
pub enum TagPosition {
    Head,
    BodyClose,
    BodyOpen,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TagPriority {
    Critical,
    High,
    Low,
    Number(i32),
    Before(String),
    After(String),
}

#[derive(Debug, Clone)]
pub struct HeadTag {
    pub tag: TagKey,
    pub props: HashMap<String, String>,
    pub key: Option<String>,
    pub inner_html: Option<String>,
    pub text_content: Option<String>,
    pub tag_position: Option<TagPosition>,
    pub tag_priority: Option<TagPriority>,

    /// Position
    pub(crate) p: Option<i32>,
    /// Dedupe key
    pub(crate) d: Option<String>,
}
