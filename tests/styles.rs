use unhead::{head, render_ssr_head, use_head, Unhead};

#[test]
fn head() {
    let mut head = Unhead::new();

    use_head(&mut head, vec![head::style()]);

    let result = render_ssr_head(head);

    println!("{:#?}", result);
}
