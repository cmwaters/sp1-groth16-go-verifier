# ZK Compatibility System

This project demonstrates a complete zero-knowledge proof system that:

1. **SP1 Circuit**: Proves that a given number `x` is the nth Fibonacci number
2. **Rust Prover**: Generates SP1 proofs and compresses them using Groth16
3. **Go Verifier**: Verifies the Groth16 proofs using gnark's implementation

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   SP1 Circuit   │───▶│   Rust Prover    │───▶│  Go Verifier    │
│  (Fibonacci)    │    │ (Groth16 Comp.)  │    │ (gnark Groth16) │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## Components

### 1. SP1 Circuit (`program/`)
- **Language**: Rust (compiled to RISC-V)
- **Function**: Proves that `x` is the nth Fibonacci number
- **Input**: `(n, expected_fib_value)`
- **Output**: Commits `n` and `computed_fib_value` as public values

### 2. Rust Prover (`src/main.rs`)
- **Function**: 
  - Generates SP1 proof for the Fibonacci circuit
  - Compresses the proof using Groth16
  - Outputs proof components to binary files
- **Dependencies**: SP1 SDK, clap, tokio, bincode
- **Output Files**:
  - `output/proof.bin` - Groth16 proof
  - `output/public_inputs.bin` - Public inputs
  - `output/verifier_key.bin` - Verifier key

### 3. Go Verifier (`verifier/`)
- **Function**: Reads proof components and verifies using gnark
- **Dependencies**: gnark, gnark-crypto
- **Input**: Binary files from Rust prover
- **Output**: Verification result

## Prerequisites

1. **Rust** with SP1 toolchain:
   ```bash
   curl -L https://sp1.succinct.xyz | bash
   sp1up
   ```

2. **Go** (version 1.19+):
   ```bash
   # Install Go from https://golang.org/dl/
   ```

## Usage

### Step 1: Build the SP1 Circuit

```bash
cd program
cargo prove build
cd ..
```

### Step 2: Generate Proof

Run the Rust prover to generate a proof for the 10th Fibonacci number:

```bash
cargo run --bin fibonacci-prover -- --index 10
```

This will:
- Compute Fibonacci(10) = 55
- Generate an SP1 proof that 55 is indeed the 10th Fibonacci number
- Compress the proof using Groth16
- Save proof components to `output/` directory

### Step 3: Verify Proof

Run the Go verifier to verify the Groth16 proof:

```bash
cd verifier
go run main.go
```
