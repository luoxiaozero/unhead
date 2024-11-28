pub static SELF_CLOSING_TAGS: [&str; 3] = ["meta", "link", "base"];

pub static TAGS_WITH_INNER_CONTENT: [&str; 5] =
    ["title", "titleTemplate", "script", "style", "noscript"];

pub static HAS_ELEMENT_TAGS: [&str; 6] = ["base", "meta", "link", "style", "script", "noscript"];

pub static UNIQUE_TAGS: [&str; 6] = [
    "base",
    "title",
    "titleTemplate",
    "bodyAttrs",
    "htmlAttrs",
    "templateParams",
];
