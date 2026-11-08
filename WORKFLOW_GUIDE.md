# Rune Contribution & UI Development Guide

This document records the end-to-end workflow, architectural insights, and practical component development implemented during this session for the **Rune** GPU-rendered UI framework.

---

## 1. Executive Summary

During this development session, we established a streamlined Git contribution pipeline, explored Rune's architecture, and implemented functional enhancements across multiple example applications in the repository:

1. **Contribution Workflow & Git Automation**:
   - Mastered the fast-forward-only `dev` branch model.
   - Configured an automatic Git `post-commit` hook to instantly synchronize local commits with GitHub.
   - Authored the Indonesian developer contribution guide (`CONTRIBUTING.id.md`).
   - Resolved upstream Pull Request collaborator restrictions by utilizing fork comparison branches.

2. **UI Component Implementations & Enhancements**:
   - **`examples/todo`**: Transformed from a static mock into a fully functional productivity application with:
     - Header progress statistics (`X of Y done`).
     - Conditional `"Clear completed"` action.
     - Accessible empty state card (`"All caught up! 🎉"`).
     - Component integration from `crates/rune-kit`: `Badge` (status tags) and `Divider` (GPU separator).
     - Interactive `TextInput` with dynamic task additions and input buffer reset.
   - **`examples/hello`**: Enhanced with an accent color palette picker (4 swatches) and a real-time flip counter.
   - **`examples/settings`**: Implemented an entirely new `"Notifications"` section with custom toggle switches and sound alerts.

3. **Codebase Navigation & Safety Classification**:
   - Identified low-risk ("Safe Zones") versus high-risk areas in the repository to guarantee stability and prevent breaking changes.

---

## 2. Git & GitHub Contribution Pipeline

### 2.1 Branching Strategy
- **Core Branch**: All development work targets `dev`. In accordance with `AGENTS.md`, `dev` is **fast-forward only** (never force-pushed or rewritten).
- **Feature Branch Pattern**:
  ```bash
  git checkout dev
  git pull origin dev
  git checkout -b feature-<name>
  ```

### 2.2 Automated GitHub Upload Hook
To streamline developer workflow, a Git `post-commit` hook was installed at `.git/hooks/post-commit`:
```sh
#!/bin/sh
branch=$(git rev-parse --abbrev-ref HEAD)
echo "--> Uploading commit to GitHub (branch: $branch)..."
git push -u origin "$branch"
```
Every `git commit` automatically triggers an upstream sync without requiring manual `git push` invocations.

### 2.3 Handling Upstream PR Restrictions
When contributing from a fork:
- The upstream repository (`anomalyco/rune`) restricts direct public Pull Requests in favor of agent-driven issue tickets.
- To maintain and track changes within personal forks:
  ```text
  Base Repository:  <your-username>/rune  (branch: dev)
  Head Repository:  <your-username>/rune  (branch: feature-<name>)
  ```
- Merging to `dev` is executed cleanly via fast-forward:
  ```bash
  git checkout dev
  git merge --ff-only feature-<name>
  git push origin dev
  git branch -d feature-<name>
  git push origin --delete feature-<name>
  ```

---

## 3. Architecture & UI Component Insights

### 3.1 The Three-Layer Design
- **`crates/rune`**: The low-level GPU engine (wgpu quad rendering, SDF shaders, spring physics, Taffy layout arena, and AccessKit accessibility).
- **`crates/rune-kit`**: Theme-agnostic, token-driven component library (`Button`, `Switch`, `Badge`, `Card`, `TextInput`, `Select`, `Dialog`).
- **`examples/`**: Self-contained demonstration applications that showcase features and headless testing capabilities.

### 3.2 The State-View-Action Pattern
Rune avoids Virtual DOM diffing overhead by using a retained tree driven by plain view functions:
```rust
struct State {
    count: i32,
}

fn view(s: &State, cx: &mut Cx<State>) -> El {
    let k = cx.kit();
    row()
        .child(text(format!("{}", s.count)).motion(motion::SNAPPY))
        .child(Button::new("+").on_press(|s: &mut State| s.count += 1).build(k))
}
```

### 3.3 Spring Physics vs Static Timers
Animations utilize continuous analytic springs (`Spring::new(duration, bounce)`). If a user taps an element mid-motion, the existing velocity is preserved mathematically rather than resetting the animation.

---

## 4. Summary of Code Changes Implemented

### 4.1 Indonesian Contribution Guide (`CONTRIBUTING.id.md`)
- Added comprehensive documentation in Indonesian explaining repository conventions, lightweight Rust GNU installation alternatives, and Git guidelines.

### 4.2 Todo Application Enhancements (`examples/todo/`)
- **`Cargo.toml`**: Added dependency on `crates/rune-kit`.
- **`src/main.rs`**:
  - Added `TextEdit` state to capture user keyboard input.
  - Added `rune_kit::TextInput` with input reset on task addition.
  - Added `rune_kit::Badge` indicating `"Done"` (Accent) and `"Pending"` (Subtle).
  - Added `rune_kit::Divider` separating controls from the virtual list.
  - Added dynamic empty state banner and `"Clear completed"` filter.

### 4.3 Hello Application Enhancements (`examples/hello/`)
- **`src/main.rs`**:
  - Added `flips: u32` tracking toggle interactions.
  - Added `color_index: usize` mapped to `tok::accent(i, true)`.
  - Added interactive palette swatches with active selection rings.

### 4.4 Settings Application Enhancements (`examples/settings/`)
- **`src/main.rs`**:
  - Added `notifications: bool` and `sound_alerts: bool` to `Settings`.
  - Added a new `"Notifications"` card section with toggle switches matching macOS-style design tokens.

---

## 5. Contribution Safety Matrix

| Zone | Path | Risk Level | Description |
| :--- | :--- | :--- | :--- |
| **Safe Zone** | `examples/*` | Low | Completely isolated demo applications. Ideal for UI experimentation. |
| **Safe Zone** | `site/*`, `*.md` | Low | Documentation and guides. Zero compile risk. |
| **Safe Zone** | `crates/rune-i18n` | Low | Localization strings and format catalogs. |
| **Moderate Zone** | `crates/rune-kit` | Medium | Reusable components. Must preserve public builder signatures. |
| **High-Risk Zone** | `crates/rune/src/renderer` | High | Low-level GPU shaders and memory pipelines. |
| **High-Risk Zone** | Core Structs | High | Types like `AppSpec`, `Env`, `TextStyle` must not add breaking public fields. |

---

## 6. Windows Rust Setup Recommendation
For developers on Windows seeking to compile and test applications without installing the heavy Microsoft Visual Studio C++ suite (5–10 GB):
1. Install `rustup-init.exe` from [rustup.rs](https://rustup.rs).
2. Choose **Option 2 (Customize installation)**.
3. Set **Default host triple** to `x86_64-pc-windows-gnu`.
4. Proceed with installation. This provides a lightweight GCC/MinGW toolchain capable of running `cargo run` directly.
