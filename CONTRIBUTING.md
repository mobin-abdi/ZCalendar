# Contributing to ZCalendar 🤝

Thank you for your interest in contributing to **ZCalendar**!

ZCalendar is an open-source Persian (Jalali) calendar for Linux, built with **Rust, GTK4, and Libadwaita**. Contributions of all kinds are welcome — from fixing bugs and improving the UI to adding new features and improving documentation.

---

## 📌 Before You Start

Before working on an issue, please:

1. Check the existing [Issues](../../issues) and [Pull Requests](../../pulls).
2. Make sure nobody is already working on the same issue.
3. If the issue is not assigned to anyone, leave a comment saying that you would like to work on it.
4. For larger changes, please discuss the idea with the maintainers before starting implementation.

This helps prevent duplicated work and keeps the project organized.

---

## 🛠️ Development Setup

### Requirements

You will need:

* Rust and Cargo
* GTK4 development libraries
* Libadwaita development libraries
* Git

Make sure Rust is installed and up to date.

Clone the repository:

```bash
git clone https://github.com/mobin-abdi/ZCalendar.git
cd ZCalendar
```

Build the project:

```bash
cargo build
```

Run it:

```bash
cargo run
```

---

## 🌿 Branches

Please avoid making changes directly on the `main` branch.

Create a new branch for your work:

```bash
git checkout -b feature/day-selection
```

Use a descriptive branch name.

Examples:

```text
feature/day-selection
feature/date-converter
feature/holidays
fix/calendar-layout
fix/date-calculation
docs/update-readme
```

---

## 💻 Making Changes

Before submitting a Pull Request:

### Format the code

```bash
cargo fmt
```

### Check the project

```bash
cargo check
```

### Run tests

```bash
cargo test
```

### Run Clippy

```bash
cargo clippy
```

Please make sure your changes do not introduce unnecessary warnings or break existing functionality.

---

## 📝 Commit Messages

ZCalendar follows a simple **Conventional Commits** style.

Examples:

```text
feat: add day selection
fix: correct calendar layout
docs: improve contributing guide
refactor: simplify calendar rendering
chore: update dependencies
```

Common prefixes:

| Prefix      | Purpose            |
| ----------- | ------------------ |
| `feat:`     | New feature        |
| `fix:`      | Bug fix            |
| `docs:`     | Documentation      |
| `refactor:` | Code restructuring |
| `test:`     | Tests              |
| `chore:`    | Maintenance        |

Keep commit messages short and descriptive.

### 🔐 Signed Commits

Signed commits are encouraged.

If you have commit signing configured, please sign your commits:

```bash
git commit -S -m "feat: add day selection"
```

Signed commits help contributors verify the identity associated with a commit.

---

## 🔀 Pull Requests

When your work is ready:

1. Push your branch to your fork.
2. Open a Pull Request against `main`.
3. Clearly explain what you changed.
4. Link the relevant issue.
5. Mention anything that still needs discussion or testing.

A good Pull Request should explain:

```text
What changed?
Why was it needed?
How was it tested?
```

For example:

> Closes #12

### Keep Pull Requests Focused

Please avoid mixing unrelated changes in the same PR.

A PR that fixes a calendar layout bug should not also completely rewrite the application architecture.

Small, focused PRs are easier to review and merge. ❤️

---

## 🐛 Reporting Bugs

When reporting a bug, please include:

* What happened
* What you expected to happen
* Steps to reproduce the problem
* Operating system and distribution
* Relevant application output or error messages
* Screenshots, if they help explain the problem

A good bug report makes it easier for everyone to reproduce and fix the issue.

---

## 💡 Suggesting Features

Feature ideas are welcome!

Before opening a feature request, please check whether a similar idea already exists.

When suggesting a feature, explain:

* What problem it solves
* How you imagine it working
* Why it would be useful to ZCalendar users

Not every proposed feature will necessarily be accepted. The goal is to keep ZCalendar useful, simple, and maintainable.

---

## 🎨 UI Contributions

ZCalendar is designed primarily for **Persian-speaking Linux users**.

When changing the UI, please consider:

* Persian text and readability
* Right-to-left layout
* Persian/Jalali calendar conventions
* Accessibility
* Different screen sizes
* GTK4 and Libadwaita design guidelines

Avoid introducing UI elements that make the application unnecessarily complicated.

---

## 🌍 Localization

Persian is the primary language of ZCalendar.

If localization support is expanded in the future, please keep user-facing strings easy to translate and avoid hard-coding text where a translation system is more appropriate.

---

## 📜 Code Style

Please follow normal Rust conventions and keep the code readable.

Prefer:

* Clear names
* Small, focused functions
* Simple solutions
* Comments only where they provide useful context
* Existing project patterns

Avoid unnecessary abstractions and over-engineering.

---

## 🤝 Code Review

Pull Requests may receive suggestions or requested changes.

This is a normal part of open-source development.

The goal of code review is to improve the project, not to criticize the contributor.

Please be open to discussion, explain your reasoning when necessary, and keep discussions respectful.

---

## ❤️ Thank You

Every contribution matters.

You can help ZCalendar by:

* Writing code
* Fixing bugs
* Improving the UI
* Testing
* Writing documentation
* Reporting issues
* Suggesting ideas
* Sharing the project

Even a small improvement can make ZCalendar better for someone else.

**Thank you for contributing to ZCalendar! 🗓️🦀**
