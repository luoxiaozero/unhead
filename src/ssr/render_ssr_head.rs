use super::util::ssr_render_tags;
use crate::{schema::SSRHeadPayload, Unhead};

pub fn render_ssr_head(head: Unhead) -> SSRHeadPayload {
    //   const beforeRenderCtx: ShouldRenderContext = { shouldRender: true }
    //   await head.hooks.callHook('ssr:beforeRender', beforeRenderCtx)
    //   if (!beforeRenderCtx.shouldRender) {
    //     return {
    //       headTags: '',
    //       bodyTags: '',
    //       bodyTagsOpen: '',
    //       htmlAttrs: '',
    //       bodyAttrs: '',
    //     }
    //   }
    let tags = head.resolve_tags();
    //   const ctx = { tags: await head.resolveTags() }
    //   await head.hooks.callHook('ssr:render', ctx)

    let html: SSRHeadPayload = ssr_render_tags(tags);
    //   const renderCtx: SSRRenderContext = { tags: ctx.tags, html }
    //   await head.hooks.callHook('ssr:rendered', renderCtx)
    html
}
