use crate::{
    schema::DomRenderTagContext,
    shared::{hash_tag, HAS_ELEMENT_TAGS},
    Unhead,
};

pub async fn render_dom_head(head: Unhead) -> Result<(), ()> {
    let tags = head.resolve_tags().into_iter().map(|tag| {
        let id = if HAS_ELEMENT_TAGS.contains(&tag.tag.as_str()) {
            hash_tag(&tag)
        } else {
            tag.tag.as_str().to_string()
        };
        DomRenderTagContext {
            id,
            should_render: true,
            tag,
        }
    });

    Ok(())
}
