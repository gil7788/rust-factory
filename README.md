# Rust Factory

## Overview

`rust-factory` is a command-line application built in Rust that allows users to manage a to-do list through a simple interface. Users can create, edit, delete, and view to-do items, which are stored in a JSON file. This project demonstrates basic Rust concepts, file handling, and the use of external crates such as `serde_json` for JSON serialization/deserialization.

## Features

- **Create To-Do Items**: Add new items to your to-do list.
- **Edit To-Do Items**: Update the status of existing items.
- **Delete To-Do Items**: Remove items from your to-do list.
- **View To-Do Items**: Display the details of to-do items.

## Dependencies

- Rust Programming Language
- Cargo (Rust's package manager)
- `serde` and `serde_json` for JSON handling

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed on your system. You can install both by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/gil7788/rust-factory.git
   cd rust-factory
   ```
2. Build the project:
   ```bash
   cargo build
   ```

### Running the Application

To run the application, use the Cargo run command followed by specific arguments based on the action you want to perform (`get`, `edit`, `delete`, `create`), the title of the to-do item, and its status:

```bash
cargo run [COMMAND] [TODO_TITLE] [TODO_STATUS]
```

#### Commands

- **get**: Display the details of a specific to-do item.
- **edit**: Update the status of a to-do item.
- **delete**: Remove a to-do item from the list.
- **create**: Add a new to-do item to the list.

#### Example

To add a new to-do item with the title "Learn Rust" and the status "pending":

```bash
cargo run create "Learn Rust" pending
```

## File Structure

- **src/main.rs**: The entry point of the application.
- **src/state.rs**: Handles reading from and writing to the state file.
- **src/processes.rs**: Contains logic for processing user commands.
