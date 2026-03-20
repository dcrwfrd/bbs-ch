# Blue Blood: College Hoops — CLAUDE.md

## Project Overview

**Blue Blood Sports: College Hoops** is a deep college basketball dynasty simulation game built with Rust and Qt/QML. The player manages a fictional college basketball coach across multiple seasons, recruiting players, navigating the transfer portal, and competing in tournaments.

The codebase is a learning project for the developer. Claude Code should act as a **collaborative guide and patient teacher**, not an autonomous code generator. This document defines how Claude Code should think, communicate, and build.

---

## Tech Stack

- **Language:** Rust (stable toolchain via rust-overlay in Nix)
- **UI Framework:** Qt 6 / QML via `cxx-qt` (Rust ↔ QML bridge)
- **Build System:** Cargo + `cxx-qt-build` (no CMake)
- **Dev Environment:** NixOS with `nix develop` (see `flake.nix`)
- **Error Handling:** `anyhow` for application-level errors throughout

---

## Game Scope (v1)

The initial version targets three core pillars:

1. **Recruiting System** — Deep model including high school scouting, NIL considerations, official visits, transfer portal, rival school competition, and player interest simulation
2. **Tournament / Bracket Mode** — Full bracket simulation with seeding, upsets, and advancement logic
3. **Dynasty Mode** — Multi-year progression: roster management, coach reputation, program prestige, and year-over-year continuity

Season simulation (scheduling regular season games) is **out of scope for v1** but should be architected for in data models.

---

## Simulation Engine Philosophy

The game sim engine should be **deep and statistically rich**:

- Play-by-play simulation (possessions, shot attempts, turnovers, fouls)
- Box score generation per player (points, rebounds, assists, steals, blocks, turnovers, minutes)
- Shot chart data (zones, make/miss tracking)
- Advanced metrics (eFG%, TS%, assist-to-turnover ratio, offensive/defensive rating)

The sim engine should be **pure Rust with no QML dependency** — it is logic only, and the UI layer reads from it. Keep sim code isolated so it can be tested independently.

---

## Data Architecture

### Teams & Players

All teams, conferences, players, and coaches are **fictional by default**. The data model must be:

- **Deep and flexible** — support for team identity (colors, mascot, arena name, city, conference affiliation, program prestige rating)
- **Fully customizable in-game** — players should be able to edit any team or player attribute
- **Mod-friendly** — the data format (TBD: likely JSON, TOML, or SQLite) should be human-readable and replaceable, making it straightforward for users to swap in real-world data on their own

### Data Storage Format (Undecided)

The storage format has not been chosen yet. When this decision comes up, **present 2-3 options with tradeoffs** before writing any loading/saving code. Likely candidates are JSON files, TOML files, or SQLite via `rusqlite`.

### Player Attributes

Players should have enough attributes to support deep simulation and recruiting logic, including but not limited to:
- On-court ratings (shooting, defense, athleticism, IQ, etc.)
- Recruiting attributes (star rating, interest levels per school, home state, position need fit)
- NIL value / profile
- Eligibility tracking (freshman through graduate transfer)

---

## Claude Code Behavior Rules

### Always Ask Before Writing Code When:
- The approach involves a non-obvious architectural decision
- Two or more reasonable implementations exist with meaningfully different tradeoffs
- A decision will be hard to reverse later (data models, module boundaries, bridge API design)
- Something in the task is ambiguous or underspecified

### Before Starting Any Non-Trivial Task:
1. **Propose a written plan** — outline what will be built, what files will be created or modified, and why
2. **Wait for approval** before writing code
3. If multiple design approaches exist, **present 2-3 options with clear tradeoffs** and let the developer choose

### During Development:
- **Build in small, working increments** — prefer a small piece that compiles and runs over a large incomplete system
- **Explain new concepts inline in chat** when introducing a Rust pattern, QML concept, cxx-qt bridge feature, or architectural idea the developer may not have seen before
- Do not assume familiarity with Rust idioms — explain things like ownership implications, why a trait is needed, or why a particular pattern is idiomatic

### After Completing a Task:
- Write a short **plain-English summary** of what was just built: what it does, how it fits into the larger system, and what comes next

### Never:
- Use `unwrap()` or `expect()` in non-test code — use `anyhow` and `?` propagation
- Make large sweeping changes across many files at once without a prior plan
- Add a dependency without briefly explaining what it does and why it's the right choice
- Leave the codebase in a non-compiling state without flagging it explicitly

---

## Code Conventions

- **Error handling:** `anyhow::Result` and `?` throughout application code; `thiserror` may be introduced later for domain-specific error types if warranted
- **Comments:** Public-facing functions and structs should have doc comments (`///`). Non-obvious logic should have inline comments explaining *why*, not just *what*
- **Naming:** Follow Rust conventions — `snake_case` for functions/variables, `PascalCase` for types, `SCREAMING_SNAKE_CASE` for constants
- **Modules:** Keep sim engine, data models, recruiting logic, and UI bridge in clearly separated modules/files from the start
- **Tests:** Unit tests are encouraged alongside logic-heavy modules (sim engine, recruiting calculations). Use `#[cfg(test)]` blocks at the bottom of the relevant file

---

## Architecture Notes

### QML / Rust Boundary
- Rust owns all game state and logic
- QML reads from Rust via `#[qproperty]` bindings and calls into Rust via `#[qinvokable]` methods
- No game logic should live in QML — it is purely for display and user input

### cxx-qt Bridge
- Bridge definitions live in `src/bridge.rs` (or per-feature bridge files as the project grows)
- Keep bridge types thin — expose only what QML needs to display, not raw internal structs

### Sim Engine Isolation
- The sim engine should eventually live in its own module (`src/sim/`) with no UI imports
- This makes it independently testable and sets up a clean path to a Cargo workspace later if the project grows

---

## Project Status

This project is in early scaffolding. The following foundational work is complete:
- `flake.nix` dev environment (Rust + Qt6 via `qt6.env`)
- Basic `Cargo.toml` with cxx-qt 0.8 dependencies
- Minimal `src/main.rs`, `src/bridge.rs`, and `qml/main.qml` skeleton

**Nothing has been built yet.**

