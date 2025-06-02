use anyhow::Result;
use sp1_sdk::{HashableKey, ProverClient, SP1ProofWithPublicValues, SP1VerifyingKey};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting proof verification...");

    // Read the serialized proof
    let proof_path = Path::new("output/full_proof.bin");
    println!("Reading proof from {:?}", proof_path);
    let mut proof_file = File::open(proof_path)?;
    let mut proof_bytes = Vec::new();
    proof_file.read_to_end(&mut proof_bytes)?;
    println!("Read {} bytes of proof data", proof_bytes.len());
    let proof: SP1ProofWithPublicValues = bincode::deserialize(&proof_bytes)?;
    println!("Successfully deserialized proof");

    // Read the verifier key
    let vk_path = Path::new("output/verifier_key.bin");
    println!("Reading verifier key from {:?}", vk_path);
    let mut vk_file = File::open(vk_path)?;
    let mut vk_bytes = Vec::new();
    vk_file.read_to_end(&mut vk_bytes)?;
    println!("Read {} bytes of verifier key data", vk_bytes.len());
    let vk: SP1VerifyingKey = bincode::deserialize(&vk_bytes)?;
    println!("Successfully deserialized verifier key");


    let groth16_proof = proof.proof.try_as_groth_16_ref().unwrap();
    println!("Verifier Key Hash: {:?}", groth16_proof.public_inputs[0]);
    println!("Public Input Hash: {:?}", groth16_proof.public_inputs[1]);

    // Create client and verify the proof
    println!("Initializing SP1 client...");
    let client = ProverClient::from_env();
    println!("Verifying proof...");
    client.verify(&proof, &vk)?;

    println!("Proof verification successful!");
    Ok(())
} 