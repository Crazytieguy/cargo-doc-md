use std::collections::HashMap;

pub struct Container<T> {
    pub items: Vec<T>,
}

impl<T> Container<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T> Default for Container<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RefStruct<'a> {
    pub data: &'a str,
}

pub enum Status {
    Idle,
    Running { progress: f32 },
    Completed,
    Failed { error: String },
}

pub type StringMap = HashMap<String, String>;

pub const DEFAULT_CAPACITY: usize = 10;
