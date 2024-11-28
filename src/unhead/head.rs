use crate::schema::{HeadTag, TagKey, TagPosition};
use std::{collections::HashMap, string::ToString};

macro_rules! head_elements {
        ($(
            $ty:ident
            [$($attr:ty),*]
            [$($prop:ty),*]
          ),*
          $(,)?
        ) => {
            paste::paste! {
                $(
                    pub fn [<$ty:lower>]() -> $ty {
                        $ty::default()
                    }

                    #[derive(Debug, Default)]
                    pub struct $ty {
                        props: HashMap<String, String>,
                        key: Option<String>,
                        text_content: Option<String>,
                        inner_html: Option<String>,
                        tag_position: Option<TagPosition>,
                    }

                    impl $ty {
                        $(
                            pub fn $attr(mut self, $attr: impl ToString) -> Self {
                                let key = stringify!($attr);
                                let value = $attr.to_string();

                                if key == "text_content" {
                                    self.text_content = Some(value);
                                    return self;
                                }
                                if key == "inner_html" {
                                    self.inner_html = Some(value);
                                    return self;
                                }
                                if key == "key" {
                                    self.key = Some(value);
                                    return self;
                                }

                                let key = if key == "http_equiv" {
                                    "http-equiv"
                                } else {
                                    key
                                };

                                self.props.insert(key.to_string(), value);
                                self
                            }
                        )*
                    }

                    impl $ty {
                        $(
                            pub fn $prop(mut self, $prop: [<$prop:camel>]) -> Self {
                                self.$prop = Some($prop);
                                self
                            }
                        )*
                    }

                    impl $ty {
                        pub fn build(self) -> HeadTag {
                            HeadTag {
                                tag: TagKey::$ty,
                                props: self.props,
                                key: self.key,
                                text_content: self.text_content,
                                inner_html: self.inner_html,
                                tag_position: self.tag_position
                            }
                        }
                    }
                )*
            }
        }
    }

head_elements! {
    Base [href, target] [],
    Meta [charset, content, http_equiv, name] [],
    Link [r#as, blocking, crossorigin, fetchpriority, href, hreflang, imagesizes, imagesrcset, integrity, media, rel, referrerpolicy, sizes, r#type] [],
    Noscript [] [],
    Script [inner_html, r#async, crossorigin, defer, fetchpriority, integrity, nomodule, referrerpolicy, src, r#type, blocking] [],
    Style [inner_html, key, id, blocking, media, nonce, title] [tag_position],
    Title [text_content, inner_html] [],
}
