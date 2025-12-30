use rsa::{RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;

pub struct User {
    pub id: usize,
    pub private_key: RsaPrivateKey,
    pub public_key: RsaPublicKey,
}

impl User {
    pub fn new(id: usize) -> Self {
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);

        User {
            id,
            private_key,
            public_key,
        }
    }
}