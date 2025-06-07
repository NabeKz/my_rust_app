# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

- `cargo run` - Start the web server on localhost:5000
- `mise run serve` - Start the server with file watching using watchexec (recommended for development)
- `mise run format` - Format and fix code with cargo fix
- `cargo build` - Build the project
- `cargo test` - Run tests

## Architecture Overview

This is a Rust web application built with Actix-web following a layered architecture pattern:

### Core Layers
- **Presentation Layer** (`src/presentation/`): HTTP handlers and HTML generation
- **Use Case Layer** (`src/features/*/usecase.rs`): Business logic and application services
- **Domain Layer** (`src/features/*/model.rs`): Domain entities and repository traits
- **Infrastructure Layer** (`src/features/*/infra/`): Repository implementations

### Key Architectural Patterns
- **Repository Pattern**: BookRepository trait with in-memory implementation
- **Dependency Injection**: Context struct provides shared repository instances via Arc
- **Server-Side Rendering**: Custom HTML generation with form helpers and method override support
- **Method Override**: POST forms with `_method` query parameter to simulate PUT/DELETE

### Application Structure
The app is organized around features (currently just "book") with each feature containing:
- `model.rs` - Domain entities and repository trait
- `usecase.rs` - Business logic functions
- `infra/` - Repository implementations
- Presentation handlers in `src/presentation/book/pages/`

### HTML Generation
Custom HTML generation system in `presentation/shared.rs` with:
- Form helpers for POST/PUT/DELETE operations
- Table generation utilities
- Input and validation helpers
- Built-in CSS styling

The server runs on localhost:5000 with routes for CRUD operations on books using HTML forms with method override support.