#[allow(unused)]
use trusted_carrier::{Badge, Unique};

fn main() {
    /*
    // Badges must be unique
    fn assert_unique<'id, Id: Unique>(_: Badge<'id, Id>, _: Badge<'id, Id>) {}

    let id1 = trusted_carrier::badge!();
    let id2 = trusted_carrier::badge!();
    assert_unique(id1, id2);
    */

    /*
    // Badges must be constructed from Unique seeds
    use trusted_carrier::InvariantLifetime;

    fn assert_unique<'id, Id: Unique>(_: Badge<'id, Id>, _: Badge<'id, Id>) {}

    let f = |_| InvariantLifetime::new();
    let id1 = Badge::new(f);
    let id2 = Badge::new(f);
    assert_unique(id1, id2);
    */

    /*
    // Unique cannot be unified
    use trusted_carrier::InvariantLifetime;

    fn assert_unique<U: Unique>(_: U, _: U) {}

    fn unify() -> impl Unique {
        |_| InvariantLifetime::new()
    }

    let id1 = unify();
    let id2 = unify();
    assert_unique(id1, id2);
    */
}
