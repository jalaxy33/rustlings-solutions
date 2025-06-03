fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}

// Hint
// `for` loops over `Option` values are more clearly expressed as an `if-let`
// statement.

// Not required to solve this exercise, but if you are interested in when iterating
// over `Option` can be useful, read the following section in the documentation:
// https://doc.rust-lang.org/std/option/#iterating-over-option
