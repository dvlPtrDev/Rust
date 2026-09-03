use std::path::Path;
use std::fs;
use std::fs::File;


pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}
pub fn join_path(base: &str, path: &str) -> String {
    Path::new(base)
        .join(path)
        .to_string_lossy()
        .into_owned()
}

pub fn new_file(path: &str) -> Result<File, std::io::Error> {
    File::create(path)
}
pub fn delete_file(path: &str) {
    let _ = fs::remove_file(path);  
}
pub fn move_file(src: &str, dest: &str) {
    let _ = fs::rename(src, dest);
}
