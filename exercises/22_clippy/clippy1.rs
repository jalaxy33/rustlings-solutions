// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises, the code will fail to compile when there are Clippy
// warnings. Check Clippy's suggestions from the output to solve the exercise.

use std::f32::consts::PI;

fn main() {
    // TODO: Fix the Clippy lint in this line.
    let pi = PI;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}

// Hint
// Rust stores the highest precision version of some long or infinite precision
// mathematical constants in the Rust standard library:
// https://doc.rust-lang.org/stable/std/f32/consts/index.html

// We may be tempted to use our own approximations for certain mathematical
// constants, but clippy recognizes those imprecise mathematical constants as a
// source of potential error.

// See the suggestions of the Clippy warning in the compile output and use the
// appropriate replacement constant from `std::f32::consts`.
