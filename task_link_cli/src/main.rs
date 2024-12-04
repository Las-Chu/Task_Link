use clap::{Command, Arg};
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct Task {
    description: String,
    status: String,
    dependencies: Option<Vec<String>>,
    linked_files: Option<Vec<String>>,
    linked_urls: Option<Vec<String>>,
}

impl Task {
    fn new(description: String) -> Self {
        Task {
            description,
            status: "Initialized".to_string(),
            dependencies: None,
            linked_files: None,
            linked_urls: None,
        }
    }
}

fn add_task(task: Task) {
    let mut tasks = load_tasks();
    tasks.push(task.clone()); 
    save_tasks_file(tasks);
    println!("Task created: {}", task.description); 
}


fn load_tasks() -> Vec<Task> {
    let path = Path::new("tasks.json");
    if !path.exists() {
        return Vec::new(); // doesn't exist!
    }

    let mut file = File::open(path).expect("Unable to open tasks file.");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read tasks file.");

    serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()) // parse json to get tasks
}

fn save_tasks_file(tasks: Vec<Task>) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tasks.json")
        .expect("Unable to open tasks file.");
    
    serde_json::to_writer(file, &tasks).expect("Unable to write tasks to file.");
}

fn main() {
    let matches = Command::new("tasklink")
        .version("1.0")
        .author("Laasya Chukka")
        .about("A basic task management CLI tool")
        .arg(
            Arg::new("create")
                .short('c')
                .long("create")
                .help("Create a new task"),
        )
        .get_matches();

    if let Some(task_description) = matches.get_one::<String>("create") {
        let task = Task::new(task_description.to_string());
        add_task(task);
    } else {
        println!("Please add a task description!");
    }
}
