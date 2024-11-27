use super::HeadTag;

pub struct DomRenderTagContext {
    pub id: String,
    // el: Element,
    pub should_render: bool,
    pub tag: HeadTag,
    // entry?: HeadEntry<any>
    // markSideEffect: (key: string, fn: () => void) => void
}
