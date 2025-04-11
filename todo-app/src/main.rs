use colored::*;
use std::io;

const COMPLETED_STRING: &str = "[✓]";
const NOT_COMPLETED_STRING: &str = "[]";

enum Choice {
    Add,
    List,
    Delete,
    MarkAsDone,
}

#[derive(Debug, Clone)]
struct Task {
    description: String,
    is_completed: bool,
}
struct TodoApp {
    items: Vec<Task>,
}

impl TodoApp {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn handle_choice(&mut self, choice: Choice) {
        match choice {
            Choice::Add => Self::add_item(self),
            Choice::List => Self::list_items(self),
            Choice::Delete => Self::delete_item(self),
            Choice::MarkAsDone => Self::mark_as_done(self),
        }
    }

    fn add_item(&mut self) {
        clear_screen();

        println!("{}", "Please enter description".blue());
        let name: String = get_name();

        self.items.push(Task {
            description: name,
            is_completed: false,
        });
        self.list_items();
    }

    fn delete_item(&mut self) {
        clear_screen();

        if self.items.is_empty() {
            println!("Please add item first");
            return;
        };

        println!("{}", "Please enter index of task".blue());

        if let Some(index) = self.get_item_index() {
            let removed_task = self.items.remove(index);
            println!("Deleted task: {:?}", removed_task);
        }
    }

    fn list_items(&mut self) {
        println!("\n{}", "=== YOUR TASKS ===");

        if self.items.is_empty() {
            println!("No tasks available")
        } else {
            for (index, task) in self.items.iter().enumerate() {
                let status = if task.is_completed {
                    COMPLETED_STRING
                } else {
                    NOT_COMPLETED_STRING
                };

                println!("{}: {} {}", index + 1, status, task.description);
            }
        }

        println!("==================\n");
    }

    fn mark_as_done(&mut self) {
        clear_screen();
        self.list_items();

        println!("{}", "Please enter index of task".blue());

        if let Some(index) = self.get_item_index() {
            let mut updated_task = self.items[index].clone();

            updated_task.description.red().dimmed();
            updated_task.is_completed = true;

            self.items[index] = updated_task;

            println!("Task marked as done!");
        }
    }

    fn get_item_index(&self) -> Option<usize> {
        let input = get_name();
        return match input.trim().parse::<usize>() {
            Ok(num) if num > 0 && num <= self.items.len() => Some(num - 1),
            _ => {
                println!("{}", "Please enter a valid index".red());
                return None;
            }
        };
    }
}

impl Choice {
    fn from_u8(num: u8) -> Option<Choice> {
        match num {
            1 => Some(Choice::List),
            2 => Some(Choice::Add),
            3 => Some(Choice::Delete),
            4 => Some(Choice::MarkAsDone),
            _ => None,
        }
    }
}

fn get_name() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read name");

    return input.trim().to_string();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H"); // ANSI escape code to clear screen
}

fn main() {
    println!("{}\n\n\n", "Welcome to Todo App!!".yellow());

    let mut todo_app = TodoApp::new();

    loop {
        println!("Please enter a choice: \n1.List\t2.Add\t3.Delete\t4.Mark as done\t5.Exit\t\n");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read command");

        let choice: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please enter a number between 1 and 4");
                return;
            }
        };

        if choice == 5 {
            println!("Exiting todo App. Goodbye!");
            break;
        }

        match Choice::from_u8(choice) {
            Some(valid_choice) => todo_app.handle_choice(valid_choice),
            None => println!("Invalid choice. Please enter a valid choice"),
        }
    }
}
