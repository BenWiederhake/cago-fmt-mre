fn run_test(instruction_prefix: &[u16], max_steps: usize) {
    println!(
        "Got: {:?}, {:?}",
        instruction_prefix, max_steps
    );
}

fn main() {
    run_test(
        &[
                    // asdf
            0x31,   // ↓
            0x4100, // lw r1, a
            0x32,   // ↓
                    // asdf
            0x4200, // lw r2, b
            0x60,   // binary.function r2, r1
                    // asdf
        ],
        5,
    );
}
