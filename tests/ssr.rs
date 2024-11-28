use unhead::{head, render_ssr_head, Unhead};

#[test]
fn head() {
    let mut head = Unhead::new();

    head.push(vec![
        head::base().target("_blank").build(),
        head::title().inner_html("unhead").build(),
        head::style()
            .id("id")
            .inner_html("body { background: #fff }")
            .build(),
        head::meta().charset("utf-8").build(),
    ]);

    let result = render_ssr_head(head);

    println!("{:#?}", result);
}
