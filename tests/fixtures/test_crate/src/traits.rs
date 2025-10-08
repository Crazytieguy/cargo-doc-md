use std::fmt;

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        0
    }
}

pub trait Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

pub trait FromIterator<A>: Sized {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}

pub trait Converter {
    type Input;
    type Output;
    type Error;

    const MAX_RETRIES: u32 = 3;

    fn convert(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;

    fn batch_convert(&self, inputs: Vec<Self::Input>) -> Vec<Result<Self::Output, Self::Error>>
    where
        Self::Input: Clone,
    {
        inputs.iter().map(|i| self.convert(i.clone())).collect()
    }
}

pub trait ExtensionTrait {
    fn extension_method(&self) -> String;
}

impl<T: fmt::Display> ExtensionTrait for T {
    fn extension_method(&self) -> String {
        format!("Extended: {}", self)
    }
}

pub trait ComplexBounds<T>
where
    T: Clone + fmt::Debug + Send + Sync + 'static,
{
    fn process(&self, item: T) -> T;
}

pub trait DefaultImpl {
    fn has_default(&self) -> bool {
        true
    }
}

impl<T> DefaultImpl for T {}

pub trait Sealed: private::SealedTrait {}

mod private {
    pub trait SealedTrait {}
}

pub struct SealedType;
impl private::SealedTrait for SealedType {}
impl Sealed for SealedType {}

pub trait GenericTrait<T, U = String> {
    fn method(&self, t: T, u: U) -> (T, U);
}

pub trait SuperTrait: Clone + fmt::Debug {
    fn super_method(&self);
}

pub trait Associated {
    type Assoc: fmt::Display + Clone;

    fn get_assoc(&self) -> Self::Assoc;
}

pub struct AssociatedImpl;

impl Associated for AssociatedImpl {
    type Assoc = String;

    fn get_assoc(&self) -> Self::Assoc {
        "associated".to_string()
    }
}
