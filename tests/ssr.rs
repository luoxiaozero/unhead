use unhead::{head, render_ssr_head, SSRHeadPayload, Unhead};

#[test]
fn head() {
    let mut head = Unhead::new();

    head.push(vec![
        head::link().href("link").build(),
        head::base().target("_blank").build(),
        head::title().inner_html("unhead").build(),
        head::style()
            .id("id")
            .inner_html("body { background: #fff }")
            .build(),
        head::meta().charset("utf-8").build(),
    ]);

    let result = render_ssr_head(head);

    assert_eq!(result, SSRHeadPayload {
        head_tags: "\n<meta charset=\"utf-8\"><base target=\"_blank\"><title>unhead</title><style id=\"id\">body { background: #fff }</style><link href=\"link\">".to_string(),
        body_tags: "".to_string(),
        body_tags_open: "".to_string(),
        html_attrs: "".to_string(),
        body_attrs: "".to_string(),
    });
}
