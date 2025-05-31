#![no_main]

sp1_zkvm::entrypoint!(main);

/// Computes the nth Fibonacci number
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

pub fn main() {
    // Read the input: (n, expected_fib_value)
    let n = sp1_zkvm::io::read::<u32>();
    let expected_value = sp1_zkvm::io::read::<u64>();
    
    // Compute the nth Fibonacci number
    let computed_value = fibonacci(n);
    
    // Verify that the computed value matches the expected value
    assert_eq!(computed_value, expected_value, "Fibonacci value mismatch");
    
    // Commit the public outputs
    sp1_zkvm::io::commit(&n);
    sp1_zkvm::io::commit(&computed_value);
} 