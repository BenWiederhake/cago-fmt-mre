fn run_test(instruction_prefix: &[u16], data_prefix: &[u16], max_steps: usize) {
    println!(
        "Got: {:?}, {:?}, {:?}",
        instruction_prefix, data_prefix, max_steps
    );
}

fn main() {
    let a: u16 = 15;
    let b: u16 = 13;
    let function: u16 = 42;
    let result: u16 = 1337;
    run_test(
        &[
                                        // asdf
            0x3100 | (a & 0xFF),        // ↓
            0x4100 | ((a >> 8) & 0xFF), // lw r1, a
            0x3200 | (b & 0xFF),        // ↓
                                        // asdf
            0x4200 | ((b >> 8) & 0xFF), // lw r2, b
            0x6012 | (function << 8),   // binary.function r2, r1
                                        // asdf
        ],
        &[],
        5,
    );
}
