# Product Guidelines

## Documentation Tone & Style
- **Philosophical Inquiry:** Documentation and code comments should go beyond technical "what" and "how." They should engage with the "why," reflecting the diverse and sometimes conflicting philosophies of the Technical Committee.
- **Dialectical Commentary:** Expect documentation to present multiple perspectives (e.g., a "Linus Torvalds" pragmatic view vs. a "Donald Knuth" emphasis on algorithmic beauty).

## Code Quality Standards
- **Strictly Idiomatic:** The project aims for high-quality, idiomatic Rust, utilizing `clippy` and `rustfmt` to enforce community standards.
- **Exploratory Pedagogy:** While the end goal is idiomatic code, the *process* allows for "intentional anti-patterns" followed by refactoring to understand the evolution and necessity of Rust's safety and design principles.

## Integration of the Technical Committee
- **Committee Insights:** Each major exercise must include a dedicated section highlighting relevant insights from committee members.
- **Philosophical Journaling:** Maintain a log of reflections on how various programming philosophies apply to the current Rust learning task.
- **Simulated Reviews:** Use the persona of committee members to conduct simulated code reviews, providing feedback that challenges architectural and design choices.

## Visual and TUI Design
- **Tooling:** Use `ratatui` for TUI experiments.
- **Aesthetic:** Adhere to "Simple and Beautiful" design principles, favoring clean, minimalist interfaces with clear hierarchies.
- **Functionality:** Prioritize layout clarity and intuitive interactions over visual complexity.

## Dependency Policy
- **Standard Library First:** To master Rust's foundations, prioritize the standard library. Use external crates only when the learning objective specifically requires them (e.g., learning a specific library or when the standard library is insufficient for a complex task).
