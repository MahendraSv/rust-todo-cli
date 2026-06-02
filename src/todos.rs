use std::fmt;
use std::fmt::{Formatter};

#[derive(Debug)]
pub struct Todo {
    pub id: usize,
    pub text: String,
    pub is_completed: bool
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let status = if self.is_completed {
            "✓"
        } else {
            " "
        };
        write!(f, "[{}] {} - {}", status, self.id, self.text)
    }
}

pub fn show_todos(todos: &[Todo]) {
    if todos.is_empty() {
        println!("No todos found.");
        return;
    }

    for todo in todos {
        println!("{todo}");
    }
}

pub fn add_todo(todos: &mut Vec<Todo>, next_id: &mut usize, text: impl Into<String>) {
    todos.push(Todo {
        id: *next_id,
        text: text.into(),
        is_completed: false
    });
    *next_id += 1;
}

pub fn delete_todo (todos: &mut Vec<Todo>, id: usize) -> bool {
    let original_len = todos.len();
    todos.retain(|t| t.id != id);
    todos.len() != original_len
}

pub fn complete_todo(todos: &mut [Todo], id: usize) -> bool {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.is_completed = true;
        true
    } else {
        false
    }
}

pub fn display_options() {
    println!();
    println!("******* TODO APP *******");
    println!("1 - Display Todos");
    println!("2 - Add Todo");
    println!("3 - Delete Todo");
    println!("4 - Complete Todo");
    println!("5 - Quit");
    println!();
    print!("Choose an option: ");
}