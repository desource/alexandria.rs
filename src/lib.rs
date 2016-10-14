extern crate ring;
extern crate byteorder;

use std::str::FromStr;
use ring::rand;

pub mod keys;
pub mod encrypt;
pub mod decrypt;
mod base58;


pub fn gen_key() -> Result<keys::PrivateKey, ()> {
    let rng = rand::SystemRandom::new();
    match keys::PrivateKey::generate(&rng) {
        Err(_) => Err(()),
        Ok(key) => Ok(key),
    }
}


pub fn pub_key(s: &str) -> Result<keys::PublicKey, ()> {
    let priv_key = try!(keys::PrivateKey::from_str(s));
    Ok(priv_key.public_key())
}
