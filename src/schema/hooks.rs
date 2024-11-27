use super::HeadTag;

#[derive(Debug)]
pub struct SSRHeadPayload {
    pub head_tags: String,
    pub body_tags: String,
    pub body_tags_open: String,
    pub html_attrs: String,
    pub body_attrs: String,
}

pub struct DomRenderTagContext {
    pub id: String,
    // el: Element,
    pub should_render: bool,
    pub tag: HeadTag,
    // entry?: HeadEntry<any>
    // markSideEffect: (key: string, fn: () => void) => void
}
