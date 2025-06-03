#![allow(clippy::needless_late_init)]

fn main() {
    // Reading uninitialized variables isn't allowed in Rust!
    // Therefore, we need to assign a value first.
    let x: i32 = 42;

    println!("Number {x}");

    // It is possible to declare a variable and initialize it later.
    // But it can't be used before initialization.
    let y: i32;
    y = 42;
    println!("Number {y}");
}

// Hint
// In this exercise, we have a variable binding that we've created in the `main`
// function, and we're trying to use it in the next line, but we haven't given it
// a value.

// We can't print out something that isn't there; try giving `x` a value!

// This is an error that can cause bugs that's very easy to make in any
// programming language -- thankfully the Rust compiler has caught this for us!
