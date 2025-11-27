use std::io::{self, Write};
use colored::Colorize; // Import the Colorize trait
use prettytable::{Table, row}; // Removed unused `cell`

// Define a Task struct
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: usize, description: String) -> Task {
        Task {
            id,
            description,
            completed: false,
        }
    }

    fn status(&self) -> String {
        if self.completed {
            "✓".green().to_string()
        } else {
            " ".yellow().to_string()
        }
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut next_id = 1;

    println!("{}", "🦀 Welcome to Rust Task Manager!".bright_purple().bold());
    println!("{}", "Use the menu to manage tasks.\n".bright_blue());

    loop {
        println!("{}", "📌 Menu:".bright_blue().bold());
        println!("1️⃣  Add task");
        println!("2️⃣  List tasks");
        println!("3️⃣  Complete task");
        println!("4️⃣  Quit");
        print!("{}", "> ".bright_green());
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("{}", "Enter task description: ".bright_yellow());
                io::stdout().flush().unwrap();

                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();
                let desc = desc.trim();

                if desc.is_empty() {
                    println!("{}", "❌ Task description cannot be empty".red());
                } else {
                    tasks.push(Task::new(next_id, desc.to_string()));
                    println!("✅ Task {} added!", next_id);
                    next_id += 1;
                }
            }
            "2" => {
                if tasks.is_empty() {
                    println!("{}", "📝 No tasks yet. Add one first.".yellow());
                } else {
                    let mut table = Table::new();
                    table.add_row(row!["ID", "Description", "Status"]);
                    for task in &tasks {
                        table.add_row(row![task.id, task.description, task.status()]);
                    }
                    table.printstd();
                }
            }
            "3" => {
                print!("{}", "Enter task ID to complete: ".bright_yellow());
                io::stdout().flush().unwrap();

                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();

                if let Ok(id) = id_input.trim().parse::<usize>() {
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
            "4" => {
                println!("{}", "👋 Goodbye!".bright_magenta());
                break;
            }
            _ => {
                println!("{}", "❓ Invalid choice, please try again.".red());
            }
        }
        println!(); // spacing
    }
}
