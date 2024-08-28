mod args;
mod dir;
pub mod task;

use args::{ArgTaskID, Commands, FolderCommands, CLI};
use clap::Parser;
use dir::get_data_dir;
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::Read, process::ExitCode};

const FILE_VER: u32 = 1;
#[derive(Debug, Serialize, Deserialize)]
struct TaskFile {
    version: u32,
    tasks: Vec<task::Task>,
    folders: Vec<task::Folder>,
}
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

fn find_task_by_name<'tasks>(
    tasks: &'tasks TaskFile,
    name: &str,
) -> Option<(usize, &'tasks task::Task)> {
    let filter: Vec<(usize, &task::Task)> = tasks
        .tasks
        .iter()
        .enumerate()
        .filter(|(_, task)| task.name.find(name).is_some())
        .collect();
    //if multiple then error and tell user
    if filter.len() > 1 {
        println!("Error. Found multiple matches for \"{}\"", name);
        for (i, task) in filter {
            println!("{} {}", i, task);
        }
        return None;
    } else if filter.len() == 0 {
        println!("Error. Zero matches found for \"{}\"", name);
        return None;
    }
    Some(filter[0])
}
fn find_folder_by_name<'tasks>(
    tasks: &'tasks TaskFile,
    name: &str,
) -> Option<(usize, &'tasks task::Folder)> {
    let filter: Vec<(usize, &task::Folder)> = tasks
        .folders
        .iter()
        .enumerate()
        .filter(|(_, task)| task.name.find(name).is_some())
        .collect();
    //if multiple then error and tell user
    if filter.len() > 1 {
        println!("Error. Found multiple matches for \"{}\"", name);
        for (i, task) in filter {
            println!("{} {}", i, task);
        }
        return None;
    } else if filter.len() == 0 {
        println!("Error. Zero matches found for \"{}\"", name);
        return None;
    }
    Some(filter[0])
}

fn get_tasks_by_id(tasks: &TaskFile, id: &ArgTaskID) -> Option<Vec<usize>> {
    if let Some(id) = id.id {
        Some(vec![(id)])
    } else if let Some(name) = &id.name {
        find_task_by_name(&tasks, name).map(|(i, _)| vec![i])
    } else if id.all {
        Some((0..tasks.tasks.len()).into_iter().collect())
    } else {
        unimplemented!()
    }
}

fn main() -> ExitCode {
    let mut tasks = get_tasks();
    let cli = CLI::parse();
    match &cli.commands {
        Commands::Complete(args) => {
            let tasks_result = match get_tasks_by_id(&tasks, &args.id) {
                Some(x) => x,
                None => return ExitCode::FAILURE,
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
            let task = task::Task::new(args.name.clone(), None);
            tasks.tasks.push(task);
            println!("Added \"{}\"", args.name);
        }
        Commands::Remove(args) => {
            if let Some(id) = args.id.id {
                let len = tasks.tasks.len();
                if id >= len {
                    println!("ID too large/not in list");
                    return ExitCode::FAILURE;
                }
                println!("Removed \"{}\"", tasks.tasks[id]);
                tasks.tasks.remove(id);
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
            } else if let Some(name) = &args.id.name {
                match find_task_by_name(&tasks, name) {
                    Some((id, _)) => {
                        println!("Removed \"{}\"", tasks.tasks[id]);
                        tasks.tasks.remove(id);
                    }
                    None => {
                        return ExitCode::FAILURE;
                    }
                }
            } else if args.id.all {
                println!("Removed all");
                tasks.tasks.clear();
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
                    if let Some(id) = args.id.id {
                        let len = tasks.folders.len();
                        if id >= len {
                            println!("ID too large/not in list");
                            return ExitCode::FAILURE;
                        }
                        println!("Removed \"{}\"", tasks.folders[id]);
                        tasks.folders.remove(id);
                    } else if let Some(name) = &args.id.name {
                        match find_folder_by_name(&tasks, name) {
                            Some((id, _)) => {
                                println!("Removed \"{}\"", tasks.folders[id]);
                                tasks.folders.remove(id);
                            }
                            None => {
                                return ExitCode::FAILURE;
                            }
                        }
                    } else if args.id.all {
                        println!("Removed all");
                        tasks.folders.clear();
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
                    if let Some(id) = args.id.id {
                        println!("Set desc of [{}] to \"{}\"", id, args.desc);
                        tasks.folders[id].desc = args.desc.clone();
                    } else if let Some(name) = &args.id.name {
                        match find_folder_by_name(&tasks, name) {
                            Some((id, _)) => {
                                println!("Set desc of [{}] to \"{}\"", id, args.desc);
                                tasks.folders[id].desc = args.desc.clone();
                            }
                            None => {
                                return ExitCode::FAILURE;
                            }
                        }
                    } else if args.id.all {
                        println!("Set desc of all to \"{}\"", args.desc);
                        tasks.folders.clear();
                    }
                }
                FolderCommands::Task(args) => {
                    let tasks_result = match get_tasks_by_id(&tasks, &args.task) {
                        Some(x) => x,
                        None => return ExitCode::FAILURE,
                    };
                    let id = args.folder;
                    for index in tasks_result {
                        //check if already in
                        if tasks.folders[id].tasks.contains(&index) {
                            println!("Folder already contains that task");
                            return ExitCode::FAILURE;
                        }
                        println!("Added task to folder [{}]", id);
                        tasks.folders[id].tasks.push(index);
                    }
                }
                FolderCommands::TaskRemove(args) => {
                    let tasks_result = match get_tasks_by_id(&tasks, &args.task) {
                        Some(x) => x,
                        None => return ExitCode::FAILURE,
                    };
                    let id = args.folder;
                    for index in tasks_result {
                        //check if already in
                        if !tasks.folders[id].tasks.contains(&index) {
                            println!("Folder doesn't contain this task");
                            return ExitCode::FAILURE;
                        }
                        println!("Removed task from folder [{}]", id);
                        tasks.folders[id].tasks.remove(index);
                    }
                }
            }
        }
    }
    set_tasks(&tasks);
    ExitCode::SUCCESS
}
