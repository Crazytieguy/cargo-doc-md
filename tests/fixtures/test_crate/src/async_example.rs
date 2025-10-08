use std::future::Future;
use std::pin::Pin;

pub async fn simple_async() -> String {
    "async result".to_string()
}

pub async fn async_with_args(name: &str, count: usize) -> Vec<String> {
    vec![name.to_string(); count]
}

pub trait AsyncTrait {
    async fn async_method(&self) -> String;

    async fn async_with_default(&self) -> i32 {
        42
    }
}

pub struct AsyncStruct {
    pub data: String,
}

impl AsyncStruct {
    pub async fn async_new(data: String) -> Self {
        Self { data }
    }

    pub async fn process(&self) -> Result<String, String> {
        Ok(self.data.clone())
    }

    pub async fn fetch(&self, url: &str) -> Result<Vec<u8>, String> {
        Ok(url.as_bytes().to_vec())
    }
}

pub fn returns_future() -> impl Future<Output = String> {
    async { "future result".to_string() }
}

pub fn boxed_future() -> Pin<Box<dyn Future<Output = i32>>> {
    Box::pin(async { 42 })
}

pub async fn generic_async<T: Clone>(item: T) -> T {
    item
}

pub trait AsyncIterator {
    type Item;

    async fn next(&mut self) -> Option<Self::Item>;
}

pub struct AsyncCounter {
    count: usize,
    max: usize,
}

impl AsyncCounter {
    pub fn new(max: usize) -> Self {
        Self { count: 0, max }
    }
}

impl AsyncIterator for AsyncCounter {
    type Item = usize;

    async fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            let current = self.count;
            self.count += 1;
            Some(current)
        } else {
            None
        }
    }
}
