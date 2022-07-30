#![allow(unused)]

use trusted_carrier::*;

fn main() {
    // Grant(s) should not outlive their Auth, anyway Guard prevents
    // the creation of new Auth(s) (until all grants are dropped).
    let mut guard = Guard;
    let identity = copy_identity(|x| x);

    let grant = {
        let auth = Auth::new(&mut guard, identity);
        auth.grant()
    };

    // The line below won't compile: cannot create a new Auth
    // with the same guard until all grants are dropped.
    //let auth = Auth::new(&mut guard, identity);

    // The lines below won't compile: same identity but different guards.
    //let mut new_guard = Guard;
    //let new_auth = Auth::new(&mut new_guard, identity);
    //same_type(grant, new_auth.grant());
}

fn same_type<T>(_: T, _: T) {}

trait CopyIdentity: Copy + Identity {}

impl<T> CopyIdentity for T where T: Copy + Identity {}

fn copy_identity<Id>(id: Id) -> impl CopyIdentity
where
    Id: Copy + for<'a> FnOnce(InvariantLifetime<'a>) -> InvariantLifetime<'a>,
{
    id
}
