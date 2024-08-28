use std::path::PathBuf;

pub fn get_data_dir() -> PathBuf {
    let data_dir = dirs::data_dir().unwrap();
    let dir = data_dir.join("todo");
    // println!("dir {:?}",dir);
    dir
}
pub fn get_data_file() -> PathBuf {
    let data_file = get_data_dir().join("todo.yml");
    // println!("data_file: {:?}",data_file);
    data_file
}
