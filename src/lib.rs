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

// To prevent forgery, Auth instances must be neither default nor trivially constructible.
// An Identity instance must be consumed in order to construct a new Auth.
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
    pub fn grant(&self) -> Grant<'id, Id> {
        Grant::new(self)
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

pub struct Grant<'id, Id>(PhantomData<Auth<'id, Id>>)
where
    Id: Identity;

impl<'id, Id> Grant<'id, Id>
where
    Id: Identity,
{
    fn new(_: &Auth<'id, Id>) -> Self {
        Self(PhantomData)
    }

    pub fn to<T>(self, data: T) -> Trusted<'id, Id, T> {
        Trusted::new(self, data)
    }
}

impl<'id, Id> Drop for Grant<'id, Id>
where
    Id: Identity,
{
    fn drop(&mut self) {}
}

pub struct Trusted<'id, Id, T>
where
    Id: Identity,
{
    value: T,
    #[allow(dead_code)]
    grant: Grant<'id, Id>,
}

impl<'id, Id, T> Trusted<'id, Id, T>
where
    Id: Identity,
{
    fn new(grant: Grant<'id, Id>, value: T) -> Self {
        Self { value, grant }
    }
}

impl<'id, Id, T> Trusted<'id, Id, T>
where
    T: Copy,
    Id: Identity,
{
    pub fn value(&self) -> T {
        self.value
    }
}

impl<'id, Id, T> AsRef<T> for Trusted<'id, Id, T>
where
    Id: Identity,
{
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<'id, Id, T> Drop for Trusted<'id, Id, T>
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

        let auth = auth!();
        assert_eq!(0, mem::size_of_val(&auth));

        let grant = auth.grant();
        assert_eq!(0, mem::size_of_val(&grant));

        let trusted = grant.to(());
        assert_eq!(0, mem::size_of_val(&trusted));
    }
}
