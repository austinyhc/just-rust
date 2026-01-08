# Specification: Rust String Types Deep Dive (TUI)

## Overview
This track involves creating an interactive TUI (Text User Interface) application designed to explore and visualize the various string types in Rust. The focus is on understanding memory layout (Stack vs. Heap), ownership, and transitions between types, specifically targeting an audience familiar with C and C++.

## Functional Requirements
- **Interactive TUI:** Build a multi-pane TUI using `ratatui`.
- **Comprehensive Type Support:** Demonstrate the following types:
    - `String` vs `&str`
    - `PathBuf` vs `Path`
    - `OsString` vs `OsStr`
    - `CString` vs `CStr`
    - `Cow<'a, str>`
    - `Vec<u8>` vs `&[u8]` (Byte strings)
- **Dynamic Memory Visualizer:** 
    - A dedicated pane that renders a text-based diagram of the stack and heap layout for a user-provided string.
    - Visualize pointers, lengths, capacities, and the actual data buffer.
- **C++ Conceptual Mapping:**
    - Provide side-by-side conceptual explanations in the UI or code comments comparing Rust types to C++ equivalents (e.g., `String` vs `std::string`, `&str` vs `std::string_view` or `char*`).
- **Type Transitions:** Interactive examples showing how to convert between these types (e.g., `&str` -> `String`, `OsString` -> `PathBuf`).

## Non-Functional Requirements
- **Idiomatic Rust:** Strict adherence to ownership and borrowing best practices.
- **Educational Clarity:** Focus on visual clarity in the memory diagrams.
- **Performance:** The TUI should be responsive to user input.

## Acceptance Criteria
- [ ] TUI application compiles and runs as a standalone binary in `src/bin/string_lab.rs`.
- [ ] User can switch between different string type categories within the TUI.
- [ ] The memory visualizer accurately represents stack and heap allocation for user-provided input.
- [ ] Documentation/Comments clearly explain the C++ analogues for each Rust concept.
- [ ] The application handles non-UTF8 input gracefully where appropriate (e.g., `OsString`, `Vec<u8>`).

## Out of Scope
- Compiling or executing actual C++ code.
- Advanced lifetime visualization (focus remains on memory layout).
- Networked or file-system-heavy operations beyond basic `Path` demonstrations.
