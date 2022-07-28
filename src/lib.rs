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

pub trait Identity: for<'a> FnOnce(InvariantLifetime<'a>) -> InvariantLifetime<'a>
where
    Self: seal::Sealed,
{
}

impl<T> seal::Sealed for T where T: for<'a> FnOnce(InvariantLifetime<'a>) -> InvariantLifetime<'a> {}

impl<T> Identity for T where T: for<'a> FnOnce(InvariantLifetime<'a>) -> InvariantLifetime<'a> {}

// Auth instances must be neither default nor trivially constructible
pub struct Auth<'id, Id>(PhantomData<Cell<&'id mut Id>>)
where
    Id: Identity;

impl<'id, Id> Auth<'id, Id>
where
    Id: for<'a> FnOnce(InvariantLifetime<'a>) -> InvariantLifetime<'a>,
    // Id: Identity, /* DO NOT WORK, DON'T KNOW WHY */
{
    pub fn new(_: Id) -> Self {
        Self(PhantomData)
    }
}

impl<'id, Id> Auth<'id, Id>
where
    Id: Identity,
{
    pub fn grant_once(&self) -> OnceToken<'id, Id> {
        OnceToken::new(self)
    }

    pub fn grant_shared(&self) -> SharedToken<'id, Id> {
        SharedToken::new(self)
    }
}

impl<'id, Id> Drop for Auth<'id, Id>
where
    Id: Identity,
{
    fn drop(&mut self) {}
}

#[macro_export]
macro_rules! auth {
    () => {
        $crate::Auth::new(|_| $crate::InvariantLifetime::new())
    };
}

pub struct OnceToken<'id, Id>(PhantomData<Auth<'id, Id>>)
where
    Id: Identity;

impl<'id, Id> OnceToken<'id, Id>
where
    Id: Identity,
{
    fn new(_: &Auth<'id, Id>) -> Self {
        Self(PhantomData)
    }
}

impl<'id, Id> Drop for OnceToken<'id, Id>
where
    Id: Identity,
{
    fn drop(&mut self) {}
}

#[derive(Copy, Clone)]
pub struct SharedToken<'id, Id>(PhantomData<Auth<'id, Id>>)
where
    Id: Identity;

impl<'id, Id> SharedToken<'id, Id>
where
    Id: Identity,
{
    fn new(_: &Auth<'id, Id>) -> Self {
        Self(PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn zero_sized() {
        assert_eq!(0, mem::size_of::<InvariantLifetime>());

        let auth = auth!();
        assert_eq!(0, mem::size_of_val(&auth));
        assert_eq!(0, mem::size_of_val(&auth.grant_once()));
        assert_eq!(0, mem::size_of_val(&auth.grant_shared()));
    }
}
