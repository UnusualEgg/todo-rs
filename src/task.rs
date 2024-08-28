use std::{
    fmt::Display,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    done: bool,
    pub name: String,
    path: Option<PathBuf>,
}
impl Task {
    pub fn new(name: String, path: Option<PathBuf>) -> Self {
        Task {
            done: false,
            name,
            path,
        }
    }
}
impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{}]{}",
            if self.done { 'X' } else { ' ' },
            self.name
        ))
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub name: String,
    pub desc: String,
    pub tasks: Vec<ID>,
    //color:
}
impl Folder {
    pub fn new(name: String, description: String) -> Self {
        Folder {
            name,
            desc: description,
            tasks: Vec::new(),
        }
    }
}
impl Display for Folder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}: {}",
            self.name, self.desc
        ))
    }
}
pub type ID = usize;
// #[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
// pub struct FolderID(pub ID);
// #[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
// pub struct TaskID(pub ID);
// static ID_COUNT: AtomicUsize = AtomicUsize::new(0);
// fn get_id() -> usize {
//     ID_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
// }
// static ID_FOLDER_COUNT: AtomicUsize = AtomicUsize::new(0);
// fn get_folder_id() -> usize {
//     ID_FOLDER_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
// }
