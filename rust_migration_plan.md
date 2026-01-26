# Rust-First Migration Plan for Alfred CLI (Ratatui Edition)

This document outlines the strategy for migrating Alfred CLI to a nearly **100% Rust architecture**. By using `ratatui` for the TUI, we eliminate the need for Node.js/Ink for the core experience, keeping only a minimal TypeScript wrapper for NPM distribution and installation.

## 1. Objectives

- **95%+ Rust Coverage**: The entire application—TUI, Agent Loop, RAG, and Tools—will be written in Rust.
- **Ratatui TUI**: Use `ratatui` + `crossterm` for a fast, memory-safe, and dependency-free terminal interface.
- **Native Performance**: No Node.js runtime overhead during core execution.
- **Optional NPM Bridge**: Maintain a thin TS entry point that downloads/executes the pre-compiled Rust binary.

## 2. Proposed Architecture

The entire project moves into a unified Rust workspace.

### 2.1 Rust Workspace

```text
alfred-cli/
├── Cargo.toml
├── crates/
│   ├── alfred-cli/           # BINARY: Ratatui TUI, Event Loop, View logic
│   ├── alfred-core/          # Agent Loop, Planner, Command Router, State Management
│   ├── alfred-tools/         # Tool implementations (FS, Git, Shell)
│   ├── alfred-rag/           # Vector/Lexical Search, Chunking, Indexing
│   └── alfred-node-bridge/   # OPTIONAL: NAPI-RS bindings if Node.js interop is needed
└── packages/
    └── alfred/               # Minimal NPM package. Installs the native binary.
```

## 3. Key Migration Shifts

- **TUI Framework**: Shift from React-style components (Ink) to immediate-mode rendering with `ratatui`.
- **Event Handling**: Use `crossterm` for cross-platform terminal events (keyboard, mouse, resize).
- **Concurrency**: Use `tokio` to manage the background Agent Loop while keeping the TUI responsive.

## 4. Migration Phases

### Phase 1: Rust TUI Prototype
1.  Initialize `crates/alfred-cli` with `ratatui`.
2.  Implement a basic chat interface with scrollable history and an input field.
3.  Set up the async bridge between the TUI thread and the Agent Core.

### Phase 2: Core Logic Porting
1.  Port `LLMClient` and Agent orchestration to `crates/alfred-core`.
2.  Re-implement Tools and RAG in their respective Rust crates.
3.  Integrate the Core into the Ratatui TUI.

### Phase 3: NPM Distribution
1.  Configure `cargo-dist` or similar to build binaries for Linux, macOS, and Windows.
2.  Create a thin `@alfred/cli` NPM package that uses a "postinstall" script or a binary downloader to fetch the correct native executable.

## 5. Technology Selection

- **TUI**: `ratatui` + `crossterm`.
- **Async**: `tokio`.
- **LLM**: `async-openai`.
- **State**: `ratatui-textarea` (for input), `serde` (for persistence).
- **Styling**: `ratatui`'s native styling and layout engine.

## 6. Verification
- **Responsiveness**: Ensure the TUI remains 60fps even during heavy RAG operations.
- **Portability**: Test the standalone binary on Linux, macOS, and Windows.
- **Installation**: Verify the NPM installation flow correctly deploys the native binary.
