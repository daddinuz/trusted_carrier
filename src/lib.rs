use std::{cell::Cell, marker::PhantomData, ops::Deref};

mod seal {
    pub trait Sealed {}
}

pub struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {}
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

#[macro_export]
macro_rules! identity {
    () => {{
        fn cast<Id>(id: Id) -> impl $crate::Identity
        where
            Id: for<'a> FnOnce($crate::InvariantLifetime<'a>) -> $crate::InvariantLifetime<'a>,
        {
            id
        }

        cast(|x| x)
    }};
}

// To prevent forgery, Auth instances must be neither default nor trivially constructible.
// A Guard and an Identity instance must be consumed in order to construct a new Auth.
pub struct Auth<'guard, Id>(PhantomData<(InvariantLifetime<'guard>, Id)>)
where
    Id: Identity;

impl<'guard, Id> Auth<'guard, Id>
where
    Id: Identity,
{
    pub fn new(_: &'guard mut Guard, _: Id) -> Self {
        Self(PhantomData)
    }
}

impl<'guard, Id> Auth<'guard, Id>
where
    Id: Identity,
{
    pub fn grant(&self) -> Grant<'guard, Id> {
        Grant::new(self)
    }
}

impl<'guard, Id> Drop for Auth<'guard, Id>
where
    Id: Identity,
{
    fn drop(&mut self) {}
}

pub struct Grant<'guard, Id>(PhantomData<Auth<'guard, Id>>)
where
    Id: Identity;

impl<'guard, Id> Grant<'guard, Id>
where
    Id: Identity,
{
    fn new(_: &Auth<'guard, Id>) -> Self {
        Self(PhantomData)
    }

    pub fn to<T>(self, value: T) -> Trusted<'guard, Id, T> {
        Trusted::new(self, value)
    }
}

impl<'guard, Id> Drop for Grant<'guard, Id>
where
    Id: Identity,
{
    fn drop(&mut self) {}
}

pub struct Trusted<'guard, Id, T>
where
    Id: Identity,
{
    value: T,
    #[allow(dead_code)]
    grant: Grant<'guard, Id>,
}

impl<'guard, Id, T> Trusted<'guard, Id, T>
where
    Id: Identity,
{
    fn new(grant: Grant<'guard, Id>, value: T) -> Self {
        Self { value, grant }
    }
}

impl<'guard, Id, T> Trusted<'guard, Id, T>
where
    T: Copy,
    Id: Identity,
{
    pub fn value(&self) -> T {
        self.value
    }
}

impl<'guard, Id, T> AsRef<T> for Trusted<'guard, Id, T>
where
    Id: Identity,
{
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<'guard, Id, T> Deref for Trusted<'guard, Id, T>
where
    Id: Identity,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'guard, Id, T> Drop for Trusted<'guard, Id, T>
where
    Id: Identity,
{
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn zero_sized() {
        assert_eq!(0, mem::size_of::<InvariantLifetime>());

        let mut guard = Guard;
        let auth = Auth::new(&mut guard, identity!());
        assert_eq!(0, mem::size_of_val(&auth));

        let grant = auth.grant();
        assert_eq!(0, mem::size_of_val(&grant));

        let trusted = grant.to(());
        assert_eq!(0, mem::size_of_val(&trusted));
    }
}
