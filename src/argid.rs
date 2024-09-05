use std::fmt::Display;

use crate::args;
use crate::task;
use crate::TaskFile;
use args::ArgTaskID;

enum FindType {
    Name(String),
    Id(usize),
}
pub struct FindError {
    find_type: FindType,
}
impl From<FindType> for FindError {
    fn from(value: FindType) -> Self {
        Self { find_type: value }
    }
}
impl Display for FindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.find_type {
            FindType::Id(id) => {
                write!(f, "Coldn't find id: {}", id)
            }
            FindType::Name(name) => {
                write!(f, "Coldn't find name: \"{}\"", name)
            }
        }
    }
}

//Args
pub fn find_tasks_by_argid(
    task_file: &TaskFile,
    argid: &ArgTaskID,
) -> Result<Vec<usize>, FindError> {
    let mut found_tasks: Vec<usize>;
    println!("argid:{:?}", argid);
    if argid.all {
        found_tasks = (0..task_file.tasks.len()).collect();
        return Ok(found_tasks);
    } else {
        found_tasks = Vec::new();
        //name
        for name in &argid.name {
            match find_task_by_name(&task_file, name) {
                Some(id) => found_tasks.push(id),
                None => {
                    return Err(FindType::Name(name.clone()).into());
                }
            }
        }
        //id
        let len = task_file.tasks.len();
        for id in &argid.id {
            if *id < len {
                found_tasks.push(*id);
            } else {
                return Err(FindType::Id(*id).into());
            }
        }
        println!("found {:?}", &found_tasks);
        return Ok(found_tasks);
    }
}
pub fn find_folders_by_argid(
    task_file: &TaskFile,
    argid: &ArgTaskID,
) -> Result<Vec<usize>, FindError> {
    let mut found_folders: Vec<usize>;
    if argid.all {
        found_folders = (0..task_file.folders.len()).collect();
        return Ok(found_folders);
    } else {
        found_folders = Vec::with_capacity(argid.id.len() + argid.name.len());
        //name
        for name in &argid.name {
            match find_folder_by_name(&task_file, name) {
                Some(id) => found_folders.push(id),
                None => {
                    return Err(FindType::Name(name.clone()).into());
                }
            }
        }
        //id
        let len = task_file.folders.len();
        for id in &argid.id {
            if *id < len {
                found_folders.push(*id);
            }
        }
        return Ok(found_folders);
    }
}
fn find_task_by_name(task_file: &TaskFile, name: &str) -> Option<usize> {
    let filter: Vec<usize> = task_file
        .tasks
        .iter()
        .enumerate()
        .filter(|(i, task)| task.name.find(name).is_some())
        .map(|(i, _)| i)
        .collect();
    //if multiple then error and tell user
    if filter.len() > 1 {
        eprintln!("Error. Found multiple matches for \"{}\"", name);
        for i in filter {
            eprintln!("{} {}", i, task_file.tasks[i]);
        }
        return None;
    } else if filter.len() == 0 {
        eprintln!("Error. Zero matches found for \"{}\"", name);
        return None;
    }
    Some(filter[0])
}

fn find_folder_by_name<'tasks>(task_file: &'tasks TaskFile, name: &str) -> Option<usize> {
    let filter: Vec<usize> = task_file
        .folders
        .iter()
        .enumerate()
        .filter(|(_, task)| task.name.find(name).is_some())
        .map(|(i, _)| i)
        .collect();
    //if multiple then error and tell user
    if filter.len() > 1 {
        eprintln!("Error. Found multiple matches for \"{}\"", name);
        for i in filter {
            eprintln!("{} {}", i, task_file.folders[i]);
        }
        return None;
    } else if filter.len() == 0 {
        println!("Error. Zero matches found for \"{}\"", name);
        return None;
    }
    Some(filter[0])
}
