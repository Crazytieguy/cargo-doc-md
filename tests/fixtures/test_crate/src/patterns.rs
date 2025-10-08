use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Builder {
    name: Option<String>,
    value: Option<i32>,
    enabled: bool,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            name: None,
            value: None,
            enabled: false,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn value(mut self, value: i32) -> Self {
        self.value = Some(value);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn build(self) -> Result<Built, &'static str> {
        Ok(Built {
            name: self.name.ok_or("name is required")?,
            value: self.value.unwrap_or(0),
            enabled: self.enabled,
        })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Built {
    pub name: String,
    pub value: i32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Newtype(pub u64);

impl Newtype {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn inner(&self) -> u64 {
        self.0
    }
}

impl From<u64> for Newtype {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Newtype> for u64 {
    fn from(value: Newtype) -> Self {
        value.0
    }
}

pub struct TypeState<State> {
    data: String,
    _state: PhantomData<State>,
}

pub struct Open;
pub struct Closed;

impl TypeState<Open> {
    pub fn new(data: String) -> Self {
        Self {
            data,
            _state: PhantomData,
        }
    }

    pub fn close(self) -> TypeState<Closed> {
        TypeState {
            data: self.data,
            _state: PhantomData,
        }
    }
}

impl TypeState<Closed> {
    pub fn open(self) -> TypeState<Open> {
        TypeState {
            data: self.data,
            _state: PhantomData,
        }
    }

    pub fn data(&self) -> &str {
        &self.data
    }
}

pub struct Handle<T> {
    inner: Box<T>,
}

impl<T> Handle<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }

    pub fn get(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn into_inner(self) -> T {
        *self.inner
    }
}

#[derive(Debug, Clone)]
pub struct Visitor;

impl Visitor {
    pub fn visit_string(&self, _s: &str) {}
    pub fn visit_number(&self, _n: i32) {}
    pub fn visit_bool(&self, _b: bool) {}
}
