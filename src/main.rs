use std::io::{ self, Write };

mod todos;

fn read_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().to_string()
}

fn read_usize(prompt: &str) -> Option<usize> {
    read_input(prompt).parse::<usize>().ok()
}

fn main() {
    let mut todos: Vec<todos::Todo> = Vec::new();
    let mut next_id = 1;

    loop {
        todos::display_options();

        let opton = read_input("");

        match opton.as_str() {
            "1" => {
                todos::show_todos(&todos);
            }
            "2" => {
                let text = read_input("Enter todo text: ");
                if text.is_empty() {
                    println!("Todo cannot be empty.");
                    continue;
                }
                todos::add_todo(&mut todos, &mut next_id, text);
                println!("Todo added.")
            }
            "3" => {
                match read_usize("Enter ID to delete: ") {
                    None => println!("Invalid Id"),
                    Some(id) => {
                        if todos::delete_todo(&mut todos, id) {
                            println!("Todo deleted.");
                        } else {
                            println!("Todo not found");
                        }
                    }
                }
            }
            "4" => {
                match read_usize("Enter ID to complete: ") {
                    None => println!("Invalid ID."),
                    Some(id) => {
                        if todos::complete_todo(&mut todos, id) {
                            println!("Todo completed");
                        } else {
                            println!("Todo not found");
                        }
                    }
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option.");
            }
        }
    }
}