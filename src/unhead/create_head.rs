pub struct Unhead<T> {
    entries: Vec<HeadEntry<T>>
}

impl<T> Unhead<T> {
    pub fn new() -> Self {
        let head = Self::new_core();

        head
    }

    fn new_core() -> Self {
        let entries = vec![];
        Self {
            entries
        }
    }

    pub fn push(&mut self, input: T) {
        let entry = HeadEntry {
            input
        };

        self.entries.push(entry);
    }
}

struct HeadEntry<Input> {
    input: Input,
}