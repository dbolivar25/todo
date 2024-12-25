# Todo CLI

A feature-rich command-line todo application built in Rust, featuring both CLI
commands and an interactive REPL mode. This project demonstrates working with
SQLite database integration using `rusqlite` and interactive command-line
interfaces using `reedline`.

## Features

- Command-line and interactive REPL interface
- SQLite-backed persistent storage
- Task prioritization with weights (low, medium, high)
- Start dates and deadlines for tasks
- Task completion tracking
- Flexible task listing with sorting and filtering options

## Installation

The application stores its database in `~/.todo/todos.db` using SQLite with WAL
journaling mode.

```bash
cargo install --path .
```

## Usage

The application can be used either through direct CLI commands or in interactive
REPL mode.

### CLI Mode

Add a new task:

```bash
todo add "Complete documentation" --weight high --description "Write project documentation" --days-to-complete 7
```

List all tasks:

```bash
todo list
```

List completed tasks:

```bash
todo list --completed
```

Mark a task as complete:

```bash
todo complete "Complete documentation"
```

Edit a task:

```bash
todo edit "Complete documentation" --new-name "Update documentation" --weight medium
```

Remove a task:

```bash
todo remove "Update documentation"
```

### REPL Mode

Start the interactive REPL by running `todo` without any commands. The REPL
provides the same functionality as the CLI mode with command history and
auto-completion:

```bash
todo> add "Read book" --weight low
✓ Added new task: Read book
  Weight: low

todo> list
Tasks

[ ] Read book (low)
    Start: Not set
    Deadline: Not set
    Created: 2024-12-24 10:30
────────────────────────────────────────

todo> complete "Read book"
✓ Marked as complete: Read book
```

### Task Properties

- **Name**: Unique identifier for the task
- **Description**: Optional detailed description
- **Weight**: Priority level (low, medium, high)
- **Start Date**: Optional date when the task should begin
- **Deadline**: Optional completion deadline
- **Status**: Pending or completed

### List Filtering and Sorting

The `list` command supports various filtering and sorting options:

- `--weight <low|medium|high>`: Filter by priority
- `--completed`: Show only completed tasks
- `--sort-by-deadline`: Sort tasks by deadline
- `--sort-by-weight`: Sort tasks by priority weight

## Technical Details

- Built with Rust
- Uses `rusqlite` for SQLite database management
- Uses `reedline` for REPL functionality
- Uses `clap` for command-line and readline argument parsing
- Uses `chrono` for date/time handling
- Uses `thiserror` for error handling

## Project Structure

- `main.rs`: Application entry point and command execution
- `cli.rs`: Command-line interface definitions using `clap`
- `repo.rs`: Database operations and task management
- `prompt.rs`: REPL prompt customization
- `error.rs`: Error types and handling
