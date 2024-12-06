use clap::{Command, Arg};
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions, read_dir};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use dirs;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Clone)]
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

    fn add_linked_files(&mut self, files: Vec<String>) {
        if let Some(linked_files) = &mut self.linked_files {
            linked_files.extend(files);
        } else {
            self.linked_files = Some(files);
        }
    }

    fn mark_completed(&mut self) {
        self.status = "Completed".to_string();
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
        return Vec::new();
    }

    let mut file = File::open(path).expect("Unable to open tasks file!");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read tasks file!");

    serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
}

fn save_tasks_file(tasks: Vec<Task>) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tasks.json")
        .expect("Unable to open tasks file!");
    
    serde_json::to_writer(file, &tasks).expect("Unable to write tasks to file!");
}

fn link_files_to_task(task_name: &str) -> Vec<String> {
    let mut linked_files = Vec::new();
    
    let home_dir = dirs::home_dir().expect("Unable to find home directory!");
    let dir_path = WalkDir::new(home_dir.join("Documents"));
    for entry in dir_path.into_iter().filter_map(Result::ok) {
        let file_name = entry.file_name().to_string_lossy().to_string();
        if file_name.starts_with(&format!("[{}]", task_name)) {
            linked_files.push(entry.path().to_string_lossy().into_owned());
        }
    }

    linked_files
}

fn update_task(task_description: &str) {
    let mut tasks = load_tasks();
    if let Some(task) = tasks.iter_mut().find(|t| t.description == task_description) {
        let linked_files = link_files_to_task(task_description);
        task.add_linked_files(linked_files);
        save_tasks_file(tasks);
        println!("Task '{}' updated with linked files!", task_description);
    } else {
        println!("Task not found!");
    }
}

fn mark_task_completed(task_description: &str) {
    let mut tasks = load_tasks();
    if let Some(task) = tasks.iter_mut().find(|t| t.description == task_description) {
        task.mark_completed();
        save_tasks_file(tasks);
        println!("Task '{}' marked as completed!", task_description);
    } else {
        println!("Task not found!");
    }
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
                .help("Create a new task with a description"),
        )
        .arg(
            Arg::new("update")
                .short('u')
                .long("update")
                .help("Update a task with linked files based on task name"),
        )
        .arg(
            Arg::new("complete")
                .short('m')
                .long("mark-complete")
                .help("Mark a task as complete"),
        )
        .get_matches();

    if let Some(task_description) = matches.get_one::<String>("create") {
        let task = Task::new(task_description.to_string());
        add_task(task);
    } else if let Some(task_description) = matches.get_one::<String>("update") {
        update_task(task_description);
    } else if let Some(task_description) = matches.get_one::<String>("complete") {
        mark_task_completed(task_description);
    } else {
        println!("No valid command provided. Use --help for options.");
    }
}
