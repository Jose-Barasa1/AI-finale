# 🦀 Rust Task Manager CLI

Simple command-line task manager built for Moringa AI Capstone Project.

## Quick Start

### Prerequisites
- Rust installed (get it at https://rustup.rs/)

### Run the App
```bash
cargo run
```

## Commands
- `add <task>` - Add a new task
- `list` - Show all tasks  
- `complete <id>` - Mark task as done
- `quit` - Exit

## Example Usage
```
> add Learn Rust ownership
✅ Task 1 added!
> add Build CLI app
✅ Task 2 added!
> list

📋 Your Tasks:
[ ] 1. Learn Rust ownership
[ ] 2. Build CLI app

> complete 1
🎉 Task 1 completed!
> quit
👋 Goodbye!
```

## Full Documentation
See the complete toolkit document in the submission.

## Tech Stack
- Rust 1.75+
- Standard library only (no external dependencies)