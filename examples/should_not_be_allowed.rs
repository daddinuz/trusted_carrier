#[allow(unused)]
use trusted_carrier::*;

fn main() {
    // Tokens should not outlive auth
    // (anyway auth is unique and it has been dropped so the token is now useless)
    let _token = {
        let auth = trusted_carrier::auth!();
        auth.grant_once()
    };
}
