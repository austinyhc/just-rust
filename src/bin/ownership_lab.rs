//! # Exercise: Ownership Laboratory
//!
//! Build a CLI tool to explore Ownership, Borrowing, and Moves through string transformations.
//!
//! ## Committee Insights
//!
//! *   **Graydon Hoare:** [Insight regarding safety, concurrency, or practical systems programming.]
//! *   **Niko Matsakis:** [Insight regarding the borrow checker, ownership, or async mechanics.]
//! *   **Linus Torvalds:** [Pragmatic insight regarding systems programming, performance, or maintainability.]
//! *   **Donald Knuth:** [Insight regarding algorithmic elegance, clarity, or mathematical structure.]
//! *   **Steve Jobs:** [Insight regarding simplicity, elegance, and user-centric design.]
//!
//! ## Philosophical Journal
//!
//! Ownership is the core innovation of Rust. By enforcing strict rules about
//! who 'owns' data at compile time, we eliminate entire classes of bugs (double free,
//! use-after-free) without the overhead of a garbage collector.

fn main() {
    let s1 = String::from("Hello Rust");

    // s1 is borrowed here. Ownership remains with main.
    let len = borrows_string(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // s1 is moved here. takes_ownership now owns the data.
    let s2 = takes_ownership(s1);

    // println!("{}", s1); // This would cause a compile error!
    println!("Ownership moved. New string: {}", s2);
}

/// Demonstrates moving ownership into a function.
///
/// *   **Graydon Hoare:** "Safety is not just about avoiding crashes; it's about
///     knowing exactly who is responsible for the data."
pub fn takes_ownership(s: String) -> String {
    println!("I now own: {}", s);
    s // Ownership is moved back to the caller
}

/// Demonstrates borrowing data without taking ownership.
///
/// *   **Niko Matsakis:** "References allow multiple parts of the code to read
///     data simultaneously, but the rules ensure that no one can change it while
///     others are reading."
pub fn borrows_string(s: &str) -> usize {
    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_takes_ownership() {
        let s = String::from("hello");
        let result = takes_ownership(s);
        assert_eq!(result, "hello");
        // s is moved here, cannot use it
    }

    #[test]
    fn test_borrows_string() {
        let s = String::from("hello");
        let len = borrows_string(&s); // Works because String derefs to &str
        assert_eq!(len, 5);
        assert_eq!(s, "hello");
    }
}
