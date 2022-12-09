fn some_func(foo: &[u16], bar: usize) {
    println!("Got: {:?}, {:?}", foo, bar);
}
fn main() {
    some_func(
        &[
                    // Let's say I have a bunch of numbers.
            0,      // For example, a short 0. But lots of comments are needed.
            0x1234, // Or a slightly longer value.
            0xFF,   // Fun fact: My personal use-case is providing hand-written
                    // assembly, and not all directives translate to
                    // instructions (e.g. labels).
            0x666,  // Wouldn't it be nice if all comments were aligned?
            0x42,   // However, cargo-fmt strongly disagrees with that.
                    // Hence, this minimal reproducing example.
        ],
        5, // Necessary, for some reason.
    );
}
