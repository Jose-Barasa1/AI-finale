# Rust Programming Language - Beginner's Toolkit

## 1. Title & Objective

### What is Rust?
Rust is a modern systems programming language focused on safety, speed, and concurrency. It prevents common programming errors like null pointer dereferencing and data races at compile time, making it ideal for building reliable and efficient software.

### Why I Chose Rust
I selected Rust for my AI capstone project because:
- **Memory Safety**: Rust's ownership system prevents memory leaks and segmentation faults without requiring a garbage collector
- **Performance**: Rust provides performance comparable to C/C++ while being significantly safer
- **Growing Adoption**: Major companies like Microsoft, Amazon, and Google are investing heavily in Rust
- **Modern Tooling**: Cargo (Rust's package manager) provides an excellent developer experience
- **Career Relevance**: Rust consistently ranks as one of the most loved programming languages

### End Goal
By the end of this toolkit, I will build a **Simple Task Manager CLI Application** that can:
- Add tasks to a list
- Display all tasks
- Mark tasks as complete
- Save tasks to a file for persistence

---

## 2. Quick Summary

### What is Rust?
Rust is a statically-typed, compiled programming language that emphasizes performance, reliability, and productivity. It was originally designed at Mozilla Research and first released in 2010, with version 1.0 launching in 2015.

### Where is Rust Used?
- **Systems Programming**: Operating systems, device drivers, embedded systems
- **Web Development**: Backend services, WebAssembly applications
- **Networking**: High-performance servers and network tools
- **Blockchain**: Cryptocurrency platforms and smart contracts
- **CLI Tools**: Command-line utilities and developer tools

### Real-World Example
**Discord** (the popular communication platform) uses Rust for their Read States service, which handles billions of events per day. They switched from Go to Rust and saw significant performance improvements with reduced latency spikes. The Rust implementation handles the same workload with better memory efficiency and more predictable performance.

---

## 3. System Requirements

### Operating System
- **Linux**: Any modern distribution (Ubuntu 20.04+, Fedora 35+, Arch, etc.)
- **macOS**: macOS 10.12 (Sierra) or later
- **Windows**: Windows 7 or later (Windows 10/11 recommended)

### Tools & Editors Needed
- **Required**:
  - Terminal/Command Prompt
  - Internet connection (for installation)
  
- **Recommended Code Editors**:
  - Visual Studio Code (with rust-analyzer extension)
  - IntelliJ IDEA (with Rust plugin)
  - Sublime Text or Vim/Neovim (with appropriate plugins)

### Packages Required
- **Rust Toolchain**: Includes rustc (compiler), cargo (package manager), and rustup (toolchain manager)
- **C Compiler** (for linking): 
  - Linux: GCC or Clang (usually pre-installed)
  - macOS: Xcode Command Line Tools
  - Windows: Microsoft C++ Build Tools

---

## 4. Installation & Setup Instructions

### Linux / macOS

1. **Install Rust using rustup** (official installer):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Follow the on-screen prompts** (press 1 for default installation)

3. **Configure your current shell**:
```bash
source "$HOME/.cargo/env"
```

4. **Verify installation**:
```bash
rustc --version
cargo --version
```

Expected output:
```
rustc 1.xx.x (xxxxxxx 20xx-xx-xx)
cargo 1.xx.x (xxxxxxx 20xx-xx-xx)
```

5. **Install a code editor** (example for VS Code on Ubuntu):
```bash
sudo snap install code --classic
code --install-extension rust-lang.rust-analyzer
```

### Windows

1. **Download and install Microsoft C++ Build Tools**:
   - Visit: https://visualstudio.microsoft.com/visual-cpp-build-tools/
   - Install "Desktop development with C++" workload

2. **Install Rust**:
   - Visit: https://rustup.rs/
   - Download and run `rustup-init.exe`
   - Follow the installer prompts (choose default installation)

3. **Restart your terminal** (to update PATH variables)

4. **Verify installation** (in PowerShell or CMD):
```powershell
rustc --version
cargo --version
```

5. **Install VS Code** (optional but recommended):
   - Download from: https://code.visualstudio.com/
   - Install the `rust-analyzer` extension from the Extensions marketplace

### Additional Configuration (All Platforms)

**Update Rust to the latest version**:
```bash
rustup update
```

**Add common components**:
```bash
rustup component add clippy rustfmt
```
- `clippy`: A linting tool for catching common mistakes
- `rustfmt`: Code formatter for consistent styling

---

## 5. Minimal Working Example

### Simple Task Manager CLI Application

This application demonstrates core Rust concepts including structs, enums, file I/O, error handling, and user input.

**Create a new project**:
```bash
cargo new task_manager
cd task_manager
```

**Edit `src/main.rs` with the following code**:

```rust
// Import necessary modules from the standard library
use std::fs;
use std::io::{self, Write};

// Define a Task struct to represent individual tasks
#[derive(Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

// Define the TaskManager struct to manage our task list
struct TaskManager {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskManager {
    // Constructor: Create a new TaskManager with an empty task list
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // Add a new task to the list
    fn add_task(&mut self, description: String) {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✓ Task added successfully!");
    }

    // Display all tasks with their status
    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks yet. Add one to get started!");
            return;
        }

        println!("\n=== Your Tasks ===");
        for task in &self.tasks {
            let status = if task.completed { "✓" } else { " " };
            println!("[{}] {} - {}", status, task.id, task.description);
        }
        println!("==================\n");
    }

    // Mark a task as complete by its ID
    fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("✓ Task {} marked as complete!", id);
        } else {
            println!("✗ Task with ID {} not found.", id);
        }
    }

    // Save tasks to a file for persistence
    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut content = String::new();
        for task in &self.tasks {
            content.push_str(&format!(
                "{}|{}|{}\n",
                task.id, task.description, task.completed
            ));
        }
        fs::write(filename, content)?;
        println!("✓ Tasks saved to {}", filename);
        Ok(())
    }

    // Load tasks from a file
    fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        if let Ok(content) = fs::read_to_string(filename) {
            self.tasks.clear();
            let mut max_id = 0;

            for line in content.lines() {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() == 3 {
                    if let Ok(id) = parts[0].parse::<usize>() {
                        let task = Task {
                            id,
                            description: parts[1].to_string(),
                            completed: parts[2] == "true",
                        };
                        if id > max_id {
                            max_id = id;
                        }
                        self.tasks.push(task);
                    }
                }
            }
            self.next_id = max_id + 1;
            println!("✓ Tasks loaded from {}", filename);
        }
        Ok(())
    }
}

// Helper function to read user input
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure prompt is displayed
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.txt";

    // Try to load existing tasks
    let _ = manager.load_from_file(filename);

    println!("=== Welcome to Task Manager CLI ===\n");

    // Main application loop
    loop {
        println!("Commands:");
        println!("  1 - Add task");
        println!("  2 - List tasks");
        println!("  3 - Complete task");
        println!("  4 - Save & Exit");

        let choice = read_input("\nEnter your choice: ");

        match choice.as_str() {
            "1" => {
                let description = read_input("Enter task description: ");
                if !description.is_empty() {
                    manager.add_task(description);
                } else {
                    println!("✗ Task description cannot be empty.");
                }
            }
            "2" => {
                manager.list_tasks();
            }
            "3" => {
                let id_str = read_input("Enter task ID to complete: ");
                if let Ok(id) = id_str.parse::<usize>() {
                    manager.complete_task(id);
                } else {
                    println!("✗ Invalid ID. Please enter a number.");
                }
            }
            "4" => {
                let _ = manager.save_to_file(filename);
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("✗ Invalid choice. Please try again.");
            }
        }
        println!(); // Add spacing between iterations
    }
}
```

### Running the Application

**Compile and run**:
```bash
cargo run
```

### Expected Output

```
=== Welcome to Task Manager CLI ===

Commands:
  1 - Add task
  2 - List tasks
  3 - Complete task
  4 - Save & Exit

Enter your choice: 1
Enter task description: Learn Rust basics
✓ Task added successfully!

Commands:
  1 - Add task
  2 - List tasks
  3 - Complete task
  4 - Save & Exit

Enter your choice: 1
Enter task description: Build a CLI app
✓ Task added successfully!

Commands:
  1 - Add task
  2 - List tasks
  3 - Complete task
  4 - Save & Exit

Enter your choice: 2

=== Your Tasks ===
[ ] 1 - Learn Rust basics
[ ] 2 - Build a CLI app
==================

Commands:
  1 - Add task
  2 - List tasks
  3 - Complete task
  4 - Save & Exit

Enter your choice: 3
Enter task ID to complete: 1
✓ Task 1 marked as complete!

Commands:
  1 - Add task
  2 - List tasks
  3 - Complete task
  4 - Save & Exit

Enter your choice: 2

=== Your Tasks ===
[✓] 1 - Learn Rust basics
[ ] 2 - Build a CLI app
==================

Commands:
  1 - Add task
  2 - List tasks
  3 - Complete task
  4 - Save & Exit

Enter your choice: 4
✓ Tasks saved to tasks.txt
Goodbye!
```

### What This Code Does

1. **Struct Definitions**: Creates `Task` and `TaskManager` structures to organize data
2. **Methods**: Implements functionality for adding, listing, and completing tasks
3. **File I/O**: Saves tasks to a text file and loads them on startup for persistence
4. **User Input**: Reads commands from the user in an interactive loop
5. **Error Handling**: Uses Rust's `Result` type for safe file operations
6. **Memory Safety**: Demonstrates Rust's ownership system with no manual memory management

---

## 6. AI Prompt Journal

### Template for Documentation

**Prompt Used:**
```
[Space for you to write the exact prompt you used with the AI assistant]
```

**Link to Curriculum:**
```
[Space for relevant course materials, assignment guidelines, or learning objectives]
```

**AI Response Summary:**
```
[Space to summarize what the AI provided - was it code, explanation, debugging help?]
```

**Evaluation of Helpfulness:**
```
Rate: [1-5 stars]

What worked well:
- [Point 1]
- [Point 2]

What could be improved:
- [Point 1]
- [Point 2]

How I modified the AI's response:
- [Your changes and why you made them]
```

---

## 7. Common Issues & Fixes

### Issue 1: "cargo: command not found" or "rustc: command not found"

**Problem**: The Rust toolchain is not in your system's PATH.

**Solution**:
```bash
# Linux/macOS: Add to your shell profile
source "$HOME/.cargo/env"

# Or manually add to ~/.bashrc or ~/.zshrc:
export PATH="$HOME/.cargo/bin:$PATH"

# Windows: Restart your terminal or computer after installation
# The installer should add to PATH automatically
```

### Issue 2: Ownership and Borrowing Errors

**Problem**: Error messages like "cannot borrow as mutable" or "value moved here".

**Example Error**:
```rust
let s = String::from("hello");
let s2 = s; // s is moved to s2
println!("{}", s); // ERROR: s no longer valid
```

**Solution**: Understand Rust's ownership rules:
- Each value has one owner
- When ownership is transferred, the original variable becomes invalid
- Use references (`&`) to borrow without taking ownership

**Fixed Code**:
```rust
let s = String::from("hello");
let s2 = &s; // Borrow s instead of moving it
println!("{}", s); // OK: s is still valid
```

### Issue 3: Mismatched Types in match Statements

**Problem**: "expected type X, found type Y" in match arms.

**Example Error**:
```rust
let x = Some(5);
match x {
    Some(val) => val,      // Returns i32
    None => "nothing",     // Returns &str - ERROR!
}
```

**Solution**: All match arms must return the same type.

**Fixed Code**:
```rust
let x = Some(5);
let result = match x {
    Some(val) => val.to_string(),  // Convert to String
    None => "nothing".to_string(), // Both arms return String
};
```

### Issue 4: Lifetime Errors with References

**Problem**: "borrowed value does not live long enough".

**Example Error**:
```rust
fn get_first<'a>() -> &'a str {
    let s = String::from("hello");
    &s // ERROR: s is dropped at end of function
}
```

**Solution**: Return owned data instead of references, or ensure the data lives long enough.

**Fixed Code**:
```rust
fn get_first() -> String {
    let s = String::from("hello");
    s // Return owned String
}
```

### Issue 5: "cannot borrow as mutable more than once"

**Problem**: Trying to have multiple mutable references to the same data.

**Example Error**:
```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s; // ERROR: second mutable borrow
println!("{}, {}", r1, r2);
```

**Solution**: Rust allows only one mutable reference at a time to prevent data races.

**Fixed Code**:
```rust
let mut s = String::from("hello");
{
    let r1 = &mut s;
    println!("{}", r1);
} // r1 goes out of scope here

let r2 = &mut s; // OK: r1 is no longer in scope
println!("{}", r2);
```

### Issue 6: Package/Dependency Compilation Errors

**Problem**: Errors when compiling external crates.

**Solution**:
```bash
# Update Rust to the latest version
rustup update

# Clean build artifacts and rebuild
cargo clean
cargo build

# Update dependencies to latest compatible versions
cargo update
```

### Issue 7: "error: linker 'cc' not found" (Windows)

**Problem**: Missing C++ build tools on Windows.

**Solution**:
- Install Microsoft C++ Build Tools from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
- Select "Desktop development with C++" workload
- Restart your computer after installation

---

## 8. References

### Official Documentation
- **The Rust Programming Language (The Book)**: https://doc.rust-lang.org/book/
  - Comprehensive guide covering all Rust concepts
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
  - Learn Rust through annotated example programs
- **The Cargo Book**: https://doc.rust-lang.org/cargo/
  - Complete guide to Rust's package manager
- **Standard Library Documentation**: https://doc.rust-lang.org/std/
  - API reference for Rust's standard library

### Interactive Learning
- **Rustlings**: https://github.com/rust-lang/rustlings
  - Small exercises to practice reading and writing Rust code
- **Exercism Rust Track**: https://exercism.org/tracks/rust
  - Free coding exercises with mentorship

### Video Tutorials
- **Let's Get Rusty**: https://www.youtube.com/c/LetsGetRusty
  - Excellent YouTube channel for Rust beginners
- **Rust Crash Course by Traversy Media**: https://www.youtube.com/watch?v=zF34dRivLOw
  - Quick overview for getting started

### Community Resources
- **Rust Users Forum**: https://users.rust-lang.org/
  - Active community for asking questions
- **r/rust Subreddit**: https://www.reddit.com/r/rust/
  - News, articles, and discussions
- **Rust Discord**: https://discord.gg/rust-lang
  - Real-time chat with Rust developers

### Additional Tools
- **Rust Playground**: https://play.rust-lang.org/
  - Run Rust code in your browser without installing anything
- **Crates.io**: https://crates.io/
  - Official Rust package registry
- **Lib.rs**: https://lib.rs/
  - Alternative package search with better categorization

### Books (Beyond the Official Book)
- **Programming Rust** by Jim Blandy and Jason Orendorff (O'Reilly)
- **Rust in Action** by Tim McNamara (Manning)
- **Zero To Production In Rust** by Luca Palmieri

---

**Document Version**: 1.0  
**Last Updated**: November 2025  
**Created for**: AI Capstone Project