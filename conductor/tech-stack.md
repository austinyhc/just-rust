# Tech Stack

## Core Language & Tooling
- **Language:** Rust (Stable)
- **Build System & Package Manager:** Cargo
- **Compiler:** `rustc`
- **Edition:** 2021 (Inferred as current standard)

## Project Architecture
- **Structure:** Single Crate with Multiple Binaries. Individual exercises/tools are located in `src/bin/`.
- **Target Platform:** Host OS (Linux)

## Key Libraries & Frameworks
- **Standard Library:** Primary focus for fundamental learning.
- **TUI Framework:** `ratatui` for text-based user interfaces.
- **Asynchronous Runtime:** `tokio` (used selectively for learning concurrency and async patterns).
- **Ecosystem:** Selective use of industry-standard crates (e.g., `serde` for serialization, `clap` for CLI parsing) as needed for specific learning objectives.

## Development & Quality Assurance
- **Testing:** Cargo's built-in test runner.
- **Formatting:** `rustfmt` (Default settings).
- **Linting:** `clippy`.
- **Debugging:** `gdb` or `lldb` (Host OS standard).
