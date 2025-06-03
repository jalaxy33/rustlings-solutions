// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}

// Hint
// Rust requires that all parts of a function's signature have type annotations,
// but `call_me` is missing the type annotation of `num`.
