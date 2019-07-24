#[macro_use]
extern crate criterion;

extern crate untrusted;

pub mod signature {
    pub mod ecdsa {
        use ring::{rand, signature};
        use criterion::Criterion;

        pub fn generate_key_pair(c: &mut Criterion) {
            let rng = rand::SystemRandom::new();
            c.bench_function("signature::ecdsa::generate_key_pair", move |b| {
                b.iter(|| {
                    signature::EcdsaKeyPair::generate_pkcs8(
                        &signature::ECDSA_P256_SHA256_FIXED_SIGNING,
                        &rng,
                    )
                    .unwrap();
                });
            });
        }

        pub fn sign_empty(c: &mut Criterion) {
            let rng = rand::SystemRandom::new();
            let key_pair = signature::EcdsaKeyPair::generate_pkcs8(
                &signature::ECDSA_P256_SHA256_FIXED_SIGNING,
                &rng,
            )
            .unwrap();
            let key_pair = signature::EcdsaKeyPair::from_pkcs8(
                &signature::ECDSA_P256_SHA256_FIXED_SIGNING,
                untrusted::Input::from(key_pair.as_ref()),
            )
            .unwrap();
            c.bench_function("signature::ecdsa::sign_empty", move |b| {
                b.iter(|| {
                    let signature = key_pair.sign(&rng, untrusted::Input::from(b""));
                    let _ = signature.as_ref();
                });
            });
        }
    }
}

criterion_group!(benches, signature::ecdsa::generate_key_pair, signature::ecdsa::sign_empty);
criterion_main!(benches);
