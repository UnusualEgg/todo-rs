mod argid;
mod args;
mod dir;
pub mod task;

use args::{Commands, FolderCommands, CLI};
use clap::Parser;
use dir::get_data_dir;
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::Read, process::ExitCode};

const FILE_VER: u32 = 1;
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskFile {
    version: u32,
    pub tasks: Vec<task::Task>,
    pub folders: Vec<task::Folder>,
}

//TaskFile
fn get_tasks() -> TaskFile {
    let f = dir::get_data_file();
    if !f.exists() {
        return TaskFile {
            version: FILE_VER,
            tasks: Vec::new(),
            folders: Vec::new(),
        };
    }
    let mut data_file = OpenOptions::new().read(true).write(false).open(f).unwrap();
    let mut buf = String::new();
    data_file.read_to_string(&mut buf).unwrap();
    serde_yml::from_str(&buf).unwrap()
}
fn set_tasks(tasks: &TaskFile) {
    let data_dir = get_data_dir();
    if !data_dir.exists() {
        std::fs::create_dir(data_dir).unwrap();
    }
    let buf = serde_yml::to_string(tasks).unwrap();
    std::fs::write(dir::get_data_file(), buf).unwrap()
}

fn main() -> ExitCode {
    let mut tasks = get_tasks();
    let cli = CLI::parse();
    match &cli.commands {
        Commands::Rename(args) => {
            let tasks_result = match argid::find_tasks_by_argid(&tasks, &args.id) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::FAILURE;
                }
            };
            for index in tasks_result {
                tasks.tasks[index].name = args.new.clone();
            }
        }
        Commands::Complete(args) => {
            let tasks_result = match argid::find_tasks_by_argid(&tasks, &args.id) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::FAILURE;
                }
            };
            for index in tasks_result {
                tasks.tasks[index].done = true;
            }
        }
        Commands::List => {
            for (i, task) in tasks.tasks.iter().enumerate() {
                println!("[{}] {}:", i, task);
            }
        }
        Commands::Add(args) => {
            let path = if args.path {
                match std::env::current_dir() {
                    Ok(p) => Some(p),
                    Err(e) => {
                        println!("{}", e);
                        return ExitCode::FAILURE;
                    }
                }
            } else {
                None
            };
            let task = task::Task::new(args.name.clone(), path);
            tasks.tasks.push(task);
            println!("Added \"{}\"", args.name);
        }
        Commands::Remove(args) => {
            let tasks_result = match argid::find_tasks_by_argid(&tasks, &args.id) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::FAILURE;
                }
            };
            let mut removed = Vec::new();
            //-1 for every index in hedr that's below current
            for id in tasks_result {
                let mut down = 0;
                for rem in &removed {
                    if *rem < id {
                        down += 1;
                    }
                }
                let id = id - down;
                println!("Removed \"{}\"", tasks.tasks[id]);
                tasks.tasks.remove(id);
                removed.push(id);
                for folder in tasks.folders.iter_mut() {
                    folder.tasks.sort();
                    if let Ok(index) = folder.tasks.binary_search(&id) {
                        folder.tasks.remove(index);
                    }
                    folder.tasks.iter_mut().for_each(|index| {
                        if *index > id {
                            (*index) -= 1
                        }
                    });
                }
            }
        }

        Commands::Folder(cmd) => {
            match cmd {
                FolderCommands::Add(args) => {
                    let folder = task::Folder::new(args.name.clone(), args.desc.clone());
                    tasks.folders.push(folder);
                    println!("Added \"{}\"", args.name);
                }
                FolderCommands::Remove(args) => {
                    let tasks_result = match argid::find_folders_by_argid(&tasks, &args.id) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("{}", e);
                            return ExitCode::FAILURE;
                        }
                    };
                    let mut removed = Vec::new();
                    //-1 for every index in hedr that's below current
                    for id in tasks_result {
                        let mut down = 0;
                        for rem in &removed {
                            if *rem < id {
                                down += 1;
                            }
                        }
                        let id = id - down;
                        println!("Removed \"{}\"", tasks.folders[id]);
                        tasks.folders.remove(id);
                        removed.push(id);
                    }
                }
                FolderCommands::List(args) => {
                    for (id, folder) in tasks.folders.iter().enumerate() {
                        println!("[{}] {}:", id, folder.name);
                        if args.desc {
                            println!("({})", folder.desc);
                        }
                        if args.tasks {
                            for index in folder.tasks.iter() {
                                println!("\t[{}] {}", index, tasks.tasks[*index]);
                            }
                        }
                    }
                }
                FolderCommands::SetDesc(args) => {
                    let tasks_result = match argid::find_folders_by_argid(&tasks, &args.id) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("{}", e);
                            return ExitCode::FAILURE;
                        }
                    };
                    for id in tasks_result {
                        println!("Set desc of [{}] to \"{}\"", id, args.desc);
                        tasks.folders[id].desc = args.desc.clone();
                    }
                }
                FolderCommands::Task(args) => {
                    let tasks_result = match argid::find_tasks_by_argid(&tasks, &args.task) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("{}", e);
                            return ExitCode::FAILURE;
                        }
                    };
                    let id = args.folder;
                    for index in tasks_result {
                        //check if already in
                        if tasks.folders[id].tasks.contains(&index) {
                            eprintln!("Folder already contains that task");
                            return ExitCode::FAILURE;
                        }
                        println!("Added task to folder [{}]", id);
                        tasks.folders[id].tasks.push(index);
                    }
                }
                FolderCommands::TaskRemove(args) => {
                    let tasks_result = match argid::find_folders_by_argid(&tasks, &args.task) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("{}", e);
                            return ExitCode::FAILURE;
                        }
                    };
                    let id = args.folder;
                    let mut removed: Vec<usize> = Vec::new();
                    //-1 for every index in hedr that's below current
                    for index in tasks_result {
                        println!("Removed \"{}\"", tasks.folders[id].tasks[index]);
                        let mut down = 0;
                        for rem in &removed {
                            if *rem < id {
                                down -= 1;
                            }
                        }
                        let id = id - down;
                        tasks.folders[id].tasks.remove(index);
                        removed.push(index);
                    }
                }
            }
        }
    }
    set_tasks(&tasks);
    ExitCode::SUCCESS
}
