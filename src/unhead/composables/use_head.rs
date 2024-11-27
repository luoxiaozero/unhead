use crate::Unhead;

fn use_head<T>(head: Unhead<T>) {
    
}

struct HeadInput<T> {
    head: Unhead<T>
}

impl<T> HeadInput<T> {
    fn style(&self) -> HeadStyle<T> {
        HeadStyle {

        }
    }
}

struct HeadStyle<T> {
    head: Unhead<T>
}

impl HeadStyle<T> {}