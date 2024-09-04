#![no_main]

use bls12_381::{
    fp::Fp, fp2::Fp2, multi_miller_loop, pairing, G1Affine, G1Projective, G2Affine, G2Prepared,
    G2Projective, Scalar,
};
use ff::Field;
use group::Group;
use rand::thread_rng;

sp1_zkvm::entrypoint!(main);

// Optionally include the generated constants
include!("./generated_constants.rs");

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if SIGNATURES, PUBLIC_KEYS, and VERIFICATION_KEYS exist
    if SIGNATURES.is_none() || PUBLIC_KEYS.is_none() || VERIFICATION_KEYS.is_none() {
        return Err("SIGNATURES, PUBLIC_KEYS, or VERIFICATION_KEYS are missing".into());
    }

    let signatures = SIGNATURES.as_ref().unwrap();
    let public_keys = PUBLIC_KEYS.as_ref().unwrap();
    let verification_keys = VERIFICATION_KEYS.as_ref().unwrap();

    for (sig, pk, vk) in signatures.iter().zip(public_keys.iter()).zip(verification_keys.iter()) {
        let sig_scalar = Scalar::from_bytes(sig)?;
        let pk_point = G1Affine::from(pk);
        let vk_point = G2Prepared::from(vk);

        let is_valid = pairing(&pk_point, &vk_point)
            == pairing(
                &G1Affine::generator(),
                &G2Prepared::from(G2Projective::generator() * sig_scalar),
            );
        println!("Signature is valid: {}", is_valid);
    }

    Ok(())
}
