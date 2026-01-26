# Rust-First Plan for Alfred CLI (Ratatui Edition)

Context: the current codebase is an early JS/TS skeleton with no implemented features. The goal is to move to a Rust-first architecture with ratatui for the TUI and a thin Node wrapper only for distribution.

## Objectives

- **Rust-first**: TUI, agent loop, tools, and RAG written in Rust; Node kept only for an installer/bridge.
- **Fast, portable UX**: ratatui + crossterm UI with responsive event handling on Linux/macOS/Windows.
- **Safe execution**: clear boundaries for shell/fs/git tools, sane defaults for config/secrets, and predictable error handling.
- **Simple distribution**: prebuilt binaries plus an npm installer that fetches the right target.

## Target Architecture

```
alfred-cli/
├── Cargo.toml
├── crates/
│   ├── alfred-cli/         # binary: ratatui UI, input/output loop
│   ├── alfred-core/        # agent loop, planner/router, session state
│   ├── alfred-tools/       # fs/git/shell abstractions
│   ├── alfred-rag/         # chunk/index/query pipeline
│   └── alfred-node-bridge/ # optional NAPI-RS bindings if interop is needed
└── packages/
    └── alfred/             # minimal npm package: downloads/executes the binary
```

## Assumptions & Parity

- No legacy features to port; define an initial capability set in Rust rather than replicating JS.
- Create a **parity checklist** as features land: UI views, commands, tool behaviors, config/secrets, logging/telemetry. Track must-have vs. nice-to-have for GA.

## Technology Choices

- **TUI**: `ratatui` + `crossterm`, `ratatui-textarea` for input.
- **Async**: `tokio`.
- **LLM**: `async-openai` (pluggable via trait for future providers); mock implementation for tests.
- **Serialization**: `serde`, JSON/RON for state/config.
- **Pathing**: `camino`/`dunce` for cross-platform paths.
- **Packaging**: `cargo-dist` (or `cross`) for binaries; npm postinstall downloader.

## Phased Plan

### Phase 0: Workspace Foundations
- Initialize Cargo workspace, crates, shared tooling (`rustfmt`, `clippy`), and CI skeleton (lint + tests).
- Define error taxonomy and logging/tracing setup; panic hooks and terminal restore for TUI.

### Phase 1: TUI Skeleton (ratatui)
- Implement layout with scrollback and input box; render loop with crossterm events.
- Mock agent channel to display fake streaming responses; backpressure via bounded channels.
- Graceful shutdown and terminal cleanup; basic keybindings and resize handling.

### Phase 2: Core & Tools
- `alfred-core`: agent loop scaffolding, message/session models, router trait for tools/RAG, cancellation support.
- `alfred-tools`: fs (read/write/list), shell (PTY and non-PTY), git (status/diff/apply); normalize paths and outputs; Windows-safe behavior.
- Config/secrets: load from XDG/AppData with env overrides; validation and redaction in logs.

### Phase 3: LLM & RAG Foundations
- LLM client using `async-openai` with timeout/retry/backoff; injectable mock for tests.
- RAG crate skeleton: chunker, embedder trait, index abstraction (choose backend later), in-memory prototype for dev.
- CLI hooks for ingest/query to exercise the pipeline headlessly.

### Phase 4: Integration & UX Pass
- Wire TUI to core/LLM/tools with bounded channels; surface status/errors and cancellation.
- Add minimal session persistence (history saved to disk) and configurable keybindings/colors.
- Optional telemetry (opt-in) and file-based logging with secret redaction.

### Phase 5: Testing & Quality
- Unit tests: tools (fs/git/shell), LLM client with mock, RAG chunk/index/query.
- Integration: agent loop with mocked LLM/tools; non-interactive TUI snapshot tests for key screens.
- CI matrix for linux/macos/windows; fmt + clippy gates.

### Phase 6: Distribution & Rollout
- `cargo-dist` builds with checksums; macOS codesign/notarize and Windows signing if available.
- npm package postinstall downloader with proxy/offline fallback and clear errors; cache downloads.
- Release/versioning: semantic versioning, changelog, GitHub releases with artifacts.
- Rollout: keep JS path as fallback during early beta; config flag to opt into Rust binary until GA.

## Acceptance Checklist (to fill as features land)

- TUI: scrollback, input, streaming responses, resize handling, keybindings documented.
- Core: agent loop runs with cancellation; structured errors surfaced to UI; logging/tracing works.
- Tools: fs/git/shell pass unit tests on linux/macos/windows; safe path handling.
- LLM/RAG: mockable clients; basic ingest/query path works; timeouts/retries validated.
- Distribution: binaries for three platforms downloadable via npm; terminal restored after crashes; installer handles offline/proxy.
- Docs: README updated for Rust build/run and npm install flow; migration notes for users.
