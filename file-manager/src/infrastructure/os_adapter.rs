use std::path::PathBuf;
use std::process;
use std::env;


pub fn get_user_home() -> Option<PathBuf> {
    env::home_dir()
}
pub fn terminate(code: i32) -> ! {
    process::exit(code)
}
