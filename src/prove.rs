use anyhow::Result;
use clap::Parser;
use sp1_sdk::{ProverClient, SP1Stdin};
use std::fs;

/// Fibonacci proof generator and Groth16 compressor
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The index n to compute the nth Fibonacci number
    #[arg(short, long)]
    index: u32,
}

/// Computes the nth Fibonacci number (same logic as in the circuit)
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    let mut a = 0u64;
    let mut b = 1u64;
    
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    
    b
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let n = args.index;
    
    println!("Generating proof for Fibonacci number at index {}", n);
    
    // Compute the expected Fibonacci value
    let fib_value = fibonacci(n);
    println!("Fibonacci({}) = {}", n, fib_value);
    
    // Setup the prover client
    let client = ProverClient::from_env();
    
    // Setup the program
    let elf = include_bytes!("../program/target/elf-compilation/riscv32im-succinct-zkvm-elf/release/fibonacci-circuit");
    let (pk, vk) = client.setup(elf);
    
    // Setup the inputs
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);
    stdin.write(&fib_value);
    
    println!("Generating SP1 proof and compressing with Groth16...");
    
    // Compress the proof using Groth16
    let compressed_proof = client.prove(&pk, &stdin).groth16().run()?;
    
    println!("Groth16 proof completed!");
    
    // Create output directory
    fs::create_dir_all("output")?;
    
    // Extract and save the Groth16 proof components
    let proof_bytes = compressed_proof.bytes();
    let sp1_public_values = compressed_proof.public_values.clone();

    // Save just the groth16 proof
    fs::write("output/proof.bin", &proof_bytes)?;
    println!("Proof saved to output/proof.bin");

    // Save the entire proof struct
    fs::write("output/full_proof.bin", bincode::serialize(&compressed_proof)?)?;
    println!("Full proof saved to output/full_proof.bin");
    
    // Save public inputs (the committed values from the circuit)
    fs::write("output/sp1_public_inputs.bin", &sp1_public_values)?;
    println!("Public inputs saved to output/sp1_public_inputs.bin");

    // Save verifier key 
    fs::write("output/verifier_key.bin", bincode::serialize(&vk)?)?;
    println!("Verifier key saved to output/verifier_key.bin");
    
    println!("All files generated successfully!");
    println!("Proof components saved in output/ directory");

    Ok(())
}

