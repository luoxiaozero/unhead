use unhead::{head, render_ssr_head, Unhead};

#[test]
fn head() {
    let mut head = Unhead::new();
    head.push(vec![head::style().build()]);

    let result = render_ssr_head(head);

    println!("{:#?}", result);
}
