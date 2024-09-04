use bls12_381::{pairing, G1Affine, G2Affine, G2Prepared, Scalar};
use sp1_zkvm::prelude::*;

// Import the generated constants
mod generated_constants;
use generated_constants::{NUM_SIGNATURES, PUBLIC_KEYS, SIGNATURES, VERIFICATION_KEYS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if SIGNATURES.is_none() || PUBLIC_KEYS.is_none() || VERIFICATION_KEYS.is_none() {
        return Err("Missing generated constants".into());
    }

    let signatures = SIGNATURES.as_ref().unwrap();
    let public_keys = PUBLIC_KEYS.as_ref().unwrap();
    let verification_keys = VERIFICATION_KEYS.as_ref().unwrap();

    for i in 0..NUM_SIGNATURES {
        let sig = &signatures[i];
        let pk = &public_keys[i];
        let vk = &verification_keys[i];

        let sig_scalar = Scalar::from_bytes(sig.try_into()?).unwrap();
        let pk_point = G1Affine::from_compressed(pk.try_into()?).unwrap();
        let vk_point = G2Affine::from_compressed(vk.try_into()?).unwrap();

        let sig_point = (G2Affine::generator() * sig_scalar).to_affine();
        let is_valid = pairing(&G1Affine::generator(), &sig_point) == pairing(&pk_point, &vk_point);
        if !is_valid {
            return Err(format!("Invalid signature at index {}", i).into());
        }
    }

    println!("All signatures are valid!");
    Ok(())
}
