# A.L.F.R.E.D. CLI

![Rust](https://img.shields.io/badge/-Rust-000000?style=flat&logo=rust&logoColor=white)
![TypeScript](https://img.shields.io/badge/-TypeScript-007ACC?style=flat&logo=typescript&logoColor=white)

**A.L.F.R.E.D.** (**A**gentic **L**ocal **F**ramework for **R**epository **E**nhancement and **D**evelopment) is a cross-platform, open-source terminal agent designed to assist with software engineering tasks directly from your shell.

## Overview

Alfred CLI provides a modern, interactive terminal user interface (TUI) built with [Ink](https://github.com/vadimdemedes/ink). It features a pluggable agent core that connects to various LLM providers to perform tasks such as:

*   **Interactive Chat:** coding assistance, debugging, and general inquiries.
*   **File Editing:** Reading, writing, and patching files within your local repository.
*   **Tool Execution:** Running shell commands, git operations, and custom tools safely.
*   **RAG (Retrieval-Augmented Generation):** Indexing and searching your codebase and documentation for context-aware answers.

## Architecture

The project is structured as a monorepo with the following packages:

*   **`@alfred/cli`**: The TUI entry point, command router, and UI components.
*   **`@alfred/core`**: The agent runtime, handling conversation state, planning, and orchestration.
*   **`@alfred/tools`**: A collection of tools for filesystem access, shell execution, and git operations.
*   **`@alfred/rag`**: The retrieval layer for indexing and searching code and documents.
*   **`@alfred/ext`**: Extension loader for custom tools and user-defined slash commands.

## Getting Started

### Prerequisites

*   Node.js (v18 or higher)
*   npm (v9 or higher)

### Installation

```bash
git clone https://github.com/AIByJohannes/alfred-cli.git
cd alfred-cli
npm install
npm run build
```

### Running Tests

```bash
npm test
```

