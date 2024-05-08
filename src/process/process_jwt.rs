use anyhow::Result;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn process_jwt_sign(sub: &str, aud: &str, exp: &str) -> Result<()> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret")?;
    let mut claims = BTreeMap::new();
    claims.insert("sub", sub);
    claims.insert("aud", aud);
    claims.insert("exp", exp);

    let token = claims.sign_with_key(&key)?;
    println!("{:?}", token);
    Ok(())
}

pub fn process_jwt_verify(token: &str) -> Result<()> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret")?;
    let claims: BTreeMap<String, String> = token.verify_with_key(&key)?;

    println!("{:?}", claims);
    Ok(())
}
