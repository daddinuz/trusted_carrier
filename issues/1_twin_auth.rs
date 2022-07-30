use trusted_carrier::*;

fn main() {
    // (FIXED) Issue: Twin Auth (https://github.com/daddinuz/trusted_carrier/issues/1#issue-1323199006)
    // It's possible to construct two or more Auth(s) with the very same type.
    // In this way one could forge malicious Grant(s), thus breaking safety assumptions
    // based on Auth uniqueness on which client code relies upon.
    let mut guard = Guard;
    let (a1, a2) = twin_auth(&mut guard, copy_identity(|x| x));
    same_type(a1, a2);
}

trait CopyIdentity: Copy + Identity {}

impl<T> CopyIdentity for T where T: Copy + Identity {}

fn twin_auth<'guard, Id: CopyIdentity>(
    guard: &'guard mut Guard,
    id: Id,
) -> (Auth<'guard, Id>, Auth<'guard, Id>) {
    (Auth::new(guard, id), Auth::new(guard, id))
}

fn copy_identity<Id>(id: Id) -> impl CopyIdentity
where
    Id: Copy + for<'a> FnOnce(InvariantLifetime<'a>) -> InvariantLifetime<'a>,
{
    id
}

fn same_type<T>(_: T, _: T) {}
