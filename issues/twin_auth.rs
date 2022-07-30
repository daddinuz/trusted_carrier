use trusted_carrier::*;

fn main() {
    // Issue: Twin Auth (https://github.com/daddinuz/trusted_carrier/issues/1#issue-1323199006)
    // It's possible to construct two or more Auth(s) with the very same type.
    // In this way one could forge malicious Grant(s), thus breaking safety assumptions
    // based on Auth uniqueness on which client code relies upon.
    let (a1, a2) = twin_auth(cast(|x| x));
    same_type(a1, a2);
}

trait TwinIdentity: Copy + Identity {}

impl<T> TwinIdentity for T where T: Copy + Identity {}

fn twin_auth<'id, Id: TwinIdentity>(id: Id) -> (Auth<'id, Id>, Auth<'id, Id>) {
    (Auth::new(id), Auth::new(id))
}

fn cast<Id>(id: Id) -> impl TwinIdentity
where
    Id: Copy + for<'a> FnOnce(InvariantLifetime<'a>) -> InvariantLifetime<'a>,
{
    id
}

fn same_type<T>(_: T, _: T) {}
