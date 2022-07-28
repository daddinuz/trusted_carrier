#[allow(unused)]
use trusted_carrier::*;

fn main() {
    /*
    // Auths must be unique
    fn assert_unique<'id, Id: Identity>(_: Auth<'id, Id>, _: Auth<'id, Id>) {}

    let auth1 = trusted_carrier::auth!();
    let auth2 = trusted_carrier::auth!();
    assert_unique(auth1, auth2);
    */

    /*
    // Auths must be constructed from unique identities
    fn assert_unique<'id, Id: Identity>(_: Auth<'id, Id>, _: Auth<'id, Id>) {}

    let id = |_| InvariantLifetime::new();
    let auth1 = Auth::new(id);
    let auth2 = Auth::new(id);
    assert_unique(auth1, auth2);
    */

    /*
    // Identity cannot be unified
    fn assert_unique<Id: Identity>(_: Id, _: Id) {}

    fn forge_unified() -> impl Identity {
        |_| InvariantLifetime::new()
    }

    let id1 = forge_unified();
    let id2 = forge_unified();
    assert_unique(id1, id2);
    */

    /*
    // Auth cannot be forged from tokens
    let auth = trusted_carrier::auth!();
    let token = auth.grant_once();
    let forged = forge(token);

    fn forge<'id, Id: Identity>(token: OnceToken<'id, Id>) -> Auth<'id, Id> {
        Auth::new(|_| InvariantLifetime::new())
    }
    */
}
