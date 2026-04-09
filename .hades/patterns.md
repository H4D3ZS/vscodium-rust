# Coding Patterns
## Project-Specific Rules

### 1. Rust Backend
- Use `anyhow` for errors.
- Prefer `Arc<Mutex<T>>` for shared state across AI tools.
- All modifications MUST use `PatchEngine` search/replace logic.

### 2. Frontend React
- Functional components only.
- Prefer `framer-motion` for transitions.
- Monaco `DiffEditor` for all agentic code reviews.
