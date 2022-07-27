use std::{cell::Cell, marker::PhantomData};

mod seal {
    pub trait Sealed {}
}

#[derive(Default)]
pub struct InvariantLifetime<'a>(PhantomData<Cell<&'a mut ()>>);

impl<'a> InvariantLifetime<'a> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'a> Drop for InvariantLifetime<'a> {
    fn drop(&mut self) {}
}

pub trait Unique: for<'a> FnOnce(InvariantLifetime<'a>) -> InvariantLifetime<'a>
where
    Self: seal::Sealed,
{
}

impl<T> seal::Sealed for T where T: for<'a> FnOnce(InvariantLifetime<'a>) -> InvariantLifetime<'a> {}

impl<T> Unique for T where T: for<'a> FnOnce(InvariantLifetime<'a>) -> InvariantLifetime<'a> {}

// Badge instances must be neither default nor trivially constructible
pub struct Badge<'id, Id>(PhantomData<Cell<&'id mut Id>>)
where
    Id: Unique;

impl<'id, Id> Badge<'id, Id>
where
    Id: for<'a> FnOnce(InvariantLifetime<'a>) -> InvariantLifetime<'a>,
    // Id: Unique, /* DO NOT WORK, DON'T KNOW WHY */
{
    pub fn new(_: Id) -> Self {
        Self(PhantomData)
    }
}

impl<'id, Id> Badge<'id, Id>
where
    Id: Unique,
{
    pub fn proxy(&self) -> Proxy<'id, Id> {
        Proxy::new(self)
    }
}

impl<'id, Id> Drop for Badge<'id, Id>
where
    Id: Unique,
{
    fn drop(&mut self) {}
}

#[macro_export]
macro_rules! badge {
    () => {
        Badge::new(|_| $crate::InvariantLifetime::new())
    };
}

pub struct Proxy<'id, Id>(PhantomData<Badge<'id, Id>>)
where
    Id: Unique;

impl<'id, Id> Proxy<'id, Id>
where
    Id: Unique,
{
    pub fn new(_: &Badge<'id, Id>) -> Self {
        Self(PhantomData)
    }
}

impl<'id, Id> Drop for Proxy<'id, Id>
where
    Id: Unique,
{
    fn drop(&mut self) {}
}
