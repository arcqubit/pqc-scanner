use rsa::{RsaPrivateKey, RsaPublicKey};
use ecdsa::SigningKey;

fn main() {
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
    let signing_key = SigningKey::random(&mut rng);
}
