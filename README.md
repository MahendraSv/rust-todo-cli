# Todo App

A small interactive command-line todo application written in Rust.

The app keeps an in-memory list of todos while it is running. Each todo gets a
numeric ID, some text, and a completion status. From the menu, you can display
todos, add new items, delete existing items, mark items as complete, or quit.

## Project Structure

```text
.
├── Cargo.toml
├── src
│   ├── main.rs
│   └── todos.rs
└── README.md
```

## How It Works

`src/main.rs` contains the interactive program loop. It:

- Creates an empty `Vec<Todo>` to store todos.
- Tracks the next todo ID with `next_id`.
- Prints the menu repeatedly.
- Reads user input from standard input.
- Calls functions from `src/todos.rs` based on the selected menu option.

`src/todos.rs` contains the todo model and todo operations:

- `Todo` is the main struct with `id`, `text`, and `is_completed` fields.
- `show_todos` prints all todos or shows a message when the list is empty.
- `add_todo` creates a new incomplete todo and increments the next ID.
- `delete_todo` removes a todo by ID.
- `complete_todo` marks a todo as completed by ID.
- `display_options` prints the menu.

The `Todo` struct also implements `Display`, so todos are printed in a readable
format:

```text
[ ] 1 - Learn Rust
[✓] 2 - Build a CLI app
```

## Menu Options

When the app runs, it shows this menu:

```text
******* TODO APP *******
1 - Display Todos
2 - Add Todo
3 - Delete Todo
4 - Complete Todo
5 - Quit
```

## Running the App

Make sure Rust is installed, then run:

```bash
cargo run
```

Example flow:

```text
Choose an option: 2
Enter todo text: Learn ownership
Todo added.

Choose an option: 1
[ ] 1 - Learn ownership

Choose an option: 4
Enter ID to complete: 1
Todo completed

Choose an option: 1
[✓] 1 - Learn ownership
```

## Notes

- Todos are stored only in memory, so they are lost when the program exits.
- Invalid menu options are handled with an `Invalid option.` message.
- Empty todo text is rejected.
- Invalid numeric input for IDs is rejected.

## Dependency

The project currently lists `crossterm` in `Cargo.toml`, but the current source
code does not use it yet. The app works as a standard terminal program using
Rust's standard input and output APIs.
