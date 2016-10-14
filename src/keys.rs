use std::fmt;
use std::str::FromStr;

use super::base58;
use ring;
use ring::rand;

pub struct PrivateKey {
    bytes: [u8; 32],
}

impl PrivateKey {
    pub fn generate(rng: &rand::SecureRandom) -> Result<PrivateKey, ring::error::Unspecified> {
        let mut key = PrivateKey{ bytes: [0; 32] };
        try!(rng.fill(&mut key.bytes));
        Ok(key)
    }

    pub fn public_key(&self) -> PublicKey {
        let mut key = PublicKey{ bytes: [0; 32] };
        unsafe { GFp_x25519_public_from_private(key.bytes.as_mut_ptr(), self.bytes.as_ptr()); }
        key
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", base58::encode_slice(&self.bytes))
    }
}

impl FromStr for PrivateKey {
    type Err = ();
    fn from_str(s: &str) -> Result<PrivateKey, ()> {
        let mut key = PrivateKey{ bytes: [0; 32] };
        try!(base58::decode_slice_to(s, &mut key.bytes));
        Ok(key)
    }
}

pub struct PublicKey {
    bytes: [u8; 32],
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", base58::encode_slice(&self.bytes))
    }
}

impl FromStr for PublicKey {
    type Err = ();
    fn from_str(s: &str) -> Result<PublicKey, ()> {
        let mut key = PublicKey{ bytes: [0; 32] };
        try!(base58::decode_slice_to(s, &mut key.bytes));
        Ok(key)
    }
}

extern {
    fn GFp_x25519_public_from_private(public_key_out: *mut u8,
                                      private_key: *const u8);
}


#[cfg(test)]
mod tests {
    use super::PrivateKey;

    use std::str::FromStr;
    use ring::rand;

    #[test]
    fn test_generate_private_key() {
        let rng = rand::SystemRandom::new();

        let key = PrivateKey::generate(&rng);

        assert!(key.is_ok());

        println!("genkey:   `{}`", key.unwrap());
    }

    #[test]
    fn test_decode_private_key() {
        let s = "BzipR6YYxf4pPz97Rv9oHsVYSWbxg5vqMsVWwhRDdWeN";

        let key = PrivateKey::from_str(s);
        assert!(key.is_ok());

        let key = key.unwrap();
        assert_eq!(s, format!("{}", key));
    }

    #[test]
    fn test_generate_public_key_from_private() {
        let s = "BzipR6YYxf4pPz97Rv9oHsVYSWbxg5vqMsVWwhRDdWeN";

        let priv_key = PrivateKey::from_str(s).unwrap();

        assert_eq!(format!("{}", priv_key.public_key()),
                   "B24AJu9zFenYNaDBUmjg4Q5nuKnq1UHP9TuzoLy2vApG");

        assert_eq!(format!("{}", priv_key.public_key()),
                   "B24AJu9zFenYNaDBUmjg4Q5nuKnq1UHP9TuzoLy2vApG");
    }

}
