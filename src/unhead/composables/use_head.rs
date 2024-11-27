use crate::{schema::HeadTag, unhead::create_head::IntoHeadTag, Unhead};

pub fn use_head<T>(head: &mut Unhead, input: Vec<T>)
where
    T: IntoHeadTag,
{
    head.push(
        input
            .into_iter()
            .map(|i| i.into_head_tag())
            .collect::<Vec<HeadTag>>(),
    );
}

pub mod head {
    use crate::{
        schema::{HeadTag, TagKey, TagPosition},
        unhead::create_head::IntoHeadTag,
    };
    use std::collections::HashMap;

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
                        tag_position: Option<TagPosition>,
                    }

                    impl $ty {
                        $(
                            pub fn $attr(mut self, $attr: String) -> Self {
                                self.props.insert("id".to_string(), $attr);
                                self
                            }
                        )*
                    }

                    impl $ty {
                        $(
                            pub fn $prop(mut self, $prop: Option<[<$prop:camel>]>) -> Self {
                                self.$prop = $prop;
                                self
                            }
                        )*
                    }

                    impl IntoHeadTag for $ty {
                        fn into_head_tag(self) -> HeadTag {
                            HeadTag {
                                tag: TagKey::$ty,
                                props: self.props,
                                key: None,
                                tag_position: self.tag_position
                            }
                        }
                    }
                )*
            }
        }
    }

    head_elements! {
        Style [id, key] [tag_position],
        Title [text_content] [],
    }
}
