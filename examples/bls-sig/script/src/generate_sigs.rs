use bls12_381::{G1Projective, G2Projective, Scalar};
use rand::thread_rng;
use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
};

const NUM_SIGNATURES: usize = 10;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = thread_rng();
    let mut signatures = Vec::with_capacity(NUM_SIGNATURES);
    let mut public_keys = Vec::with_capacity(NUM_SIGNATURES);
    let mut verification_keys = Vec::with_capacity(NUM_SIGNATURES);

    for _ in 0..NUM_SIGNATURES {
        let sk = Scalar::random(&mut rng);
        let pk = G1Projective::generator() * sk;
        let vk = G2Projective::generator() * sk;
        let sig = sk.to_bytes();
        signatures.push(sig);
        public_keys.push(pk);
        verification_keys.push(vk);
    }

    let mut output = String::new();
    output.push_str("pub const SIGNATURES: [&[u8; 32]; NUM_SIGNATURES] = [\n");

    for sig in signatures {
        output.push_str("    &[");
        for byte in sig {
            output.push_str(&format!("0x{:02x}, ", byte));
        }
        output.push_str("],\n");
    }

    output.push_str("];\n\n");

    output.push_str("pub const PUBLIC_KEYS: [G1Projective; NUM_SIGNATURES] = [\n");

    for pk in public_keys {
        output.push_str("    G1Projective::from_bytes(&[");
        for byte in pk.to_bytes() {
            output.push_str(&format!("0x{:02x}, ", byte));
        }
        output.push_str("])?,\n");
    }

    output.push_str("];\n\n");

    output.push_str("pub const VERIFICATION_KEYS: [G2Projective; NUM_SIGNATURES] = [\n");

    for vk in verification_keys {
        output.push_str("    G2Projective::from_bytes(&[");
        for byte in vk.to_bytes() {
            output.push_str(&format!("0x{:02x}, ", byte));
        }
        output.push_str("])?,\n");
    }

    output.push_str("];\n\n");
    output.push_str(&format!("pub const NUM_SIGNATURES: usize = {};", NUM_SIGNATURES));

    // Read the target file
    let target_path = "../../program/src/main.rs";
    let mut target_content = fs::read_to_string(target_path)?;

    // Find the position after the last use statement
    let insert_pos = target_content
        .rfind("use ")
        .map(|pos| target_content[pos..].find(';').map(|p| pos + p + 1))
        .flatten()
        .unwrap_or(0);

    // Insert the generated constants
    target_content.insert_str(insert_pos, &format!("\n\n{}", output));

    // Write back to the target file
    let mut target_file = OpenOptions::new().write(true).truncate(true).open(target_path)?;
    target_file.write_all(target_content.as_bytes())?;

    println!("Constants written to {}", target_path);

    Ok(())
}
