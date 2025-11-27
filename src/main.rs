use std::io::{self, Write};

// Define a Task struct to hold task information
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

// Implement methods for Task
impl Task {
    fn new(id: usize, description: String) -> Task {
        Task {
            id,
            description,
            completed: false,
        }
    }

    fn display(&self) {
        let status = if self.completed { "✓" } else { " " };
        println!("[{}] {}. {}", status, self.id, self.description);
    }
}

// Main function
fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut next_id = 1;

    println!("🦀 Welcome to Rust Task Manager!");
    println!("Commands: add, list, complete, quit\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure prompt displays immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Parse command using pattern matching
        match input {
            cmd if cmd.starts_with("add ") => {
                let description = cmd[4..].to_string();
                tasks.push(Task::new(next_id, description));
                println!("✅ Task {} added!", next_id);
                next_id += 1;
            }
            "list" => {
                if tasks.is_empty() {
                    println!("📝 No tasks yet. Add one with 'add <task>'");
                } else {
                    println!("\n📋 Your Tasks:");
                    for task in &tasks {
                        task.display();
                    }
                    println!();
                }
            }
            cmd if cmd.starts_with("complete ") => {
                if let Ok(id) = cmd[9..].parse::<usize>() {
                    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                        task.completed = true;
                        println!("🎉 Task {} completed!", id);
                    } else {
                        println!("❌ Task {} not found", id);
                    }
                } else {
                    println!("❌ Invalid task ID");
                }
            }
            "quit" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => {
                println!("❓ Unknown command. Try: add, list, complete, quit");
            }
        }
    }
}