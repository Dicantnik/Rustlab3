use std::fs::{OpenOptions, File};
use std::io::{BufRead, BufReader, Write};
use std::process;
use chrono::NaiveDate;

#[derive(Debug)]
struct User {
    id: u32,
    username: String,
    password: String,
}

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    date: String,
    content: String,
    user_id: u32,
    status: String,
}

fn register_user(user_file: &str) {
    let mut username = String::new();
    println!("Please enter your desired username: ");
    std::io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    if username_exists(user_file, username) {
        println!("Username already exists. Please enter another username.");
        return;
    }

    let mut password = String::new();
    println!("Please enter your password (min 8 characters): ");
    std::io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    if password.len() < 8 {
        println!("Password must be at least 8 characters long. Please try again.");
        return;
    }

    let mut confirm_password = String::new();
    println!("Please confirm your password: ");
    std::io::stdin().read_line(&mut confirm_password).unwrap();
    let confirm_password = confirm_password.trim();

    if password != confirm_password {
        println!("Passwords do not match. Please try again.");
        return;
    }

    let user_id = get_next_user_id(user_file);
    let new_user = User {
        id: user_id,
        username: username.to_string(),
        password: password.to_string(),
    };
    save_user(user_file, &new_user);
    println!("Registration successful! Please login to continue.");
}

fn username_exists(user_file: &str, username: &str) -> bool {
    let file = File::open(user_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<&str> = line.split(',').collect();
        if values.len() >= 2 && values[1] == username {
            return true;
        }
    }
    false
}

fn get_next_user_id(user_file: &str) -> u32 {
    let file = File::open(user_file).unwrap_or_else(|_| File::create(user_file).unwrap());
    let reader = BufReader::new(file);

    let mut max_id = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<&str> = line.split(',').collect();
        if let Ok(id) = values[0].parse::<u32>() {
            if id > max_id {
                max_id = id;
            }
        }
    }
    max_id + 1
}

fn save_user(user_file: &str, user: &User) {
    let mut file = OpenOptions::new().append(true).create(true).open(user_file).unwrap();
    writeln!(file, "{},{},{}", user.id, user.username, user.password).unwrap();
}

fn login_user(user_file: &str) -> Option<User> {
    let mut username = String::new();
    println!("Please enter your username: ");
    std::io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    let mut password = String::new();
    println!("Please enter your password: ");
    std::io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    let file = File::open(user_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<&str> = line.split(',').collect();
        if values.len() >= 3 && values[1] == username && values[2] == password {
            let user = User {
                id: values[0].parse::<u32>().unwrap(),
                username: values[1].to_string(),
                password: values[2].to_string(),
            };
            println!("Login successful! Welcome, {}.", username);
            return Some(user);
        }
    }

    println!("Invalid username or password. Please try again.");
    println!("Returning to main menu...");
    None
}

fn display_user_tasks(task_file: &str, user_id: u32) {
    let file = File::open(task_file).unwrap_or_else(|_| File::create(task_file).unwrap());
    let reader = BufReader::new(file);

    println!("Your incomplete tasks:");
    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<&str> = line.split(',').collect();
        if values.len() >= 5 && values[3].parse::<u32>().unwrap() == user_id && values[4] == "in progress" {
            println!("Task ID: {}, Date: {}, Content: {}", values[0], values[1], values[2]);
        }
    }
}

fn create_task(task_file: &str, user_id: u32) {
    let date = loop {
        let mut date = String::new();
        println!("Enter the task date (e.g., 2024-12-06): ");
        std::io::stdin().read_line(&mut date).unwrap();
        let date = date.trim().to_string();

        if NaiveDate::parse_from_str(&date, "%Y-%m-%d").is_ok() {
            break date;
        } else {
            println!("Invalid date format. Please enter the date in the format YYYY-MM-DD.");
        }
    };

    let mut content = String::new();
    println!("Enter the task content: ");
    std::io::stdin().read_line(&mut content).unwrap();
    let content = content.trim().to_string();

    let task_id = get_next_task_id(task_file);
    let new_task = Task {
        id: task_id,
        date,
        content,
        user_id,
        status: "in progress".to_string(),
    };
    save_task(task_file, &new_task);
    println!("Task created successfully!");
}

fn get_next_task_id(task_file: &str) -> u32 {
    let file = File::open(task_file).unwrap_or_else(|_| File::create(task_file).unwrap());
    let reader = BufReader::new(file);

    let mut max_id = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<&str> = line.split(',').collect();
        if let Ok(id) = values[0].parse::<u32>() {
            if id > max_id {
                max_id = id;
            }
        }
    }
    max_id + 1
}

fn save_task(task_file: &str, task: &Task) {
    let mut file = OpenOptions::new().append(true).create(true).open(task_file).unwrap();
    writeln!(file, "{},{},{},{},{}", task.id, task.date, task.content, task.user_id, task.status).unwrap();
}

fn delete_task(task_file: &str, user_id: u32) {
    let file = File::open(task_file).unwrap_or_else(|_| File::create(task_file).unwrap());
    let reader = BufReader::new(file);
    let mut tasks: Vec<Task> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<&str> = line.split(',').collect();
        if values.len() >= 5 {
            let task = Task {
                id: values[0].parse::<u32>().unwrap(),
                date: values[1].to_string(),
                content: values[2].to_string(),
                user_id: values[3].parse::<u32>().unwrap(),
                status: values[4].to_string(),
            };
            tasks.push(task);
        }
    }

    println!("Your tasks:");
    let user_tasks: Vec<&Task> = tasks.iter().filter(|task| task.user_id == user_id).collect();
    for task in &user_tasks {
        println!("Task ID: {}, Date: {}, Content: {}, Status: {}", task.id, task.date, task.content, task.status);
    }

    let mut task_id_str = String::new();
    println!("Enter the Task ID to delete: ");
    std::io::stdin().read_line(&mut task_id_str).unwrap();
    let task_id: u32 = match task_id_str.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid Task ID.");
            return;
        }
    };

    if !user_tasks.iter().any(|task| task.id == task_id) {
        println!("Task ID not found. Returning to menu...");
        return;
    }

    println!("Are you sure you want to delete Task ID {}? (1 for Yes, 2 for No): ", task_id);
    let mut confirm = String::new();
    std::io::stdin().read_line(&mut confirm).unwrap();
    let confirm = confirm.trim();

    if confirm == "1" {
        tasks.retain(|task| task.id != task_id || task.user_id != user_id);
        let mut file = File::create(task_file).unwrap();
        for task in &tasks {
            writeln!(file, "{},{},{},{},{}", task.id, task.date, task.content, task.user_id, task.status).unwrap();
        }
        println!("Task deleted successfully!");
    } else {
        println!("Task deletion cancelled.");
    }
}

fn update_task(task_file: &str, user_id: u32) {
    let file = File::open(task_file).unwrap_or_else(|_| File::create(task_file).unwrap());
    let reader = BufReader::new(file);
    let mut tasks: Vec<Task> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<&str> = line.split(',').collect();
        if values.len() >= 5 {
            let task = Task {
                id: values[0].parse::<u32>().unwrap(),
                date: values[1].to_string(),
                content: values[2].to_string(),
                user_id: values[3].parse::<u32>().unwrap(),
                status: values[4].to_string(),
            };
            tasks.push(task);
        }
    }

    println!("Your tasks:");
    let user_tasks: Vec<&Task> = tasks.iter().filter(|task| task.user_id == user_id).collect();
    for task in &user_tasks {
        println!("Task ID: {}, Date: {}, Content: {}, Status: {}", task.id, task.date, task.content, task.status);
    }

    let mut task_id_str = String::new();
    println!("Enter the Task ID to update: ");
    std::io::stdin().read_line(&mut task_id_str).unwrap();
    let task_id: u32 = match task_id_str.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid Task ID.");
            return;
        }
    };

    if !user_tasks.iter().any(|task| task.id == task_id) {
        println!("Task ID not found. Returning to menu...");
        return;
    }

    println!("What would you like to update? (1 for Date, 2 for Content): ");
    let mut update_choice = String::new();
    std::io::stdin().read_line(&mut update_choice).unwrap();
    let update_choice = update_choice.trim();

    let updated_task = tasks.iter_mut().find(|task| task.id == task_id && task.user_id == user_id).unwrap();

    match update_choice {
        "1" => {
            updated_task.date = loop {
                let mut new_date = String::new();
                println!("Enter the new date (e.g., 2024-12-06): ");
                std::io::stdin().read_line(&mut new_date).unwrap();
                let new_date = new_date.trim().to_string();

                if NaiveDate::parse_from_str(&new_date, "%Y-%m-%d").is_ok() {
                    break new_date;
                } else {
                    println!("Invalid date format. Please enter the date in the format YYYY-MM-DD.");
                }
            };
            println!("Task date updated successfully!");
        }
        "2" => {
            let mut new_content = String::new();
            println!("Enter the new content: ");
            std::io::stdin().read_line(&mut new_content).unwrap();
            let new_content = new_content.trim().to_string();

            updated_task.content = new_content;
            println!("Task content updated successfully!");
        }
        _ => {
            println!("Invalid choice. Returning to menu...");
            return;
        }
    }

    let mut file = File::create(task_file).unwrap();
    for task in &tasks {
        writeln!(file, "{},{},{},{},{}", task.id, task.date, task.content, task.user_id, task.status).unwrap();
    }
}

fn mark_task_as_completed(task_file: &str, user_id: u32) {
    let file = File::open(task_file).unwrap_or_else(|_| File::create(task_file).unwrap());
    let reader = BufReader::new(file);
    let mut tasks: Vec<Task> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<&str> = line.split(',').collect();
        if values.len() >= 5 {
            let task = Task {
                id: values[0].parse::<u32>().unwrap(),
                date: values[1].to_string(),
                content: values[2].to_string(),
                user_id: values[3].parse::<u32>().unwrap(),
                status: values[4].to_string(),
            };
            tasks.push(task);
        }
    }

    let user_tasks: Vec<&Task> = tasks.iter().filter(|task| task.user_id == user_id && task.status == "in progress").collect();
    if user_tasks.is_empty() {
        println!("No incomplete tasks found. Returning to menu...");
        return;
    }

    println!("Your incomplete tasks:");
    for task in &user_tasks {
        println!("Task ID: {}, Date: {}, Content: {}", task.id, task.date, task.content);
    }

    let mut task_id_str = String::new();
    println!("Enter the Task ID to mark as completed: ");
    std::io::stdin().read_line(&mut task_id_str).unwrap();
    let task_id: u32 = match task_id_str.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid Task ID. Returning to menu...");
            return;
        }
    };

    if let Some(task) = tasks.iter_mut().find(|task| task.id == task_id && task.user_id == user_id && task.status == "in progress") {
        task.status = "completed".to_string();
        println!("Task marked as completed!");
    } else {
        println!("Task ID not found or already completed. Returning to menu...");
        return;
    }

    let mut file = File::create(task_file).unwrap();
    for task in &tasks {
        writeln!(file, "{},{},{},{},{}", task.id, task.date, task.content, task.user_id, task.status).unwrap();
    }
}

fn load_tasks_from_file(task_file: &str, user_id: u32) {
    let file_path = "src/task_upload.csv";
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => {
            println!("Could not open the specified file. Returning to menu...");
            return;
        }
    };
    let reader = BufReader::new(file);

    let mut tasks: Vec<Task> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<&str> = line.split(',').collect();
        if values.len() >= 2 {
            let status = if values.len() > 2 { values[2].to_string() } else { "in progress".to_string() };
            let new_task = Task {
                id: get_next_task_id(task_file),
                date: values[0].to_string(),
                content: values[1].to_string(),
                user_id: user_id,
                status,
            };
            tasks.push(new_task);
        }
    }

    let mut file = OpenOptions::new().append(true).create(true).open(task_file).unwrap();
    for task in tasks {
        writeln!(file, "{},{},{},{},{}", get_next_task_id(task_file), task.date, task.content, task.user_id, task.status).unwrap();
    }
    println!("Tasks loaded successfully!");
}

fn main() {
    let user_file = "src/users.csv";
    let task_file = "src/tasks.csv";
    let mut current_user: Option<User> = None;

    loop {
        if current_user.is_none() {
            println!("Welcome to the Todo List Console Application!");
            println!("1. Register");
            println!("2. Login");
            println!("3. Exit");
            println!("Enter your choice: ");

            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();

            match choice {
                "1" => register_user(user_file),
                "2" => {
                    if let Some(user) = login_user(user_file) {
                        current_user = Some(user);
                    } else {
                        println!("Returning to main menu...");
                    }
                }
                "3" => {
                    println!("Exiting... Goodbye!");
                    process::exit(0);
                }
                _ => println!("Invalid choice, please try again."),
            }
        } else {
            let user = current_user.as_ref().unwrap();
            display_user_tasks(task_file, user.id);

            println!("Menu:");
            println!("1. Create Task");
            println!("2. Delete Task");
            println!("3. Load Tasks from File");
            println!("4. Update Task");
            println!("5. Mark Task as Completed");
            println!("6. Logout");
            println!("Enter your choice: ");

            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();

            match choice {
                "1" => create_task(task_file, user.id),
                "2" => delete_task(task_file, user.id),
                "3" => load_tasks_from_file(task_file, user.id),
                "4" => update_task(task_file, user.id),
                "5" => mark_task_as_completed(task_file, user.id),
                "6" => {
                    current_user = None;
                    println!("Logged out successfully.");
                }
                _ => println!("Functionality not implemented yet."),
            }
        }
    }
}

// Notes:
// This implementation adds user login functionality and displays incomplete tasks after successful login.
// Additional features such as task creation, deletion, loading tasks from file, updating tasks, and marking tasks as completed will be implemented in the next parts.
// Each step will handle the required CSV files appropriately to ensure user data is properly managed.
