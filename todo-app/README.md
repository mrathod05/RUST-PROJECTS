# Todo App (Rust)

This is a simple command-line Todo application written in Rust. It allows you to manage your tasks by adding, listing, deleting, and marking them as completed.

## Features

* **Add Tasks:** Easily add new tasks to your todo list.
* **List Tasks:** View all your tasks with their completion status.
* **Delete Tasks:** Remove tasks from your list.
* **Mark Tasks as Done:** Mark tasks as completed.
* **Clear Screen:** Uses ANSI escape codes to clear the terminal for a cleaner interface.
* **Colored Output:** Uses the `colored` crate to provide colored output for better readability.

## Prerequisites

* Rust and Cargo installed on your system. You can install them from [rustup.rs](https://rustup.rs/).

## How to Run

1.  **Clone the Repository (or copy the code into a file):**

    If you cloned the repository, navigate to the project directory. If you copied the code into a file, save it as `main.rs` and make sure you have a `Cargo.toml` file in the same directory. The `Cargo.toml` file should look like this:

    ```toml
    [package]
    name = "todo-app"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    colored = "3.0.0"

    ```

2.  **Build the Project:**

    Open a terminal in the project directory and run:

    ```bash
    cargo build --release
    ```

3.  **Run the Application:**

    Run the built executable:

    ```bash
    cargo run --release
    ```

## Usage

Once the application is running, you will be presented with a menu:


```text
Please enter a choice:
1. List   2. Add   3. Delete   4. Mark as done   5. Exit
```

* **1. List:** Lists all the tasks in your to-do list.
* **2. Add:** This prompts you to enter a description for a new task.
* **3. Delete:** This prompts you to enter the task index you want to delete.
* **4. Mark as done:** This prompts you to enter the task index you want to mark as completed.
* **5. Exit:** Exits the application.

## Dependencies

* `colored`: Used for colored output in the terminal.

## Code Structure

* `main.rs`: Contains the main application logic, including the `TodoApp` struct and its methods, as well as the main function.
* `Choice` enum: Represents the different choices available in the menu.
* `Task` struct: Represents a single task with its description and completion status.
* `TodoApp` struct: Manages the collection of tasks.
* `get_name()`: Helper function to read input from the user.
* `clear_screen()`: Helper function to clear the terminal screen.
