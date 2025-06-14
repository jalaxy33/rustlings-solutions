// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    // String::from("blue")
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}

// Hint
// The `current_favorite_color` function is currently returning a string slice
// with the `'static` lifetime. We know this because the data of the string lives
// in our code itself -- it doesn't come from a file or user input or another
// program -- so it will live as long as our program lives.

// But it is still a string slice. There's one way to create a `String` by
// converting a string slice covered in the Strings chapter of the book, and
// another way that uses the `From` trait.
