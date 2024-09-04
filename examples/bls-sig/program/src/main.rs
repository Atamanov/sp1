#![no_main]

use bls12_381::{
    fp::Fp, fp2::Fp2, multi_miller_loop, pairing, G1Affine, G1Projective, G2Affine, G2Prepared,
    G2Projective, Scalar,
};
use ff::Field;
use group::Group;
use rand::thread_rng;

sp1_zkvm::entrypoint!(main);

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    for (sig, pk, vk) in SIGNATURES.iter().zip(PUBLIC_KEYS.iter()).zip(VERIFICATION_KEYS.iter()) {
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
