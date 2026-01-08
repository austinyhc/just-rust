# Implementation Plan: Rust String Types Deep Dive (TUI)

## Phase 1: Foundation and TUI Skeleton [checkpoint: 1dfce99]
- [x] Task: Scaffold `src/bin/string_lab.rs` with basic `ratatui` event loop f105def
- [x] Task: Implement TUI layout with Main Menu, Content Pane, and Memory Visualizer Pane c333383
- [x] Task: Create a base `StringType` trait or enum to handle different demonstrations 6093f83
- [x] Task: Conductor - User Manual Verification 'Phase 1: Foundation and TUI Skeleton' (Protocol in workflow.md) 1dfce99

## Phase 2: Core Strings and Memory Visualization
- [x] Task: Implement `String` and `&str` demonstration logic dfb4956
- [x] Task: Develop the Dynamic Memory Visualizer for stack/heap pointers and buffers 3811258
- [x] Task: Add C++ conceptual comparison notes for `std::string` and `std::string_view` 11fa468
- [~] Task: Write tests for memory layout calculation logic (e.g., pointer/len/cap offsets)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Core Strings and Memory Visualization' (Protocol in workflow.md)

## Phase 3: OS and Path Strings
- [ ] Task: Implement `PathBuf`/`Path` and `OsString`/`OsStr` demonstration logic
- [ ] Task: Update Visualizer to handle potentially non-UTF8 data in `OsString`
- [ ] Task: Add C++ conceptual comparison notes for `std::filesystem::path`
- [ ] Task: Write tests for path manipulation and conversion logic
- [ ] Task: Conductor - User Manual Verification 'Phase 3: OS and Path Strings' (Protocol in workflow.md)

## Phase 4: FFI Strings and Byte Buffers
- [ ] Task: Implement `CString` and `CStr` demonstration logic
- [ ] Task: Implement `Vec<u8>` and `&[u8]` (Byte string) demonstration logic
- [ ] Task: Add C++ conceptual comparison notes for `char*` and `std::vector<uint8_t>`
- [ ] Task: Write tests for FFI string safety and null-termination checks
- [ ] Task: Conductor - User Manual Verification 'Phase 4: FFI Strings and Byte Buffers' (Protocol in workflow.md)

## Phase 5: Advanced Types and Final Polish
- [ ] Task: Implement `Cow<'a, str>` demonstration (Clone-on-Write)
- [ ] Task: Refactor UI for smooth transitions between categories
- [ ] Task: Final review of all C++ comparison documentation for accuracy
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Advanced Types and Final Polish' (Protocol in workflow.md)
