#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Hint
// To find the answer, you can consult the book section "References and Borrowing":
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// The first problem is that `get_char` is taking ownership of the string. So
// `data` is moved and can't be used for `string_uppercase`. `data` is moved to
// `get_char` first, meaning that `string_uppercase` can't manipulate the data.

// Once you've fixed that, `string_uppercase`'s function signature will also need
// to be adjusted.
