// In this exercise, we are given a `Vec` of `u32` called `numbers` with values
// ranging from 0 to 99. We would like to use this set of numbers within 8
// different threads simultaneously. Each thread is going to get the sum of
// every eighth value with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, …
// The second thread (offset 1), will sum 1, 9, 17, …
// The third thread (offset 2), will sum 2, 10, 18, …
// …
// The eighth thread (offset 7), will sum 7, 15, 23, …
//
// Each thread should own a reference-counting pointer to the vector of
// numbers. But `Rc` isn't thread-safe. Therefore, we need to use `Arc`.
//
// Don't get distracted by how threads are spawned and joined. We will practice
// that later in the exercises about threads.

// Don't change the lines below.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: Define `shared_numbers` by using `Arc`.
    // let shared_numbers = ???;

    let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: Define `child_numbers` using `shared_numbers`.
        // let child_numbers = ???;

        let child_numbers = Arc::clone(&shared_numbers);

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}

// Hint
// Make `shared_numbers` be an `Arc` from the `numbers` vector. Then, in order
// to avoid creating a copy of `numbers`, you'll need to create `child_numbers`
// inside the loop but still in the main thread.

// `child_numbers` should be a clone of the `Arc` of the numbers instead of a
// thread-local copy of the numbers.

// This is a simple exercise if you understand the underlying concepts, but if this
// is too much of a struggle, consider reading through all of Chapter 16 in The
// Book:
// https://doc.rust-lang.org/book/ch16-00-concurrency.html
