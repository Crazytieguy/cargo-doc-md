use std::fmt;

pub struct BorrowedData<'a> {
    pub data: &'a str,
    pub metadata: &'a [u8],
}

impl<'a> BorrowedData<'a> {
    pub fn new(data: &'a str, metadata: &'a [u8]) -> Self {
        Self { data, metadata }
    }

    pub fn get_data(&self) -> &'a str {
        self.data
    }
}

pub struct DoubleBorrow<'a, 'b> {
    pub first: &'a str,
    pub second: &'b str,
}

pub struct LifetimeWithBound<'a, T: 'a> {
    pub reference: &'a T,
}

impl<'a, T: 'a + fmt::Display> LifetimeWithBound<'a, T> {
    pub fn display(&self) -> String {
        format!("{}", self.reference)
    }
}

pub fn lifetime_function<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn multiple_lifetimes<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str
where
    'b: 'a,
{
    x
}

pub trait LifetimeTrait<'a> {
    type Output: 'a;

    fn process(&self, input: &'a str) -> Self::Output;
}

pub struct LifetimeStruct<'a, T>
where
    T: 'a,
{
    pub data: &'a T,
    pub name: String,
}

impl<'a, T> LifetimeStruct<'a, T>
where
    T: 'a + Clone,
{
    pub fn new(data: &'a T, name: String) -> Self {
        Self { data, name }
    }

    pub fn clone_data(&self) -> T {
        self.data.clone()
    }
}

pub enum LifetimeEnum<'a> {
    Borrowed(&'a str),
    Owned(String),
    Multiple { first: &'a str, second: &'a [u8] },
}

pub fn higher_ranked_trait_bound<F>(f: F) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    f("test").to_string()
}
