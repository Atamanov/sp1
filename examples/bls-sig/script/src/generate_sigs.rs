use bls12_381::{G1Projective, G2Projective, Scalar};
use rand::thread_rng;
use std::{fs::OpenOptions, io::Write};

const NUM_SIGNATURES: usize = 10;

fn main() {
    let mut rng = thread_rng();
    let mut signatures = Vec::with_capacity(NUM_SIGNATURES);
    let mut public_keys = Vec::with_capacity(NUM_SIGNATURES);
    let mut verification_keys = Vec::with_capacity(NUM_SIGNATURES);

    for _ in 0..NUM_SIGNATURES {
        let sk = Scalar::random(&mut rng);
        let pk = G1Projective::generator() * sk;
        let vk = G2Projective::generator() * sk;
        signatures.push(sk.to_bytes());
        public_keys.push(pk.to_bytes());
        verification_keys.push(vk.to_bytes());
    }

    let mut output = String::new();
    output.push_str("pub const SIGNATURES: Option<Vec<[u8; 32]>> = Some(vec![\n");

    for sig in &signatures {
        output.push_str("    [");
        for byte in sig {
            output.push_str(&format!("0x{:02x}, ", byte));
        }
        output.push_str("],\n");
    }

    output.push_str("]);\n\n");

    output.push_str("pub const PUBLIC_KEYS: Option<Vec<[u8; 48]>> = Some(vec![\n");

    for pk in &public_keys {
        output.push_str("    [");
        for byte in pk {
            output.push_str(&format!("0x{:02x}, ", byte));
        }
        output.push_str("],\n");
    }

    output.push_str("]);\n\n");

    output.push_str("pub const VERIFICATION_KEYS: Option<Vec<[u8; 96]>> = Some(vec![\n");

    for vk in &verification_keys {
        output.push_str("    [");
        for byte in vk {
            output.push_str(&format!("0x{:02x}, ", byte));
        }
        output.push_str("],\n");
    }

    output.push_str("]);\n\n");
    output.push_str(&format!("pub const NUM_SIGNATURES: usize = {};", NUM_SIGNATURES));

    // Write the generated constants to a file
    let out_dir = "../../program/src";
    let target_path = format!("{}/generated_constants.rs", out_dir);
    let mut target_file =
        OpenOptions::new().write(true).truncate(true).create(true).open(target_path).unwrap();
    target_file.write_all(output.as_bytes()).unwrap();

    println!("Constants written to {}", target_path);
}
