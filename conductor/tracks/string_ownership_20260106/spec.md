# Specification: The Ownership Laboratory

## Goal
Deepen understanding of Rust's core ownership model by implementing functions that demonstrate 'Move', 'Immutable Borrow', and 'Mutable Borrow' semantics.

## Requirements
- Implementation in `src/bin/ownership_lab.rs`.
- Implement a function `takes_ownership(s: String)` that demonstrates the value moving out of scope.
- Implement a function `borrows_string(s: &String)` that allows reading without taking ownership.
- Implement a function `borrows_mut_string(s: &mut String)` that modifies the string in place.
- Each function must be accompanied by philosophical commentary reflecting on memory safety and design choices.
- Adhere to the 'Baby Steps' approach: clear, isolated examples.
