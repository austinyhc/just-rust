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
//! [Personal reflections on the learning process, the challenges faced, and how the
//! committee's advice influenced the implementation.]

fn main() {
    // Your implementation here
    println!("Exercise Ownership Laboratory initialized.");
}

pub fn takes_ownership(s: String) -> String {
    unimplemented!()
}

pub fn borrows_string(s: &String) -> usize {
    unimplemented!()
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
        let len = borrows_string(&s);
        assert_eq!(len, 5);
        assert_eq!(s, "hello"); // s is still available because it was borrowed
    }
}
